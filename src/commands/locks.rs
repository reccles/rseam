use crate::api_client::SeamClient;
use crate::cli::LockCommands;
use crate::error::SeamResult;
use super::print_output;
use serde_json::json;

pub async fn execute(
    client: &SeamClient,
    command: LockCommands,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    match command {
        LockCommands::Get { device_id } => {
            get_lock(client, device_id, id_only, raw).await
        }
        LockCommands::List { device_id } => {
            list_locks(client, device_id, id_only, raw).await
        }
        LockCommands::UnlockDoor { device_id } => {
            unlock_door(client, device_id, id_only, raw).await
        }
        LockCommands::LockDoor { device_id } => {
            lock_door(client, device_id, id_only, raw).await
        }
    }
}

async fn get_lock(
    client: &SeamClient,
    device_id: String,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let params = json!({
        "device_id": device_id,
    });

    let response = client.post("/locks/get", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}

async fn list_locks(
    client: &SeamClient,
    device_id: Option<String>,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let mut params = json!({});

    if let Some(id) = device_id {
        params["device_id"] = id.into();
    }

    let response = client.post("/locks/list", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}

async fn unlock_door(
    client: &SeamClient,
    device_id: String,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let params = json!({
        "device_id": device_id,
    });

    let response = client.post("/locks/unlock_door", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}

async fn lock_door(
    client: &SeamClient,
    device_id: String,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let params = json!({
        "device_id": device_id,
    });

    let response = client.post("/locks/lock_door", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}
