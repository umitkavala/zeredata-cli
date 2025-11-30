#!/bin/bash
set -e

# Zere CLI Installation Script
# Usage: curl -sSL https://raw.githubusercontent.com/umitkavala/zeredata-cli/main/install.sh | bash

REPO="umitkavala/zeredata-cli"
BINARY_NAME="zere"
INSTALL_DIR="/usr/local/bin"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Installing Zere CLI...${NC}"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $OS in
    darwin)
        OS_TYPE="darwin"
        ;;
    linux)
        OS_TYPE="linux"
        ;;
    *)
        echo -e "${RED}Unsupported OS: $OS${NC}"
        exit 1
        ;;
esac

case $ARCH in
    x86_64|amd64)
        ARCH_TYPE="amd64"
        ;;
    aarch64|arm64)
        ARCH_TYPE="arm64"
        ;;
    *)
        echo -e "${RED}Unsupported architecture: $ARCH${NC}"
        exit 1
        ;;
esac

BINARY_FILE="${BINARY_NAME}-${OS_TYPE}-${ARCH_TYPE}"
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${BINARY_FILE}"

echo -e "${YELLOW}Detected: ${OS_TYPE}-${ARCH_TYPE}${NC}"
echo -e "${YELLOW}Download URL: ${DOWNLOAD_URL}${NC}"

# Create temporary directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Download binary
echo -e "${YELLOW}Downloading Zere CLI...${NC}"
if command -v curl &> /dev/null; then
    curl -L -o "$BINARY_NAME" "$DOWNLOAD_URL"
elif command -v wget &> /dev/null; then
    wget -O "$BINARY_NAME" "$DOWNLOAD_URL"
else
    echo -e "${RED}Error: curl or wget required${NC}"
    exit 1
fi

# Make executable
chmod +x "$BINARY_NAME"

# Install binary
echo -e "${YELLOW}Installing to ${INSTALL_DIR}...${NC}"
if [ -w "$INSTALL_DIR" ]; then
    mv "$BINARY_NAME" "$INSTALL_DIR/"
else
    echo -e "${YELLOW}Requesting sudo access to install to ${INSTALL_DIR}${NC}"
    sudo mv "$BINARY_NAME" "$INSTALL_DIR/"
fi

# Cleanup
cd -
rm -rf "$TMP_DIR"

# Verify installation
if command -v zere &> /dev/null; then
    VERSION=$(zere --version 2>&1 || echo "unknown")
    echo -e "${GREEN}✓ Zere CLI installed successfully!${NC}"
    echo -e "${GREEN}Version: ${VERSION}${NC}"
    echo ""
    echo -e "${YELLOW}Get started:${NC}"
    echo "  zere login              # Login to your account"
    echo "  zere --interactive      # Launch TUI mode"
    echo "  zere --help             # Show all commands"
else
    echo -e "${RED}Installation failed. Please check your PATH.${NC}"
    exit 1
fi
