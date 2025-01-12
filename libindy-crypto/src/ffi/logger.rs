extern crate libc;

use self::libc::{c_void, c_char};

use crate::errors::ErrorCode;

extern crate time;
extern crate log;

use crate::errors::ToErrorCode;

use crate::cl::logger::{EnabledCB, LogCB, FlushCB, IndyCryptoLogger, IndyCryptoDefaultLogger};
use crate::ffi::ctypes::CTypesUtils;

/// Set custom logger implementation.
///
/// Allows library user to provide custom logger implementation as set of handlers.
///
/// #Params
/// context: pointer to some logger context that will be available in logger handlers.
/// enabled: (optional) "enabled" operation handler - calls to determines if a log record would be logged. (false positive if not specified)
/// log: "log" operation handler - calls to logs a record.
/// flush: (optional) "flush" operation handler - calls to flushes buffered records (in case of crash or signal).
///
/// #Returns
/// Error code
#[no_mangle]
pub extern fn indy_crypto_set_logger(context: *const c_void,
                                     enabled: Option<EnabledCB>,
                                     log: Option<LogCB>,
                                     flush: Option<FlushCB>) -> ErrorCode {
    trace!("indy_crypto_set_logger >>> context: {:?}, enabled: {:?}, log: {:?}, flush: {:?}", context, log, enabled, flush);

    check_useful_c_callback!(log, ErrorCode::CommonInvalidParam3);

    let res = match IndyCryptoLogger::init(context, enabled, log, flush) {
        Ok(()) => ErrorCode::Success,
        Err(err) => err.to_error_code()
    };

    trace!("indy_crypto_set_logger: <<< res: {:?}", res);

    res
}

/// Set default logger implementation.
///
/// Allows library user use `env_logger` logger as default implementation.
/// More details about `env_logger` and its customization can be found here: https://crates.io/crates/env_logger
///
/// #Params
/// pattern: (optional) pattern that corresponds with the log messages to show.
///
/// NOTE: You should specify either `pattern` parameter or `RUST_LOG` environment variable to init logger.
///
/// #Returns
/// Error code
#[no_mangle]
pub extern fn indy_crypto_set_default_logger(pattern: *const c_char) -> ErrorCode {
    trace!("indy_crypto_set_default_logger >>> pattern: {:?}", pattern);

    check_useful_opt_c_str!(pattern, ErrorCode::CommonInvalidParam1);

    trace!("indy_crypto_set_default_logger: entities >>> pattern: {:?}", pattern);

    let res = match IndyCryptoDefaultLogger::init(pattern) {
        Ok(()) => ErrorCode::Success,
        Err(err) => err.to_error_code()
    };

    trace!("indy_crypto_set_default_logger: <<< res: {:?}", res);

    res
}