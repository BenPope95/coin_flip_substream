#[path = "./abi/CoinFlip.rs"]
mod CoinFlip;

mod helpers;
mod pb;

use pb::schema::{Approval, Approvals, StateChange, StateChanges, Transfer, Transfers};
use substreams::log::println;
use substreams::pb::substreams::Clock;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{StoreGet, StoreGetProto, StoreNew, StoreSet, StoreSetProto, DeltaProto, Deltas};
//use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::pb::eth::v2::{Call, StorageChange};
use substreams_ethereum::{pb::eth, Event};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;

use helpers::*;

pub const ADDRESS: &str = "0xb4aFb4a1dF99C2333DDC57Ec33E57D26E87E78E4";
const START_BLOCK: u64 = 9844317;

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
    let  mut ordinal = 1;
    let mut keys_in_vec = Vec::new();
    for item in statechanges.state_changes {
        let current_key = &item.state_variable;
        let mut key_already_exists = false;

        for existing_key in &keys_in_vec {
            if current_key == existing_key {
                key_already_exists = true;
                break;
            }
        }

        if key_already_exists {
            // Key is already in the array, increment ordinal
            ordinal += 1;
        }
        s.set(ordinal, current_key, &item);
        keys_in_vec.push(current_key.clone());
    }
}

#[substreams::handlers::map]
fn map_stores(
    store: StoreGetProto<StateChange>,
) -> Result<StateChanges, substreams::errors::Error> {
    // let keys = [
    //     "min_bet",
    //     "max_profit",
    //     "total_wei_won",
    //     "total_wei_lost",
    //     "contract_balance",
    // ];

    let mut state_changes = Vec::new();
    // let mut ordinal = 1;
    // let mut keys_in_vec = Vec::new();

    
    if let Some(value) = store.get_at(1, "total_wei_lost") {
        state_changes.push(value);
    }

    if let Some(value) = store.get_at(2, "total_wei_lost") {
        state_changes.push(value);
    }

    if let Some(value) = store.get_at(3, "total_wei_lost") {
        state_changes.push(value);
    }



    // for key in keys {
    //     if keys_in_vec.contains(&key.to_string()) {
    //         // Key is already in the array, increment ordinal
    //         ordinal += 1;
    //     }
    //     if let Some(value) = store.get_at(ordinal, key) {
    //         state_changes.push(value);
    //     }
    //     keys_in_vec.push(key.to_string());
    // }

    Ok(StateChanges { state_changes })
}

fn db_out(
    clock: substreams::pb::substreams::Clock,
    state_changes: StateChanges,
    store_deltas: Deltas<DeltaProto<StateChange>>,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    for state_change in state_changes.state_changes {
        tables
            .create_row(
                "state_change",
              &state_change.state_variable,
            )
            .set("state_variable", state_change.state_variable)
            .set("old_value", state_change.old_value)
            .set("new_value", state_change.new_value);
            
    }

    for delta in store_deltas.deltas {
        tables
            .create_row("variable_name", delta.key)
            .set("variable_name", delta.new_value.state_variable)
            .set("old_value", delta.new_value.old_value)
            .set("new_value", delta.new_value.new_value)
            .set("block_number", clock.number);
    }

    Ok(tables.to_database_changes())
}

// .map(|state_change_data| {
//     match state_change_data {

//     }
//     StateChange {
//     from: format_hex(&transfer.from),
//     to: format_hex(&transfer.to),
//     token_id: transfer.token_id.to_string(),
//     tx_hash: hash,
// }})
// .collect::<Vec<Transfer>>();

// fn map_transfers(block: eth::v2::Block) -> Result<Transfers, substreams::errors::Error> {
//     let transfers = block
//         .logs()
//         .filter_map(|log| {
//             if format_hex(log.address()) == ADDRESS.to_lowercase() {
//                 if let Some(transfer) = TransferEvent::match_and_decode(log) {
//                     Some((transfer, format_hex(&log.receipt.transaction.hash)))
//                 } else {
//                     None
//                 }
//             } else {
//                 None
//             }
//         })
//         .map(|(transfer, hash)| Transfer {
//             from: format_hex(&transfer.from),
//             to: format_hex(&transfer.to),
//             token_id: transfer.token_id.to_string(),
//             tx_hash: hash,
//         })
//         .collect::<Vec<Transfer>>();

//     Ok(Transfers { transfers })
// }

// #[substreams::handlers::map]
// fn map_approvals(block: eth::v2::Block) -> Result<Approvals, substreams::errors::Error> {
//     let approvals = block
//         .logs()
//         .filter_map(|log| {
//             if format_hex(log.address()) == ADDRESS.to_lowercase() {
//                 if let Some(approval) = ApprovalEvent::match_and_decode(log) {
//                     Some((approval, format_hex(&log.receipt.transaction.hash)))
//                 } else {
//                     None
//                 }
//             } else {
//                 None
//             }
//         })
//         .map(|(approval, hash)| Approval {
//             owner: format_hex(&approval.owner),
//             approved: format_hex(&approval.approved),
//             token_id: approval.token_id.to_string(),
//             tx_hash: hash,
//         })
//         .collect::<Vec<Approval>>();

//     Ok(Approvals { approvals })
// }

// #[substreams::handlers::map]
// pub fn graph_out(
//     clock: Clock,
//     transfers: Transfers,
//     approvals: Approvals,
// ) -> Result<EntityChanges, substreams::errors::Error> {
//     let mut tables = Tables::new();

//     if clock.number == START_BLOCK {
//         // Create the collection, we only need to do this once
//         tables.create_row("Collection", ADDRESS.to_string());
//     }

//     transfers_to_table_changes(&mut tables, &transfers);

//     approvals_to_table_changes(&mut tables, &approvals);

//     Ok(tables.to_entity_changes())
// }
