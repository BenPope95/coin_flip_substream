#[path = "./abi/CoinFlip.rs"]
mod CoinFlip;

mod helpers;
mod pb;

use pb::schema::{Approval, Approvals, StateChange, StateChanges, Transfer, Transfers};
use substreams::log::println;
use substreams::pb::substreams::Clock;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{
    DeltaProto, Deltas, StoreGet, StoreGetProto, StoreNew, StoreSet, StoreSetProto,
};

use std::collections::HashMap;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::{Call, StorageChange};
use substreams_ethereum::{pb::eth, Event};

use helpers::*;

pub const ADDRESS: &str = "0xb4aFb4a1dF99C2333DDC57Ec33E57D26E87E78E4";

#[substreams::handlers::map]
fn map_state_changes(block: eth::v2::Block) -> Result<StateChanges, substreams::errors::Error> {
    let mut state_changes = Vec::new();

    for tx in block.calls() {
        if tx.call.storage_changes.len() > 0
            && format_hex(&tx.call.address) == ADDRESS.to_lowercase()
        {
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
        }
    }
    Ok(StateChanges { state_changes })
}

#[substreams::handlers::store]
fn store_state_changes(statechanges: StateChanges, s: StoreSetProto<StateChange>) {
    let mut key_counters = HashMap::new();

    for item in statechanges.state_changes {
        let current_key = &item.state_variable;

        let counter = key_counters.entry(current_key.clone()).or_insert(0);
        *counter += 1;

        let ordinal = *counter;

        s.set(ordinal, current_key, &item);
    }
}

#[substreams::handlers::map]
fn map_stores(
    store: StoreGetProto<StateChange>,
) -> Result<StateChanges, substreams::errors::Error> {
    let mut state_changes = Vec::new();

    if let Some(value) = store.get_at(1, "total_wei_lost") {
        state_changes.push(value);
    }

    if let Some(value) = store.get_at(2, "total_wei_lost") {
        state_changes.push(value);
    }

    if let Some(value) = store.get_at(3, "total_wei_lost") {
        state_changes.push(value);
    }

    Ok(StateChanges { state_changes })
}
#[substreams::handlers::map]
fn db_out(
    clock: Clock,
    state_changes: StateChanges,
    store_deltas: Deltas<DeltaProto<StateChange>>,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    let mut id_1 = 1;
    let mut id_2 = 1;

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

    Ok(tables.to_database_changes())
}
