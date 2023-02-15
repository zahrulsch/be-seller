use crate::{error::OhMyError, prelude::*};
use bigseller_client::client::account_info_client::AccountInfoData;
use std::{fs::read_to_string, io::Error, path::PathBuf};
use tauri::AppHandle;

fn file_reader(input: (String, PathBuf)) -> std::result::Result<(String, String), Error> {
    let s_string = read_to_string(input.1)?;
    Ok((input.0, s_string))
}

#[tauri::command]
pub async fn get_profiles(handler: AppHandle) -> Result<Vec<AccountInfoData>> {
    let Some(rs_dir) = handler.path_resolver().resource_dir() else {
        return Err(OhMyError::new("path error", "tidak dapat resolve resource directory"))
    };

    let sess_path = rs_dir.join("sessions");
    let sess_dir = sess_path.read_dir()?;
    let sess_files = sess_dir
        .filter_map(|f| {
            if let Ok(entry) = f {
                return Some(entry);
            }
            None
        })
        .filter_map(|file| {
            let file_name = file.file_name();
            let Some(f_name) = file_name.to_str() else {
                return None
            };

            if f_name.contains(".session") {
                Some((f_name.to_string(), file.path()))
            } else {
                None
            }
        });

    let session_strings = sess_files
        .map(file_reader)
        .filter_map(|f| {
            let Ok(session) = f else {
            return None
        };

            Some(session)
        })
        .collect::<Vec<_>>();

    let mut account_infos = vec![];
    for session in session_strings {
        let mut account_info = AccountInfoData::new();
        account_info.make_a_request(&session.1, &session.0).await?;

        account_infos.push(account_info)
    }

    Ok(account_infos)
}
