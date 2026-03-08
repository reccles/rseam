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
        name: Option<String>,
    },

    /// Get a specific device
    Get {
        #[arg(long)]
        device_id: Option<String>,

        #[arg(long)]
        name: Option<String>,
    },

    /// Update a device
    Update {
        #[arg(long)]
        device_id: String,

        #[arg(long)]
        name: Option<String>,
    },

    /// Delete a device
    Delete {
        #[arg(long)]
        device_id: String,
    },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum AccessCodeCommands {
    /// Create an access code
    Create {
        #[arg(long)]
        device_id: String,

        /// PIN/code value (omit for offline codes - server generates algorithmic code)
        #[arg(long)]
        code: Option<String>,

        #[arg(long)]
        name: Option<String>,

        /// ISO8601 start time (e.g. 2024-01-15T09:00:00Z)
        #[arg(long)]
        starts_at: Option<String>,

        /// ISO8601 end time (e.g. 2024-01-15T17:00:00Z)
        #[arg(long)]
        ends_at: Option<String>,

        /// Code works offline without internet (server generates code). Device-specific (igloohome, dormakaba, Lockly)
        #[arg(long)]
        offline: bool,

        /// One-time-use code (offline only). Expires after first use.
        #[arg(long)]
        one_time: bool,
    },

    /// Get an access code
    Get {
        #[arg(long)]
        access_code_id: String,
    },

    /// List access codes
    List {
        #[arg(long)]
        device_id: String,
    },

    /// Update an access code
    Update {
        #[arg(long)]
        access_code_id: String,

        #[arg(long)]
        name: Option<String>,

        #[arg(long)]
        code: Option<String>,

        #[arg(long)]
        starts_at: Option<String>,

        #[arg(long)]
        ends_at: Option<String>,
    },

    /// Delete an access code
    Delete {
        #[arg(long)]
        access_code_id: String,
    },

    /// Generate a new access code automatically
    GenerateCode {
        #[arg(long)]
        device_id: String,

        #[arg(long)]
        name: Option<String>,
    },

    /// Create multiple access codes at once
    CreateMultiple {
        #[arg(long)]
        device_id: String,

        /// JSON array of codes, e.g. '[{"code":"1234","name":"Guest"}]'
        #[arg(long)]
        codes_json: String,
    },

    /// Update multiple access codes at once
    UpdateMultiple {
        /// JSON array of updates, e.g. '[{"access_code_id":"ac_123","name":"New"}]'
        #[arg(long)]
        updates_json: String,
    },

    /// Get timeline of access code changes
    GetTimeline {
        #[arg(long)]
        access_code_id: String,
    },

    /// Pull backup access code from provider
    PullBackupAccessCode {
        #[arg(long)]
        access_code_id: String,
    },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum LockCommands {
    /// Get a lock
    Get {
        #[arg(long)]
        device_id: String,
    },

    /// List all locks
    List {
        #[arg(long)]
        device_id: Option<String>,
    },

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
        match cli.command {
            Some(Commands::Devices { command }) => {
                assert_eq!(command, DeviceCommands::List { name: None });
            }
            _ => panic!("Expected Devices command"),
        }
    }

    #[test]
    fn test_cli_parses_devices_get_with_id() {
        let cli = Cli::parse_from(["rseam", "devices", "get", "--device-id", "dev_123"]);
        match cli.command {
            Some(Commands::Devices { command }) => {
                assert_eq!(command, DeviceCommands::Get {
                    device_id: Some("dev_123".to_string()),
                    name: None
                });
            }
            _ => panic!("Expected Devices command"),
        }
    }

    #[test]
    fn test_cli_parses_devices_update() {
        let cli = Cli::parse_from(["rseam", "devices", "update", "--device-id", "dev_123", "--name", "New"]);
        match cli.command {
            Some(Commands::Devices { command }) => {
                assert_eq!(command, DeviceCommands::Update {
                    device_id: "dev_123".to_string(),
                    name: Some("New".to_string())
                });
            }
            _ => panic!("Expected Devices command"),
        }
    }

    #[test]
    fn test_cli_parses_devices_delete() {
        let cli = Cli::parse_from(["rseam", "devices", "delete", "--device-id", "dev_123"]);
        match cli.command {
            Some(Commands::Devices { command }) => {
                assert_eq!(command, DeviceCommands::Delete { device_id: "dev_123".to_string() });
            }
            _ => panic!("Expected Devices command"),
        }
    }

    #[test]
    fn test_cli_parses_locks_get() {
        let cli = Cli::parse_from(["rseam", "locks", "get", "--device-id", "dev_123"]);
        match cli.command {
            Some(Commands::Locks { command }) => {
                assert_eq!(command, LockCommands::Get { device_id: "dev_123".to_string() });
            }
            _ => panic!("Expected Locks command"),
        }
    }

    #[test]
    fn test_cli_parses_locks_list() {
        let cli = Cli::parse_from(["rseam", "locks", "list"]);
        match cli.command {
            Some(Commands::Locks { command }) => {
                assert_eq!(command, LockCommands::List { device_id: None });
            }
            _ => panic!("Expected Locks command"),
        }
    }

    #[test]
    fn test_cli_parses_locks_unlock() {
        let cli = Cli::parse_from(["rseam", "locks", "unlock-door", "--device-id", "dev_456"]);
        match cli.command {
            Some(Commands::Locks { command }) => {
                assert_eq!(command, LockCommands::UnlockDoor { device_id: "dev_456".to_string() });
            }
            _ => panic!("Expected Locks command"),
        }
    }

    #[test]
    fn test_cli_parses_access_codes_get() {
        let cli = Cli::parse_from(["rseam", "access-codes", "get", "--access-code-id", "ac_123"]);
        match cli.command {
            Some(Commands::AccessCodes { command }) => {
                assert_eq!(command, AccessCodeCommands::Get { access_code_id: "ac_123".to_string() });
            }
            _ => panic!("Expected AccessCodes command"),
        }
    }

    #[test]
    fn test_cli_parses_access_codes_update() {
        let cli = Cli::parse_from([
            "rseam", "access-codes", "update", "--access-code-id", "ac_123",
            "--name", "New Name", "--code", "9999"
        ]);
        match cli.command {
            Some(Commands::AccessCodes { command }) => {
                assert_eq!(command, AccessCodeCommands::Update {
                    access_code_id: "ac_123".to_string(),
                    name: Some("New Name".to_string()),
                    code: Some("9999".to_string()),
                    starts_at: None,
                    ends_at: None
                });
            }
            _ => panic!("Expected AccessCodes command"),
        }
    }

    #[test]
    fn test_cli_parses_access_codes_generate_code() {
        let cli = Cli::parse_from([
            "rseam", "access-codes", "generate-code", "--device-id", "dev_123", "--name", "Guest"
        ]);
        match cli.command {
            Some(Commands::AccessCodes { command }) => {
                assert_eq!(command, AccessCodeCommands::GenerateCode {
                    device_id: "dev_123".to_string(),
                    name: Some("Guest".to_string())
                });
            }
            _ => panic!("Expected AccessCodes command"),
        }
    }

    #[test]
    fn test_cli_parses_access_codes_create() {
        let cli = Cli::parse_from([
            "rseam", "access-codes", "create", "--device-id", "dev_123",
            "--code", "1234", "--name", "Guest",
        ]);
        match cli.command {
            Some(Commands::AccessCodes { command }) => {
                assert_eq!(command, AccessCodeCommands::Create {
                    device_id: "dev_123".to_string(),
                    code: Some("1234".to_string()),
                    name: Some("Guest".to_string()),
                    starts_at: None,
                    ends_at: None,
                    offline: false,
                    one_time: false,
                });
            }
            _ => panic!("Expected AccessCodes command"),
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
    fn test_cli_parses_global_flags() {
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
                assert_eq!(command, ConnectWebviewCommands::Create { accepted_providers: None });
            }
            _ => panic!("Expected ConnectWebviews command"),
        }
    }
}
