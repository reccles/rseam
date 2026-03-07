use crate::api_client::SeamClient;
use crate::cli::ConnectWebviewCommands;
use crate::error::SeamResult;
use super::print_output;
use serde_json::json;

pub async fn execute(
    client: &SeamClient,
    command: ConnectWebviewCommands,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    match command {
        ConnectWebviewCommands::Create { accepted_providers } => {
            create(client, accepted_providers, id_only, raw).await
        }
    }
}

async fn create(
    client: &SeamClient,
    accepted_providers: Option<String>,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let mut params = json!({});

    if let Some(providers) = accepted_providers {
        let provider_list: Vec<&str> = providers.split(',').collect();
        params["accepted_providers"] = serde_json::to_value(provider_list)?;
    }

    let response = client.post("/connect_webviews/create", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}
