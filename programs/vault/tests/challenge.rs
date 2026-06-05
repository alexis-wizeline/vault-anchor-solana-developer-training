use anchor_lang::solana_program::rent::Rent;
use solana_signer::Signer;

mod utils;

#[test]
fn test_challenge() {
    let program_id = vault::id();
    let (payer, mut svm) = utils::setup();

    let pdas = utils::derive_addresses(&payer.pubkey());
    let vault_pda = pdas.vault_address;
    let vault_state_pda = pdas.vault_state_address;

    let tx = utils::Transaction {
        program_id,
        vault_pda,
        vault_state_pda,
    };

    let res = tx.run_initialize(&mut svm, &payer, Some(5_000_000_000));
    assert!(res.is_ok());

    let rent: Rent = svm.get_sysvar();
    let rent_exepmt_lamports = rent.minimum_balance(0);

    match svm.get_account(&vault_pda) {
        Some(acc) => {
            assert_eq!(
                rent_exepmt_lamports, acc.lamports,
                "expect vault to have lamports to be rent exempt"
            );
        }
        None => panic!("expect the avault to be initialized"),
    }

    let lamports_to_deposit = 15_000_000_000;
    let deposit_res = tx.run_deposit(&mut svm, &payer, lamports_to_deposit);
    assert!(deposit_res.is_ok());
    match svm.get_account(&vault_pda) {
        Some(acc) => {
            assert_eq!(
                rent_exepmt_lamports + lamports_to_deposit,
                acc.lamports,
                "expect vault to have lamports to be rent exempt + deposited lamports"
            );
        }
        None => panic!("expect the avault to be initialized"),
    }

    let lamports_to_withdraw = 5_000_000_000;
    let valid_withdraw_res = tx.run_withdraw(&mut svm, &payer, lamports_to_withdraw);
    assert!(valid_withdraw_res.is_ok());
    match svm.get_account(&vault_pda) {
        Some(acc) => {
            assert_eq!(
                rent_exepmt_lamports + (lamports_to_deposit - lamports_to_withdraw),
                acc.lamports,
                "expect vault to have lamports to be rent exempt + deposited lamports - withdrawed lamports"
            );
        }
        None => panic!("expect the avault to be initialized"),
    }

    let invalid_withdraw_res = tx.run_withdraw(&mut svm, &payer, 6_000_000_000);
    assert!(invalid_withdraw_res.is_err());
    match svm.get_account(&vault_pda) {
        Some(acc) => {
            assert_eq!(
                rent_exepmt_lamports + (lamports_to_deposit - lamports_to_withdraw),
                acc.lamports,
                "expect vault to have lamports to not change since last withdraw"
            );
        }
        None => panic!("expect the avault to be initialized"),
    }

    let close_res = tx.run_close(&mut svm, &payer);
    assert!(close_res.is_ok());

    match svm.get_account(&vault_pda) {
        Some(_) => panic!("expect the account to be close"),
        None => {}
    };
}
