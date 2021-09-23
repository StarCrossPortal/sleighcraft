//
//  Copyright 2021 StarCrossTech
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use pyo3::exceptions::PyOSError;
use pyo3::PyErr;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    NotEnoughBytes,
    CppException(cxx::Exception),
    ArchNotFound(String),
    MissingArg(String),
    PyException(String),
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<cxx::Exception> for Error {
    fn from(err: cxx::Exception) -> Self {
        Self::CppException(err)
    }
}

impl From<PyErr> for Error {
    fn from(err: PyErr) -> Self {
        Self::PyException(err.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError(e) => {
                write!(f, "io error: {}", e)
            }
            Self::NotEnoughBytes => {
                write!(f, "bytes not enough when decoding")
            }
            Self::CppException(e) => {
                write!(f, "cpp exception: {}", e)
            }
            Self::ArchNotFound(s) => {
                write!(f, "arch {} is not found in preset", s)
            }
            Self::MissingArg(s) => {
                write!(f, "missing argument: {}", s)
            }
            Self::PyException(s) => {
                write!(f, "python exception: {}", s)
            }
        }
    }
}

impl std::convert::From<Error> for PyErr {
    fn from(_err: Error) -> PyErr {
        PyOSError::new_err(_err.to_string())
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Self::IoError(ref e) => Some(e),
            Self::CppException(ref e) => Some(e),
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
