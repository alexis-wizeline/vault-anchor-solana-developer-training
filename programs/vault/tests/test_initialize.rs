mod utils;

use {
    anchor_lang::{solana_program::rent::Rent, AccountDeserialize},
    solana_signer::Signer,
};

#[test]
fn test_initialize() {
    let program_id = vault::id();
    let (payer, mut svm) = utils::setup();
    let payer_address = payer.pubkey();

    let pdas = utils::derive_addresses(&payer_address);
    let vault_pda = pdas.vault_address;
    let vault_state_pda = pdas.vault_state_address;

    let tx = utils::Transaction {
        program_id,
        vault_pda,
        vault_state_pda,
    };

    let res = tx.run_intialize(&mut svm, &payer, None);

    assert!(res.is_ok());

    let vault_state_account = match svm.get_account(&vault_state_pda) {
        Some(acc) => acc,
        None => panic!("expect vault state to exist"),
    };

    let vault_state =
        match vault::VaultState::try_deserialize(&mut vault_state_account.data.as_slice()) {
            Ok(state) => state,
            Err(err) => panic!("unable to deserialize data: {}", err),
        };

    assert_eq!(
        vault_state.bump, pdas.vault_state_bump,
        "vault state bump assertion failed:  expect: {}, got: {}",
        pdas.vault_state_bump, vault_state.bump
    );

    assert_eq!(
        vault_state.vault_bump, pdas.vault_bump,
        "vault bump assertion failed:  expect: {}, got: {}",
        pdas.vault_bump, vault_state.vault_bump
    );

    assert_eq!(
        vault_state.owner, payer_address,
        "expect address to be the payer"
    );

    let rent: Rent = svm.get_sysvar();
    match svm.get_balance(&vault_pda) {
        Some(balance) => {
            assert_eq!(
                balance,
                rent.minimum_balance(0),
                "initialized with the wrong ammount"
            )
        }
        None => panic!("expect balance to be present for the vault account"),
    };
}
