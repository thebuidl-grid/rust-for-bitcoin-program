use bitcoin::Block;
use rusqlite::Connection;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use tokio::fs::read_dir;
use crate::db::insert_block;

// Magic bytes from the regtest blk file
const REGTEST_MAGIC: [u8; 4] = [0x83, 0x9d, 0xe4, 0x11];

// Parse a single block from reader
fn parse_block(reader: &mut impl Read) -> io::Result<Block> {
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic)?;

    if magic != REGTEST_MAGIC {
        eprintln!("Invalid magic: {:02x?}, expected regtest: {:02x?}", magic, REGTEST_MAGIC);
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid magic"));
    }

    let mut size = [0u8; 4];
    reader.read_exact(&mut size)?;
    let size = u32::from_le_bytes(size) as usize;
    println!("Block size: {}", size);

    let mut block_data = vec![0u8; size];
    reader.read_exact(&mut block_data)?;

    let block = bitcoin::consensus::deserialize(&block_data)
        .map_err(|e| {
            eprintln!("Consensus decode failed: {}", e);
            io::Error::new(io::ErrorKind::InvalidData, e)
        })?;

    Ok(block)
}

// Index all blocks from a directory of .blk files
pub async fn index_blocks(db_conn: &Connection, blocks_dir: &Path) -> anyhow::Result<()> {
    let mut entries = read_dir(blocks_dir).await?;
    let mut height = 0;

    while let Some(entry) = entries.next_entry().await? {
        let filename = entry.file_name();
        let filename_str = filename.to_string_lossy();

        if filename_str.starts_with("blk") && filename_str.ends_with(".dat") {
            println!("Processing file: {}", filename_str);

            let file = File::open(entry.path())?;
            let mut reader = BufReader::new(file);

            while let Ok(block) = parse_block(&mut reader) {
                match insert_block(db_conn, &block, height) {
                    Ok(_) => {
                        println!("Indexed block at height {}: {}", height, block.block_hash());
                        height += 1;
                    }
                    Err(e) => {
                        eprintln!("Error inserting block at height {}: {}", height, e);
                    }
                }
            }
        }
    }

    println!("Finished indexing {} blocks", height);
    Ok(())
}