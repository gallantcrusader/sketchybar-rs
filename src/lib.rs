#![deny(clippy::pedantic)]

use std::{
    error::Error,
    ffi::{c_char, CStr, CString},
    fmt,
};

#[derive(Debug)]
pub enum SketchybarError {
    MessageConversionError,
    Other(String),
}

impl fmt::Display for SketchybarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SketchybarError::MessageConversionError => {
                write!(f, "Failed to convert message to CString")
            }
            SketchybarError::Other(description) => {
                write!(f, "Sketchybar error: {description}")
            }
        }
    }
}

impl Error for SketchybarError {}

pub type MachHandler = extern "C" fn(Env);
#[link(name = "sketchybar", kind = "static")]
extern "C" {
    pub fn sketchybar(message: *mut c_char) -> *mut c_char;
    pub fn event_server_begin(
        event_handler: MachHandler,
        bootstrap_name: *mut c_char,
    );
    pub fn env_get_value_for_key(env: EnvRaw, key: *mut c_char) -> *mut c_char;

}
type EnvRaw = *mut c_char;
#[repr(transparent)]
pub struct Env {
    inner: EnvRaw,
}
impl Env {
    pub fn get_v_for_c(&self, key: &str) -> String {
        let string = CString::new(key).unwrap();
        let leak = string.into_raw();
        let foo =
            unsafe { env_get_value_for_key(self.inner, leak) };
        let result = unsafe {
            core::str::from_utf8_unchecked(CStr::from_ptr(foo).to_bytes())
        }
        .to_owned();
        let _ = unsafe { CString::from_raw(leak) };
        return result;

    }
}
/// Sends a message to `SketchyBar` and returns the response.
///
/// # Arguments
///
/// * `message` - A string slice containing the message to be sent to
/// `SketchyBar`.
///
/// # Returns
///
/// * `Ok(String)` - A `Result` containing a `String` with the response from
/// `SketchyBar` upon success.
/// * `Err(Box<dyn std::error::Error>)` - A `Result` containing an error if any
/// error occurs during the operation.
///
/// # Errors
///
/// This function will return an error if:
/// * The provided message cannot be converted to a `CString`.
/// * Any other unexpected condition occurs.
///
/// # Safety
///
/// This function contains unsafe code that calls into a C library (sketchybar).
/// Ensure the C library is correctly implemented to avoid undefined behavior.
///
/// # Examples
///
/// ```no-run
/// use sketchybar_rs::message;
///
/// fn main() {
///     let response = message("--query bar").unwrap();
///
///     println!("Response from SketchyBar: {}", response);
/// }
/// ```
pub fn message(message: &str) -> Result<String, SketchybarError> {
    let command = CString::new(message)
        .map_err(|_| SketchybarError::MessageConversionError)?;
    let leak = command.into_raw();
    let result = unsafe {
        CStr::from_ptr(sketchybar(leak))
            .to_string_lossy()
            .into_owned()
    };
    let _ = unsafe{ CString::from_raw(leak) };

    Ok(result)
}

pub fn server_begin(event_handler: MachHandler, bootstrap_name: &str) {
    let string = CString::new(bootstrap_name).unwrap();
    let leak = string.into_raw();
    let _ = unsafe {
        event_server_begin(
            event_handler,
            leak,
        )
    };
    let _ = unsafe {
        CString::from_raw(leak)
    };
}
