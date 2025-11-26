use mpl_account_compression::{ConcurrentMerkleTree, state::CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1};
use mpl_bubblegum::instructions::CreateTreeConfigV2Builder;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair, signer::Signer,
    system_instruction, transaction::Transaction,
};

#[derive(Debug)]
pub struct CreateTreeArgs {
    pub max_depth: u32,
    pub max_buffer_size: u32,
    pub public: bool,
}

// modified from mpl_account_compression::state::merkle_tree_get_size, better way?
fn get_tree_size(max_depth: u32, max_buffer_size: u32) -> usize {
    match (max_depth, max_buffer_size) {
        (3, 8) => size_of::<ConcurrentMerkleTree<3, 8>>(),
        (5, 8) => size_of::<ConcurrentMerkleTree<5, 8>>(),
        (6, 16) => size_of::<ConcurrentMerkleTree<6, 16>>(),
        (7, 16) => size_of::<ConcurrentMerkleTree<7, 16>>(),
        (8, 16) => size_of::<ConcurrentMerkleTree<8, 16>>(),
        (9, 16) => size_of::<ConcurrentMerkleTree<9, 16>>(),
        (10, 32) => size_of::<ConcurrentMerkleTree<10, 32>>(),
        (11, 32) => size_of::<ConcurrentMerkleTree<11, 32>>(),
        (12, 32) => size_of::<ConcurrentMerkleTree<12, 32>>(),
        (13, 32) => size_of::<ConcurrentMerkleTree<13, 32>>(),
        (14, 64) => size_of::<ConcurrentMerkleTree<14, 64>>(),
        (14, 256) => size_of::<ConcurrentMerkleTree<14, 256>>(),
        (14, 1024) => size_of::<ConcurrentMerkleTree<14, 1024>>(),
        (14, 2048) => size_of::<ConcurrentMerkleTree<14, 2048>>(),
        (15, 64) => size_of::<ConcurrentMerkleTree<15, 64>>(),
        (16, 64) => size_of::<ConcurrentMerkleTree<16, 64>>(),
        (17, 64) => size_of::<ConcurrentMerkleTree<17, 64>>(),
        (18, 64) => size_of::<ConcurrentMerkleTree<18, 64>>(),
        (19, 64) => size_of::<ConcurrentMerkleTree<19, 64>>(),
        (20, 64) => size_of::<ConcurrentMerkleTree<20, 64>>(),
        (20, 256) => size_of::<ConcurrentMerkleTree<20, 256>>(),
        (20, 1024) => size_of::<ConcurrentMerkleTree<20, 1024>>(),
        (20, 2048) => size_of::<ConcurrentMerkleTree<20, 2048>>(),
        (24, 64) => size_of::<ConcurrentMerkleTree<24, 64>>(),
        (24, 256) => size_of::<ConcurrentMerkleTree<24, 256>>(),
        (24, 512) => size_of::<ConcurrentMerkleTree<24, 512>>(),
        (24, 1024) => size_of::<ConcurrentMerkleTree<24, 1024>>(),
        (24, 2048) => size_of::<ConcurrentMerkleTree<24, 2048>>(),
        (26, 512) => size_of::<ConcurrentMerkleTree<26, 512>>(),
        (26, 1024) => size_of::<ConcurrentMerkleTree<26, 1024>>(),
        (26, 2048) => size_of::<ConcurrentMerkleTree<26, 2048>>(),
        (30, 512) => size_of::<ConcurrentMerkleTree<30, 512>>(),
        (30, 1024) => size_of::<ConcurrentMerkleTree<30, 1024>>(),
        (30, 2048) => size_of::<ConcurrentMerkleTree<30, 2048>>(),
        _ => 0,
    }
}

fn get_concurrent_merkle_tree_size(
    max_depth: u32,
    max_buffer_size: u32,
    canopy_depth: Option<u32>,
) -> usize {
    let tree_size = get_tree_size(max_depth, max_buffer_size);
    let canopy = 32 * ((2 << canopy_depth.unwrap_or(0)) - 2);
    CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1 + tree_size + canopy
}

pub async fn create_tree(
    payer: &Keypair,
    client: &RpcClient,
    args: CreateTreeArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let merkle_tree = Keypair::new();

    let tree_config =
        Pubkey::find_program_address(&[&merkle_tree.pubkey().to_bytes()], &mpl_bubblegum::ID);

    let space = get_concurrent_merkle_tree_size(args.max_depth, args.max_buffer_size, None);

    let rent = client
        .get_minimum_balance_for_rent_exemption(space)
        .await
        .unwrap();

    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &merkle_tree.pubkey(),
        rent,
        space as u64,
        &mpl_account_compression::ID,
    );

    let create_tree_config_ix = CreateTreeConfigV2Builder::new()
        .merkle_tree(merkle_tree.pubkey())
        .tree_config(tree_config.0)
        .payer(payer.pubkey())
        .max_depth(args.max_depth)
        .max_buffer_size(args.max_buffer_size)
        .public(args.public)
        .instruction();

    let signers = vec![&merkle_tree, &payer];

    let last_blockhash = client.get_latest_blockhash().await;

    let create_tree_tx = Transaction::new_signed_with_payer(
        &[create_account_ix, create_tree_config_ix],
        Some(&payer.pubkey()),
        &signers,
        last_blockhash.unwrap(),
    );

    let res = client
        .send_transaction_with_config(
            &create_tree_tx,
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

    println!("Signature: {:?}", res);

    Ok(())
}
