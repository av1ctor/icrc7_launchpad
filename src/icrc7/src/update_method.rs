use candid::Principal;
use ic_cdk_macros::update;
use crate::{
    errors::TransferFromError, 
    guards::authenticated_guard, 
    icrc37_types::{
        ApprovalInfo, ApproveTokenArg, 
        TransferFromArg, TransferFromResult
    }, 
    state::STATE, 
    BurnArg, BurnResult, MintArg, MintResult, 
    TransferArg, TransferResult
};

#[update(guard = "authenticated_guard")]
pub fn icrc7_transfer(
    args: Vec<TransferArg>
) -> Vec<Option<TransferResult>> {
    let caller = ic_cdk::caller();
    STATE.with(|s| s.borrow_mut().icrc7_transfer(&caller, args))
}

#[update(guard = "authenticated_guard")]
pub fn mint_and_grant_transfer_approval(
    arg: MintArg
) -> MintResult {
    let caller = ic_cdk::caller();
    let owner = arg.to.owner.clone();
    
    // 1st: try to mint (only the minting authority is allowed)
    let (tx_id, token_id) = STATE.with_borrow_mut(|s| { 
        s.mint(&caller, arg)
    })?;

    // 2nd: allow the minting authority to do transfers of this token
    grant_minting_authority_transfer_approval(
        &owner, 
        vec![token_id]
    );

    Ok((tx_id, token_id))
}

#[update(guard = "authenticated_guard")]
pub fn transfer_from_and_grant_transfer_approval(
    arg: TransferFromArg
) -> TransferFromResult {
    let caller = ic_cdk::caller();
    let owner = arg.from.owner.clone();

    // 1st: do the transfer to the new account
    let res = STATE.with_borrow_mut(|s| {
        s.transfer_from(&caller, vec![arg.clone()])
    });

    let tx_id = match &res[0] {
        Some(res) => {
            match res {
                Ok(tx_id) => {
                    *tx_id
                },
                Err(msg) => {
                    return Err(msg.clone());
                },
            }
        },
        None => {
            return Err(TransferFromError::NonExistingTokenId)
        },
    };

    // 2nd: allow the minting authority to do transfers of this token
    grant_minting_authority_transfer_approval(
        &owner,
        vec![arg.token_id]
    );

    Ok(tx_id)
}

#[update(guard = "authenticated_guard")]
pub fn burn(
    args: Vec<BurnArg>
) -> Vec<Option<BurnResult>> {
    let caller = ic_cdk::caller();
    STATE.with(|s| s.borrow_mut().burn(&caller, args))
}

fn grant_minting_authority_transfer_approval(
    owner: &Principal,
    token_ids: Vec<u128>
) {
    let minting_authority = STATE.with_borrow(|s| 
        s.minting_authority.unwrap()
    );
    
    STATE.with_borrow_mut(|s| {
        let args = token_ids.iter().map(|token_id|
            ApproveTokenArg { 
                token_id: *token_id, 
                approval_info: ApprovalInfo::new(
                    None, 
                    minting_authority, 
                    None, 
                    None, 
                    Some(ic_cdk::api::time())
                )
            }
        ).collect();

        s.approve(owner, args)
    });
}