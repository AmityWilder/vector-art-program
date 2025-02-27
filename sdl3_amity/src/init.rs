use bitflags::bitflags;
use sdl3_sys::init::*;
use crate::error::*;

bitflags! {
    /// Initialization flags for [`init`] and/or [`init_subsystem`]
    ///
    /// These are the flags which may be passed to [`init()`]. You should specify
    /// the subsystems which you will be using in your application.
    ///
    /// ### See also
    /// - [`init`]
    /// - [`Sdl::drop`]
    /// - [`init_subsystem`]
    /// - [`Sdl::quit_subsystem`]
    /// - [`SDL_WasInit`]
    pub struct InitFlags: SDL_InitFlags {
        /// [`InitFlags::Audio`] implies [`InitFlags::Events`]
        const Audio    = SDL_INIT_AUDIO;
        /// [`InitFlags::Video`] implies [`InitFlags::Events`], should be initialized on the main thread
        const Video    = SDL_INIT_VIDEO;
        /// [`InitFlags::Joystick`] implies [`InitFlags::Events`], should be initialized on the same thread as [`InitFlags::Video`] on Windows if you don't set [`SDL_HINT_JOYSTICK_THREAD`]
        const Joystick = SDL_INIT_JOYSTICK;
        const Haptic   = SDL_INIT_HAPTIC;
        /// [`InitFlags::Gamepad`] implies [`InitFlags::Joystick`]
        const Gamepad  = SDL_INIT_GAMEPAD;
        const Events   = SDL_INIT_EVENTS;
        /// [`InitFlags::Sensor`] implies [`InitFlags::Events`]
        const Sensor   = SDL_INIT_SENSOR;
        /// [`InitFlags::Camera`] implies [`InitFlags::Events`]
        const Camera   = SDL_INIT_CAMERA;
    }
}

/// Represents that SDL has been initialized and that this is the main thread. Quits SDL when dropped.
pub struct Sdl(());

impl !Send for Sdl {}
impl !Sync for Sdl {}

impl Drop for Sdl {
    /// Clean up all initialized subsystems.
    ///
    /// You should call this function even if you have already shutdown each
    /// initialized subsystem with [`Sdl::quit_subsystem()`]. It is safe to call this
    /// function even in the case of errors in initialization.
    ///
    /// You can use this function with atexit() to ensure that it is run when your
    /// application is shutdown, but it is not wise to do this from a library or
    /// other dynamically loaded code.
    ///
    /// ### See also
    /// - [`init`]
    /// - [`Sdl::quit_subsystem`]
    fn drop(&mut self) {
        unsafe { SDL_Quit(); }
    }
}

/// Initialize the SDL library.
///
/// [`init()`] simply forwards to calling [`init_subsystem()`]. Therefore, the
/// two may be used interchangeably. Though for readability of your code
/// [`init_subsystem()`] might be preferred.
///
/// The file I/O (for example: [`SDL_IOFromFile`]) and threading ([`SDL_CreateThread`])
/// subsystems are initialized by default. Message boxes
/// ([`SDL_ShowSimpleMessageBox`]) also attempt to work without initializing the
/// video subsystem, in hopes of being useful in showing an error dialog when
/// [`init`] fails. You must specifically initialize other subsystems if you
/// use them in your application.
///
/// Logging (such as [`SDL_Log`]) works without initialization, too.
///
/// `flags` may be any of the following OR'd together:
///
/// - [`InitFlags::Audio`]: audio subsystem; automatically initializes the events subsystem
/// - [`InitFlags::Video`]: video subsystem; automatically initializes the events subsystem, should be initialized on the main thread.
/// - [`InitFlags::Joystick`]: joystick subsystem; automatically initializes the events subsystem
/// - [`InitFlags::Haptic`]: haptic (force feedback) subsystem
/// - [`InitFlags::Gamepad`]: gamepad subsystem; automatically initializes the joystick subsystem
/// - [`InitFlags::Events`]: events subsystem
/// - [`InitFlags::Sensor`]: sensor subsystem; automatically initializes the events subsystem
/// - [`InitFlags::Camera`]: camera subsystem; automatically initializes the events subsystem
///
/// Subsystem initialization is ref-counted, you must call [`Sdl::quit_subsystem()`]
/// for each [`init_subsystem()`] to correctly shutdown a subsystem manually (or
/// call [`Sdl::drop()`] to force shutdown). If a subsystem is already loaded then
/// this call will increase the ref-count and return.
///
/// Consider reporting some basic metadata about your application before
/// calling [`init`], using either [`SDL_SetAppMetadata()`] or
/// [`SDL_SetAppMetadataProperty()`].
///
/// ### See also
/// - [`SDL_SetAppMetadata`]
/// - [`SDL_SetAppMetadataProperty`]
/// - [`init_subsystem`]
/// - [`Sdl::drop`]
/// - [`SDL_SetMainReady`]
/// - [`SDL_WasInit`]
#[inline]
pub fn init(err_buf: &mut ErrorBuffer, flags: InitFlags) -> Result<Sdl, SdlError<'_>> {
    init_subsystem(err_buf, flags)
}

/// Compatibility function to initialize the SDL library.
///
/// This function and [`init()`] are interchangeable.
///
/// ### See also
/// - [`init`]
/// - [`Sdl::drop`]
/// - [`Sdl::quit_subsystem`]
pub fn init_subsystem<'err>(err_buf: &'err mut ErrorBuffer, flags: InitFlags) -> Result<Sdl, SdlError<'err>> {
    unsafe { SDL_InitSubSystem(flags.bits()) }
        .sdl_err(err_buf)
        .map(|()| Sdl(()))
}

impl Sdl {
    /// Shut down specific SDL subsystems.
    ///
    /// You still need to call [`Sdl::drop()`] even if you close all open subsystems
    /// with [`Sdl::quit_subsystem()`].
    ///
    /// ### See also
    /// - [`init_subsystem`]
    /// - [`Sdl::drop`]
    pub fn quit_subsystem(&mut self, flags: InitFlags) {
        unsafe { SDL_QuitSubSystem(flags.bits()); }
    }
}
