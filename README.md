# Coin Flip Substream. Tracking variables by storage changes.

A substream tracking state changes throughout a coin flip gambling smart contract. This substream will track the contract balance, minimum bet, maximum allowable profit, total wei won, and total wei lost throughout the contracts lifetime.

## Contract
CoinFlip
0xb4aFb4a1dF99C2333DDC57Ec33E57D26E87E78E4

## Goal of this substream
	
To track state variables of a smart contract by listening to the storage changes on the storage slot associated with that variable. 

## Modules Used
1. map_state_changes
2. store_state_changes
3. db_out

## map_state_changes

``` rust
fn map_state_changes(block: eth::v2::Block) -> Result<StateChanges, substreams::errors::Error>
```

I used the map_state_changes module grab all the storage changes for the contract. It takes in an ethereum block and returns a result type with the happy path being my StateChanges struct which contains a vector of individual StateChange structs.

### Storage layout of smart contracts
Before getting into the details of this module I want to quickly explain how smart contract storage slots work. Each storage slot in a smart contract takes up 32 bytes. In simple terms, most variables, like a number or a small piece of data, fit into one of these 32-byte slots. However, larger or more complex variables, such as strings or arrays, may span across multiple slots due to their size. 

The order of these slots is determined by the sequence in which variables are declared in the contract. It starts with the first variable you declare. If a contract inherits from other contracts, it gets a bit more interesting. The inherited state variables are allocated to slots before the variables declared in the child contract. This means if Contract B inherits from Contract A, Contract A's variables are slotted first, followed by Contract B's own variables.

You can also pack multiple smaller variables into one slot to save storage space. However these variables need to be declared next to each other in the contract in order to do so. Otherwise they will take up an entire storage slot even if the data is smaller than 32 bytes. This is a smart way to optimize storage and cut costs. It's especially handy when you have several small variables because you can make the most of the 32 bytes in each slot without wasting space.

### Back to the Module 
 First I defined an empty mutable vector name state_changes and then filtered through the calls and grab all of the calls that were to the contract address and that included storage changes.
``` rust
let mut state_changes = Vec::new()

for tx in block.calls() {
	if tx.call.storage_changes.len() > 0
	&& format_hex(&tx.call.address) == ADDRESS.to_lowercase()
	{
```

Next, Inside the if block I looped through each individual storage change for the call and matched the storage slot to the corresponding variable name and then Instantiated my StateChange struct with the variable name, old value, and new value from the storage change, and pushed the StateChange to the state_changes vector. 

``` rust
for item in &tx.call.storage_changes {
	let state_variable = match BigInt::from_unsigned_bytes_be(&item.key)
	.to_string()
	.as_str()
{
	"6" => String::from("min_bet"),
	"7" => String::from("max_profit"),
	"15" => String::from("total_wei_won"),
	"16" => String::from("total_wei_lost"),
	"17" => String::from("contract_balance"),
	_ => {
		continue;
	}
};

state_changes.push(StateChange {
	state_variable,
	old_value: BigInt::from_unsigned_bytes_be(&item.old_value).to_string(),
	new_value: BigInt::from_unsigned_bytes_be(&item.new_value).to_string(),
});

}
```

Finally I instantiate and return my StateChanges struct which contains my vector of state_changes.

``` rust
		}
	}
	Ok(StateChanges { state_changes })
}
```

## store_state_changes

``` rust
fn store_state_changes(statechanges: StateChanges, s: StoreSetProto<StateChange>) {
```

I used my store_state_changes module to store what the current variable value is on a particular block. For arguments it takes in my StateChanges struct and also the StoreSetProto struct with the protobuff type set to StateChange.

First I defined a mutable HashMap named key_counters to keep track of how many times a variable is changed during a block.

``` rust
let mut key_counters = HashMap::new();
```

Next I looped over the state_changes vector inside of StateChanges and define the variable current_key and set it to the current variable name.

I then define a varable named counter and set it to `key_counters.entry(current_key.clone()).or_insert(0);`

This uses the entry method on key_counters HashMap to either access or create an entry for the current_key depending on if it exists or not. The `.or_insert(0)` creates a new entry for the key and sets it to 0 if it does not already exist. 

After defining counter I immediately increment the counter and define a variable named ordinal that I set to the value of the counter. 

Finally I use the set method from StoreSetProto to set my store with the ordinal, variable name, and the StateChange. The ordinal ensures that even when multiple storage changes happen to the same variable within the same block they will all be stored with the correct values.

## db_out

``` rust
fn db_out(
clock: Clock,
state_changes: StateChanges,
store_deltas: Deltas<DeltaProto<StateChange>>,
) -> Result<DatabaseChanges, substreams::errors::Error> {
```

The final module that is used is the db_out module. This module was used to output my substream data into a postgres database.  As arguments it takes in the clock (to get the block number) , StateChanges (The data from my map module), and Deltas<DeltaProto><StateChange>> (Data from my store module). It returns a result type with DatabaseChanges being the happy path.

The first thing I did is define my local variables. I defined a mutable variable name tables and set it to a new Instance of the Tables struct, and then id_1 and id_2 where I set both to 1. The purpose of the Id variables is to help give each primary key a unique value in the table. 

``` rust
let mut tables = Tables::new();
let mut id_1 = 1;
let mut id_2 = 1;
```

Next I created my rows for my state_changes table that will contain the data from my map_state_changes module. I set the primary key to be a formatted string containing the variable name, id_1 number, and the current block ensuring it will be unique for every entry. I then set my variable_name, old_value, and new_value fields for the row with the corresponding data from the StateChange struct and the block_number from the clock.  Once a row is set the id_1 is incremented before the next row is set.

``` rust
for state_change in state_changes.state_changes {

tables

.create_row(

"state_changes",

format!("{}-{}-{}", state_change.state_variable, id_1, clock.number),

)

.set("variable_name", state_change.state_variable)

.set("old_value", state_change.old_value)

.set("new_value", state_change.new_value)

.set("block_number", clock.number);

id_1 += 1;

}
```
I then created my rows for my variable_tracking table that will contain the data from my store module. This time i set the primary key to be a formatted string with the variable name, ordinal, id_2, and block number. Like the previous table I set the rest of the values in the row to the corresponding data in the store module and the block number and then increment the id_2 variable before the next row is set. 

``` rust
for delta in store_deltas.deltas {

tables

.create_row(

"variable_tracking",

format!("{}-{}-{}-{}", delta.key, delta.ordinal, id_2, clock.number),

)

.set("variable_name", delta.new_value.state_variable)

.set("old_value", delta.new_value.old_value)

.set("new_value", delta.new_value.new_value)

.set("block_number", clock.number);

id_2 += 1;

}
```
Finally I returned my tables as DatabaseChanges.

``` rust
Ok(tables.to_database_changes())

}
```

