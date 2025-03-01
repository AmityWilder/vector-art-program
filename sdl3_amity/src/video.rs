use std::{ffi::CStr, marker::PhantomData, mem::ManuallyDrop, ops::{Deref, DerefMut}, ptr::{null, null_mut}};
use sdl3_sys::{stdinc::{SDL_FunctionPointer, SDL_free}, video::*};
use bitflags::bitflags;
use crate::{amy_util::{null_term_sdl_parray::NullTermSdlPArray, sdl_slice::SdlSlice}, error::*, init::Sdl, properties::SdlPropertiesID};

pub mod gl;

pub struct DisplayID(SDL_DisplayID);

impl From<SDL_DisplayID> for DisplayID {
    fn from(value: SDL_DisplayID) -> Self {
        Self(value)
    }
}

impl DisplayID {
    pub const fn get(&self) -> SDL_DisplayID {
        self.0
    }
}

/// The structure that defines a display mode.
///
/// ### See also
/// - [`DisplayMode::fullscreen_modes`]
/// - [`DisplayMode::desktop`]
/// - [`DisplayMode::current`]
/// - [`Window::set_fullscreen_mode`]
/// - [`Window::fullscreen_mode`]
#[repr(transparent)]
pub struct DisplayMode<'a>(&'a SDL_DisplayMode);

impl <'a> DisplayMode<'a> {
    /// the display this mode is associated with
    pub fn display_id(&self) -> DisplayID {
        DisplayID(self.0.displayID)
    }

    /// pixel format
    pub fn format(&self) -> &sdl3_sys::pixels::SDL_PixelFormat {
        &self.0.format
    }

    /// width
    pub fn w(&self) -> Result<u32, std::num::TryFromIntError> {
        self.0.w.try_into()
    }

    /// height
    pub fn h(&self) -> Result<u32, std::num::TryFromIntError> {
        self.0.h.try_into()
    }

    /// scale converting size to pixels (e.g. a 1920x1080 mode with 2.0 scale would have 3840x2160 pixels)
    pub fn pixel_density(&self) -> &f32 {
        &self.0.pixel_density
    }

    /// refresh rate (or 0.0f for unspecified)
    pub fn refresh_rate(&self) -> &f32 {
        &self.0.refresh_rate
    }

    /// precise refresh rate numerator (or 0 for unspecified)
    pub fn refresh_rate_numerator(&self) -> Result<Option<std::num::NonZeroU32>, std::num::TryFromIntError> {
        self.0.refresh_rate_numerator.try_into()
            .map(|n| std::num::NonZeroU32::new(n))
    }

    /// precise refresh rate denominator
    pub fn refresh_rate_denominator(&self) -> Result<std::num::NonZeroU32, std::num::TryFromIntError> {
        let denom = self.0.refresh_rate_denominator;
        // we are fine with truncating negative max because any negative will error regardless
        let n = denom.saturating_sub(1);
        // confirm `denom - 1 >= 0` and return a TryFromIntError otherwise; which is equivalent to `denom >= 1`
        _ = u32::try_from(n)?;
        // this is safe because we already know `u32::MIN <= 1 (NonZeroU32::MIN) <= denom <= i32::MAX <= u32::MAX`, meeting the requirements of both u32 and NonZero
        Ok({
            #[cfg(not(debug_assertions))]
            unsafe { std::num::NonZeroU32::new_unchecked(denom as u32) }
            #[cfg(debug_assertions)]
            std::num::NonZeroU32::new(denom.try_into()
                .expect("refresh rate denominator should be positive"))
                .expect("refresh rate denominator should be non-zero")
        })
    }
}

/// The structure that defines a display mode.
///
/// ### See also
/// - [`DisplayMode::fullscreen_modes`]
/// - [`DisplayMode::desktop`]
/// - [`DisplayMode::current`]
/// - [`Window::set_fullscreen_mode`]
/// - [`Window::fullscreen_mode`]
#[repr(transparent)]
pub struct DisplayModeMut<'a>(&'a mut SDL_DisplayMode);

impl<'a> Deref for DisplayModeMut<'a> {
    type Target = DisplayMode<'a>;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> DisplayMode<'a> {
    /// Get a list of fullscreen display modes available on a display.
    ///
    /// The display modes are sorted in this priority:
    ///
    /// - w -> largest to smallest
    /// - h -> largest to smallest
    /// - bits per pixel -> more colors to fewer colors
    /// - packed pixel layout -> largest to smallest
    /// - refresh rate -> highest to lowest
    /// - pixel density -> lowest to highest
    ///
    /// ### See also
    /// - [`SDL_GetDisplays`]
    pub fn fullscreen_modes<'err>(_: &Sdl, err_buf: &'err mut ErrorBuffer, display_id: DisplayID) -> Result<SdlSlice<&'a mut SDL_DisplayMode>, SdlOrIntError<'err>> {
        let mut count: i32 = -1;
        let mem = unsafe { SDL_GetFullscreenDisplayModes(display_id.get(), &raw mut count) };
        SdlSlice::try_from_raw_parts(mem.cast(), count.try_into()?)
            .sdl_erri(err_buf)
    }

    /// Get information about the desktop's display mode.
    ///
    /// There's a difference between this function and [`DisplayMode::current()`]
    /// when SDL runs fullscreen and has changed the resolution. In that case this
    /// function will return the previous native display mode, and not the current
    /// display mode.
    ///
    /// ### See also
    /// - [`DisplayMode::current`]
    /// - [`SDL_GetDisplays`]
    pub fn desktop<'err>(_: &Sdl, err_buf: &'err mut ErrorBuffer, display_id: DisplayID) -> Result<DisplayMode<'a>, SdlError<'err>> {
        unsafe { SDL_GetDesktopDisplayMode(display_id.get()).as_ref::<'a>() }
            .map(|mode| Self(mode))
            .sdl_err(err_buf)
    }

    /// Get information about the current display mode.
    ///
    /// There's a difference between this function and [`DisplayMode::desktop()`]
    /// when SDL runs fullscreen and has changed the resolution. In that case this
    /// function will return the current display mode, and not the previous native
    /// display mode.
    ///
    /// ### See also
    /// - [`DisplayMode::desktop`]
    /// - [`SDL_GetDisplays`]
    pub fn current<'err>(_: &Sdl, err_buf: &'err mut ErrorBuffer, display_id: DisplayID) -> Result<DisplayMode<'a>, SdlError<'err>> {
        unsafe { SDL_GetCurrentDisplayMode(display_id.get()).as_ref::<'a>() }
            .map(|mode| Self(mode))
            .sdl_err(err_buf)
    }
}

