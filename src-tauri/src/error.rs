#![allow(dead_code)]

use auto_requester::error::OhMyError as AutoRequestError;
use bigseller_client::error::OhMyError as BigSellerError;
use sea_orm::DbErr;
use serde::{ser::SerializeStruct, Serialize};
use thiserror::Error;
use tokio::io::Error as TokioIoError;

#[derive(Debug, serde::Serialize, serde::Deserialize, Error)]
pub struct InvalidArgument {
    pub message: String,
    pub field: String,
}

impl InvalidArgument {
    #[allow(dead_code)]
    pub fn create_error<Y>(field: Y, message: Y) -> OhMyError
    where
        Y: Into<String>,
    {
        OhMyError::Arguments(Self {
            field: field.into(),
            message: message.into(),
        })
    }
}

impl std::fmt::Display for InvalidArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
            InvalidArgument {{
              message: \"{}\",
              field: \"{}\"
            }}        
          ",
            self.message, self.field
        )
    }
}

#[derive(Debug, Error)]
pub enum OhMyError {
    #[error(transparent)]
    Client(#[from] AutoRequestError),
    #[error("database error")]
    DB(#[from] DbErr),
    #[error("arguments error")]
    Arguments(#[from] InvalidArgument),
    #[error("big seller error")]
    BigSellerClient(#[from] BigSellerError),
    #[error("stable error")]
    StableError { cause: String, name: String },
    #[error("tokio io error")]
    TokioIo(#[from] TokioIoError)
}

impl OhMyError {
    pub fn new<T>(name: T, cause: T) -> Self
    where
        T: Into<String>,
    {
        Self::StableError {
            cause: cause.into(),
            name: name.into(),
        }
    }
}

impl Serialize for OhMyError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_struct("Error", 2)?;

        match self {
            OhMyError::Client(e) => {
                seq.serialize_field("name", "client error")?;
                seq.serialize_field("cause", &e.to_string())?;
            }
            OhMyError::DB(e) => {
                seq.serialize_field("name", "database error")?;
                seq.serialize_field("cause", &e.to_string())?;
            }
            OhMyError::Arguments(e) => {
                seq.serialize_field("name", "arguments error")?;
                seq.serialize_field("cause", &e)?;
            }
            OhMyError::BigSellerClient(bse) => {
                seq.serialize_field("name", "bigseller error")?;
                seq.serialize_field("cause", &bse.to_string())?;
            }
            OhMyError::StableError { cause, name } => {
                seq.serialize_field("name", name)?;
                seq.serialize_field("cause", cause)?;
            }
            OhMyError::TokioIo(e) => {
                seq.serialize_field("name", "tokio io error")?;
                seq.serialize_field("cause", &e.to_string())?;
            },
        }

        seq.end()
    }
}
