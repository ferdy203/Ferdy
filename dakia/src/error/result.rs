use std::sync::PoisonError;
use tokio::sync::RwLockReadGuard;

use super::DakiaError;

pub type DakiaResult<T> = Result<T, Box<Error>>;

#[derive(Debug)]
pub enum Error {
    DakiaError(DakiaError),
    PoisonError(String),
    PingoraError(pingora_core::Error),
}

impl<T> From<PoisonError<RwLockReadGuard<'_, T>>> for Error {
    fn from(err: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        Error::PoisonError(err.to_string())
    }
}

impl From<pingora_core::Error> for Error {
    fn from(err: pingora_core::Error) -> Self {
        Error::PingoraError(err)
    }
}

impl From<DakiaError> for Error {
    fn from(err: DakiaError) -> Self {
        Error::DakiaError(err)
    }
}

impl From<Box<Error>> for Box<pingora_core::Error> {
    fn from(value: Box<Error>) -> Box<pingora_core::Error> {
        match *value {
            Error::PingoraError(pe) => Box::new(pe),
            // TODO: implement conversion for other errors
            _ => pingora_core::Error::new(pingora::ErrorType::InternalError),
        }
    }
}
