// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//A convenient error wrapper,
//largely inspired by GGEZ's error and Result type : Concise and totally do the job.

use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::io::Error as FileError;
use std::sync;
use std::fs::DirEntry;

use toml::ser::Error as TomlSerializationError;
use toml::de::Error as TomlDeserializationError;

use std::env::VarError;
//The GameError enum implement the Error trait, bound to the Debug + Display traits
//Some enum's struct contain only a description, while other contain a lower-level error,
//which is meant to be passed to the cause() function from the Error trait.

//If a GameError type have multiple causes of failure, create another enum specialized in this system
//for example, if LogError can have multiple cause of failure, create the enum LogErrorType.
#[derive(Debug)]
pub enum GameError {
    IOError(String, FileError),
    FileSystemError(String),
    UnknownError(String),
    ThreadPoolError(String),
    DeserializationError(String, TomlDeserializationError),
    SerializationError(String, TomlSerializationError),
    EnvironmentError(String, VarError),
}

impl Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &GameError::IOError(ref description, _) => write!(f, "IO error: {}", description),
            &GameError::FileSystemError(ref description) => write!(f, "File system error: {}", description),
            &GameError::UnknownError(ref description) => write!(f, "Unknown error: {}", description),
            &GameError::ThreadPoolError(ref description) => write!(f, "ThreadPool error: {}", description),
            &GameError::DeserializationError(ref description, _) => write!(f, "TOML deserialization error: {}", description),
            &GameError::SerializationError(ref description, _) => write!(f, "TOML serialization error: {}", description),
            &GameError::EnvironmentError(ref description, _) => write!(f, "Environment variable error: {}", description),
        }
    }
}

impl Error for GameError {
    fn description(&self) -> &str {
        match self {
            &GameError::IOError(_, _) => "LogError",
            &GameError::FileSystemError(_) => "FileSystemError",
            &GameError::UnknownError(_) => "UnknownError",
            &GameError::ThreadPoolError(_) => "ThreadPoolError",
            &GameError::DeserializationError(_, _) => "DeserializationError",
            &GameError::SerializationError(_, _) => "SerializationError",
            &GameError::EnvironmentError(_, _) => "EnvironmentError",
        }
    }
    fn cause(&self) -> Option<&Error> {
        match self {
            &GameError::IOError(_, ref file_error) => {
                Some(file_error)
            },
            &GameError::FileSystemError(_) => {
                None
            },
            &GameError::UnknownError(_) => {
                None
            },
            &GameError::ThreadPoolError(_) => {
                None
            },
            &GameError::DeserializationError(_, ref deserialization_error) => {
                Some(deserialization_error)
            },
            &GameError::SerializationError(_, ref serialization_error) => {
                Some(serialization_error)
            },
            &GameError::EnvironmentError(_, ref var_error) => {
                Some(var_error)
            },
        }
    }
}

pub type GameResult<T> = Result<T, GameError>;

impl From<FileError> for GameError {
    fn from(error: FileError) -> Self {
        GameError::IOError(format!("Error while dealing with file"), error)
    }
}

impl From<TomlDeserializationError> for GameError {
    fn from(error: TomlDeserializationError) -> Self {
        GameError::DeserializationError(format!("Error while deserializing a TOML file"), error)
    }
}

impl From<TomlSerializationError> for GameError {
    fn from(error: TomlSerializationError) -> Self {
        GameError::SerializationError(format!("Error while serializing a TOML file"), error)
    }
}

impl From<VarError> for GameError {
    fn from(error: VarError) -> Self {
        GameError::EnvironmentError(format!("Error while reading an envrionment variable"), error)
    }
}