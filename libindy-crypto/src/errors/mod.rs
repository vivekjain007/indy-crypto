#[cfg(feature = "serialization")]
extern crate serde_json;
extern crate log;

use std::error::Error;
use std::{fmt, io};

#[derive(Debug, PartialEq, Copy, Clone, Serialize)]
#[repr(usize)]
pub enum ErrorCode
{
    Success = 0,

    // Common errors

    // Caller passed invalid value as param 1 (null, invalid json and etc..)
    CommonInvalidParam1 = 100,

    // Caller passed invalid value as param 2 (null, invalid json and etc..)
    CommonInvalidParam2 = 101,

    // Caller passed invalid value as param 3 (null, invalid json and etc..)
    CommonInvalidParam3 = 102,

    // Caller passed invalid value as param 4 (null, invalid json and etc..)
    CommonInvalidParam4 = 103,

    // Caller passed invalid value as param 5 (null, invalid json and etc..)
    CommonInvalidParam5 = 104,

    // Caller passed invalid value as param 6 (null, invalid json and etc..)
    CommonInvalidParam6 = 105,

    // Caller passed invalid value as param 7 (null, invalid json and etc..)
    CommonInvalidParam7 = 106,

    // Caller passed invalid value as param 8 (null, invalid json and etc..)
    CommonInvalidParam8 = 107,

    // Caller passed invalid value as param 9 (null, invalid json and etc..)
    CommonInvalidParam9 = 108,

    // Caller passed invalid value as param 10 (null, invalid json and etc..)
    CommonInvalidParam10 = 109,

    // Caller passed invalid value as param 11 (null, invalid json and etc..)
    CommonInvalidParam11 = 110,

    // Caller passed invalid value as param 11 (null, invalid json and etc..)
    CommonInvalidParam12 = 111,

    // Invalid library state was detected in runtime. It signals library bug
    CommonInvalidState = 112,

    // Object (json, config, key, credential and etc...) passed by library caller has invalid structure
    CommonInvalidStructure = 113,

    // IO Error
    CommonIOError = 114,

    // Trying to issue non-revocation credential with full anoncreds revocation accumulator
    AnoncredsRevocationAccumulatorIsFull = 115,

    // Invalid revocation accumulator index
    AnoncredsInvalidRevocationAccumulatorIndex = 116,

    // Credential revoked
    AnoncredsCredentialRevoked = 117,

    // Proof rejected
    AnoncredsProofRejected = 118,
}

pub trait ToErrorCode {
    fn to_error_code(&self) -> ErrorCode;
}

#[derive(Debug)]
    pub enum IndyCryptoError {
    InvalidParam1(String),
    InvalidParam2(String),
    InvalidParam3(String),
    InvalidParam4(String),
    InvalidParam5(String),
    InvalidParam6(String),
    InvalidParam7(String),
    InvalidParam8(String),
    InvalidParam9(String),
    InvalidState(String),
    InvalidStructure(String),
    IOError(io::Error),
    AnoncredsRevocationAccumulatorIsFull(String),
    AnoncredsInvalidRevocationAccumulatorIndex(String),
    AnoncredsCredentialRevoked(String),
    AnoncredsProofRejected(String),
}

impl fmt::Display for IndyCryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IndyCryptoError::InvalidParam1(ref description) => write!(f, "Invalid param 1: {}", description),
            IndyCryptoError::InvalidParam2(ref description) => write!(f, "Invalid param 2: {}", description),
            IndyCryptoError::InvalidParam3(ref description) => write!(f, "Invalid param 3: {}", description),
            IndyCryptoError::InvalidParam4(ref description) => write!(f, "Invalid param 4: {}", description),
            IndyCryptoError::InvalidParam5(ref description) => write!(f, "Invalid param 4: {}", description),
            IndyCryptoError::InvalidParam6(ref description) => write!(f, "Invalid param 4: {}", description),
            IndyCryptoError::InvalidParam7(ref description) => write!(f, "Invalid param 4: {}", description),
            IndyCryptoError::InvalidParam8(ref description) => write!(f, "Invalid param 4: {}", description),
            IndyCryptoError::InvalidParam9(ref description) => write!(f, "Invalid param 4: {}", description),
            IndyCryptoError::InvalidState(ref description) => write!(f, "Invalid library state: {}", description),
            IndyCryptoError::InvalidStructure(ref description) => write!(f, "Invalid structure: {}", description),
            IndyCryptoError::IOError(ref err) => err.fmt(f),
            IndyCryptoError::AnoncredsRevocationAccumulatorIsFull(ref description) => write!(f, "Revocation accumulator is full: {}", description),
            IndyCryptoError::AnoncredsInvalidRevocationAccumulatorIndex(ref description) => write!(f, "Invalid revocation accumulator index: {}", description),
            IndyCryptoError::AnoncredsCredentialRevoked(ref description) => write!(f, "Credential revoked: {}", description),
            IndyCryptoError::AnoncredsProofRejected(ref description) => write!(f, "Proof rejected: {}", description),
        }
    }
}