bitflags! {
    pub struct WindowFlags: SDL_WindowFlags {
        /// fullscreen window at desktop resolution
        const Fullscreen        = SDL_WINDOW_FULLSCREEN;
        /// window usable with an OpenGL context
        const OpenGL            = SDL_WINDOW_OPENGL;
        /// window partially or completely obscured by another window
        const Occluded          = SDL_WINDOW_OCCLUDED;
        /// window is not visible \
        /// it is neither mapped onto the desktop nor shown in the taskbar/dock/window list; [`Window::show()`] is required for it to become visible
        const Hidden            = SDL_WINDOW_HIDDEN;
        /// no window decoration
        const Borderless        = SDL_WINDOW_BORDERLESS;
        /// window can be resized
        const Resizable         = SDL_WINDOW_RESIZABLE;
        /// window is minimized
        const Minimized         = SDL_WINDOW_MINIMIZED;
        /// window is maximized
        const Maximized         = SDL_WINDOW_MAXIMIZED;
        /// window has grabbed mouse focus
        const MouseGrabbed      = SDL_WINDOW_MOUSE_GRABBED;
        /// window has input focus
        const InputFocus        = SDL_WINDOW_INPUT_FOCUS;
        /// window has mouse focus
        const MouseFocus        = SDL_WINDOW_MOUSE_FOCUS;
        /// window not created by SDL
        const External          = SDL_WINDOW_EXTERNAL;
        /// window is modal
        const Modal             = SDL_WINDOW_MODAL;
        /// window uses high pixel density back buffer if possible
        const HighPixelDensity  = SDL_WINDOW_HIGH_PIXEL_DENSITY;
        /// window has mouse captured (unrelated to `MouseGrabbed`)
        const MouseCapture      = SDL_WINDOW_MOUSE_CAPTURE;
        /// window has relative mode enabled
        const MouseRelativeMode = SDL_WINDOW_MOUSE_RELATIVE_MODE;
        /// window should always be above others
        const AlwaysOnTop       = SDL_WINDOW_ALWAYS_ON_TOP;
        /// window should be treated as a utility window, not showing in the task bar and window list
        const Utility           = SDL_WINDOW_UTILITY;
        /// window should be treated as a tooltip and does not get mouse or keyboard focus, requires a parent window
        const Tooltip           = SDL_WINDOW_TOOLTIP;
        /// window should be treated as a popup menu, requires a parent window
        const PopupMenu         = SDL_WINDOW_POPUP_MENU;
        /// window has grabbed keyboard input
        const KeyboardGrabbed   = SDL_WINDOW_KEYBOARD_GRABBED;
        /// window usable with a Vulkan instance
        const Vulkan            = SDL_WINDOW_VULKAN;
        /// window usable with a Metal instance
        const Metal             = SDL_WINDOW_METAL;
        /// window with transparent buffer
        const Transparent       = SDL_WINDOW_TRANSPARENT;
        /// window should not be focusable
        const NotFocusable      = SDL_WINDOW_NOT_FOCUSABLE;
    }

    pub struct PopupFlags: SDL_WindowFlags {
        /// window usable with an OpenGL context
        const OpenGL            = SDL_WINDOW_OPENGL;
        /// window partially or completely obscured by another window
        const Occluded          = SDL_WINDOW_OCCLUDED;
        /// window is not visible \
        /// it is neither mapped onto the desktop nor shown in the taskbar/dock/window list; [`Window::show()`] is required for it to become visible
        const Hidden            = SDL_WINDOW_HIDDEN;
        /// window can be resized
        const Resizable         = SDL_WINDOW_RESIZABLE;
        /// window has grabbed mouse focus
        const MouseGrabbed      = SDL_WINDOW_MOUSE_GRABBED;
        /// window has input focus
        const InputFocus        = SDL_WINDOW_INPUT_FOCUS;
        /// window has mouse focus
        const MouseFocus        = SDL_WINDOW_MOUSE_FOCUS;
        /// window not created by SDL
        const External          = SDL_WINDOW_EXTERNAL;
        /// window uses high pixel density back buffer if possible
        const HighPixelDensity  = SDL_WINDOW_HIGH_PIXEL_DENSITY;
        /// window has mouse captured (unrelated to `MouseGrabbed`)
        const MouseCapture      = SDL_WINDOW_MOUSE_CAPTURE;
        /// window has relative mode enabled
        const MouseRelativeMode = SDL_WINDOW_MOUSE_RELATIVE_MODE;
        /// window should always be above others
        const AlwaysOnTop       = SDL_WINDOW_ALWAYS_ON_TOP;
        /// window should be treated as a tooltip and does not get mouse or keyboard focus, requires a parent window
        const Tooltip           = SDL_WINDOW_TOOLTIP;
        /// window should be treated as a popup menu, requires a parent window
        const PopupMenu         = SDL_WINDOW_POPUP_MENU;
        /// window has grabbed keyboard input
        const KeyboardGrabbed   = SDL_WINDOW_KEYBOARD_GRABBED;
        /// window usable with a Vulkan instance
        const Vulkan            = SDL_WINDOW_VULKAN;
        /// window usable with a Metal instance
        const Metal             = SDL_WINDOW_METAL;
        /// window with transparent buffer
        const Transparent       = SDL_WINDOW_TRANSPARENT;
        /// window should not be focusable
        const NotFocusable      = SDL_WINDOW_NOT_FOCUSABLE;
    }
}

