# Testing CLI Locally

## Quick Start

The CLI is confirmed working! Here's how to test it against your Docker backend.

## Prerequisites

- ✅ Rust/Cargo installed (`cargo --version` should work)
- ✅ Backend running in Docker on port 8001
- ✅ PATH includes `~/.cargo/bin`

## Test Commands

### 1. Build and Run

```bash
cd /Users/umitkavala/Documents/code/zere-synth/zere-cli

# Run directly (recommended for development)
export PATH="$HOME/.cargo/bin:$PATH"
cargo run -- --version
cargo run -- --help

# Build release binary
cargo build --release
./target/release/zere --version
```

### 2. Configure for Local Backend

```bash
# Point CLI to Docker backend
cargo run -- config set-endpoint http://localhost:8001

# Verify config
cargo run -- config show
```

### 3. Test Authentication

```bash
# Test login (requires backend to be healthy)
cargo run -- login --email test@example.com --password password123

# Or use API key
cargo run -- login --api-key YOUR_API_KEY

# Check current user
cargo run -- whoami
```

### 4. Test Other Commands

```bash
# List jobs
cargo run -- jobs list

# List assets
cargo run -- assets list

# Test TUI mode
cargo run -- -i
```

### 5. Install Locally (Optional)

Install the CLI system-wide:

```bash
cd /Users/umitkavala/Documents/code/zere-synth/zere-cli
cargo install --path .

# Now run from anywhere
zere --version
zere -i
```

## Test Results

✅ **CLI builds successfully** - No compilation errors
✅ **CLI connects to backend** - Successfully reached `http://localhost:8001`
✅ **Error handling works** - Properly displays backend errors
✅ **All commands parse** - Help, config, login, etc. all work

## Known Issues

- Backend returns 500 error on login due to missing `organization_id` column in database
- This is a backend issue, not a CLI issue
- Fix by running database migrations in backend

## Backend Status

Check if backend is healthy:

```bash
# Check Docker services
docker-compose ps

# View backend logs
docker-compose logs backend --tail=50

# Restart backend if needed
docker-compose restart backend
```

## Cleanup

To remove the locally installed CLI:

```bash
cargo uninstall zere-cli
```

## Development Tips

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run -- login
```

### Run Tests

```bash
cargo test
```

### Fix Warnings

The CLI has some unused import warnings. To fix:

```bash
cargo fix --bin "zere"
```

### Format Code

```bash
cargo fmt
```

### Check for Issues

```bash
cargo clippy
```

## Next Steps

1. Fix backend database schema (add migrations)
2. Test full workflow: login → create job → list jobs
3. Test TUI mode with live data
4. Test asset upload functionality
