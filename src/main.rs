use clap::Parser;

#[cfg(target_os = "linux")]
use {
    std::path::Path,
    rustix::fs::{statx, AtFlags, StatxFlags, CWD},
    chrono::DateTime,
};

#[derive(Parser)]
#[command(name = "statx-sample")]
#[command(about = "A simple program to get file metadata using statx API")]
struct Args {
    /// Path to the file to stat
    file_path: String,
}

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let path = Path::new(&args.file_path);
    
    // Call statx to get file metadata
    // We want modified time and size
    let stat_result = statx(
        CWD,                    // Use current working directory as base
        path,                   // Path to the file
        AtFlags::empty(),       // No special AT flags
        StatxFlags::MTIME | StatxFlags::SIZE, // Request mtime and size
    )?;
    
    // Print file information
    println!("File: {}", args.file_path);
    
    // Print file size
    println!("Size: {} bytes", stat_result.stx_size);
    
    // Convert modified time to human-readable format
    let mtime_secs = stat_result.stx_mtime.tv_sec;
    let mtime_nsecs = stat_result.stx_mtime.tv_nsec;
    
    // Create DateTime from timestamp
    let datetime = DateTime::from_timestamp(mtime_secs, mtime_nsecs as u32)
        .ok_or("Invalid timestamp")?;
    
    println!("Modified time: {} (UTC)", datetime.format("%Y-%m-%d %H:%M:%S%.9f"));
    println!("Modified time (timestamp): {}.{:09}", mtime_secs, mtime_nsecs);
    
    // Print which fields are valid (statx may not always return all requested fields)
    println!("\nValid fields in statx result:");
    if stat_result.stx_mask & StatxFlags::SIZE.bits() != 0 {
        println!("  - Size: valid");
    } else {
        println!("  - Size: not available");
    }
    
    if stat_result.stx_mask & StatxFlags::MTIME.bits() != 0 {
        println!("  - Modified time: valid");
    } else {
        println!("  - Modified time: not available");
    }
    
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("This program only works on Linux, as it uses the statx system call.");
    std::process::exit(1);
}
