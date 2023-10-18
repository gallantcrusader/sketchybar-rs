#![deny(clippy::pedantic)]

use std::{
    error::Error,
    ffi::{CStr, CString},
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

pub type mach_handler = *mut libc::c_void;
#[link(name = "sketchybar", kind = "static")]
extern "C" {
    pub fn sketchybar(message: *mut i8) -> *mut i8;
    pub fn event_server_begin(event_handler: mach_handler, bootstrap_name: *mut i8);

}
pub type Env = CString;
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

    let result = unsafe {
        CStr::from_ptr(sketchybar(command.into_raw()))
            .to_string_lossy()
            .into_owned()
    };

    Ok(result)
}

pub fn server_begin(mut event_handler: &dyn Fn(), bootstrap_name: &str) {
    let string = CString::new(bootstrap_name).unwrap();
    let _ = unsafe { 
        event_server_begin(
            &mut event_handler as *mut _ as *mut libc::c_void,
            string.into_raw())
    };
}
