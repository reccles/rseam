use crate::api_client::SeamClient;
use crate::cli::AccessCodeCommands;
use crate::error::SeamResult;
use super::print_output;
use serde_json::json;

pub async fn execute(
    client: &SeamClient,
    command: AccessCodeCommands,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    match command {
        AccessCodeCommands::Create {
            device_id,
            code,
            name,
        } => create(client, device_id, code, name, id_only, raw).await,
        AccessCodeCommands::List { device_id } => {
            list(client, device_id, id_only, raw).await
        }
        AccessCodeCommands::Delete { access_code_id } => {
            delete(client, access_code_id, id_only, raw).await
        }
    }
}

async fn create(
    client: &SeamClient,
    device_id: String,
    code: String,
    name: Option<String>,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let mut params = json!({
        "device_id": device_id,
        "code": code,
    });

    if let Some(n) = name {
        params["name"] = n.into();
    }

    let response = client.post("/access_codes/create", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}

async fn list(
    client: &SeamClient,
    device_id: Option<String>,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let mut params = json!({});

    if let Some(id) = device_id {
        params["device_id"] = id.into();
    }

    let response = client.post("/access_codes/list", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}

async fn delete(
    client: &SeamClient,
    access_code_id: String,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let params = json!({
        "access_code_id": access_code_id,
    });

    let response = client.post("/access_codes/delete", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}
