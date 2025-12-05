// Task 3: Enums & Pattern Matching
// This demonstrates enum definition, pattern matching, and references

/// Bitcoin network types
/// Each variant represents a different Bitcoin network environment
#[derive(Debug)]
enum Network {
    /// Production network - real Bitcoin transactions
    Mainnet,
    /// Test network - for development and testing
    Testnet,
    /// Regression test network - local testing environment
    Regtest,
}

/// Returns the RPC URL for a given network
///
/// # Arguments
/// * `network` - Reference to a Network enum
///
/// # Returns
/// * `&str` - The RPC endpoint URL as a string slice
fn get_rpc_url(network: &Network) -> &str {
    match network {
        Network::Mainnet => "https://mainnet.bitcoin.org:8332",
        Network::Testnet => "https://testnet.bitcoin.org:18332",
        Network::Regtest => "http://localhost:18443",
    }
}

/// Prints detailed information about the network
///
/// # Arguments
/// * `network` - Reference to a Network enum
fn print_network_details(network: &Network) {
    match network {
        Network::Mainnet => {
            println!("Network: Mainnet");
            println!("Description: Bitcoin production network");
            println!("Chain ID: 0");
            println!("Default Port: 8333");
            println!("Uses real BTC with real value");
            println!("⚠ Exercise caution - transactions are irreversible!");
        }
        Network::Testnet => {
            println!("Network: Testnet");
            println!("Description: Bitcoin test network");
            println!("Chain ID: 1");
            println!("Default Port: 18333");
            println!("Uses test BTC (tBTC) with no real value");
            println!("✓ Safe for development and testing");
        }
        Network::Regtest => {
            println!("Network: Regtest (Regression Test)");
            println!("Description: Local private test network");
            println!("Chain ID: Custom");
            println!("Default Port: 18444");
            println!("Fully controlled local environment");
            println!("✓ Ideal for automated testing and development");
        }
    }
}

/// Returns the block time for the network
///
/// # Arguments
/// * `network` - Reference to a Network enum
///
/// # Returns
/// * `u32` - Average block time in seconds
fn get_block_time(network: &Network) -> u32 {
    match network {
        Network::Mainnet => 600, // ~10 minutes
        Network::Testnet => 600, // ~10 minutes
        Network::Regtest => 0,   // Instant (manual mining)
    }
}

/// Returns whether the network requires real funds
///
/// # Arguments
/// * `network` - Reference to a Network enum
///
/// # Returns
/// * `bool` - True if real BTC is used
fn requires_real_funds(network: &Network) -> bool {
    match network {
        Network::Mainnet => true,
        Network::Testnet | Network::Regtest => false,
    }
}

/// Provides a recommendation based on the use case
///
/// # Arguments
/// * `network` - Reference to a Network enum
fn recommend_use_case(network: &Network) {
    print!("Recommended for: ");
    match network {
        Network::Mainnet => println!("Production applications, real transactions, live services"),
        Network::Testnet => println!("Integration testing, staging environments, public testing"),
        Network::Regtest => println!("Unit tests, local development, rapid prototyping"),
    }
}

fn main() {
    println!("=== Bitcoin Network Configuration ===\n");

    // Create instances of each network type
    let networks = vec![Network::Mainnet, Network::Testnet, Network::Regtest];

    // Iterate through each network and display information
    for network in &networks {
        println!("{}", "─".repeat(60));
        print_network_details(&network);
        println!("\nRPC URL: {}", get_rpc_url(&network));
        println!("Block Time: {} seconds", get_block_time(&network));
        println!("Real Funds Required: {}", requires_real_funds(&network));
        recommend_use_case(&network);
        println!();
    }

    println!("{}", "─".repeat(60));

    // Demonstrate network selection logic
    println!("\n=== Network Selection Example ===\n");

    let selected_network = Network::Testnet;

    println!("Selected Network: {:?}", selected_network);
    println!("Connecting to: {}", get_rpc_url(&selected_network));

    // Pattern matching for configuration
    match selected_network {
        Network::Mainnet => {
            println!(" WARNING: You are about to connect to Mainnet!");
            println!("   Ensure you have backed up your keys.");
        }
        Network::Testnet => {
            println!(" Safe testing environment selected.");
            println!("   Get test coins from a faucet.");
        }
        Network::Regtest => {
            println!(" Local development environment.");
            println!("   Mine blocks instantly with generatetoaddress.");
        }
    }
}

// Key Learning Points:
// 1. Enum definition with #[derive(Debug)] for printing
// 2. Pattern matching with 'match' keyword
// 3. References with '&' to avoid moving ownership
// 4. String slices with &str lifetime
// 5. Multiple patterns with '|' operator (OR)
// 6. Exhaustive matching - all enum variants must be handled
