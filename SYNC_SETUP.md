# Setting Up Public Repository Sync

This guide walks you through the one-time setup to enable automatic syncing from the private monorepo to the public `zeredata-cli` repository.

## Prerequisites

- ✅ Public repository created: https://github.com/umitkavala/zeredata-cli
- ✅ You have admin access to both repositories

## Step 1: Create Personal Access Token (PAT)

1. Go to GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Click "Generate new token (classic)"
3. Give it a name: `zeredata-cli-sync`
4. Set expiration: 90 days or No expiration
5. Select scopes:
   - ✅ `repo` (Full control of private repositories)
   - ✅ `workflow` (Update GitHub Action workflows)
6. Click "Generate token"
7. **IMPORTANT:** Copy the token immediately (you won't see it again)

## Step 2: Add Secret to Private Repository

1. Go to private repository: https://github.com/umitkavala/zere-synth
2. Settings → Secrets and variables → Actions
3. Click "New repository secret"
4. Name: `PUBLIC_REPO_PAT`
5. Value: Paste the token from Step 1
6. Click "Add secret"

## Step 3: Initial Manual Sync

Before the automated workflow works, we need to do an initial sync manually:

```bash
# Clone the public repository
cd /tmp
git clone git@github.com:umitkavala/zeredata-cli.git
cd zeredata-cli

# Copy files from private monorepo
cp -r /Users/umitkavala/Documents/code/zere-synth/zere-cli/* .

# Remove build artifacts
rm -rf target/
rm -f .env

# Create initial README note
cat > SYNC_NOTE.md << 'EOF'
# 📦 ZereData CLI

> **Note:** This is the public distribution repository. Development happens in our private monorepo and is automatically synced here.

For the full documentation, see [README.md](README.md).
EOF

# Commit and push
git add .
git commit -m "Initial sync from private monorepo"
git push origin main
```

## Step 4: Test Automatic Sync

Now test that the workflow works:

1. Make a small change in the private repo:
   ```bash
   cd /Users/umitkavala/Documents/code/zere-synth/zere-cli
   echo "# Test sync" >> README.md
   git add README.md
   git commit -m "test: verify auto-sync to public repo"
   git push
   ```

2. Watch the workflow:
   - Go to: https://github.com/umitkavala/zere-synth/actions
   - Look for "Sync CLI to Public Repository"
   - It should run automatically within a few seconds

3. Verify in public repo:
   - Go to: https://github.com/umitkavala/zeredata-cli
   - The change should appear within ~30 seconds

4. Revert test change if desired:
   ```bash
   cd /Users/umitkavala/Documents/code/zere-synth/zere-cli
   git revert HEAD
   git push
   ```

## Step 5: Add GitHub Actions to Public Repo

The public repo needs its own workflows for releases:

1. In the public repo, create `.github/workflows/` directory
2. Copy the release workflow:
   ```bash
   cd /tmp/zeredata-cli
   mkdir -p .github/workflows
   cp /Users/umitkavala/Documents/code/zere-synth/.github/workflows/release-cli.yml \
      .github/workflows/release.yml
   git add .github/
   git commit -m "ci: add release workflow"
   git push
   ```

## Step 6: Test Release Process

Create a test release to verify everything works:

1. In the public repo, create a tag:
   ```bash
   cd /tmp/zeredata-cli
   git tag cli-v0.1.0
   git push origin cli-v0.1.0
   ```

2. Watch GitHub Actions:
   - Go to: https://github.com/umitkavala/zeredata-cli/actions
   - The "Release Zere CLI" workflow should start
   - It will build binaries for all platforms

3. Check the release:
   - Go to: https://github.com/umitkavala/zeredata-cli/releases
   - You should see "cli-v0.1.0" with binary attachments

## Step 7: Test Installation

Test that users can install from the public repo:

**Test install script:**
```bash
# In a new terminal (to ensure clean environment)
curl -sSL https://raw.githubusercontent.com/umitkavala/zeredata-cli/main/install.sh | bash
```

**Test cargo install:**
```bash
cargo install --git https://github.com/umitkavala/zeredata-cli
```

**Verify:**
```bash
zere --version
# Should output: zere-cli 0.1.0
```

## Workflow Diagram

```
Private Monorepo (zere-synth)
  └── zere-cli/
      ├── Developer makes changes
      ├── Commits and pushes to main
      └── Triggers: sync-cli-public.yml
          ↓
          [GitHub Actions]
          ├── Checks out private repo
          ├── Checks out public repo
          ├── Copies zere-cli/* → public repo
          ├── Removes build artifacts
          └── Commits and pushes
              ↓
Public Repository (zeredata-cli)
  ├── Receives synced code
  ├── On tag push (cli-v*):
  │   └── Triggers: release.yml
  │       ├── Builds for all platforms
  │       ├── Creates GitHub Release
  │       └── Publishes Docker image
  └── Users install from here
```

## Troubleshooting

### Sync workflow fails with "Permission denied"

**Problem:** The PUBLIC_REPO_PAT doesn't have the right permissions

**Solution:**
1. Regenerate the PAT with `repo` and `workflow` scopes
2. Update the secret in GitHub Settings

### Sync workflow doesn't trigger

**Problem:** The `paths` filter doesn't match

**Solution:**
1. Check that changes were in `zere-cli/**`
2. Try manually triggering: Actions → Sync CLI → Run workflow

### Files missing in public repo

**Problem:** The sync script might have exclusion patterns

**Solution:**
1. Check `.github/workflows/sync-cli-public.yml`
2. Verify the `rsync` or `cp` command includes all needed files

### Release workflow fails

**Problem:** Missing secrets or incorrect configuration

**Solution:**
1. Ensure GitHub Actions is enabled in public repo
2. Check that GITHUB_TOKEN has sufficient permissions
3. For Docker publishing, ensure container registry permissions

## Maintenance

### Updating the Sync Workflow

If you need to change what gets synced:

1. Edit `.github/workflows/sync-cli-public.yml` in private repo
2. Test with workflow_dispatch (manual trigger)
3. Commit changes

### Rotating the PAT

PATs expire. To rotate:

1. Generate new PAT (same steps as above)
2. Update `PUBLIC_REPO_PAT` secret in private repo
3. Delete old PAT from GitHub settings

### Handling Public Contributions

When someone submits a PR to the public repo:

1. Review the PR on GitHub
2. If approved, manually apply changes to private repo
3. Commit in private repo with attribution
4. Sync will push changes back to public repo
5. Close the public PR with a thank you note

## Security Checklist

Before first sync, verify:

- [ ] No secrets in code (API keys, passwords, etc.)
- [ ] No internal URLs (staging servers, etc.)
- [ ] No customer data or proprietary examples
- [ ] No internal documentation
- [ ] LICENSE file is present (MIT)
- [ ] README doesn't mention private infrastructure

## Success Criteria

✅ Sync workflow runs successfully
✅ Public repo receives updates within 1 minute
✅ Install script downloads from public releases
✅ Users can install without authentication
✅ GitHub Releases work for tagged versions
✅ Docker images publish to GHCR

## Next Steps After Setup

1. [ ] Publish to crates.io (see CRATES_IO_PUBLISH.md)
2. [ ] Create Homebrew tap repository
3. [ ] Add badges to README (build status, crates.io version, etc.)
4. [ ] Set up issue templates in public repo
5. [ ] Add CONTRIBUTING.md to public repo
6. [ ] Announce public availability

## Contact

Questions? Open an issue in the public repo or contact:
- Email: umit@zeredata.com
- GitHub: @umitkavala
