# Zere CLI

A Rust-based command-line interface for the ZereData platform - synthetic data generation for warehouse robotics and computer vision.

## Features

- 🔐 **Authentication** - Login with email/password or API key
- 📦 **Asset Management** - Upload, list, and manage 3D assets
- 🎬 **Job Management** - Create and monitor rendering jobs
- 📊 **Dataset Management** - Download generated datasets
- 🖥️ **TUI Mode** - Interactive terminal UI with live dashboard

## Installation

### Quick Install (Recommended)

**macOS and Linux:**
```bash
curl -sSL https://raw.githubusercontent.com/umitkavala/zeredata-cli/main/install.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/umitkavala/zeredata-cli/main/install.ps1 | iex
```

This will download the latest binary and install it to your PATH.

### Homebrew (macOS/Linux)

```bash
brew tap umitkavala/zere
brew install zere-cli
```

### Cargo (Rust Package Manager)

If you have Rust installed:
```bash
cargo install zere-cli
```

Or from this repository:
```bash
cargo install --git https://github.com/umitkavala/zeredata-cli
```

### Download Binary

Download pre-built binaries from [GitHub Releases](https://github.com/umitkavala/zeredata-cli/releases):

- **macOS (Apple Silicon)**: `zere-darwin-arm64`
- **macOS (Intel)**: `zere-darwin-amd64`
- **Linux (x86_64)**: `zere-linux-amd64`
- **Linux (ARM64)**: `zere-linux-arm64`
- **Windows (x64)**: `zere-windows-amd64.exe`

After downloading, make it executable and move to PATH:

**macOS/Linux:**
```bash
chmod +x zere-*
sudo mv zere-* /usr/local/bin/zere
```

**Windows:**
Move to `C:\Program Files\Zere\` and add to PATH.

### Build from Source

**Prerequisites:**
- Rust 1.70+ (install from https://rustup.rs)

**Build and Install:**
```bash
git clone https://github.com/umitkavala/zeredata-cli.git
cd zeredata-cli
cargo build --release
sudo cp target/release/zere /usr/local/bin/
```

Or install directly:
```bash
cargo install --path .
```

### Docker

Run without installing:
```bash
docker run --rm -it -v ~/.config/zere:/root/.config/zere \
  ghcr.io/umitkavala/zeredata-cli:latest --help
```

Create an alias for convenience:
```bash
alias zere='docker run --rm -it -v ~/.config/zere:/root/.config/zere ghcr.io/umitkavala/zeredata-cli:latest'
```

### Verify Installation

```bash
zere --version
```

## Usage

### Authentication

#### 🔒 Secure Login (Recommended)

```bash
# Interactive login - Password hidden, not saved in history
zere login
# Prompts:
#   Email: user@example.com
#   Password: ******* (hidden input)

# Show current user
zere whoami

# Logout
zere logout
```

#### 🔑 API Key Authentication (For Automation)

```bash
# Using environment variable (RECOMMENDED for scripts/CI)
export ZERE_API_KEY=your_api_key_here
zere login

# Using flag (visible in process list - use with caution)
zere login --api-key YOUR_API_KEY

# One-liner for scripts
ZERE_API_KEY=your_key zere jobs list
```

#### ⚠️ Insecure Methods (NOT Recommended)

```bash
# ❌ INSECURE: Password visible in command history
zere login --email user@example.com --password mypassword
# CLI will show security warning!

# ✅ BETTER: Use interactive prompt instead
zere login --email user@example.com
# Prompts: Password: ******* (hidden)
```

#### Security Best Practices

**For Interactive Use:**
1. ✅ Use `zere login` without flags
2. ✅ Password input is hidden
3. ✅ Token stored securely in `~/.config/zere/config.toml` (600 permissions)
4. ✅ No password in command history

**For Automation/CI:**
1. ✅ Use environment variable: `ZERE_API_KEY`
2. ✅ Generate API key from web dashboard
3. ✅ Store in CI secrets (GitHub Actions, GitLab CI, etc.)
4. ✅ Never commit API keys to git

**What NOT to Do:**
- ❌ Never use `--password` flag in production
- ❌ Never commit credentials to git repositories
- ❌ Never share API keys in screenshots/logs
- ❌ Never use `--password` in shell scripts (visible in `ps aux`)

#### Configuration File

Credentials stored in:
```
~/.config/zere/config.toml (macOS/Linux)
%APPDATA%\zere\config.toml (Windows)
```

File permissions automatically set to `600` (owner read/write only).

```toml
[api]
endpoint = "http://localhost:8001"

[auth]
api_key = "your_token_here"  # Saved securely after login
```

### Asset Management

```bash
# List all assets
zere assets list

# Upload an asset
zere assets upload model.fbx --name "Robot Arm" --category industrial

# Upload multiple files with tags
zere assets upload *.fbx --tags robot warehouse

# Get asset information
zere assets info ASSET_ID

# Delete an asset
zere assets delete ASSET_ID
```

### Job Management

```bash
# List all jobs
zere jobs list

# Create a new job
zere jobs create --name "Warehouse Test" --num-scenes 100

# Create job with config file
zere jobs create --name "Custom Job" --num-scenes 500 --config config.yaml

# Quick Start - Generate without assets (30 seconds to start!)
zere jobs quick-start --num-scenes 100

# Quick Start with custom parameters
zere jobs quick-start --num-scenes 500 --objects 20-30 --environment warehouse_shelf

# Get job status
zere jobs status JOB_ID

# Watch job progress in real-time
zere jobs watch JOB_ID

# Cancel a running job
zere jobs cancel JOB_ID
```

#### Quick Start Mode

**New!** Generate synthetic data in 30 seconds without uploading assets:

```bash
# Generate 100 warehouse scenes with procedural objects
zere jobs quick-start -s 100

# Customize object count (25-35 objects per scene)
zere jobs quick-start -s 100 --objects 25-35

# Choose environment type
zere jobs quick-start -s 100 --environment floor
```

**Available environments:**
- `warehouse_shelf` - Warehouse bin picking (default)
- `floor` - Floor-based scene
- `table` - Tabletop scene

**What's generated:**
- Procedural objects (boxes, bottles, pouches, cylinders)
- Realistic materials with color variation
- Physics-based object placement
- Complete annotations (YOLO, COCO, BOP formats)

**Perfect for:**
- Testing the platform
- Quick demos
- Prototyping ML pipelines
- Benchmarking rendering performance
```

### Dataset Management

```bash
# List all datasets
zere datasets list

# Get dataset information
zere datasets info DATASET_ID

# Download a dataset
zere datasets download JOB_ID --output ./my-dataset.zip
```

### Interactive TUI Mode

```bash
# Launch interactive terminal UI
zere --interactive

# Or use the short form
zere -i
```

**TUI Features:**
- **Dashboard** - Overview with job statistics and gauges
- **Jobs View** - Sortable table of all rendering jobs
- **Assets View** - Browse uploaded assets with size info
- **Help** - Keyboard shortcuts reference

**Keyboard Shortcuts:**
- `Tab` / `Shift+Tab` - Switch between views
- `↑↓` or `j/k` - Navigate lists
- `r` - Refresh data from API
- `c` - Create new job (opens wizard)
- `/` - Toggle search mode (fuzzy search)
- `?` - Toggle help screen
- `q` - Quit

**Job Creation Wizard:**
Press `c` to launch an interactive 4-step wizard:
1. **Job Name** - Enter descriptive name
2. **Scene Count** - Number of scenes to generate
3. **Config File** - Optional YAML config path
4. **Confirm** - Review and submit

Navigate with Tab/Shift+Tab, type to enter values, Enter to submit, Esc to cancel.

**Fuzzy Search:**
Press `/` to activate search mode, then type to filter jobs or assets.
- Matches on name, status, file type
- Results sorted by match score
- Supports partial and fuzzy matching
- Esc to clear search

Data auto-refreshes every 5 seconds.

### Configuration

```bash
# Set API endpoint (for self-hosted or local development)
zere config set-endpoint http://localhost:8000

# Show current configuration
zere config show
```

## Configuration File

The CLI stores configuration in `~/.config/zere/config.toml` (or OS-specific config directory):

```toml
[api]
endpoint = "https://api.zeredata.com"

[auth]
api_key = "your_api_key_here"
```

## Development

### Project Structure

```
zere-cli/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── error.rs             # Error types
│   ├── config.rs            # Configuration management
│   ├── api/                 # API client
│   │   ├── mod.rs
│   │   ├── client.rs        # HTTP client wrapper
│   │   ├── auth.rs          # Auth endpoints
│   │   ├── assets.rs        # Asset endpoints
│   │   ├── jobs.rs          # Job endpoints
│   │   └── datasets.rs      # Dataset endpoints
│   ├── commands/            # CLI commands
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── assets.rs
│   │   ├── jobs.rs
│   │   ├── datasets.rs
│   │   └── config.rs
│   └── tui/                 # Terminal UI (coming soon)
│       └── ...
├── Cargo.toml
└── README.md
```

### Build & Test

```bash
# Build
cargo build

# Run with logging
RUST_LOG=debug cargo run -- --help

# Test a command
cargo run -- login --api-key test_key

# Run tests
cargo test
```

## Roadmap

### Phase 1 ✅
- [x] Project setup with dependencies
- [x] Config file handling
- [x] Basic API client structure
- [x] Error types

### Phase 2 ✅
- [x] Auth commands (login, logout, whoami)
- [x] Asset commands (list, upload, delete, info)
- [x] Job commands (create, list, status, watch, cancel)
- [x] Dataset commands (list, download, info)

### Phase 3 ✅
- [x] Basic TUI app scaffold with ratatui
- [x] Dashboard view with statistics and gauges
- [x] Asset browser with size formatting
- [x] Job list view with status highlighting
- [x] Help overlay
- [x] Keyboard navigation and auto-refresh

### Phase 4 ✅
- [x] Job creation wizard (interactive, multi-step)
- [x] Fuzzy search in TUI with match scoring
- [x] Enhanced error messages with status popups
- [x] Improved help system with detailed docs

### Phase 5 (Planned)
- [ ] Asset upload from TUI
- [ ] Real-time job progress monitoring
- [ ] Job cancellation from TUI
- [ ] Configuration editor

## Contributing

This CLI is part of the ZereData platform. For backend API integration, ensure the backend server is running at the configured endpoint.

## License

MIT

## Related

- Backend API: `../backend/`
- Frontend: `../frontend/`
- Documentation: `../docs/`
