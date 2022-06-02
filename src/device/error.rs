use std::ffi::OsStr;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::{error, fmt};

use thiserror::Error;

pub type ConnectionResult<T> = Result<T, ConnectionError>;

#[derive(Error)]
#[non_exhaustive]
pub enum ConnectionError {
    #[error("Device not found")]
    NotFound,
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl Debug for ConnectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionError::NotFound => f.debug_tuple("NotFound").finish(),
            ConnectionError::Unexpected(err) => Debug::fmt(err, f),
        }
    }
}

impl ConnectionError {
    pub fn new_unexpected<E>(err: E) -> ConnectionError
    where
        E: error::Error + Send + Sync + 'static,
    {
        ConnectionError::Unexpected(anyhow::Error::new(err))
    }

    pub fn new_unexpected_with_context<E, C>(
        err: E,
        context: C,
    ) -> ConnectionError
    where
        E: error::Error + Send + Sync + 'static,
        C: Display + Send + Sync + 'static,
    {
        ConnectionError::Unexpected(anyhow::Error::new(err).context(context))
    }
}

pub fn is_device_exists<S: AsRef<OsStr>>(
    class: &'static str,
    device_name: S,
) -> bool {
    let mut path = PathBuf::from("/sys/class");
    path.push(class);
    path.push(device_name.as_ref());
    path.exists()
}

pub fn check_device_exists<S: AsRef<OsStr>>(
    class: &'static str,
    device_name: S,
) -> ConnectionResult<()> {
    is_device_exists(class, device_name)
        .then(|| ())
        .ok_or(ConnectionError::NotFound)
}
