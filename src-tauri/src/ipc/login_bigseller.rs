use crate::{error::OhMyError, prelude::*, utils::Utils};
use bigseller_client::client::login_client::LoginData;
use tauri::AppHandle;
use tokio::fs;

#[derive(Debug, serde::Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
    captcha: String,
    cookie_string: String,
}

#[tauri::command]
pub async fn login_bigseller(app: AppHandle, payload: LoginPayload) -> Result<String> {
    let mut bigseller_login = LoginData::new();
    let base_dir = app.path_resolver().resource_dir();

    let Some(resource_path) = base_dir else {
        return Err(OhMyError::new("path error", "folder resource tidak ditemukan"))
    };

    println!("{:?}", fs::canonicalize(&resource_path).await.unwrap());

    let save_path = resource_path.join("sessions");

    if !save_path.exists() {
        match fs::create_dir_all(&save_path).await {
            Ok(_) => {},
            Err(e) => {
                println!("{:?}", e);
                return Err(OhMyError::new("path error", "gagal membuat folder sessions"))
            }
        }
    }

    let email_to_path = Utils::email_to_path(&payload.email)?;
    let file_name = save_path.join(&email_to_path).with_extension("session");

    bigseller_login
        .try_login(
            payload.email,
            payload.password,
            payload.captcha,
            payload.cookie_string,
        )
        .await?;

    if fs::write(file_name, bigseller_login.cookies).await.is_err() {
        return Err(OhMyError::new(
            "io error",
            "gagal menyimpan data login session",
        ));
    }

    Ok("Login sukses, session berhasil disimpan".into())
}
