use crate::{
    pb::schema::{Approvals, Transfers},
    ADDRESS,
};
use substreams::Hex;

pub fn format_hex(address: &[u8]) -> String {
    format!("0x{}", Hex(address).to_string())
}
