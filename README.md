# statx-sample

A simple Rust program that demonstrates using the Linux `statx` system call via the `rustix` crate to retrieve file metadata.

## Features

- Uses the Linux-specific `statx` system call through the `rustix` crate
- Displays file size and modification timestamp
- Shows both human-readable and raw timestamp formats
- Indicates which metadata fields were successfully retrieved
- Only compiles and runs on Linux systems

## Usage

```bash
# Build for Linux (requires Linux target)
cargo build --target x86_64-unknown-linux-gnu

# Run on Linux system
./target/x86_64-unknown-linux-gnu/debug/statx-sample /path/to/file

# Example output:
# File: /etc/passwd
# Size: 2847 bytes
# Modified time: 2024-01-15 10:30:45.123456789 (UTC)
# Modified time (timestamp): 1705316245.123456789
# 
# Valid fields in statx result:
#   - Size: valid
#   - Modified time: valid
```

## Dependencies

- `rustix` - Linux system call interface
- `clap` - Command line argument parsing
- `chrono` - Date and time handling

## Cross-compilation

To build this program from macOS or other non-Linux systems:

1. Install the Linux target:
   ```bash
   rustup target add x86_64-unknown-linux-gnu
   ```

2. Install a cross-compilation toolchain (e.g., using musl):
   ```bash
   # Install musl cross-compilation tools
   brew install FiloSottile/musl-cross/musl-cross
   ```

3. Configure cargo for cross-compilation in `.cargo/config.toml`:
   ```toml
   [target.x86_64-unknown-linux-musl]
   linker = "x86_64-linux-musl-gcc"
   ```

4. Build with musl target:
   ```bash
   cargo build --target x86_64-unknown-linux-musl
   ```

## Notes

- The `statx` system call is Linux-specific and provides more detailed metadata than traditional `stat`
- This program will display an error message and exit if run on non-Linux systems
- The program specifically requests `MTIME` and `SIZE` fields from `statx` for efficiency
