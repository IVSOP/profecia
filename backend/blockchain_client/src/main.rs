use std::{collections::HashMap, env};

use anyhow::Result;
use blockchain_core::{
    accounts::event::{Event, EventOption},
    instructions::{CloseEventArgs, CreateEventArgs, MarketInstruction},
};
use serde_json::{json, Value};
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{CommitmentConfig, RpcSendTransactionConfig},
};
use solana_sdk::{
    message::{AccountMeta, Instruction, Message}, native_token::LAMPORTS_PER_SOL, program_pack::Pack, pubkey::Pubkey, signature::{Keypair, Signature}, signer::Signer, transaction::Transaction
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use uuid::Uuid;
use spl_token::state::Account as TokenAccount;

pub const DEFAULT_RPC_HTTP: &str = "http://127.0.0.1:8899";
pub const USDC_MINT: Pubkey = solana_sdk::pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const MARKETPLACE_PROGRAM: Pubkey =
    solana_sdk::pubkey!("ProirMXDTFF4AEqGyZVKPhWte4chANDd1c4Y8w7Nsd4");
pub const SYSTEM_PROGRAM: Pubkey = solana_sdk::pubkey!("11111111111111111111111111111111");

pub struct ProfeciaClient {
    pub rpc_client: RpcClient,
    pub rpc_url: String,
    pub rpc_config: RpcSendTransactionConfig,
    pub admin_wallet: Keypair,
}

impl ProfeciaClient {
    pub fn new(admin_wallet: Keypair, rpc_url: &str) -> Self {
        let rpc_client =
            RpcClient::new_with_commitment(rpc_url.into(), CommitmentConfig::confirmed());

        let rpc_config = RpcSendTransactionConfig {
            skip_preflight: false,
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
            skip_preflight: false,
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

        let mut all_signers: Vec<&dyn Signer> = vec![&self.admin_wallet];
        all_signers.extend(token_keypairs.iter().map(|k| k as &dyn Signer));
        transaction.sign(&all_signers[..], recent_blockhash);

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
    pub async fn airdrop_usdc(&self, pubkey: &Pubkey, amount: u64) -> Result<()> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "surfnet_setTokenAccount",
            "params": [
                pubkey.to_string(),
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
            anyhow::bail!("Error funding: {}", serde_json::to_string(error).unwrap_or_else(|_| "Unknown error".to_string()));
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

    pub async fn fetch_ata(&self, pubkey: &Pubkey, token: &Pubkey) -> Result<TokenAccount> {
        let ata = get_associated_token_address(pubkey, token);

        let account = self.rpc_client
            .get_account(&ata).await?;

        let token_account = TokenAccount::unpack(&account.data)?;

        Ok(token_account)
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let rpc_url = env::var("RPC_HTTP").unwrap_or_else(|_| DEFAULT_RPC_HTTP.to_string());

    let profecia_client = ProfeciaClient::new_debug(rpc_url.as_str()).await?;

    println!("GENERATED WALLET {}", profecia_client.admin_wallet.pubkey());
    println!(
        "USDC ATA {}",
        get_associated_token_address(&profecia_client.admin_wallet.pubkey(), &USDC_MINT)
    );

    let mut options = HashMap::new();
    let mut token_keypairs = Vec::new();

    {
        let yes_mint = Keypair::new();
        let no_mint = Keypair::new();

        options.insert(
            Uuid::new_v4(),
            EventOption {
                option_desc: "Some option #1".into(),
                yes_mint: (*yes_mint.pubkey().as_array()).into(),
                no_mint: (*no_mint.pubkey().as_array()).into(),
            },
        );

        token_keypairs.push(yes_mint);
        token_keypairs.push(no_mint);
    }
    {
        let yes_mint = Keypair::new();
        let no_mint = Keypair::new();

        options.insert(
            Uuid::new_v4(),
            EventOption {
                option_desc: "Some option #2".into(),
                yes_mint: (*yes_mint.pubkey().as_array()).into(),
                no_mint: (*no_mint.pubkey().as_array()).into(),
            },
        );

        token_keypairs.push(yes_mint);
        token_keypairs.push(no_mint);
    }
    let args = CreateEventArgs {
        uuid: Uuid::new_v4(),
        description: "Some event".into(),
        options,
    };

    let sig = profecia_client.create_event(&token_keypairs, &args).await?;

    println!("Sig for creating marketplace: {}", sig);

    let args = CloseEventArgs { uuid: args.uuid };

    let sig = profecia_client.close_event(&args).await?;

    println!("Sig for closing marketplace: {}", sig);

    println!("Airdropping 1000 USDC");
    let sig = profecia_client
        .create_usdc_ata(&profecia_client.admin_wallet)
        .await?;
    println!("sig: {}", sig);

    profecia_client
        .airdrop_usdc(&profecia_client.admin_wallet.pubkey(), 1000000 * 1000)
        .await?;

    println!("info: {:#?}", profecia_client.fetch_ata(&profecia_client.admin_wallet.pubkey(), &USDC_MINT).await?);

    Ok(())
}