#[repr(u64)]
pub enum PopupType {
    Tooltip   = PopupFlags::Tooltip.bits(),
    PopupMenu = PopupFlags::PopupMenu.bits(),
    Both      = PopupFlags::Tooltip.union(PopupFlags::PopupMenu).bits(),
}

pub struct PopupConfig(PopupFlags);

impl PopupConfig {
    pub const fn new(ty: PopupType, flags: PopupFlags) -> Self {
        Self(PopupFlags::from_bits_retain(ty as SDL_WindowFlags).union(flags))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum WindowPos {
    Value(i32),
    Centered = SDL_WINDOWPOS_CENTERED,
    Undefined = SDL_WINDOWPOS_UNDEFINED,
}

impl WindowPos {
    #[inline]
    pub const fn as_i32(self) -> i32 {
        match self {
            Self::Value(x) => {
                debug_assert!(x != SDL_WINDOWPOS_CENTERED, "`WindowPos::Value(...)` cannot be `SDL_WINDOWPOS_CENTERED`, use `WindowPos::Centered` instead");
                debug_assert!(x != SDL_WINDOWPOS_UNDEFINED, "`WindowPos::Value(...)` cannot be `SDL_WINDOWPOS_UNDEFINED`, use `WindowPos::Undefined` instead");
                x
            },
            Self::Centered => SDL_WINDOWPOS_CENTERED,
            Self::Undefined => SDL_WINDOWPOS_UNDEFINED,
        }
    }
}


/// The struct used as an opaque handle to a window.
///
/// Because a window can only be created from the main thread and cannot be passed between threads,
/// all [`Window`] methods are assumed to be called from the main thread.
///
/// ### See also
/// - [`Window::create`]
pub struct Window<'a>(&'a mut SDL_Window);

impl<'a> Window<'a> {
    #[inline]
    pub(crate) fn raw(&self) -> *mut SDL_Window {
        <*const SDL_Window>::from(self.0).cast_mut()
    }
}

impl !Send for Window<'_> {}
impl !Sync for Window<'_> {}

impl Drop for Window<'_> {
    /// Destroy a window.
    ///
    /// Any child windows owned by the window will be recursively destroyed as
    /// well.
    ///
    /// Note that on some platforms, the visible window may not actually be removed
    /// from the screen until the SDL event loop is pumped again, even though the
    /// [`SDL_Window`] is no longer valid after this call.
    ///
    /// ### See also
    /// - [`Window::create_popup`]
    /// - [`Window::create`]
    /// - [`Window::create_with_properties`]
    fn drop(&mut self) {
        unsafe { SDL_DestroyWindow(self.0); }
    }
}

