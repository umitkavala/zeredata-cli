.PHONY: help build install test clean release docker run

help:
	@echo "Zere CLI - Development Tasks"
	@echo ""
	@echo "Available commands:"
	@echo "  make build       - Build debug binary"
	@echo "  make release     - Build optimized release binary"
	@echo "  make install     - Install to /usr/local/bin"
	@echo "  make test        - Run tests"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make docker      - Build Docker image"
	@echo "  make run         - Run CLI in development mode"
	@echo "  make publish     - Publish to crates.io"

build:
	@echo "Building debug binary..."
	cargo build

release:
	@echo "Building release binary..."
	cargo build --release
	@echo "Binary available at: target/release/zere"

install: release
	@echo "Installing to /usr/local/bin..."
	sudo cp target/release/zere /usr/local/bin/
	@echo "Installed! Run 'zere --version' to verify."

test:
	@echo "Running tests..."
	cargo test

clean:
	@echo "Cleaning build artifacts..."
	cargo clean

docker:
	@echo "Building Docker image..."
	docker build -t zere-cli:latest .

run:
	@echo "Running in development mode..."
	cargo run -- --help

publish:
	@echo "Publishing to crates.io..."
	cargo publish

# Development shortcuts
dev-tui:
	cargo run -- --interactive

dev-login:
	cargo run -- login

dev-jobs:
	cargo run -- jobs list
