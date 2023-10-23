#[path = "./abi/CoinFlip.rs"]
mod CoinFlip;

mod helpers;
mod pb;

use pb::schema::{Approval, Approvals, Transfer, Transfers, StateChange};
use substreams::log::println;
use substreams::pb::substreams::Clock;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::pb::eth::v2::{StorageChange , Call};
use substreams_ethereum::{pb::eth, Event};
use substreams::scalar::{BigDecimal, BigInt};

use helpers::*;

pub const ADDRESS: &str = "0xb4aFb4a1dF99C2333DDC57Ec33E57D26E87E78E4";
const START_BLOCK: u64 = 9844317;

#[substreams::handlers::map]
fn map_state_changes(block: eth::v2::Block) -> Result<StateChange, substreams::errors::Error> {
    let state_changes = block
        .calls()
        .filter_map(|tx| {
            //format_hex(&transaction.call.address) == ADDRESS.to_lowercase() &&
            if  tx.call.storage_changes.len() > 0 && format_hex(&tx.call.address) == ADDRESS.to_lowercase(){
                let mut state_change_data: Vec<(String, String)> = vec![];
                for item in &tx.call.storage_changes  {
                    let state_variable = match  BigInt::from_unsigned_bytes_be(&item.key).to_string().as_str() {
                        "6" => String::from("min_bet"),
                        "7" => String::from("max_profit"),
                        "15" => String::from("totat_wei_won"),
                        "16" => String::from("total_wei_lost"),
                        "17" => String::from("contract_balance"),
                        _ => {
                            break;
                        }
                    };
                    StateChange {
                        state_variable, 
                        old_value: BigInt::from_unsigned_bytes_be(&item.new_value).to_string(),
                        new_value:  BigInt::from_unsigned_bytes_be(&item.new_value).to_string(),
                    };
                    let data = BigInt::from_unsigned_bytes_be(&item.new_value).to_string();
                    let slot = BigInt::from_unsigned_bytes_be(&item.key).to_string();
                    //println(format!("CALL ADDRESS {}", format_hex(&tx.call.address)));
                    //println(format!("DATA IS: {:?} \n SLOT IS {:?}", &data, &slot));
                    state_change_data.push((slot, data));
                } 
                Some(state_change_data)       
            }    
              else {
                None
            }
        })
        .collect::<Vec<_>>();
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

    Ok(StateChange { ..Default::default() })
}

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
