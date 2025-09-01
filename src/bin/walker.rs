use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::VecDeque;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Get the directory path from command line argument or use current directory
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };
    
    let mut counter = 0;
    
    match walk_directory_iterative(Path::new(path), &mut counter) {
        Ok(_) => println!("Directory walk completed. Total files examined: {}", counter),
        Err(e) => eprintln!("Error walking directory: {}", e),
    }
}

fn walk_directory_iterative(start_path: &Path, counter: &mut usize) -> Result<(), Box<dyn std::error::Error>> {
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
                    examine_file_attributes(&path)?;
                    *counter += 1;
                }
            }
        } else {
            // If the current path is a file, just examine it
            examine_file_attributes(&current_path)?;
            *counter += 1;
        }
    }
    
    Ok(())
}

fn examine_file_attributes(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Get file metadata to examine attributes
    let _md = fs::metadata(file_path)?;
        
    Ok(())
}
