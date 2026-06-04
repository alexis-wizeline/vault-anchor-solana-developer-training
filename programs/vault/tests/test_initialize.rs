use {
    anchor_lang::{
        solana_program::{instruction::Instruction, rent::Rent},
        system_program, AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_pubkey::Pubkey,
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_initialize() {
    let program_id = vault::id();
    let payer = Keypair::new();
    let payer_address = payer.pubkey();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/vault.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 20_000_000_000).unwrap();

    let (vault_pda, vault_bump) =
        Pubkey::find_program_address(&[vault::VAULT_SEED, payer.pubkey().as_ref()], &program_id);

    let (vault_state_pda, state_bump) = Pubkey::find_program_address(
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
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();

    let res = svm.send_transaction(tx);

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
        vault_state.bump, state_bump,
        "vault state bump assertion failed:  expect: {}, got: {}",
        state_bump, vault_state.bump
    );

    assert_eq!(
        vault_state.vault_bump, vault_bump,
        "vault bump assertion failed:  expect: {}, got: {}",
        vault_bump, vault_state.vault_bump
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
