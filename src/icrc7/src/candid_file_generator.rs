use candid::export_service;
use candid::{Nat, Principal};
use ic_cdk_macros::query;
use icrc_ledger_types::{icrc1::account::Account, icrc3::blocks::DataCertificate};
use crate::cycles::WalletReceiveResult;
use icrc7_types::{icrc3_types::*, icrc7_types::*, icrc37_types::*};

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::current_dir().unwrap());
        write(dir.join("icrc7.did"), export_candid()).expect("Write failed.");
    }
}
