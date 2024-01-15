#[path = "./abi/CoinFlip.rs"]
mod CoinFlip;

mod helpers;
mod pb;

use pb::schema::{Approval, Approvals, StateChange, StateChanges, Transfer, Transfers};
use substreams::log::println;
use substreams::pb::substreams::Clock;
use substreams::scalar::{BigDecimal, BigInt};
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
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
                substreams::log::info!(
                    "{:?}",
                    BigInt::from_unsigned_bytes_be(&item.key).to_string()
                );
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

#[substreams::handlers::map]
fn graph_out(
    clock: Clock,
    state_changes: StateChanges,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    let mut id = 1;
    // let mut id_2 = 1;

    for state_change in state_changes.state_changes {
        tables
            .create_row(
                "StateChange",
                format!("{}-{}-{}", state_change.state_variable, id, clock.number),
            )
            .set("variableName", state_change.state_variable)
            .set("oldValue", state_change.old_value)
            .set("newValue", state_change.new_value)
            .set("blockNumber", clock.number);
        id += 1;
    }

    Ok(tables.to_entity_changes())
}
