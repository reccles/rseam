use crate::api_client::SeamClient;
use crate::cli::HealthCommands;
use crate::error::SeamResult;
use super::print_output;
use serde_json::json;

pub async fn execute(
    client: &SeamClient,
    command: HealthCommands,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    match command {
        HealthCommands::GetHealth => get_health(client, id_only, raw).await,
    }
}

async fn get_health(
    client: &SeamClient,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let params = json!({});

    let response = client.post("/health/get_health", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}
