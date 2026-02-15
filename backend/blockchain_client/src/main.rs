use std::{collections::HashMap, env};

use anyhow::Result;
use blockchain_client::{DEFAULT_RPC_HTTP, ProfeciaClient, USDC_MINT};
use blockchain_core::{
    accounts::event::EventOption,
    instructions::{CloseEventArgs, CreateEventArgs, FakeCancelOrderArgs, FakeCreateOrderArgs, FakeGetRewardArgs, FakeMatchOrderArgs},
};
use solana_sdk::{
    signature::Keypair,
    signer::Signer,
};
use spl_associated_token_account::
    get_associated_token_address
;
use uuid::Uuid;

#[tokio::main]
pub async fn main() -> Result<()> {
    let rpc_url = env::var("RPC_HTTP").unwrap_or_else(|_| DEFAULT_RPC_HTTP.to_string());
    let external_rpc_url = env::var("EXTERNAL_RPC_HTTP").unwrap_or_else(|_| DEFAULT_RPC_HTTP.to_string());

    let profecia_client = ProfeciaClient::new_debug(rpc_url.as_str(), external_rpc_url.as_str()).await?;

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

    // cancel order, then buy again
    let cancel_order_yes = FakeCancelOrderArgs {
        event_uuid,
        option_uuid: some_option_uuid,
        num_shares: 5,
        price_per_share: yes_price
    };
    let sig = profecia_client.cancel_order(&client_yes.pubkey(), &cancel_order_yes).await?;
    println!("cancel order for yes {}", sig);

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
    };

    let sig = profecia_client
        .match_order(&client_yes, &client_no, &yes.pubkey(), &no.pubkey(), &args)
        .await?;

    println!("Match order sig: {}", sig);

    let args = FakeGetRewardArgs {
        event_uuid,
        option_uuid: some_option_uuid,
        num_shares: 5
    };
    let sig = profecia_client.get_reward(&client_yes, &yes.pubkey(), &args).await?;

    println!("Get reward sig: {}", sig);

    Ok(())
}
