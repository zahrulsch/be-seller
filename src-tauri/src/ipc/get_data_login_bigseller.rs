use crate::prelude::*;
use bigseller_client::client::vcode_client::VCodeData;

#[tauri::command]
pub async fn get_data_login_bigseller() -> Result<VCodeData> {
    let mut vcode_data = VCodeData::new();

    vcode_data.make_a_request().await?;

    Ok(vcode_data)
}
