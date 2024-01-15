
cargo build
cargo run -- generate --language python --library target/debug/libuniffi_trait.so --out-dir target/debug/
PYTHONPATH=target/debug/ python3 test.py