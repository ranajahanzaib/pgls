#!/bin/bash

# Exit on any error
set -e

# Constants
PGLS_REPO="https://github.com/ranajahanzaib/pgls.git"
INSTALL_DIR="/usr/local/bin"
RUST_VERSION="1.80" # Adjust to the required Rust version

# Function to check and install Rust if not already installed
install_rust() {
    if ! command -v rustup &>/dev/null; then
        echo "Rust is not installed. Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    else
        echo "Rust is already installed."
    fi

    echo "Ensuring required Rust version is installed..."
    rustup install $RUST_VERSION
    rustup default $RUST_VERSION
}

# Clone the pgls repository
clone_pgls_repo() {
    if [ -d "pgls" ]; then
        echo "Removing existing pgls directory..."
        rm -rf pgls
    fi

    echo "Cloning pgls repository..."
    git clone $PGLS_REPO
}

# Build pgls with cargo
build_pgls() {
    echo "Building pgls..."
    cd pgls
    cargo build --release
    cd ..
}

# Move the pgls binary to the system PATH
install_pgls_binary() {
    echo "Installing pgls binary to $INSTALL_DIR..."
    sudo mv pgls/target/release/pgls "$INSTALL_DIR"
    echo "Pgls installed successfully and is accessible as 'pgls'."
}

# Main script execution
main() {
    install_rust
    clone_pgls_repo
    build_pgls
    install_pgls_binary

    # Cleanup
    echo "Cleaning up..."
    rm -rf pgls
}

main
