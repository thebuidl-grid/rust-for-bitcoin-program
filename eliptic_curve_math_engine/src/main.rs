//! For implementing Clap parser for cli command arguments

use clap::{Parser, Subcommand, command};
use elliptic_curve_math_engine::cli::handler::{handle_derive, handle_generate, handle_info};

#[derive(Parser, Debug)]
#[command(name = "Elliptic Curve Math Engine")]
#[command(about = "Bitcoin secp256k1 keypair generation and management")]
#[command(version = "0.1.0")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a new keypair
    Generate {
        /// Output format: compressed, uncompressed, or x-only
        #[arg(short, long, default_value = "compressed")]
        format: String,

        /// Display private key (WARNING: insecure for production)
        #[arg(short, long)]
        show_private: bool,
    },

    /// Derive public key from private key
    Derive {
        /// Private key in hex format
        #[arg(short, long)]
        private_key: String,

        /// Output format: compressed, uncompressed, or x-only
        #[arg(short, long, default_value = "compressed")]
        format: String,
    },

    /// Display keypair information
    Info {
        /// Private key in hex format
        #[arg(short, long)]
        private_key: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Generate {
            format,
            show_private,
        } => {
            handle_generate(format, show_private);
        }
        Commands::Derive {
            private_key,
            format,
        } => {
            handle_derive(private_key, format);
        }
        Commands::Info { private_key } => {
            handle_info(private_key);
        }
    }
}