impl<'a> Window<'a> {
    /// Get the size of a window's client area.
    ///
    /// The window pixel size may differ from its window coordinate size if the
    /// window is on a high pixel density display. Use [`Window::size_in_pixels()`]
    /// or [`Renderer::get_output_size()`] to get the real client area size in pixels.
    ///
    /// ### See also
    /// - [`Renderer::get_output_size`]
    /// - [`Window::size_in_pixels`]
    /// - [`Window::set_size`]
    pub fn size<'err>(&self, err_buf: &'err mut ErrorBuffer) -> Result<(u32, u32), SdlOrIntError<'err>> {
        let mut w = -1;
        let mut h = -1;
        unsafe { SDL_GetWindowSize(self.raw(), &raw mut w, &raw mut h) }
            .sdl_erri(err_buf)
            .and_then(|()| Ok((w.try_into()?, h.try_into()?)))
    }

    /// Get the size of a window's client area, in pixels.
    ///
    /// ### See also
    /// - [`Window::create`]
    /// - [`Window::size`]
    pub fn size_in_pixels<'err>(&self, err_buf: &'err mut ErrorBuffer) -> Result<(u32, u32), SdlOrIntError<'err>> {
        let mut w = -1;
        let mut h = -1;
        unsafe { SDL_GetWindowSizeInPixels(self.raw(), &raw mut w, &raw mut h) }
            .sdl_erri(err_buf)
            .and_then(|()| Ok((w.try_into()?, h.try_into()?)))
    }

    /// Request that the size of a window's client area be set.
    ///
    /// If the window is in a fullscreen or maximized state, this request has no
    /// effect.
    ///
    /// To change the exclusive fullscreen mode of a window, use
    /// [`Window::set_fullscreen_mode()`].
    ///
    /// On some windowing systems, this request is asynchronous and the new window
    /// size may not have have been applied immediately upon the return of this
    /// function. If an immediate change is required, call [`SDL_SyncWindow()`] to
    /// block until the changes have taken effect.
    ///
    /// When the window size changes, an [`SDL_EVENT_WINDOW_RESIZED`] event will be
    /// emitted with the new window dimensions. Note that the new dimensions may
    /// not match the exact size requested, as some windowing systems can restrict
    /// the window size in certain scenarios (e.g. constraining the size of the
    /// content area to remain within the usable desktop bounds). Additionally, as
    /// this is just a request, it can be denied by the windowing system.
    ///
    /// ### See also
    /// - [`Window::size`]
    /// - [`Window::set_fullscreen_mode`]
    /// - [`SDL_SyncWindow`]
    pub fn set_size<'err>(&mut self, err_buf: &'err mut ErrorBuffer, w: u32, h: u32) -> Result<(), SdlOrIntError<'err>> {
        unsafe { SDL_SetWindowSize(self.raw(), w.try_into()?, h.try_into()?) }
            .sdl_erri(err_buf)
    }

    /// Block until any pending window state is finalized.
    ///
    /// On asynchronous windowing systems, this acts as a synchronization barrier
    /// for pending window state. It will attempt to wait until any pending window
    /// state has been applied and is guaranteed to return within finite time. Note
    /// that for how long it can potentially block depends on the underlying window
    /// system, as window state changes may involve somewhat lengthy animations
    /// that must complete before the window is in its final requested state.
    ///
    /// On windowing systems where changes are immediate, this does nothing.
    ///
    /// ### See also
    /// - [`Window::set_size`]
    /// - [`Window::set_position`]
    /// - [`Window::set_fullscreen`]
    /// - [`Window::minimize`]
    /// - [`Window::maximize`]
    /// - [`Window::restore`]
    /// - [`SDL_HINT_VIDEO_SYNC_WINDOW_OPERATIONS`]
    pub fn sync<'err>(&mut self, err_buf: &'err mut ErrorBuffer) -> Result<(), TimeoutError<'err>> {
        if unsafe { SDL_SyncWindow(self.raw()) } { Ok(()) } else { Err(TimeoutError(err_buf.get())) }
    }

    /// Show a window.
    ///
    /// ### See also
    /// - [`Window::hide`]
    /// - [`Window::raise`]
    pub fn show<'err>(&mut self, err_buf: &'err mut ErrorBuffer) -> Result<(), SdlError<'err>> {
        unsafe { SDL_ShowWindow(self.raw()) }.sdl_err(err_buf)
    }

    /// Hide a window.
    ///
    /// ### See also
    /// - [`Window::show`]
    /// - [`SDL_WINDOW_HIDDEN`]
    pub fn hide<'err>(&mut self, err_buf: &'err mut ErrorBuffer) -> Result<(), SdlError<'err>> {
        unsafe { SDL_HideWindow(self.raw()) }.sdl_err(err_buf)
    }

    /// Request that a window be raised above other windows and gain the input
    /// focus.
    ///
    /// The result of this request is subject to desktop window manager policy,
    /// particularly if raising the requested window would result in stealing focus
    /// from another application. If the window is successfully raised and gains
    /// input focus, an [`SDL_EVENT_WINDOW_FOCUS_GAINED`] event will be emitted, and
    /// the window will have the [`SDL_WINDOW_INPUT_FOCUS`] flag set.
    pub fn raise<'err>(&mut self, err_buf: &'err mut ErrorBuffer) -> Result<(), SdlError<'err>> {
        unsafe { SDL_RaiseWindow(self.raw()) }.sdl_err(err_buf)
    }

    /// Request that the window's position be set.
    ///
    /// If the window is in an exclusive fullscreen or maximized state, this
    /// request has no effect.
    ///
    /// This can be used to reposition fullscreen-desktop windows onto a different
    /// display, however, as exclusive fullscreen windows are locked to a specific
    /// display, they can only be repositioned programmatically via
    /// [`Window::set_fullscreen_mode()`].
    ///
    /// On some windowing systems this request is asynchronous and the new
    /// coordinates may not have have been applied immediately upon the return of
    /// this function. If an immediate change is required, call [`Window::sync()`] to
    /// block until the changes have taken effect.
    ///
    /// When the window position changes, an [`SDL_EVENT_WINDOW_MOVED`] event will be
    /// emitted with the window's new coordinates. Note that the new coordinates
    /// may not match the exact coordinates requested, as some windowing systems
    /// can restrict the position of the window in certain scenarios (e.g.
    /// constraining the position so the window is always within desktop bounds).
    /// Additionally, as this is just a request, it can be denied by the windowing
    /// system.
    ///
    /// ### See also
    /// - [`Window::position`]
    /// - [`Window::sync`]
    pub fn set_position<'err>(&mut self, err_buf: &'err mut ErrorBuffer, x: WindowPos, y: WindowPos) -> Result<(), SdlError<'err>> {
        unsafe { SDL_SetWindowPosition(self.raw(), x.as_i32(), y.as_i32()) }
            .sdl_err(err_buf)
    }

    /// Get the position of a window.
    ///
    /// This is the current position of the window as last reported by the
    /// windowing system.
    ///
    /// If you do not need the value for one of the positions call [`Window::position_x`]
    /// or [`Window::position_y`] instead
    ///
    /// ### See also
    /// - [`Window::set_position`]
    pub fn position<'err>(&self, err_buf: &'err mut ErrorBuffer) -> Result<(i32, i32), SdlError<'err>> {
        let mut x = 0;
        let mut y = 0;
        unsafe { SDL_GetWindowPosition(self.raw(), &raw mut x, &raw mut y) }
            .sdl_err(err_buf)
            .map(|()| (x, y))
    }

    /// Get the x-position of a window.
    ///
    /// This is the current horizintal position of the window as last reported by the
    /// windowing system.
    ///
    /// ### See also
    /// - [`Window::position`]
    /// - [`Window::position_y`]
    /// - [`Window::set_position`]
    pub fn position_x<'err>(&self, err_buf: &'err mut ErrorBuffer) -> Result<i32, SdlError<'err>> {
        let mut x = 0;
        unsafe { SDL_GetWindowPosition(self.raw(), &raw mut x, null_mut()) }
            .sdl_err(err_buf)
            .map(|()| x)
    }

    /// Get the y-position of a window.
    ///
    /// This is the current vertical position of the window as last reported by the
    /// windowing system.
    ///
    /// ### See also
    /// - [`Window::position`]
    /// - [`Window::position_x`]
    /// - [`Window::set_position`]
    pub fn position_y<'err>(&self, err_buf: &'err mut ErrorBuffer) -> Result<i32, SdlError<'err>> {
        let mut y = 0;
        unsafe { SDL_GetWindowPosition(self.raw(), null_mut(), &raw mut y) }
            .sdl_err(err_buf)
            .map(|()| y)
    }

    /// Request that the window's fullscreen state be changed.
    ///
    /// By default a window in fullscreen state uses borderless fullscreen desktop
    /// mode, but a specific exclusive display mode can be set using
    /// [`Window::set_fullscreen_mode()`].
    ///
    /// On some windowing systems this request is asynchronous and the new
    /// fullscreen state may not have have been applied immediately upon the return
    /// of this function. If an immediate change is required, call [`Window::sync()`]
    /// to block until the changes have taken effect.
    ///
    /// When the window state changes, an [`SDL_EVENT_WINDOW_ENTER_FULLSCREEN`] or
    /// [`SDL_EVENT_WINDOW_LEAVE_FULLSCREEN`] event will be emitted. Note that, as this
    /// is just a request, it can be denied by the windowing system.
    ///
    /// ### Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ### See also
    /// - [`Window::fullscreen_mode`]
    /// - [`Window::set_fullscreen_mode`]
    /// - [`Window::sync`]
    /// - [`SDL_WINDOW_FULLSCREEN`]
    pub fn set_fullscreen<'err>(&mut self, err_buf: &'err mut ErrorBuffer, fullscreen: bool) -> Result<(), SdlError<'err>> {
        unsafe { SDL_SetWindowFullscreen(self.raw(), fullscreen) }
            .sdl_err(err_buf)
    }

    /// Set the display mode to use when a window is visible and fullscreen.
    ///
    /// This only affects the display mode used when the window is fullscreen. To
    /// change the window size when the window is not fullscreen, use
    /// [`Window::set_size()`].
    ///
    /// If the window is currently in the fullscreen state, this request is
    /// asynchronous on some windowing systems and the new mode dimensions may not
    /// be applied immediately upon the return of this function. If an immediate
    /// change is required, call [`Window::sync()`] to block until the changes have
    /// taken effect.
    ///
    /// When the new mode takes effect, an [`SDL_EVENT_WINDOW_RESIZED`] and/or an
    /// [`SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED`] event will be emitted with the new mode
    /// dimensions.
    ///
    /// ### Parameters
    /// - `mode`: a pointer to the display mode to use, which can be NULL for
    ///   borderless fullscreen desktop mode, or one of the fullscreen
    ///   modes returned by [`DisplayMode::fullscreen_modes()`] to set an
    ///   exclusive fullscreen mode.
    ///
    /// ### See also
    /// - [`Window::fullscreen_mode`]
    /// - [`Window::set_fullscreen`]
    /// - [`Window::sync`]
    pub fn set_fullscreen_mode<'err>(&mut self, err_buf: &'err mut ErrorBuffer, mode: Option<DisplayMode>) -> Result<(), SdlError<'err>> {
        unsafe { SDL_SetWindowFullscreenMode(self.raw(), mode.map_or_else(null, |mode| mode.0)) }
            .sdl_err(err_buf)
    }


    /// Request that the window be minimized to an iconic representation.
    ///
    /// If the window is in a fullscreen state, this request has no direct effect.
    /// It may alter the state the window is returned to when leaving fullscreen.
    ///
    /// On some windowing systems this request is asynchronous and the new window
    /// state may not have been applied immediately upon the return of this
    /// function. If an immediate change is required, call [`Window::sync()`] to
    /// block until the changes have taken effect.
    ///
    /// When the window state changes, an [`SDL_EVENT_WINDOW_MINIMIZED`] event will be
    /// emitted. Note that, as this is just a request, the windowing system can
    /// deny the state change.
    ///
    /// ### Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ### See also
    /// - [`Window::maximize`]
    /// - [`Window::restore`]
    /// - [`Window::sync`]
    pub fn minimize<'err>(&mut self, err_buf: &'err mut ErrorBuffer) -> Result<(), SdlError<'err>> {
        unsafe { SDL_MinimizeWindow(self.raw()) }
            .sdl_err(err_buf)
    }

    /// Request that the window be made as large as possible.
    ///
    /// Non-resizable windows can't be maximized. The window must have the
    /// [`SDL_WINDOW_RESIZABLE`] flag set, or this will have no effect.
    ///
    /// On some windowing systems this request is asynchronous and the new window
    /// state may not have have been applied immediately upon the return of this
    /// function. If an immediate change is required, call [`Window::sync()`] to
    /// block until the changes have taken effect.
    ///
    /// When the window state changes, an [`SDL_EVENT_WINDOW_MAXIMIZED`] event will be
    /// emitted. Note that, as this is just a request, the windowing system can
    /// deny the state change.
    ///
    /// When maximizing a window, whether the constraints set via
    /// [`Window::set_maximum_size()`] are honored depends on the policy of the window
    /// manager. Win32 and macOS enforce the constraints when maximizing, while X11
    /// and Wayland window managers may vary.
    ///
    /// ### Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ### See also
    /// - [`Window::minimize`]
    /// - [`Window::restore`]
    /// - [`Window::sync`]
    pub fn maximize<'err>(&mut self, err_buf: &'err mut ErrorBuffer) -> Result<(), SdlError<'err>> {
        unsafe { SDL_MaximizeWindow(self.raw()) }
            .sdl_err(err_buf)
    }

    pub fn restore<'err>(&mut self, err_buf: &'err mut ErrorBuffer) -> Result<(), SdlError<'err>> {
        unsafe { SDL_RestoreWindow(self.raw()) }
            .sdl_err(err_buf)
    }

    /// Create a window with the specified dimensions and flags.
    ///
    /// `flags` may be any [`WindowFlags`] OR'd together.
    ///
    /// The [`SDL_Window`] is implicitly shown if [`WindowFlags::Hidden`] is not set.
    ///
    /// On Apple's macOS, you **must** set the NSHighResolutionCapable Info.plist
    /// property to YES, otherwise you will not receive a High-DPI OpenGL canvas.
    ///
    /// The window pixel size may differ from its window coordinate size if the
    /// window is on a high pixel density display. Use [`Window::size()`] to query
    /// the client area's size in window coordinates, and
    /// [`SDL_GetWindowSizeInPixels()`] or [`SDL_GetRenderOutputSize()`] to query the
    /// drawable size in pixels. Note that the drawable size can vary after the
    /// window is created and should be queried again if you get an
    /// [`SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED`] event.
    ///
    /// If the window is created with any of the [`WindowFlags::OpenGL`] or
    /// [`WindowFlags::Vulkan`] flags, then the corresponding LoadLibrary function
    /// ([`SDL_GL_LoadLibrary`] or [`SDL_Vulkan_LoadLibrary`]) is called and the
    /// corresponding UnloadLibrary function is called by [`Window::drop()`].
    ///
    /// If [`SDL_WINDOW_VULKAN`] is specified and there isn't a working Vulkan driver,
    /// [`Window::create()`] will fail, because [`SDL_Vulkan_LoadLibrary()`] will fail.
    ///
    /// If [`SDL_WINDOW_METAL`] is specified on an OS that does not support Metal,
    /// [`Window::create()`] will fail.
    ///
    /// If you intend to use this window with an [`SDL_Renderer`], you should use
    /// [`Self::create_with_renderer()`] instead of this function, to avoid window
    /// flicker.
    ///
    /// On non-Apple devices, SDL requires you to either not link to the Vulkan
    /// loader or link to a dynamic library version. This limitation may be removed
    /// in a future version of SDL.
    ///
    /// ### Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ### See also
    /// - [`Window::create_and_renderer`]
    /// - [`Window::create_popup`]
    /// - [`Window::create_with_properties`]
    /// - [`Window::drop`]
    pub fn create<'err>(
        _: &'a Sdl,
        err_buf: &'err mut ErrorBuffer,
        title: &CStr,
        w: u32,
        h: u32,
        flags: WindowFlags,
    ) -> Result<Self, SdlOrIntError<'err>> {
        unsafe {
            SDL_CreateWindow(
                title.as_ptr(),
                w.try_into()?,
                h.try_into()?,
                flags.bits(),
            ).as_mut()
        }
            .map(|window| Self(window))
            .sdl_erri(err_buf)
    }

    /// Create a window with the specified properties.
    ///
    /// These are the supported properties:
    ///
    /// - [`SDL_PROP_WINDOW_CREATE_ALWAYS_ON_TOP_BOOLEAN`]\: true if the window should
    ///   be always on top
    /// - [`SDL_PROP_WINDOW_CREATE_BORDERLESS_BOOLEAN`]\: true if the window has no
    ///   window decoration
    /// - [`SDL_PROP_WINDOW_CREATE_EXTERNAL_GRAPHICS_CONTEXT_BOOLEAN`]\: true if the
    ///   window will be used with an externally managed graphics context.
    /// - [`SDL_PROP_WINDOW_CREATE_FOCUSABLE_BOOLEAN`]\: true if the window should
    ///   accept keyboard input (defaults true)
    /// - [`SDL_PROP_WINDOW_CREATE_FULLSCREEN_BOOLEAN`]\: true if the window should
    ///   start in fullscreen mode at desktop resolution
    /// - [`SDL_PROP_WINDOW_CREATE_HEIGHT_NUMBER`]\: the height of the window
    /// - [`SDL_PROP_WINDOW_CREATE_HIDDEN_BOOLEAN`]\: true if the window should start
    ///   hidden
    /// - [`SDL_PROP_WINDOW_CREATE_HIGH_PIXEL_DENSITY_BOOLEAN`]\: true if the window
    ///   uses a high pixel density buffer if possible
    /// - [`SDL_PROP_WINDOW_CREATE_MAXIMIZED_BOOLEAN`]\: true if the window should
    ///   start maximized
    /// - [`SDL_PROP_WINDOW_CREATE_MENU_BOOLEAN`]\: true if the window is a popup menu
    /// - [`SDL_PROP_WINDOW_CREATE_METAL_BOOLEAN`]\: true if the window will be used
    ///   with Metal rendering
    /// - [`SDL_PROP_WINDOW_CREATE_MINIMIZED_BOOLEAN`]\: true if the window should
    ///   start minimized
    /// - [`SDL_PROP_WINDOW_CREATE_MODAL_BOOLEAN`]\: true if the window is modal to
    ///   its parent
    /// - [`SDL_PROP_WINDOW_CREATE_MOUSE_GRABBED_BOOLEAN`]\: true if the window starts
    ///   with grabbed mouse focus
    /// - [`SDL_PROP_WINDOW_CREATE_OPENGL_BOOLEAN`]\: true if the window will be used
    ///   with OpenGL rendering
    /// - [`SDL_PROP_WINDOW_CREATE_PARENT_POINTER`]\: an [`SDL_Window`] that will be the
    ///   parent of this window, required for windows with the "tooltip", "menu",
    ///   and "modal" properties
    /// - [`SDL_PROP_WINDOW_CREATE_RESIZABLE_BOOLEAN`]\: true if the window should be
    ///   resizable
    /// - [`SDL_PROP_WINDOW_CREATE_TITLE_STRING`]\: the title of the window, in UTF-8
    ///   encoding
    /// - [`SDL_PROP_WINDOW_CREATE_TRANSPARENT_BOOLEAN`]\: true if the window show
    ///   transparent in the areas with alpha of 0
    /// - [`SDL_PROP_WINDOW_CREATE_TOOLTIP_BOOLEAN`]\: true if the window is a tooltip
    /// - [`SDL_PROP_WINDOW_CREATE_UTILITY_BOOLEAN`]\: true if the window is a utility
    ///   window, not showing in the task bar and window list
    /// - [`SDL_PROP_WINDOW_CREATE_VULKAN_BOOLEAN`]\: true if the window will be used
    ///   with Vulkan rendering
    /// - [`SDL_PROP_WINDOW_CREATE_WIDTH_NUMBER`]\: the width of the window
    /// - [`SDL_PROP_WINDOW_CREATE_X_NUMBER`]\: the x position of the window, or
    ///   [`SDL_WINDOWPOS_CENTERED`], defaults to [`SDL_WINDOWPOS_UNDEFINED`]. This is
    ///   relative to the parent for windows with the "tooltip" or "menu" property
    ///   set.
    /// - [`SDL_PROP_WINDOW_CREATE_Y_NUMBER`]\: the y position of the window, or
    ///   [`SDL_WINDOWPOS_CENTERED`], defaults to [`SDL_WINDOWPOS_UNDEFINED`]. This is
    ///   relative to the parent for windows with the "tooltip" or "menu" property
    ///   set.
    ///
    /// These are additional supported properties on macOS:
    ///
    /// - `SDL_PROP_WINDOW_CREATE_COCOA_WINDOW_POINTER`: the
    ///   `(__unsafe_unretained)` NSWindow associated with the window, if you want
    ///   to wrap an existing window.
    /// - [`SDL_PROP_WINDOW_CREATE_COCOA_VIEW_POINTER`]\: the `(__unsafe_unretained)`
    ///   NSView associated with the window, defaults to `[window contentView]`
    ///
    /// These are additional supported properties on Wayland:
    ///
    /// - [`SDL_PROP_WINDOW_CREATE_WAYLAND_SURFACE_ROLE_CUSTOM_BOOLEAN`] - true if
    ///   the application wants to use the Wayland surface for a custom role and
    ///   does not want it attached to an XDG toplevel window. See
    ///   [README/wayland](README/wayland) for more information on using custom
    ///   surfaces.
    /// - [`SDL_PROP_WINDOW_CREATE_WAYLAND_CREATE_EGL_WINDOW_BOOLEAN`] - true if the
    ///   application wants an associated `wl_egl_window` object to be created and
    ///   attached to the window, even if the window does not have the OpenGL
    ///   property or [`SDL_WINDOW_OPENGL`] flag set.
    /// - [`SDL_PROP_WINDOW_CREATE_WAYLAND_WL_SURFACE_POINTER`] - the wl_surface
    ///   associated with the window, if you want to wrap an existing window. See
    ///   [README/wayland](README/wayland) for more information.
    ///
    /// These are additional supported properties on Windows:
    ///
    /// - [`SDL_PROP_WINDOW_CREATE_WIN32_HWND_POINTER`]\: the HWND associated with the
    ///   window, if you want to wrap an existing window.
    /// - [`SDL_PROP_WINDOW_CREATE_WIN32_PIXEL_FORMAT_HWND_POINTER`]\: optional,
    ///   another window to share pixel format with, useful for OpenGL windows
    ///
    /// These are additional supported properties with X11:
    ///
    /// - [`SDL_PROP_WINDOW_CREATE_X11_WINDOW_NUMBER`]\: the X11 Window associated
    ///   with the window, if you want to wrap an existing window.
    ///
    /// The window is implicitly shown if the "hidden" property is not set.
    ///
    /// Windows with the "tooltip" and "menu" properties are popup windows and have
    /// the behaviors and guidelines outlined in [`Window::create_popup()`].
    ///
    /// If this window is being created to be used with an [`SDL_Renderer`], you should
    /// not add a graphics API specific property
    /// ([`SDL_PROP_WINDOW_CREATE_OPENGL_BOOLEAN`], etc), as SDL will handle that
    /// internally when it chooses a renderer. However, SDL might need to recreate
    /// your window at that point, which may cause the window to appear briefly,
    /// and then flicker as it is recreated. The correct approach to this is to
    /// create the window with the [`SDL_PROP_WINDOW_CREATE_HIDDEN_BOOLEAN`] property
    /// set to true, then create the renderer, then show the window with
    /// [`Window::show()`].
    ///
    /// ### Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ### See also
    /// - [`SDL_CreateProperties`]
    /// - [`Window::create`]
    /// - [`Window::drop`]
    pub fn create_with_properties<'err>(
        _: &'a Sdl,
        err_buf: &'err mut ErrorBuffer,
        props: SdlPropertiesID,
    ) -> Result<Self, SdlError<'err>> {
        unsafe {
            SDL_CreateWindowWithProperties(
                props.get(),
            ).as_mut()
        }
            .map(|window| Self(window))
            .sdl_err(err_buf)
    }

    /// Create an OpenGL context for an OpenGL window, and make it current.
    ///
    /// ### Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ### See also
    /// - [`GlContext::destroy`]
    /// - [`GlContext::make_current`]
    pub fn gl_create_context<'err>(
        &mut self,
        err_buf: &'err mut ErrorBuffer,
    ) -> Result<GlContext<'a>, SdlError<'err>> {
        unsafe { SDL_GL_CreateContext(self.0).as_mut() }
            .map(|ctx| GlContext(ctx, PhantomData))
            .sdl_err(err_buf)
    }

    /// Create a child popup window of the specified parent window.
    ///
    /// The parent of a popup window can be either a regular, toplevel window,
    /// or another popup window.
    ///
    /// Popup windows cannot be minimized, maximized, made fullscreen, raised,
    /// flash, be made a modal window, be the parent of a toplevel window, or grab
    /// the mouse and/or keyboard. Attempts to do so will fail.
    ///
    /// Popup windows implicitly do not have a border/decorations and do not appear
    /// on the taskbar/dock or in lists of windows such as alt-tab menus.
    ///
    /// If a parent window is hidden or destroyed, any child popup windows will be
    /// recursively hidden or destroyed as well. Child popup windows not explicitly
    /// hidden will be restored when the parent is shown.
    ///
    /// ### Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ### See also
    /// - [`Window::create`]
    /// - [`Window::create_with_properties`]
    /// - [`Window::drop`]
    /// - [`Window::parent`]
    pub fn create_popup<'err>(
        &mut self,
        err_buf: &'err mut ErrorBuffer,
        offset_x: i32,
        offset_y: i32,
        w: u32,
        h: u32,
        flags: PopupConfig,
    ) -> Result<Self, SdlOrIntError<'err>> {
        unsafe { SDL_CreatePopupWindow(
            &raw mut *self.0,
            offset_x,
            offset_y,
            w.try_into()?,
            h.try_into()?,
            flags.0.bits(),
        ).as_mut() }
            .map(|popup| Self(popup))
            .sdl_erri(err_buf)
    }

    /// Get parent of a window.
    ///
    /// Returns [`None`] if the window has no parent.
    ///
    /// ### Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ### See also
    /// - [`Window::create_popup`]
    pub fn parent(&self) -> Option<WindowRef<'_>> {
        let parent = unsafe { SDL_GetWindowParent(<*const SDL_Window>::from(self.0).cast_mut()).as_mut() };
        parent.map(|parent| WindowRef(ManuallyDrop::new(Window(parent))))
    }

    /// Get parent of a window.
    ///
    /// Returns [`None`] if the window has no parent.
    ///
    /// ### Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ### See also
    /// - [`Window::create_popup`]
    pub fn parent_mut(&mut self) -> Option<WindowMut<'_>> {
        let parent = unsafe { SDL_GetWindowParent(self.0).as_mut() };
        parent.map(|parent| WindowMut(ManuallyDrop::new(Window(parent))))
    }
}

