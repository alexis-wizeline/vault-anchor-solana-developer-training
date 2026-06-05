use anchor_lang::{system_program, InstructionData, ToAccountMetas};
use litesvm::{types::TransactionResult, LiteSVM};
use solana_keypair::Keypair;
use solana_message::{Instruction, Message, VersionedMessage};
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;

pub fn setup() -> (Keypair, LiteSVM) {
    let program_id = vault::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../../target/deploy/vault.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 20_000_000_000).unwrap();
    (payer, svm)
}

#[allow(dead_code)]
pub struct DerivedAddresses {
    pub vault_address: Pubkey,
    pub vault_bump: u8,

    pub vault_state_address: Pubkey,
    pub vault_state_bump: u8,
}

pub fn derive_addresses(owner_key: &Pubkey) -> DerivedAddresses {
    let program_id = vault::id();
    let (vault_pda, vault_bump) =
        Pubkey::find_program_address(&[vault::VAULT_SEED, owner_key.as_ref()], &program_id);

    let (vault_state_pda, state_bump) =
        Pubkey::find_program_address(&[vault::VAULT_STATE_SEED, owner_key.as_ref()], &program_id);

    DerivedAddresses {
        vault_address: vault_pda,
        vault_bump: vault_bump,
        vault_state_address: vault_state_pda,
        vault_state_bump: state_bump,
    }
}

pub struct Transaction {
    pub program_id: Pubkey,
    pub vault_pda: Pubkey,
    pub vault_state_pda: Pubkey,
}

#[allow(dead_code)]
impl Transaction {
    pub fn run_intialize(
        &self,
        svm: &mut LiteSVM,
        payer: &Keypair,
        max_withdraw: Option<u64>,
    ) -> TransactionResult {
        let instruction = Instruction::new_with_bytes(
            self.program_id,
            &vault::instruction::Initialize { max_withdraw }.data(),
            vault::accounts::Initialize {
                owner: payer.pubkey(),
                vault: self.vault_pda,
                vault_authority: self.vault_state_pda,
                system_program: system_program::ID,
            }
            .to_account_metas(None),
        );

        let blockhash = svm.latest_blockhash();
        let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
        let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();
        svm.send_transaction(tx)
    }

    pub fn run_deposit(
        &self,
        svm: &mut LiteSVM,
        payer: &Keypair,
        deposit_lamports: u64,
    ) -> TransactionResult {
        let instruction = Instruction::new_with_bytes(
            self.program_id,
            &vault::instruction::Deposit {
                lamports_to_transfer: deposit_lamports,
            }
            .data(),
            vault::accounts::Deposit {
                owner: payer.pubkey(),
                vault: self.vault_pda,
                vault_authority: self.vault_state_pda,
                system_porgram: system_program::ID,
            }
            .to_account_metas(None),
        );

        let blockhash = svm.latest_blockhash();
        let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
        let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();
        svm.send_transaction(tx)
    }

    pub fn run_withdraw(
        &self,
        svm: &mut LiteSVM,
        payer: &Keypair,
        amount_to_withdraw: u64,
    ) -> TransactionResult {
        let instruction = Instruction::new_with_bytes(
            self.program_id,
            &vault::instruction::Withdraw { amount_to_withdraw }.data(),
            vault::accounts::Withdraw {
                owner: payer.pubkey(),
                vault: self.vault_pda,
                vault_authority: self.vault_state_pda,
                system_porgram: system_program::ID,
            }
            .to_account_metas(None),
        );
        let blockhash = &svm.latest_blockhash();
        let message =
            Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
        let tx =
            VersionedTransaction::try_new(VersionedMessage::Legacy(message), &[payer]).unwrap();
        svm.send_transaction(tx)
    }

    pub fn run_close(&self, svm: &mut LiteSVM, payer: &Keypair) -> TransactionResult {
        let instruction = Instruction::new_with_bytes(
            self.program_id,
            &vault::instruction::Close {}.data(),
            vault::accounts::Close {
                owner: payer.pubkey(),
                vault_authority: self.vault_state_pda,
                vault: self.vault_pda,
                system_program: system_program::ID,
            }
            .to_account_metas(None),
        );
        let blockhash = &svm.latest_blockhash();
        let message =
            Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
        let tx =
            VersionedTransaction::try_new(VersionedMessage::Legacy(message), &[payer]).unwrap();
        svm.send_transaction(tx)
    }
}
