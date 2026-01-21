#!/bin/bash
# setup_dependencies.sh - Install required dependencies

set -e

echo "Setting up IBKR Option Scanner dependencies..."

# Check for Rust installation
if ! command -v rustc &> /dev/null; then
    echo "Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "✅ Rust already installed"
fi

# Update Rust
echo "Updating Rust..."
rustup update

# Install required tools
echo "Installing additional tools..."

# Install jq for JSON parsing in scripts
if ! command -v jq &> /dev/null; then
    echo "Installing jq..."
    if [[ "$OSTYPE" == "darwin"* ]]; then
        brew install jq
    else
        sudo apt-get update && sudo apt-get install -y jq
    fi
else
    echo "✅ jq already installed"
fi

# Install csvkit for CSV processing (optional)
echo "To install CSV processing tools:"
echo "  pip install csvkit"

# Create log directories
echo "Creating log directories..."
mkdir -p ../logs/options

# Make scripts executable
chmod +x ../scripts/*.sh

echo ""
echo "✅ Dependencies setup complete!"
echo ""
echo "Next steps:"
echo "1. Edit the .env file with your IBKR credentials"
echo "2. Run: ./scripts/test_connection.sh to test IBKR Gateway"
echo "3. Build the project: cargo build --release"
echo "4. Run the scanner: cargo run"
