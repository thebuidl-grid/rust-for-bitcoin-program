use mini_bitcoin_node::{Node, BlockDatabase, start_server};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Bitcoin Block Explorer API - Project 2\n");
    println!("======================================\n");

    // Get port from environment or use default
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    // Create or open database
    let db_path = env::var("DB_PATH").unwrap_or_else(|_| "blockchain.db".to_string());
    println!("Opening database: {}", db_path);
    
    let db = BlockDatabase::new(&db_path)?;
    
    // If database is empty, initialize with some sample data
    if db.get_height()? == 0 {
        println!("Database is empty. Creating sample blockchain data...");
        initialize_sample_data(&db)?;
    }

    println!("Database initialized. Height: {}", db.get_height()?);
    println!();

    // Start the API server
    start_server(db, port).await?;

    Ok(())
}

fn initialize_sample_data(db: &BlockDatabase) -> Result<(), Box<dyn std::error::Error>> {
    // Create a sample node and mine some blocks
    let mut node = Node::new("miner1".to_string());
    
    // Mine a few blocks
    for i in 0..3 {
        println!("Mining block {}...", i + 1);
        node.mine_block()?;
    }
    
    // Index all blocks into database
    for (height, block) in node.blockchain.blocks.iter().enumerate() {
        db.add_block(block, height)?;
        println!("Indexed block {}: {}", height, block.hash());
    }
    
    println!("Sample data initialized with {} blocks", node.blockchain.height());
    Ok(())
}


