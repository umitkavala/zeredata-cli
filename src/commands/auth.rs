use crate::api::ApiClient;
use crate::config::Config;
use crate::error::Result;
use console::style;

pub async fn login(email: Option<String>, password: Option<String>, api_key: Option<String>) -> Result<()> {
    let mut config = Config::load()?;

    let token = if let Some(key) = api_key {
        // Use provided API key directly
        println!("{}", style("Using provided API key...").cyan());
        key
    } else {
        // Prompt for email/password if not provided
        let email = email.or_else(|| {
            inquire::Text::new("Email:")
                .prompt()
                .ok()
        }).ok_or_else(|| crate::error::CliError::Auth("Email required".to_string()))?;

        let password = password.or_else(|| {
            inquire::Password::new("Password:")
                .without_confirmation()
                .prompt()
                .ok()
        }).ok_or_else(|| crate::error::CliError::Auth("Password required".to_string()))?;

        // Login via API
        println!("{}", style("Logging in...").cyan());
        let client = ApiClient::new(config.api.endpoint.clone(), None)?;
        client.login(email, password).await?
    };

    // Save API key to config
    config.set_api_key(Some(token));
    config.save()?;

    println!("{}", style("✓ Logged in successfully").green().bold());
    Ok(())
}

pub async fn logout() -> Result<()> {
    let mut config = Config::load()?;

    if !config.is_authenticated() {
        println!("{}", style("Not currently logged in").yellow());
        return Ok(());
    }

    // Try to logout on server (best effort)
    if let Ok(client) = ApiClient::from_config(&config) {
        let _ = client.logout().await;
    }

    // Clear local credentials
    config.set_api_key(None);
    config.save()?;

    println!("{}", style("✓ Logged out successfully").green().bold());
    Ok(())
}

pub async fn whoami() -> Result<()> {
    let config = Config::load()?;

    if !config.is_authenticated() {
        println!("{}", style("Not logged in. Run 'zere login' first.").yellow());
        return Ok(());
    }

    let client = ApiClient::from_config(&config)?;
    let user = client.whoami().await?;

    println!("\n{}", style("User Information").bold().underlined());
    println!("  {} {}", style("Email:").dim(), user.email);
    println!("  {} {}", style("User ID:").dim(), user.id);

    if let Some(org_name) = user.organization_name {
        println!("  {} {}", style("Organization:").dim(), org_name);
    }

    if let Some(role) = user.role {
        println!("  {} {}", style("Role:").dim(), role);
    }

    println!("  {} {}", style("API Endpoint:").dim(), config.api.endpoint);
    println!();

    Ok(())
}
