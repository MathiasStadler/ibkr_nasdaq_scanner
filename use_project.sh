# 1. Run the setup script
chmod +x setup.sh
./setup.sh

# 2. Install dependencies
./scripts/setup_dependencies.sh

# 3. Configure your .env file
# Edit .env with your IBKR credentials

# 4. Test connection
./scripts/test_connection.sh

# 5. Build and run
cargo build --release
cargo run

# Or run the release binary
./target/release/ibkr_nasdaq_scanner