impl Error for IndyCryptoError {
    fn description(&self) -> &str {
        match *self {
            IndyCryptoError::InvalidParam1(ref description) => description,
            IndyCryptoError::InvalidParam2(ref description) => description,
            IndyCryptoError::InvalidParam3(ref description) => description,
            IndyCryptoError::InvalidParam4(ref description) => description,
            IndyCryptoError::InvalidParam5(ref description) => description,
            IndyCryptoError::InvalidParam6(ref description) => description,
            IndyCryptoError::InvalidParam7(ref description) => description,
            IndyCryptoError::InvalidParam8(ref description) => description,
            IndyCryptoError::InvalidParam9(ref description) => description,
            IndyCryptoError::InvalidState(ref description) => description,
            IndyCryptoError::InvalidStructure(ref description) => description,
            IndyCryptoError::IOError(ref err) => err.description(),
            IndyCryptoError::AnoncredsRevocationAccumulatorIsFull(ref description) => description,
            IndyCryptoError::AnoncredsInvalidRevocationAccumulatorIndex(ref description) => description,
            IndyCryptoError::AnoncredsCredentialRevoked(ref description) => description,
            IndyCryptoError::AnoncredsProofRejected(ref description) => description,
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            IndyCryptoError::InvalidParam1(_) |
            IndyCryptoError::InvalidParam2(_) |
            IndyCryptoError::InvalidParam3(_) |
            IndyCryptoError::InvalidParam4(_) |
            IndyCryptoError::InvalidParam5(_) |
            IndyCryptoError::InvalidParam6(_) |
            IndyCryptoError::InvalidParam7(_) |
            IndyCryptoError::InvalidParam8(_) |
            IndyCryptoError::InvalidParam9(_) |
            IndyCryptoError::InvalidState(_) |
            IndyCryptoError::InvalidStructure(_) => None,
            IndyCryptoError::IOError(ref err) => Some(err),
            IndyCryptoError::AnoncredsRevocationAccumulatorIsFull(_) => None,
            IndyCryptoError::AnoncredsInvalidRevocationAccumulatorIndex(_) => None,
            IndyCryptoError::AnoncredsCredentialRevoked(_) => None,
            IndyCryptoError::AnoncredsProofRejected(_) => None,
        }
    }
}

impl ToErrorCode for IndyCryptoError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            IndyCryptoError::InvalidParam1(_) => ErrorCode::CommonInvalidParam1,
            IndyCryptoError::InvalidParam2(_) => ErrorCode::CommonInvalidParam2,
            IndyCryptoError::InvalidParam3(_) => ErrorCode::CommonInvalidParam3,
            IndyCryptoError::InvalidParam4(_) => ErrorCode::CommonInvalidParam4,
            IndyCryptoError::InvalidParam5(_) => ErrorCode::CommonInvalidParam5,
            IndyCryptoError::InvalidParam6(_) => ErrorCode::CommonInvalidParam6,
            IndyCryptoError::InvalidParam7(_) => ErrorCode::CommonInvalidParam7,
            IndyCryptoError::InvalidParam8(_) => ErrorCode::CommonInvalidParam8,
            IndyCryptoError::InvalidParam9(_) => ErrorCode::CommonInvalidParam9,
            IndyCryptoError::InvalidState(_) => ErrorCode::CommonInvalidState,
            IndyCryptoError::InvalidStructure(_) => ErrorCode::CommonInvalidStructure,
            IndyCryptoError::IOError(_) => ErrorCode::CommonIOError,
            IndyCryptoError::AnoncredsRevocationAccumulatorIsFull(_) => ErrorCode::AnoncredsRevocationAccumulatorIsFull,
            IndyCryptoError::AnoncredsInvalidRevocationAccumulatorIndex(_) => ErrorCode::AnoncredsInvalidRevocationAccumulatorIndex,
            IndyCryptoError::AnoncredsCredentialRevoked(_) => ErrorCode::AnoncredsCredentialRevoked,
            IndyCryptoError::AnoncredsProofRejected(_) => ErrorCode::AnoncredsProofRejected,
        }
    }
}

impl From<serde_json::Error> for IndyCryptoError {
    fn from(err: serde_json::Error) -> IndyCryptoError {
        IndyCryptoError::InvalidStructure(err.to_string())
    }
}

impl From<log::SetLoggerError> for IndyCryptoError {
    fn from(err: log::SetLoggerError) -> IndyCryptoError{
        IndyCryptoError::InvalidState(err.description().to_owned())
    }
}