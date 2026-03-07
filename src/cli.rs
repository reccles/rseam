use crate::api_client::SeamClient;
use crate::commands;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "rseam",
    about = "Rust-based Seam CLI for non-interactive scripting",
    version,
    author
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Output only the ID field (useful for scripting)
    #[arg(long, global = true)]
    pub id_only: bool,

    /// Output raw JSON (default: pretty-printed)
    #[arg(long, global = true)]
    pub raw: bool,

    /// Display comprehensive agent context (markdown) for AI integration
    #[arg(long, global = true)]
    pub help_agent: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Device commands
    Devices {
        #[command(subcommand)]
        command: DeviceCommands,
    },

    /// Access code commands
    AccessCodes {
        #[command(subcommand)]
        command: AccessCodeCommands,
    },

    /// Lock commands
    Locks {
        #[command(subcommand)]
        command: LockCommands,
    },

    /// Health check
    Health {
        #[command(subcommand)]
        command: HealthCommands,
    },

    /// Connect webview commands
    ConnectWebviews {
        #[command(subcommand)]
        command: ConnectWebviewCommands,
    },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum DeviceCommands {
    /// List all devices
    List {
        #[arg(long)]
        device_id: Option<String>,

        #[arg(long)]
        name: Option<String>,
    },

    /// Get a specific device
    Get {
        #[arg(long)]
        device_id: Option<String>,

        #[arg(long)]
        name: Option<String>,
    },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum AccessCodeCommands {
    /// Create an access code
    Create {
        #[arg(long)]
        device_id: String,

        #[arg(long)]
        code: String,

        #[arg(long)]
        name: Option<String>,
    },

    /// List access codes
    List {
        #[arg(long)]
        device_id: Option<String>,
    },

    /// Delete an access code
    Delete {
        #[arg(long)]
        access_code_id: String,
    },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum LockCommands {
    /// Unlock a door
    UnlockDoor {
        #[arg(long)]
        device_id: String,
    },

    /// Lock a door
    LockDoor {
        #[arg(long)]
        device_id: String,
    },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum ConnectWebviewCommands {
    /// Create a connect webview
    Create {
        #[arg(long)]
        accepted_providers: Option<String>,
    },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum HealthCommands {
    /// Get health status
    GetHealth,
}

impl Cli {
    pub async fn execute(self) -> anyhow::Result<()> {
        let client = SeamClient::from_env().map_err(|e| anyhow::anyhow!(e))?;

        match self.command {
            Some(Commands::Devices { command }) => {
                commands::devices::execute(&client, command, self.id_only, self.raw)
                    .await
                    .map_err(|e| anyhow::anyhow!(e))
            }
            Some(Commands::AccessCodes { command }) => {
                commands::access_codes::execute(&client, command, self.id_only, self.raw)
                    .await
                    .map_err(|e| anyhow::anyhow!(e))
            }
            Some(Commands::Locks { command }) => {
                commands::locks::execute(&client, command, self.id_only, self.raw)
                    .await
                    .map_err(|e| anyhow::anyhow!(e))
            }
            Some(Commands::Health { command }) => {
                commands::health::execute(&client, command, self.id_only, self.raw)
                    .await
                    .map_err(|e| anyhow::anyhow!(e))
            }
            Some(Commands::ConnectWebviews { command }) => {
                commands::connect_webviews::execute(&client, command, self.id_only, self.raw)
                    .await
                    .map_err(|e| anyhow::anyhow!(e))
            }
            None => {
                println!("No command specified. Use --help for usage information.");
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_parses_help() {
        // Verify the CLI can generate help without panicking
        let _ = Cli::command().render_help();
    }

    #[test]
    fn test_cli_parses_help_agent() {
        let cli = Cli::parse_from(["rseam", "--help-agent"]);
        assert!(cli.help_agent);
    }

    #[test]
    fn test_cli_parses_devices_list() {
        let cli = Cli::parse_from(["rseam", "devices", "list"]);
        assert!(!cli.id_only);
        assert!(!cli.raw);
        match cli.command {
            Some(Commands::Devices { command }) => {
                assert_eq!(
                    command,
                    DeviceCommands::List {
                        device_id: None,
                        name: None
                    }
                );
            }
            _ => panic!("Expected Devices command"),
        }
    }

    #[test]
    fn test_cli_parses_devices_get_with_id() {
        let cli = Cli::parse_from(["rseam", "devices", "get", "--device-id", "dev_123"]);
        match cli.command {
            Some(Commands::Devices { command }) => {
                assert_eq!(
                    command,
                    DeviceCommands::Get {
                        device_id: Some("dev_123".to_string()),
                        name: None
                    }
                );
            }
            _ => panic!("Expected Devices command"),
        }
    }

    #[test]
    fn test_cli_parses_devices_get_with_name() {
        let cli = Cli::parse_from(["rseam", "devices", "get", "--name", "Front Door"]);
        match cli.command {
            Some(Commands::Devices { command }) => {
                assert_eq!(
                    command,
                    DeviceCommands::Get {
                        device_id: None,
                        name: Some("Front Door".to_string())
                    }
                );
            }
            _ => panic!("Expected Devices command"),
        }
    }

    #[test]
    fn test_cli_parses_health_get_health() {
        let cli = Cli::parse_from(["rseam", "health", "get-health"]);
        match cli.command {
            Some(Commands::Health { command }) => {
                assert_eq!(command, HealthCommands::GetHealth);
            }
            _ => panic!("Expected Health command"),
        }
    }

    #[test]
    fn test_cli_parses_locks_unlock() {
        let cli = Cli::parse_from(["rseam", "locks", "unlock-door", "--device-id", "dev_456"]);
        match cli.command {
            Some(Commands::Locks { command }) => {
                assert_eq!(
                    command,
                    LockCommands::UnlockDoor {
                        device_id: "dev_456".to_string()
                    }
                );
            }
            _ => panic!("Expected Locks command"),
        }
    }

    #[test]
    fn test_cli_parses_access_codes_create() {
        let cli = Cli::parse_from([
            "rseam",
            "access-codes",
            "create",
            "--device-id",
            "dev_123",
            "--code",
            "1234",
            "--name",
            "Guest Code",
        ]);
        match cli.command {
            Some(Commands::AccessCodes { command }) => {
                assert_eq!(
                    command,
                    AccessCodeCommands::Create {
                        device_id: "dev_123".to_string(),
                        code: "1234".to_string(),
                        name: Some("Guest Code".to_string())
                    }
                );
            }
            _ => panic!("Expected AccessCodes command"),
        }
    }

    #[test]
    fn test_cli_parses_global_flags() {
        let cli = Cli::parse_from(["rseam", "--id-only", "health", "get-health"]);
        assert!(cli.id_only);
        assert!(!cli.raw);

        let cli = Cli::parse_from(["rseam", "--raw", "health", "get-health"]);
        assert!(!cli.id_only);
        assert!(cli.raw);

        let cli = Cli::parse_from(["rseam", "--id-only", "--raw", "health", "get-health"]);
        assert!(cli.id_only);
        assert!(cli.raw);
    }

    #[test]
    fn test_cli_no_command() {
        let cli = Cli::parse_from(["rseam"]);
        assert!(cli.command.is_none());
    }

    #[test]
    fn test_cli_connect_webviews_create() {
        let cli = Cli::parse_from(["rseam", "connect-webviews", "create"]);
        match cli.command {
            Some(Commands::ConnectWebviews { command }) => {
                assert_eq!(
                    command,
                    ConnectWebviewCommands::Create {
                        accepted_providers: None
                    }
                );
            }
            _ => panic!("Expected ConnectWebviews command"),
        }
    }

    #[test]
    fn test_cli_connect_webviews_with_providers() {
        let cli = Cli::parse_from([
            "rseam",
            "connect-webviews",
            "create",
            "--accepted-providers",
            "august,level",
        ]);
        match cli.command {
            Some(Commands::ConnectWebviews { command }) => {
                assert_eq!(
                    command,
                    ConnectWebviewCommands::Create {
                        accepted_providers: Some("august,level".to_string())
                    }
                );
            }
            _ => panic!("Expected ConnectWebviews command"),
        }
    }
}