pub struct WindowRef<'a>(ManuallyDrop<Window<'a>>);

impl<'a> Deref for WindowRef<'a> {
    type Target = Window<'a>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct WindowMut<'a>(ManuallyDrop<Window<'a>>);

impl<'a> Deref for WindowMut<'a> {
    type Target = Window<'a>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for WindowMut<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct GlContext<'a>(SDL_GLContext, PhantomData<&'a Window<'a>>);

impl !Send for GlContext<'_> {}
impl !Sync for GlContext<'_> {}

impl Drop for GlContext<'_> {
    fn drop(&mut self) {
        panic!("GlContext dropped without being properly destroyed");
    }
}

impl<'a> GlContext<'a> {
    /// Delete an OpenGL context.
    /// Letting the context drop without calling this function will result in a panic.
    ///
    /// ### Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ### See also
    /// - [`Window::gl_create_context`]
    pub fn destroy<'err>(self, err_buf: &'err mut ErrorBuffer) -> Result<(), SdlError<'err>> {
        let inner = self.0;
        std::mem::forget(self);
        unsafe { SDL_GL_DestroyContext(inner) }
            .sdl_err(err_buf)
    }
}

pub struct GlLibrary(());

impl !Send for GlLibrary {}
impl !Sync for GlLibrary {}

