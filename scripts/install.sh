#!/bin/bash

set -e

# Detect operating system
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

# Detect architecture
ARCH=$(uname -m)

# Map architecture to Rust target triples
case "$ARCH" in
    "x86_64")
        ARCH="x86_64"
        ;;
    "arm64" | "aarch64")
        ARCH="arm64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Define target triple
case "$OS" in
    "linux")
        TARGET="$ARCH-unknown-linux-gnu"
        INSTALL_DIR="/usr/local/bin"
        ;;
    "darwin")
        TARGET="$ARCH-apple-darwin"
        INSTALL_DIR="/usr/local/bin"
        ;;
    "msys"* | "cygwin"* | "mingw"*)
        TARGET="$ARCH-pc-windows-msvc"
        INSTALL_DIR="C:/tools" # Adjust as needed or make this configurable
        ;;
    *)
        echo "Unsupported operating system: $OS"
        exit 1
        ;;
esac

# Define download URL
VERSION="v0.1.4"
FILENAME="surrealdb-backup-${VERSION}-${TARGET}.zip"
DOWNLOAD_URL="https://github.com/maxwellflitton/surrealdb-backup-cli/releases/download/v/$FILENAME"
# Download the zip file
echo "Downloading $FILENAME..."
curl -LO "$DOWNLOAD_URL"

# Unzip the downloaded file
echo "Unzipping $FILENAME..."
unzip -o "$FILENAME"

echo "Cleaning up: Removing $FILENAME..."
rm -f "$FILENAME"
