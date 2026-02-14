use std::{collections::HashMap, env};

use anyhow::Result;
use blockchain_core::{
    accounts::event::{Event, EventOption},
    instructions::{CloseEventArgs, CreateEventArgs, CreateOrderArgs, FakeCreateOrderArgs, FakeMatchOrderArgs, MarketInstruction},
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
    signer::Signer,
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
    pub fn new(admin_wallet: Keypair, rpc_url: &str) -> Self {
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

    pub async fn fetch_ata(&self, pubkey: &Pubkey, token: &Pubkey) -> Result<TokenAccount> {
        let ata = get_associated_token_address(pubkey, token);

        let account = self.rpc_client.get_account(&ata).await?;

        let token_account = TokenAccount::unpack(&account.data)?;

        Ok(token_account)
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

    let some_option_uuid = Uuid::new_v4();

    {
        let yes_mint = Keypair::new();
        let no_mint = Keypair::new();

        options.insert(
            some_option_uuid,
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
    let event_uuid = Uuid::new_v4();
    let args = CreateEventArgs {
        uuid: event_uuid,
        description: "Some event".into(),
        options: options.clone(),
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

    println!(
        "info: {:#?}",
        profecia_client
            .fetch_ata(&profecia_client.admin_wallet.pubkey(), &USDC_MINT)
            .await?
    );

    // create two clients
    let client_yes = profecia_client.init_new_wallet().await?;
    let client_no = profecia_client.init_new_wallet().await?;
    profecia_client.create_usdc_ata(&client_yes).await?;
    profecia_client.create_usdc_ata(&client_no).await?;
    profecia_client
        .airdrop_usdc(&client_yes.pubkey(), 100 * 1000000)
        .await?;
    profecia_client
        .airdrop_usdc(&client_no.pubkey(), 100 * 1000000)
        .await?;

    println!("Finished airdrops");

    // 60 cent yes, 40 cent no
    let yes_price = 60 * 10000;
    let no_price = 40 * 10000;

    // create order (will just transfer some usdc)
    let create_order_yes = FakeCreateOrderArgs {
        event_uuid,
        option_uuid: some_option_uuid,
        num_shares: 5,
        price_per_share: yes_price
    };
    let sig = profecia_client.create_order(&client_yes, &create_order_yes).await?;
    println!("buy order for yes {}", sig);
    let create_order_no = FakeCreateOrderArgs {
        event_uuid,
        option_uuid: some_option_uuid,
        num_shares: 5,
        price_per_share: no_price
    };
    let sig = profecia_client.create_order(&client_no, &create_order_no).await?;
    println!("buy order for no {}", sig);

    // get the first option in the map
    // let option = options.get(&some_option_uuid).unwrap();
    let yes = &token_keypairs[0];
    let no = &token_keypairs[1];

    let args = FakeMatchOrderArgs {
        event_uuid,
        option_uuid: some_option_uuid,
        num_shares: 5,
        yes_price,
        no_price,
    };

    let sig = profecia_client
        .match_order(&client_yes, &client_no, &yes.pubkey(), &no.pubkey(), &args)
        .await?;

    println!("Match order sig: {}", sig);

    Ok(())
}