impl Drop for GlLibrary {
    /// Unload the OpenGL library previously loaded by [`GlLibrary::load()`].
    ///
    /// ### Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ### See also
    /// - [`GlLibrary::load`]
    fn drop(&mut self) {
        unsafe { SDL_GL_UnloadLibrary() }
    }
}

impl GlLibrary {
    /// Dynamically load an OpenGL library.
    ///
    /// This should be done after initializing the video driver, but before
    /// creating any OpenGL windows. If no OpenGL library is loaded, the default
    /// library will be loaded upon creation of the first OpenGL window.
    ///
    /// If you do this, you need to retrieve all of the GL functions used in your
    /// program from the dynamic library using [`GlLibrary::proc_address()`].
    ///
    /// ### Parameters
    /// - `path`: the platform dependent OpenGL library name, or [`None`] to open the
    ///   default OpenGL library.
    ///
    /// ### See also
    /// - [`GlLibrary::proc_address`]
    /// - [`GlLibrary::drop`]
    pub fn load<'err>(&mut self, err_buf: &'err mut ErrorBuffer, path: Option<&CStr>) -> Result<Self, SdlError<'err>> {
        unsafe { SDL_GL_LoadLibrary(path.map_or_else(null, |path| path.as_ptr())) }
            .sdl_err(err_buf)
            .map(|()| Self(()))
    }

    /// Get an OpenGL function by name.
    ///
    /// If the GL library is loaded at runtime with [`GlLibrary::load()`], then all
    /// GL functions must be retrieved this way. Usually this is used to retrieve
    /// function pointers to OpenGL extensions.
    ///
    /// There are some quirks to looking up OpenGL functions that require some
    /// extra care from the application. If you code carefully, you can handle
    /// these quirks without any platform-specific code, though:
    ///
    /// - On Windows, function pointers are specific to the current GL context;
    ///   this means you need to have created a GL context and made it current
    ///   before calling [`GlLibrary::proc_address()`]. If you recreate your context or
    ///   create a second context, you should assume that any existing function
    ///   pointers aren't valid to use with it. This is (currently) a
    ///   Windows-specific limitation, and in practice lots of drivers don't suffer
    ///   this limitation, but it is still the way the wgl API is documented to
    ///   work and you should expect crashes if you don't respect it. Store a copy
    ///   of the function pointers that comes and goes with context lifespan.
    /// - On X11, function pointers returned by this function are valid for any
    ///   context, and can even be looked up before a context is created at all.
    ///   This means that, for at least some common OpenGL implementations, if you
    ///   look up a function that doesn't exist, you'll get a non-NULL result that
    ///   is _NOT_ safe to call. You must always make sure the function is actually
    ///   available for a given GL context before calling it, by checking for the
    ///   existence of the appropriate extension with [`SDL_GL_ExtensionSupported()`],
    ///   or verifying that the version of OpenGL you're using offers the function
    ///   as core functionality.
    /// - Some OpenGL drivers, on all platforms, *will* return NULL if a function
    ///   isn't supported, but you can't count on this behavior. Check for
    ///   extensions you use, and if you get a NULL anyway, act as if that
    ///   extension wasn't available. This is probably a bug in the driver, but you
    ///   can code defensively for this scenario anyhow.
    /// - Just because you're on Linux/Unix, don't assume you'll be using X11.
    ///   Next-gen display servers are waiting to replace it, and may or may not
    ///   make the same promises about function pointers.
    /// - OpenGL function pointers must be declared `APIENTRY` as in the example
    ///   code. This will ensure the proper calling convention is followed on
    ///   platforms where this matters (Win32) thereby avoiding stack corruption.
    ///
    /// ### Parameters
    /// - `proc`: the name of an OpenGL function.
    ///
    /// ### Return value
    /// Returns a pointer to the named OpenGL function. The returned pointer
    ///   should be cast to the appropriate function signature.
    ///
    /// ### See also
    /// - [`SDL_GL_ExtensionSupported`]
    /// - [`GlLibrary::load`]
    /// - [`GlLibrary::drop`]
    pub fn proc_address<F: From<unsafe extern "C" fn()>>(&mut self, proc: &CStr) -> Option<F> {
        unsafe { SDL_GL_GetProcAddress(proc.as_ptr()) }
        .map(|f| f.into())
    }
}
