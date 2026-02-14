use anyhow::{Result, anyhow};
use blockchain_core::{
    accounts::event::Event,
    instructions::{CloseEventArgs, CreateEventArgs, FakeCancelOrderArgs, FakeCreateOrderArgs, FakeMatchOrderArgs, MarketInstruction},
};
use serde_json::{Value, json};
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{CommitmentConfig, RpcSendTransactionConfig},
};
use solana_sdk::{
    message::{AccountMeta, Instruction, Message},
    native_token::LAMPORTS_PER_SOL,
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::{EncodableKey, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token::state::Account as TokenAccount;
use uuid::Uuid;

pub const DEFAULT_RPC_HTTP: &str = "http://127.0.0.1:8899";
pub const USDC_MINT: Pubkey = solana_sdk::pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const MARKETPLACE_PROGRAM: Pubkey =
    solana_sdk::pubkey!("ProirMXDTFF4AEqGyZVKPhWte4chANDd1c4Y8w7Nsd4");
pub const SYSTEM_PROGRAM: Pubkey = solana_sdk::pubkey!("11111111111111111111111111111111");
pub const SKIP_PREFLIGHT: bool = false;

pub struct ProfeciaClient {
    pub rpc_client: RpcClient,
    pub rpc_url: String,
    pub rpc_config: RpcSendTransactionConfig,
    pub admin_wallet: Keypair,
}

impl ProfeciaClient {
    pub fn new_with_wallet(admin_wallet: Keypair, rpc_url: &str) -> Self {
        let rpc_client =
            RpcClient::new_with_commitment(rpc_url.into(), CommitmentConfig::confirmed());

        let rpc_config = RpcSendTransactionConfig {
            skip_preflight: SKIP_PREFLIGHT,
            preflight_commitment: None,
            encoding: None,
            max_retries: None,
            min_context_slot: None,
        };

        Self {
            rpc_client,
            rpc_url: rpc_url.into(),
            rpc_config,
            admin_wallet,
        }
    }

    /// like new() but creates the wallet just for testing
    pub async fn new_debug(rpc_url: &str) -> Result<Self> {
        let admin_wallet = Keypair::new();

        let rpc_client =
            RpcClient::new_with_commitment(rpc_url.into(), CommitmentConfig::confirmed());

        let rpc_config = RpcSendTransactionConfig {
            skip_preflight: SKIP_PREFLIGHT,
            preflight_commitment: None,
            encoding: None,
            max_retries: None,
            min_context_slot: None,
        };

        let _sig = rpc_client
            .request_airdrop(&admin_wallet.pubkey(), LAMPORTS_PER_SOL * 10)
            .await?;

        Ok(Self {
            rpc_client,
            rpc_url: rpc_url.into(),
            rpc_config,
            admin_wallet,
        })
    }

    pub fn new(rpc_url: &str) -> Result<Self> {
        let wallet_path = "../blockchain_program/PRIVATE_KEY/id.json";

        let admin_wallet = Keypair::read_from_file(wallet_path).map_err(|e| anyhow!("Error creating admin wallet: {}", e.to_string()))?;

        let rpc_client =
            RpcClient::new_with_commitment(rpc_url.into(), CommitmentConfig::confirmed());

        let rpc_config = RpcSendTransactionConfig {
            skip_preflight: SKIP_PREFLIGHT,
            preflight_commitment: None,
            encoding: None,
            max_retries: None,
            min_context_slot: None,
        };

        Ok(Self {
            rpc_client,
            rpc_url: rpc_url.into(),
            rpc_config,
            admin_wallet,
        })
    }

    /// Create a new event.
    /// For each token, a token_reypair is needed. The order of these keypairs does not matter
    pub async fn create_event(
        &self,
        token_keypairs: &[Keypair],
        args: &CreateEventArgs,
    ) -> Result<Signature> {
        let instruction_args = MarketInstruction::CreateEvent(args.clone());

        let instruction_bytes = wincode::serialize(&instruction_args)?;

        let (event_pda, _) = Event::find_program_address(&args.uuid, &MARKETPLACE_PROGRAM);

        let treasury = get_associated_token_address(&event_pda, &USDC_MINT);

        // println!("event PDA is {}", event_pda);
        // println!("treasury PDA is {}\n\n", treasury);

        let mut accounts: Vec<AccountMeta> = vec![
            AccountMeta::new(self.admin_wallet.pubkey(), true),
            AccountMeta::new(event_pda, false),
            AccountMeta::new_readonly(USDC_MINT, false),
            AccountMeta::new(treasury, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ];

        for option in args.options.values() {
            accounts.push(AccountMeta::new(option.yes_mint, true));
            accounts.push(AccountMeta::new(option.no_mint, true));
        }

        let recent_blockhash = self.rpc_client.get_latest_blockhash().await?;

        let instruction =
            Instruction::new_with_bytes(MARKETPLACE_PROGRAM, instruction_bytes.as_ref(), accounts);

        let message = Message::new(&[instruction], Some(&self.admin_wallet.pubkey()));

        let mut transaction = Transaction::new_unsigned(message);

        {
            let mut all_signers: Vec<&dyn Signer> = vec![&self.admin_wallet];
            all_signers.extend(token_keypairs.iter().map(|k| k as &dyn Signer));
            transaction.sign(&all_signers[..], recent_blockhash);
        }

        let signature = self
            .rpc_client
            .send_transaction_with_config(&transaction, self.rpc_config)
            .await?;

        Ok(signature)
    }

    pub async fn close_event(&self, args: &CloseEventArgs) -> Result<Signature> {
        let instruction_args = MarketInstruction::CloseEvent(args.clone());

        let instruction_bytes = wincode::serialize(&instruction_args)?;

        let (event_pda, _) = Event::find_program_address(&args.uuid, &MARKETPLACE_PROGRAM);

        let accounts: Vec<AccountMeta> = vec![
            AccountMeta::new(self.admin_wallet.pubkey(), true),
            AccountMeta::new(event_pda, false),
        ];

        let recent_blockhash = self.rpc_client.get_latest_blockhash().await?;

        let instruction =
            Instruction::new_with_bytes(MARKETPLACE_PROGRAM, instruction_bytes.as_ref(), accounts);

        let message = Message::new(&[instruction], Some(&self.admin_wallet.pubkey()));

        let mut transaction = Transaction::new_unsigned(message);

        transaction.sign(&[&self.admin_wallet], recent_blockhash);

        let signature = self
            .rpc_client
            .send_transaction_with_config(&transaction, self.rpc_config)
            .await?;

        Ok(signature)
    }

    /// Creates a random keypair, and airdrops some funds into it
    pub async fn init_new_wallet(&self) -> Result<Keypair> {
        let wallet = Keypair::new();

        let _sig = self
            .rpc_client
            .request_airdrop(&wallet.pubkey(), LAMPORTS_PER_SOL * 10)
            .await?;

        Ok(wallet)
    }

    /// amount is in micro-usdc. Assumes the usdc ATA does not exist (basically, if this is the first time airdropping, call create_usdc_ata() first)
    pub async fn airdrop_usdc(&self, wallet: &Pubkey, amount: u64) -> Result<()> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "surfnet_setTokenAccount",
            "params": [
                wallet.to_string(),
                USDC_MINT.to_string(),
                {
                    "amount": amount,
                    "closeAuthority": "83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri",
                    "delegate": "83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri",
                    "delegatedAmount": amount,
                    "state": "initialized",
                },
            ],
        });
        let client = reqwest::Client::new();
        let res = client
            .post(&self.rpc_url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;
        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            anyhow::bail!(
                "Error funding: {}",
                serde_json::to_string(error).unwrap_or_else(|_| "Unknown error".to_string())
            );
        }

        Ok(())
    }

    pub async fn create_usdc_ata(&self, wallet: &Keypair) -> Result<Signature> {
        let instruction = create_associated_token_account(
            &wallet.pubkey(),
            &wallet.pubkey(),
            &USDC_MINT,
            &spl_token::id(),
        );

        let recent_blockhash = self.rpc_client.get_latest_blockhash().await?;

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&wallet.pubkey()),
            &[&wallet],
            recent_blockhash,
        );

        let sig = self.rpc_client.send_and_confirm_transaction(&tx).await?;

        Ok(sig)
    }

    pub async fn fetch_ata(&self, wallet: &Pubkey, token: &Pubkey) -> Result<TokenAccount> {
        let ata = get_associated_token_address(wallet, token);

        let account = self.rpc_client.get_account(&ata).await?;

        let token_account = TokenAccount::unpack(&account.data)?;

        Ok(token_account)
    }

    pub async fn fetch_usdc(&self, wallet: &Pubkey) -> Result<TokenAccount> {
        self.fetch_ata(wallet, &USDC_MINT).await
    }

    pub async fn match_order(
        &self,
        user_yes_wallet: &Keypair,
        user_no_wallet: &Keypair,
        token_yes: &Pubkey,
        token_no: &Pubkey,
        args: &FakeMatchOrderArgs,
    ) -> Result<Signature> {
        let instruction_args = MarketInstruction::FakeMatchOrder(args.clone());

        let instruction_bytes = wincode::serialize(&instruction_args)?;

        let (event_pda, _) = Event::find_program_address(&args.event_uuid, &MARKETPLACE_PROGRAM);

        // let treasury = get_associated_token_address(&event_pda, &USDC_MINT);

        // let user_yes_usdc_ata = get_associated_token_address(&user_yes_wallet.pubkey(), &USDC_MINT);
        let user_yes_token_ata = get_associated_token_address(&user_yes_wallet.pubkey(), token_yes);

        // let user_no_usdc_ata = get_associated_token_address(&user_no_wallet.pubkey(), &USDC_MINT);
        let user_no_token_ata = get_associated_token_address(&user_no_wallet.pubkey(), token_no);

        let accounts: Vec<AccountMeta> = vec![
            AccountMeta::new(user_yes_wallet.pubkey(), true),
            // AccountMeta::new(user_yes_usdc_ata, false),
            AccountMeta::new(user_yes_token_ata, false),
            AccountMeta::new(user_no_wallet.pubkey(), true),
            // AccountMeta::new(user_no_usdc_ata, false),
            AccountMeta::new(user_no_token_ata, false),
            AccountMeta::new(event_pda, false),
            // AccountMeta::new(treasury, false),
            AccountMeta::new(*token_yes, false),
            AccountMeta::new(*token_no, false),
            // AccountMeta::new_readonly(USDC_MINT, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ];

        let recent_blockhash = self.rpc_client.get_latest_blockhash().await?;

        let instruction =
            Instruction::new_with_bytes(MARKETPLACE_PROGRAM, instruction_bytes.as_ref(), accounts);

        let message = Message::new(&[instruction], Some(&user_yes_wallet.pubkey()));

        let mut transaction = Transaction::new_unsigned(message);

        transaction.sign(&[&user_yes_wallet, &user_no_wallet], recent_blockhash);

        let sig = self
            .rpc_client
            .send_transaction_with_config(&transaction, self.rpc_config)
            .await?;

        Ok(sig)
    }

    pub async fn create_order(&self, user: &Keypair, args: &FakeCreateOrderArgs) -> Result<Signature> {
        let instruction_args = MarketInstruction::FakeCreateOrder(args.clone());

        let instruction_bytes = wincode::serialize(&instruction_args)?;

        let (event_pda, _) = Event::find_program_address(&args.event_uuid, &MARKETPLACE_PROGRAM);

        let treasury = get_associated_token_address(&event_pda, &USDC_MINT);

        let user_usdc_ata = get_associated_token_address(&user.pubkey(), &USDC_MINT);

        let accounts: Vec<AccountMeta> = vec![
            AccountMeta::new(user.pubkey(), true),
            AccountMeta::new(user_usdc_ata, false),
            AccountMeta::new(event_pda, false),
            AccountMeta::new(treasury, false),
            AccountMeta::new_readonly(USDC_MINT, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ];

        let recent_blockhash = self.rpc_client.get_latest_blockhash().await?;

        let instruction =
            Instruction::new_with_bytes(MARKETPLACE_PROGRAM, instruction_bytes.as_ref(), accounts);

        let message = Message::new(&[instruction], Some(&user.pubkey()));

        let mut transaction = Transaction::new_unsigned(message);

        transaction.sign(&[&user], recent_blockhash);

        let sig = self
            .rpc_client
            .send_transaction_with_config(&transaction, self.rpc_config)
            .await?;

        Ok(sig)
    }

    pub async fn cancel_order(&self, admin: &Keypair, user: &Pubkey, args: &FakeCancelOrderArgs) -> Result<Signature> {
        let instruction_args = MarketInstruction::FakeCancelOrder(args.clone());

        let instruction_bytes = wincode::serialize(&instruction_args)?;

        let (event_pda, _) = Event::find_program_address(&args.event_uuid, &MARKETPLACE_PROGRAM);

        let treasury = get_associated_token_address(&event_pda, &USDC_MINT);

        let user_usdc_ata = get_associated_token_address(&user, &USDC_MINT);

        let accounts: Vec<AccountMeta> = vec![
            AccountMeta::new(admin.pubkey(), true),
            AccountMeta::new_readonly(*user, false),
            AccountMeta::new(user_usdc_ata, false),
            AccountMeta::new(event_pda, false),
            AccountMeta::new(treasury, false),
            AccountMeta::new_readonly(USDC_MINT, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ];

        let recent_blockhash = self.rpc_client.get_latest_blockhash().await?;

        let instruction =
            Instruction::new_with_bytes(MARKETPLACE_PROGRAM, instruction_bytes.as_ref(), accounts);

        let message = Message::new(&[instruction], Some(&admin.pubkey()));

        let mut transaction = Transaction::new_unsigned(message);

        transaction.sign(&[&admin], recent_blockhash);

        let sig = self
            .rpc_client
            .send_transaction_with_config(&transaction, self.rpc_config)
            .await?;

        Ok(sig)
    }

    pub fn derive_event_pubkey(event_id: &Uuid) -> Pubkey {
        Event::find_program_address(event_id, &MARKETPLACE_PROGRAM).0
    }

    pub fn get_account_url(&self, pubkey: &Pubkey) -> String {
        format!("https://solscan.io/account/{}?cluster=custom&customUrl={}", pubkey.to_string(), self.rpc_url)
    }
}
