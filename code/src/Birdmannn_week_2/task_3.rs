/// Task 3: Enums & Pattern Matching
/// 
/// This task demonstrates:
/// - Enum definition for Bitcoin networks
/// - Pattern matching with match blocks
/// - Functions that work with enums
/// - Returning string references

/// Represents different Bitcoin networks
#[derive(Debug, Clone, Copy)]
enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

/// Returns the RPC URL for the given network
/// 
/// # Arguments
/// * `network` - A reference to the Network enum
/// 
/// # Returns
/// A string slice containing the RPC URL
fn get_rpc_url(network: &Network) -> &str {
    match network {
        Network::Mainnet => "http://localhost:8332",
        Network::Testnet => "http://localhost:18332",
        Network::Regtest => "http://localhost:18443",
    }
}

/// Prints detailed information about the selected network
/// 
/// # Arguments
/// * `network` - A reference to the Network enum
fn print_network_details(network: &Network) {
    match network {
        Network::Mainnet => {
            println!("Network: Mainnet");
            println!("Description: The main Bitcoin network (production)");
            println!("RPC Port: 8332");
            println!("P2P Port: 8333");
            println!("Chain ID: 0");
        }
        Network::Testnet => {
            println!("Network: Testnet");
            println!("Description: Bitcoin test network for development");
            println!("RPC Port: 18332");
            println!("P2P Port: 18333");
            println!("Chain ID: 1");
        }
        Network::Regtest => {
            println!("Network: Regtest");
            println!("Description: Local regression test network");
            println!("RPC Port: 18443");
            println!("P2P Port: 18444");
            println!("Chain ID: 1");
        }
    }
}

/// Returns the network name as a string
fn get_network_name(network: &Network) -> &str {
    match network {
        Network::Mainnet => "Mainnet",
        Network::Testnet => "Testnet",
        Network::Regtest => "Regtest",
    }
}

fn main() {
    println!("=== Task 3: Enums & Pattern Matching ===\n");
    
    // Create instances of each network
    let networks = vec![
        Network::Mainnet,
        Network::Testnet,
        Network::Regtest,
    ];
    
    // Iterate through networks and display information
    for network in networks {
        println!("--- {} ---", get_network_name(&network));
        print_network_details(&network);
        println!("RPC URL: {}\n", get_rpc_url(&network));
    }
    
    // Example: Using pattern matching directly
    println!("=== Direct Pattern Matching Example ===\n");
    
    let selected_network = Network::Regtest;
    
    match selected_network {
        Network::Mainnet => println!("You selected the production network"),
        Network::Testnet => println!("You selected the test network"),
        Network::Regtest => println!("You selected the local regression test network"),
    }
    
    println!("\nRPC URL for {:?}: {}", selected_network, get_rpc_url(&selected_network));
}

