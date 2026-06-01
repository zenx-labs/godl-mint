use clap::{Parser, Subcommand};
use godl_mint_api::state::Authority;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};
use steel::{AccountDeserialize, Clock};

#[derive(Parser)]
#[command(name = "godl", version, about = "CLI for the GODL mint program")]
struct Cli {
    /// Solana RPC endpoint
    #[arg(long, env = "RPC")]
    rpc: String,

    /// Path to the payer keypair file
    #[arg(long, env = "KEYPAIR")]
    keypair: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print the on-chain authority account
    Authority,
    /// Print the on-chain clock sysvar
    Clock,
    /// Initialize the program authority
    Init,
    /// Sign and execute a base58-encoded Solana transaction
    Execute {
        /// Base58-encoded serialized transaction
        transaction: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Load environment variables from .env if present
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    let rpc = RpcClient::new(cli.rpc);

    match cli.command {
        Command::Authority => log_authority(&rpc).await?,
        Command::Clock => log_clock(&rpc).await?,
        Command::Init => {
            let payer = read_keypair_file(&cli.keypair)
                .map_err(|e| anyhow::anyhow!("Failed to read keypair file: {e}"))?;
            init(&rpc, &payer).await?;
        }
        Command::Execute { transaction } => {
            let payer = read_keypair_file(&cli.keypair)
                .map_err(|e| anyhow::anyhow!("Failed to read keypair file: {e}"))?;
            execute(&rpc, &payer, &transaction).await?;
        }
    };

    Ok(())
}

async fn init(rpc: &RpcClient, payer: &Keypair) -> Result<(), anyhow::Error> {
    let ix = godl_mint_api::sdk::init(payer.pubkey());
    submit_transaction(rpc, payer, &[ix]).await?;
    Ok(())
}

async fn execute(rpc: &RpcClient, payer: &Keypair, encoded: &str) -> Result<(), anyhow::Error> {
    let bytes = bs58::decode(encoded.trim())
        .into_vec()
        .map_err(|e| anyhow::anyhow!("Failed to decode base58 transaction: {e}"))?;

    let mut transaction: Transaction = bincode::deserialize(&bytes)
        .map_err(|e| anyhow::anyhow!("Failed to deserialize transaction: {e}"))?;

    // Refresh the blockhash so the transaction is valid at submission time
    // (e.g. tools like `solana-verify export-pda-tx` emit it unsigned with a
    // blank blockhash). This invalidates any pre-existing signatures.
    let recent_blockhash = rpc.get_latest_blockhash().await?;
    transaction
        .try_partial_sign(&[payer], recent_blockhash)
        .map_err(|e| anyhow::anyhow!("Failed to sign transaction: {e}"))?;

    match rpc.send_and_confirm_transaction(&transaction).await {
        Ok(signature) => {
            println!("Transaction submitted: {:?}", signature);
            Ok(())
        }
        Err(e) => {
            println!("Error submitting transaction: {:?}", e);
            Err(e.into())
        }
    }
}

async fn log_authority(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let authority_address = godl_mint_api::state::authority_pda().0;
    let authority = get_authority(&rpc).await?;
    println!("Authority");
    println!("  address: {}", authority_address);
    println!("  last_mint_at: {}", authority.last_mint_at);
    Ok(())
}

async fn log_clock(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let clock = get_clock(&rpc).await?;
    println!("Clock");
    println!("  slot: {}", clock.slot);
    println!("  epoch_start_timestamp: {}", clock.epoch_start_timestamp);
    println!("  epoch: {}", clock.epoch);
    println!("  leader_schedule_epoch: {}", clock.leader_schedule_epoch);
    println!("  unix_timestamp: {}", clock.unix_timestamp);
    Ok(())
}

async fn get_authority(rpc: &RpcClient) -> Result<Authority, anyhow::Error> {
    let authority_address = godl_mint_api::state::authority_pda().0;
    let account = rpc.get_account(&authority_address).await?;
    let authority = Authority::try_from_bytes(&account.data)?;
    Ok(*authority)
}

async fn get_clock(rpc: &RpcClient) -> Result<Clock, anyhow::Error> {
    let data = rpc.get_account_data(&solana_sdk::sysvar::clock::ID).await?;
    let clock = bincode::deserialize::<Clock>(&data)?;
    Ok(clock)
}

async fn submit_transaction(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
    instructions: &[solana_sdk::instruction::Instruction],
) -> Result<solana_sdk::signature::Signature, anyhow::Error> {
    let blockhash = rpc.get_latest_blockhash().await?;
    let mut all_instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(1_400_000),
        ComputeBudgetInstruction::set_compute_unit_price(1_000_000),
    ];
    all_instructions.extend_from_slice(instructions);
    let transaction = Transaction::new_signed_with_payer(
        &all_instructions,
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    match rpc.send_and_confirm_transaction(&transaction).await {
        Ok(signature) => {
            println!("Transaction submitted: {:?}", signature);
            Ok(signature)
        }
        Err(e) => {
            println!("Error submitting transaction: {:?}", e);
            Err(e.into())
        }
    }
}
