use std::{ffi::{CStr, OsStr}, path::PathBuf};
use sdl3_sys::filesystem::*;
use crate::error::*;

/// Get the directory where the application was run from.
///
/// SDL caches the result of this call internally, but the first call to this
/// function is not necessarily fast, so plan accordingly.
///
/// **macOS and iOS Specific Functionality**: If the application is in a ".app"
/// bundle, this function returns the Resource directory (e.g.
/// MyApp.app/Contents/Resources/). This behaviour can be overridden by adding
/// a property to the Info.plist file. Adding a string key with the name
/// SDL_FILESYSTEM_BASE_DIR_TYPE with a supported value will change the
/// behaviour.
///
/// Supported values for the SDL_FILESYSTEM_BASE_DIR_TYPE property (Given an
/// application in /Applications/SDLApp/MyApp.app):
///
/// - `resource`: bundle resource directory (the default). For example:
///   `/Applications/SDLApp/MyApp.app/Contents/Resources`
/// - `bundle`: the Bundle directory. For example:
///   `/Applications/SDLApp/MyApp.app/`
/// - `parent`: the containing directory of the bundle. For example:
///   `/Applications/SDLApp/`
///
/// **Nintendo 3DS Specific Functionality**: This function returns "romfs"
/// directory of the application as it is uncommon to store resources outside
/// the executable. As such it is not a writable directory.
///
/// The returned path is guaranteed to end with a path separator ('\\' on
/// Windows, '/' on most other platforms).
///
/// ### Return value
/// Returns an absolute path in UTF-8 encoding to the application data
///   directory. NULL will be returned on error or when the platform
///   doesn't implement this functionality, call [`SDL_GetError()`] for more
///   information.
///
/// ### See also
/// - [`SDL_GetPrefPath`]
pub fn base_path<'err>(err_buf: &'err mut ErrorBuffer) -> Result<PathBuf, PlatformUnimplFeatureError<'err>> {
    let path = unsafe { SDL_GetBasePath() };
    (!path.is_null())
        .sdl_err(err_buf)
        .map_err(PlatformUnimplFeatureError::filesystem_base_path)
        .map(|()| PathBuf::from(unsafe { OsStr::from_encoded_bytes_unchecked(CStr::from_ptr(path).to_bytes()) }))
}
