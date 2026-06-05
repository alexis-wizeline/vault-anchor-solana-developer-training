mod utils;

use {anchor_lang::solana_program::rent::Rent, solana_signer::Signer};

#[test]
fn test_deposit() {
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

    let deposit_lamports = 5_000_000_000;
    let deposit_res = tx.run_deposit(&mut svm, &payer, deposit_lamports);
    assert!(
        deposit_res.is_ok(),
        "deposit failed with err: {:?}",
        deposit_res.err()
    );

    let rent: Rent = svm.get_sysvar();
    let rent_exempt_lamports = rent.minimum_balance(0);

    match svm.get_balance(&vault_pda) {
        Some(balance) => assert_eq!(
            balance,
            rent_exempt_lamports + deposit_lamports,
            "vault balance mismatch, expect: {}, got: {}",
            rent_exempt_lamports + deposit_lamports,
            balance
        ),
        None => panic!("expect vault account to exist."),
    }
}
