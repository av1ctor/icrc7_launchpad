use ic_cdk_macros::update;
use crate::{
    icrc37_types::{ApprovalInfo, ApproveTokenArg}, 
    state::STATE, 
    BurnArg, BurnResult, MintArg, MintResult, TransferArg, TransferResult
};

#[update]
pub fn icrc7_transfer(args: Vec<TransferArg>) -> Vec<Option<TransferResult>> {
    let caller = ic_cdk::caller();
    STATE.with(|s| s.borrow_mut().icrc7_transfer(&caller, args))
}

#[update]
pub fn mint_with_approval(arg: MintArg) -> MintResult {
    let minting_authority = STATE.with_borrow(|s| 
        s.minting_authority.unwrap()
    );

    let owner = arg.to.owner.clone();
    
    // 1st: try to mint (only the minting authority is allowed)
    let token_id = STATE.with_borrow_mut(|s| { 
        s.mint(&ic_cdk::caller(), arg)
    })?;

    // 2nd: approve transfers to be done by the minting authority
    STATE.with_borrow_mut(|s| {
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

        s.approve(&owner, args)
    });

    Ok(token_id)
}

#[update]
pub fn burn(args: Vec<BurnArg>) -> Vec<Option<BurnResult>> {
    let caller = ic_cdk::caller();
    STATE.with(|s| s.borrow_mut().burn(&caller, args))
}
