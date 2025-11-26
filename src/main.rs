use clap::{Arg, Command, value_parser};
use solana_cli_config::Config;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    signature::Keypair,
    signer::{EncodableKey, Signer},
};

use crate::commands::{CreateTreeArgs, create_tree};

mod commands;
mod instagram;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("compressta")
        .about("Share your Instagram photos as CNFT's")
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
        let config = Config::load("/home/aditya/.config/solana/cli/config.yml").unwrap();
        let payer = Keypair::read_from_file(&config.keypair_path).unwrap();
        let client = RpcClient::new(config.json_rpc_url);

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

    if let Some(matches) = matches.subcommand_matches("create-tree") {
        let max_depth: u32 = *matches.get_one("max-depth").unwrap();
        let max_buffer_size: u32 = *matches.get_one("max-buffer-size").unwrap();
        let public = *matches.get_one("public").unwrap_or(&false);
        let config = Config::load("/home/aditya/.config/solana/cli/config.yml").unwrap();
        let payer = Keypair::read_from_file(&config.keypair_path).unwrap();
        let client = RpcClient::new(config.json_rpc_url);

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

    Ok(())
}
