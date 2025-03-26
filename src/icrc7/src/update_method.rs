use candid::Principal;
use ic_cdk_macros::update;

use crate::{
    guards::owner_guard, icrc37_types::{ApprovalInfo, ApproveTokenArg}, state::STATE, utils::principal_to_subaccount, 
    BurnArg, BurnResult, MintArg, MintResult, TransferArg, TransferResult
};
use icrc_ledger_types::icrc1::account::Account;

#[update]
pub fn icrc7_transfer(args: Vec<TransferArg>) -> Vec<Option<TransferResult>> {
    let caller = ic_cdk::caller();
    STATE.with(|s| s.borrow_mut().icrc7_transfer(&caller, args))
}

#[update]
pub fn mint(arg: MintArg) -> MintResult {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err(crate::errors::MintError::GenericBatchError {
            error_code: 100,
            message: "Anonymous Identity".into(),
        });
    }
    STATE.with(|s| s.borrow_mut().mint(&caller, arg))
}

#[update]
pub fn mint_with_approval(arg: MintArg) -> MintResult {
    let minting_authority = Account { 
        owner: STATE.with_borrow(|s| s.minting_authority.unwrap().owner), 
        // use the owner's principal as subaccount
        subaccount: Some(principal_to_subaccount(&arg.to.owner))
    };
    
    // 1st: try to mint (only the minting authority is allowed)
    let token_id = mint(arg)?;

    // 2nd: approve transfers done by the minting authority
    let args = vec![ApproveTokenArg { 
        token_id, 
        approval_info: ApprovalInfo::new(
            None, 
            minting_authority, 
            None, 
            None, 
            Some(ic_cdk::api::time())
        )
    }];

    STATE.with(|s| s.borrow_mut().approve(&ic_cdk::caller(), args));

    Ok(token_id)
}

#[update]
pub fn burn(args: Vec<BurnArg>) -> Vec<Option<BurnResult>> {
    let caller = ic_cdk::caller();
    STATE.with(|s| s.borrow_mut().burn(&caller, args))
}

#[update(guard = "owner_guard")]
pub fn set_minting_authority(minting_account: Account) -> bool {
    STATE.with(|s| s.borrow_mut().minting_authority = Some(minting_account));
    return true;
}
