use clap::Parser;

#[derive(Parser)]
#[command(name = "storb-sign")]
#[command(about = "A CLI tool for signing messages with Bittensor wallets")]
#[command(version = "0.1.0")]
struct Args {
    /// The nonce string to sign (in hex format)
    #[arg(short, long)]
    nonce: String,

    /// Path to the wallet hotkey file
    #[arg(short, long)]
    wallet_path: String,
}

fn main() {
    let args = Args::parse();
    
    let nonce = match hex::decode(&args.nonce) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to decode hex string: {}", e);
            return;
        }
    };

    if nonce.len() != 32 {
        eprintln!("Nonce must be exactly 32 bytes long.");
        return;
    }

    // load signer from directory
    let seed = match crabtensor::wallet::load_key_seed(&args.wallet_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load signer: {}", e);
            return;
        }
    };

    let signer = match crabtensor::wallet::signer_from_seed(&seed) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to create signer: {}", e);
            return;
        }
    };

    // sign the nonce
    let signature = crabtensor::sign::sign_message(&signer, &nonce);

    // print the signature in hex format
    println!("Signature: {}", hex::encode(signature));
}
