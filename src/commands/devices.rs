use crate::api_client::SeamClient;
use crate::cli::DeviceCommands;
use crate::error::SeamResult;
use super::print_output;
use serde_json::json;

pub async fn execute(
    client: &SeamClient,
    command: DeviceCommands,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    match command {
        DeviceCommands::List { device_id, name } => {
            list_devices(client, device_id, name, id_only, raw).await
        }
        DeviceCommands::Get { device_id, name } => {
            get_device(client, device_id, name, id_only, raw).await
        }
        DeviceCommands::Update { device_id, name } => {
            update_device(client, device_id, name, id_only, raw).await
        }
    }
}

async fn list_devices(
    client: &SeamClient,
    device_id: Option<String>,
    name: Option<String>,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let mut params = json!({});

    if let Some(id) = device_id {
        params["device_id"] = id.into();
    }

    if let Some(n) = name {
        params["name"] = n.into();
    }

    let response = client.post("/devices/list", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}

async fn get_device(
    client: &SeamClient,
    device_id: Option<String>,
    name: Option<String>,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let mut params = json!({});

    if let Some(id) = device_id {
        params["device_id"] = id.into();
    } else if let Some(n) = name {
        params["name"] = n.into();
    } else {
        return Err(crate::error::SeamError::MissingParameter(
            "Either --device-id or --name is required".to_string(),
        ));
    }

    let response = client.post("/devices/get", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}

async fn update_device(
    client: &SeamClient,
    device_id: String,
    name: Option<String>,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let mut params = json!({
        "device_id": device_id,
    });

    if let Some(n) = name {
        params["name"] = n.into();
    }

    let response = client.post("/devices/update", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}
