use {
    anchor_lang::{
        solana_program::{instruction::Instruction, rent::Rent},
        system_program, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_pubkey::Pubkey,
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_deposit() {
    let program_id = vault::id();
    let payer = Keypair::new();
    let payer_address = payer.pubkey();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/vault.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    let (vault_pda, _vault_bump) =
        Pubkey::find_program_address(&[vault::VAULT_SEED, payer.pubkey().as_ref()], &program_id);

    let (vault_state_pda, _state_bump) = Pubkey::find_program_address(
        &[vault::VAULT_STATE_SEED, payer.pubkey().as_ref()],
        &program_id,
    );

    let instruction = Instruction::new_with_bytes(
        program_id,
        &vault::instruction::Initialize { max_withdraw: None }.data(),
        vault::accounts::Initialize {
            owner: payer.pubkey(),
            vault: vault_pda,
            vault_authority: vault_state_pda,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();

    let res = svm.send_transaction(tx);

    assert!(res.is_ok());

    let deposit_lamports = 5_000_000_000;
    let deposit_instruction = Instruction::new_with_bytes(
        program_id,
        &vault::instruction::Deposit {
            lamports_to_transfer: deposit_lamports,
        }
        .data(),
        vault::accounts::Deposit {
            owner: payer_address,
            vault: vault_pda,
            vault_authority: vault_state_pda,
            system_porgram: system_program::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let deposit_msg =
        Message::new_with_blockhash(&[deposit_instruction], Some(&payer.pubkey()), &blockhash);
    let deposit_tx =
        VersionedTransaction::try_new(VersionedMessage::Legacy(deposit_msg), &[&payer]).unwrap();

    let deposit_res = svm.send_transaction(deposit_tx);
    assert!(
        deposit_res.is_ok(),
        "deposit failed with errL {:?}",
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
