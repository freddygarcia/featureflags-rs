use clap::{Parser, Subcommand};
use reqwest::Client;
use serde_json::json;

const DEFAULT_SERVER: &str = "http://localhost:8080";

#[derive(Parser)]
#[command(name = "ffctl")]
#[command(about = "FeatureFlags CLI")]
struct Cli {
    #[arg(long, env = "FF_SERVER_URL", default_value = DEFAULT_SERVER)]
    server: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new flag
    Create {
        name: String,
        #[arg(long)]
        description: Option<String>,
    },
    /// List all flags
    List,
    /// Enable a flag
    Enable { name: String },
    /// Disable a flag
    Disable { name: String },
    /// Add a rule to a flag
    Rule {
        #[command(subcommand)]
        sub: RuleSub,
    },
}

#[derive(Subcommand)]
enum RuleSub {
    Add {
        name: String,
        #[arg(long)]
        attribute: String,
        #[arg(long, value_parser = ["eq", "percent"])]
        operator: String,
        #[arg(long)]
        value: String,
        #[arg(long, default_value = "true")]
        enabled: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let base = cli.server.trim_end_matches('/');
    let client = Client::new();

    match cli.command {
        Commands::Create { name, description } => {
            let body = json!({
                "description": description.unwrap_or_default(),
                "enabled": true,
                "rules": []
            });
            let res = client
                .post(format!("{}/flags/{}", base, name))
                .json(&body)
                .send()
                .await?;
            if res.status().is_success() {
                println!("Created flag '{}'", name);
            } else {
                eprintln!("Error: {}", res.text().await?);
                std::process::exit(1);
            }
        }
        Commands::List => {
            let res = client.get(format!("{}/flags", base)).send().await?;
            let text = res.text().await?;
            let data: serde_json::Value = serde_json::from_str(&text)?;
            let empty: Vec<serde_json::Value> = vec![];
            let flags = data["flags"].as_array().unwrap_or(&empty);
            if flags.is_empty() {
                println!("No flags.");
                return Ok(());
            }
            println!("{:<20} {:<8} DESCRIPTION", "NAME", "ENABLED");
            println!("{}", "-".repeat(50));
            for f in flags {
                let name = f["name"].as_str().unwrap_or("-");
                let enabled = if f["enabled"].as_bool().unwrap_or(false) {
                    "yes"
                } else {
                    "no"
                };
                let desc = f["description"].as_str().unwrap_or("");
                println!("{:<20} {:<8} {}", name, enabled, desc);
            }
        }
        Commands::Enable { name } => toggle(&client, base, &name, true).await?,
        Commands::Disable { name } => toggle(&client, base, &name, false).await?,
        Commands::Rule { sub } => match sub {
            RuleSub::Add {
                name,
                attribute,
                operator,
                value,
                enabled,
            } => {
                let flag = get_flag(&client, base, &name).await?;
                let mut rules = flag["rules"].as_array().cloned().unwrap_or_default();
                let value_json: serde_json::Value = if operator == "percent" {
                    json!(value.parse::<u64>().unwrap_or(0))
                } else {
                    json!(value)
                };
                rules.push(json!({
                    "attribute": attribute,
                    "operator": operator,
                    "value": value_json,
                    "enabled": enabled,
                    "variant": null
                }));
                let body = json!({
                    "description": flag["description"].as_str().unwrap_or(""),
                    "enabled": flag["enabled"].as_bool().unwrap_or(true),
                    "rules": rules
                });
                let res = client
                    .post(format!("{}/flags/{}", base, name))
                    .json(&body)
                    .send()
                    .await?;
                if res.status().is_success() {
                    println!("Added rule to '{}'", name);
                } else {
                    eprintln!("Error: {}", res.text().await?);
                    std::process::exit(1);
                }
            }
        },
    }
    Ok(())
}

async fn get_flag(
    client: &Client,
    base: &str,
    name: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let res = client.get(format!("{}/flags", base)).send().await?;
    let data: serde_json::Value = serde_json::from_str(&res.text().await?)?;
    let flags = data["flags"].as_array().ok_or("invalid response")?;
    let flag = flags
        .iter()
        .find(|f| f["name"].as_str() == Some(name))
        .ok_or_else(|| format!("flag '{}' not found", name))?;
    Ok(flag.clone())
}

async fn toggle(
    client: &Client,
    base: &str,
    name: &str,
    enabled: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let flag = get_flag(client, base, name).await?;
    let rules = flag["rules"].as_array().cloned().unwrap_or_default();
    let body = json!({
        "description": flag["description"].as_str().unwrap_or(""),
        "enabled": enabled,
        "rules": rules
    });
    let res = client
        .post(format!("{}/flags/{}", base, name))
        .json(&body)
        .send()
        .await?;
    if res.status().is_success() {
        println!("{} '{}'", if enabled { "Enabled" } else { "Disabled" }, name);
    } else {
        eprintln!("Error: {}", res.text().await?);
        std::process::exit(1);
    }
    Ok(())
}
