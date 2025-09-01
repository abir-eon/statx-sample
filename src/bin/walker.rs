use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Get the directory path from command line argument or use current directory
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };
    
    let mut counter = 0;
    let mut total_size = 0u64;
    let start_time = Instant::now();
    
    match walk_directory_iterative(Path::new(path), &mut counter, &mut total_size) {
        Ok(_) => {
            let elapsed = start_time.elapsed();
            
            // Convert to GB (decimal: 1000^3) and GiB (binary: 1024^3)
            let gb = total_size as f64 / 1_000_000_000.0;
            let gib = total_size as f64 / 1_073_741_824.0;
            
            println!("Directory walk completed in {:.3} seconds", elapsed.as_secs_f64());
            println!("Total files examined: {}", counter);
            println!("Total size: {} bytes ({:.3} GB, {:.3} GiB)", total_size, gb, gib);
        },
        Err(e) => eprintln!("Error walking directory: {}", e),
    }
}

fn walk_directory_iterative(start_path: &Path, counter: &mut usize, total_size: &mut u64) -> Result<(), Box<dyn std::error::Error>> {
    let mut stack: VecDeque<PathBuf> = VecDeque::new();
    stack.push_back(start_path.to_path_buf());
    
    while let Some(current_path) = stack.pop_back() {
        if current_path.is_dir() {
            let entries = fs::read_dir(&current_path)?;
            
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    // Add directory to stack for later processing
                    stack.push_back(path);
                } else {
                    // This is a file, examine its attributes and increment counter
                    let file_size = examine_file_attributes(&path)?;
                    *counter += 1;
                    *total_size += file_size;
                }
            }
        } else {
            // If the current path is a file, just examine it
            let file_size = examine_file_attributes(&current_path)?;
            *counter += 1;
            *total_size += file_size;
        }
    }
    
    Ok(())
}

fn examine_file_attributes(file_path: &Path) -> Result<u64, Box<dyn std::error::Error>> {
    // Get file metadata to examine attributes
    let md = fs::metadata(file_path)?;
    
    // Return the file size in bytes
    Ok(md.len())
}
