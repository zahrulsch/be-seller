use auto_requester::error::OhMyError as AutoRequestError;
use serde::{Serialize, ser::SerializeStruct};
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OhMyError {
  #[error(transparent)]
  Client (
    #[from]
    AutoRequestError
  ),
  #[error("database error")]
  DB (
    #[from]
    DbErr
  )
}

impl Serialize for OhMyError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: serde::Serializer 
  {
      let mut seq = serializer.serialize_struct("Error", 2)?;

      match self {
          OhMyError::Client(e) => {
            seq.serialize_field("name", "client error")?;
            seq.serialize_field("cause", &e.to_string())?;
          },
          OhMyError::DB(e) => {
            seq.serialize_field("name", "client error")?;
            seq.serialize_field("cause", &e.to_string())?;
          }
      }

      seq.end()
  }
}

