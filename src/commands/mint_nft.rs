use mpl_bubblegum::{
    instructions::MintV2Builder,
    types::{MetadataArgsV2, TokenStandard},
};
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair, signer::Signer,
    transaction::Transaction,
};

#[derive(Debug)]
pub struct MintNftArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub merkle_tree: Pubkey,
    pub leaf_owner: Option<Pubkey>,
}

pub async fn mint_nft(
    payer: &Keypair,
    client: &RpcClient,
    args: MintNftArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let tree_config =
        Pubkey::find_program_address(&[&args.merkle_tree.to_bytes()], &mpl_bubblegum::ID);

    let metadata = MetadataArgsV2 {
        name: args.name,
        symbol: args.symbol,
        uri: args.uri,
        seller_fee_basis_points: 0,
        primary_sale_happened: false,
        is_mutable: false,
        creators: vec![],
        collection: None, // use collections for carousel?
        token_standard: Some(TokenStandard::NonFungible),
    };

    let mint_nft_ix = MintV2Builder::new()
        .tree_config(tree_config.0)
        .leaf_owner(args.leaf_owner.unwrap_or(payer.pubkey()))
        .merkle_tree(args.merkle_tree)
        .payer(payer.pubkey())
        .metadata(metadata)
        .instruction();

    let signers = vec![&payer];

    let latest_blockhash = client.get_latest_blockhash().await?;

    let mint_nft_tx = Transaction::new_signed_with_payer(
        &[mint_nft_ix],
        Some(&payer.pubkey()),
        &signers,
        latest_blockhash,
    );

    let res = client
        .send_transaction_with_config(
            &mint_nft_tx,
            RpcSendTransactionConfig {
                skip_preflight: false,
                preflight_commitment: Some(CommitmentConfig::confirmed().commitment),
                encoding: None,
                max_retries: None,
                min_context_slot: None,
            },
        )
        .await
        .unwrap();

    println!("Success!, Signature: {:?}", res);

    Ok(())
}
