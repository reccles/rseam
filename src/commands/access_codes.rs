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
        AccessCodeCommands::Create { device_id, code, name, starts_at, ends_at } => {
            create(client, device_id, code, name, starts_at, ends_at, id_only, raw).await
        }
        AccessCodeCommands::Get { access_code_id } => {
            get(client, access_code_id, id_only, raw).await
        }
        AccessCodeCommands::List { device_id } => {
            list(client, device_id, id_only, raw).await
        }
        AccessCodeCommands::Update { access_code_id, name, code, starts_at, ends_at } => {
            update(client, access_code_id, name, code, starts_at, ends_at, id_only, raw).await
        }
        AccessCodeCommands::Delete { access_code_id } => {
            delete(client, access_code_id, id_only, raw).await
        }
        AccessCodeCommands::GenerateCode { device_id, name } => {
            generate_code(client, device_id, name, id_only, raw).await
        }
        AccessCodeCommands::CreateMultiple { device_id, codes_json } => {
            create_multiple(client, device_id, codes_json, id_only, raw).await
        }
        AccessCodeCommands::UpdateMultiple { updates_json } => {
            update_multiple(client, updates_json, id_only, raw).await
        }
        AccessCodeCommands::GetTimeline { access_code_id } => {
            get_timeline(client, access_code_id, id_only, raw).await
        }
        AccessCodeCommands::PullBackupAccessCode { access_code_id } => {
            pull_backup_access_code(client, access_code_id, id_only, raw).await
        }
    }
}

async fn create(
    client: &SeamClient,
    device_id: String,
    code: String,
    name: Option<String>,
    starts_at: Option<String>,
    ends_at: Option<String>,
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

    if let Some(start) = starts_at {
        params["starts_at"] = start.into();
    }

    if let Some(end) = ends_at {
        params["ends_at"] = end.into();
    }

    let response = client.post("/access_codes/create", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}

async fn get(
    client: &SeamClient,
    access_code_id: String,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let params = json!({
        "access_code_id": access_code_id,
    });

    let response = client.post("/access_codes/get", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}

async fn list(
    client: &SeamClient,
    device_id: String,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let params = json!({
        "device_id": device_id,
    });

    let response = client.post("/access_codes/list", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}

async fn update(
    client: &SeamClient,
    access_code_id: String,
    name: Option<String>,
    code: Option<String>,
    starts_at: Option<String>,
    ends_at: Option<String>,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let mut params = json!({
        "access_code_id": access_code_id,
    });

    if let Some(n) = name {
        params["name"] = n.into();
    }

    if let Some(c) = code {
        params["code"] = c.into();
    }

    if let Some(s) = starts_at {
        params["starts_at"] = s.into();
    }

    if let Some(e) = ends_at {
        params["ends_at"] = e.into();
    }

    let response = client.post("/access_codes/update", params).await?;
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

async fn generate_code(
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

    let response = client.post("/access_codes/generate_code", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}

async fn create_multiple(
    client: &SeamClient,
    device_id: String,
    codes_json: String,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    // Parse the JSON array of code specs
    let codes: serde_json::Value = serde_json::from_str(&codes_json)
        .map_err(|e| crate::error::SeamError::SerdeError(e))?;

    let params = json!({
        "device_id": device_id,
        "access_codes": codes,
    });

    let response = client.post("/access_codes/create_multiple", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}

async fn update_multiple(
    client: &SeamClient,
    updates_json: String,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    // Parse the JSON array of update specs
    let updates: serde_json::Value = serde_json::from_str(&updates_json)
        .map_err(|e| crate::error::SeamError::SerdeError(e))?;

    let params = json!({
        "access_codes": updates,
    });

    let response = client.post("/access_codes/update_multiple", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}

async fn get_timeline(
    client: &SeamClient,
    access_code_id: String,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let params = json!({
        "access_code_id": access_code_id,
    });

    let response = client.post("/access_codes/get_timeline", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}

async fn pull_backup_access_code(
    client: &SeamClient,
    access_code_id: String,
    id_only: bool,
    raw: bool,
) -> SeamResult<()> {
    let params = json!({
        "access_code_id": access_code_id,
    });

    let response = client.post("/access_codes/pull_backup_access_code", params).await?;
    print_output(&response, id_only, raw);
    Ok(())
}
