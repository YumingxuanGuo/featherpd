use serde_derive::{Deserialize, Serialize};
use std::fmt::{self, Display};

/// Result returning Error
pub type Result<T> = std::result::Result<T, Error>;

/// RPC-Result returning Error
pub type RpcResult<T> = std::result::Result<tonic::Response<T>, tonic::Status>;

/// toyDB errors. All except Internal are considered user-facing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Error {
    Abort,
    Config(String),
    Internal(String),
    Parse(String),
    ReadOnly,
    Serialization,
    Value(String),
    NotLeader,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Error::Config(s) | Error::Internal(s) | Error::Parse(s) | Error::Value(s) => {
                write!(f, "{}", s)
            }
            Error::Abort => write!(f, "Operation aborted"),
            Error::Serialization => write!(f, "Serialization failure, retry transaction"),
            Error::ReadOnly => write!(f, "Read-only transaction"),
            Error::NotLeader => write!(f, "Not leader"),
        }
    }
}

impl From<Box<bincode::ErrorKind>> for Error {
    fn from(err: Box<bincode::ErrorKind>) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<config::ConfigError> for Error {
    fn from(err: config::ConfigError) -> Self {
        Error::Config(err.to_string())
    }
}

impl From<log::ParseLevelError> for Error {
    fn from(err: log::ParseLevelError) -> Self {
        Error::Config(err.to_string())
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(err: log::SetLoggerError) -> Self {
        Error::Config(err.to_string())
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(err: std::array::TryFromSliceError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Error::Parse(err.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::Parse(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Error::Internal(err.to_string())
    }
}

// see https://github.com/tokio-rs/tokio/pull/3263: remove try_recv() from mpsc types
//
// impl From<tokio::sync::mpsc::error::TryRecvError> for Error {
//     fn from(err: tokio::sync::mpsc::error::TryRecvError) -> Self {
//         Error::Internal(err.to_string())
//     }
// }

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(err: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Error::Internal(err.to_string())
    }
}

impl<T> From<tokio::sync::mpsc::error::TrySendError<T>> for Error {
    fn from(err: tokio::sync::mpsc::error::TrySendError<T>) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(err: tokio::sync::oneshot::error::RecvError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<tonic::Status> for Error {
    fn from(err: tonic::Status) -> Self {
        let chunks = err.message().split(" ").collect::<Vec<_>>();
        match chunks[0] {
            "[Config]" => Error::Config(chunks[1..].join(" ")),
            "[Internal]" => Error::Internal(chunks[1..].join(" ")),
            "[Parse]" => Error::Parse(chunks[1..].join(" ")),
            "[Value]" => Error::Value(chunks[1..].join(" ")),
            "[Abort]" => Error::Abort,
            "[ReadOnly]" => Error::ReadOnly,
            "[Serialization]" => Error::Serialization,
            "[NotLeader]" => Error::NotLeader,
            _ => Error::Internal(format!("Unknown error type: {:?}", err.message())),
        }
    }
}

impl From<Error> for tonic::Status {
    fn from(err: Error) -> Self {
        let msg = match err {
            Error::Config(s) => format!("[Config] {}", s),
            Error::Internal(s) => format!("[Internal] {}", s),
            Error::Parse(s) => format!("[Parse] {}", s),
            Error::Value(s) => format!("[Value] {}", s),
            Error::Abort => format!("[Abort] Operation aborted"),
            Error::ReadOnly => format!("[ReadOnly] Read-only transaction"),
            Error::Serialization => format!("[Serialization] Serialization failure, retry transaction"),
            Error::NotLeader => format!("[NotLeader] Not leader"),
        };
        tonic::Status::internal(msg)
    }
}