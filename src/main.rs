#![allow(deprecated)]

use crate::{
    commands::{CreateTreeArgs, MintNftArgs, create_tree, mint_nft},
    instagram::InstagramClient,
    utils::{CompresstaConfig, upload_metadata},
};
use clap::{Arg, Command, value_parser};
use pinata_sdk::PinataApi;
use solana_cli_config::Config;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::EncodableKey};
use std::{fs::File, str::FromStr};

mod commands;
mod instagram;
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let compressta_config = read_compressta_config().expect("couldn't find compressta config file");
    let solana_config = Config::load(
        &dirs::config_local_dir()
            .unwrap()
            .join("solana")
            .join("cli")
            .join("config.yml")
            .to_str()
            .unwrap(),
    )
    .expect("couldn't find solana config file");

    let payer = Keypair::read_from_file(&solana_config.keypair_path).unwrap();
    let client = RpcClient::new(solana_config.json_rpc_url);
    let ig_client = InstagramClient::new(&compressta_config.instagram_cookie)?;

    let matches = Command::new("compressta")
        .about("Share your Instagram photos as CNFT's")
        .subcommand(
            Command::new("mint")
                .about("Mint a NFT")
                .arg(Arg::new("url").short('u').long("url").required(true))
                .arg(
                    Arg::new("leaf_owner")
                        .short('o')
                        .long("leaf_owner")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("create-tree")
                .about("Create a new Bubblegum Tree")
                .arg(
                    Arg::new("max-depth")
                        .short('d')
                        .long("max-depth")
                        .value_parser(value_parser!(u32))
                        .required(true),
                )
                .arg(
                    Arg::new("max-buffer-size")
                        .short('b')
                        .long("max-buffer-size")
                        .value_parser(value_parser!(u32))
                        .required(true),
                )
                .arg(
                    Arg::new("public")
                        .short('p')
                        .long("public")
                        .value_parser(value_parser!(bool))
                        .required(false),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create-tree") {
        let max_depth: u32 = *matches.get_one("max-depth").unwrap();
        let max_buffer_size: u32 = *matches.get_one("max-buffer-size").unwrap();
        let public = *matches.get_one("public").unwrap_or(&false);

        create_tree(
            &payer,
            &client,
            CreateTreeArgs {
                max_depth,
                max_buffer_size,
                public,
            },
        )
        .await?;
    }

    if let Some(matches) = matches.subcommand_matches("mint") {
        let url = matches.get_one::<String>("url").unwrap();
        let leaf_owner = Pubkey::from_str(matches.get_one::<String>("leaf_owner").unwrap())?;
        let pinata = PinataApi::new(
            compressta_config.pinata_api_key,
            compressta_config.pinata_api_secret,
        )?;

        let media = ig_client.fetch_media(url).await?;
        let metadata = upload_metadata(&pinata, &media).await?;

        mint_nft(
            &payer,
            &client,
            MintNftArgs {
                name: metadata.0,
                symbol: "INFT".to_string(), // make this dynamic too
                uri: metadata.1,
                leaf_owner: Some(leaf_owner),
                merkle_tree: Pubkey::from_str_const(&compressta_config.merkle_tree),
            },
        )
        .await?;
    }

    Ok(())
}

pub fn read_compressta_config() -> Result<CompresstaConfig, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_local_dir()
        .unwrap()
        .join("compressta")
        .join("config.json");

    let file = File::open(&config_dir)?;
    Ok(serde_json::from_reader(file)?)
}
