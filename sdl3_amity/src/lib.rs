#![feature(negative_impls)]
#![cfg_attr(feature = "c_variadic", feature(c_variadic))]

use std::{ffi::CStr, marker::PhantomData, mem::ManuallyDrop, ops::{Deref, DerefMut}};
use sdl3_sys::{self, init::{SDL_Init, SDL_InitFlags, SDL_Quit}, video::{SDL_CreatePopupWindow, SDL_CreateWindow, SDL_DestroyWindow, SDL_DisplayID, SDL_GLContext, SDL_GL_CreateContext, SDL_GL_DestroyContext, SDL_GetWindowParent, SDL_Window, SDL_WindowFlags}};
use bitflags::bitflags;

pub mod error;
pub use error::*;

pub struct SdlDisplayID(SDL_DisplayID);

impl From<SDL_DisplayID> for SdlDisplayID {
    fn from(value: SDL_DisplayID) -> Self {
        Self(value)
    }
}

impl SdlDisplayID {
    pub const fn get(&self) -> SDL_DisplayID {
        self.0
    }
}

bitflags! {
    pub struct InitFlags: SDL_InitFlags {
        /// [`InitFlags::Audio`] implies [`InitFlags::Events`]
        const Audio    = sdl3_sys::init::SDL_INIT_AUDIO;
        /// [`InitFlags::Video`] implies [`InitFlags::Events`],
        /// should be initialized on the main thread
        const Video    = sdl3_sys::init::SDL_INIT_VIDEO;
        /// [`InitFlags::Joystick`] implies [`InitFlags::Events`],
        /// should be initialized on the same thread as [`InitFlags::Video`] on Windows if you don't set [`SDL_HINT_JOYSTICK_THREAD`]
        const Joystick = sdl3_sys::init::SDL_INIT_JOYSTICK;
        const Haptic   = sdl3_sys::init::SDL_INIT_HAPTIC;
        /// [`InitFlags::Gamepad`] implies [`InitFlags::Joystick`]
        const Gamepad  = sdl3_sys::init::SDL_INIT_GAMEPAD;
        const Events   = sdl3_sys::init::SDL_INIT_EVENTS;
        /// [`InitFlags::Sensor`] implies [`InitFlags::Events`]
        const Sensor   = sdl3_sys::init::SDL_INIT_SENSOR;
        /// [`InitFlags::Camera`] implies [`InitFlags::Events`]
        const Camera   = sdl3_sys::init::SDL_INIT_CAMERA;
    }

    pub struct WindowFlags: SDL_WindowFlags {
        /// fullscreen window at desktop resolution
        const Fullscreen        = sdl3_sys::video::SDL_WINDOW_FULLSCREEN;
        /// window usable with an OpenGL context
        const Opengl            = sdl3_sys::video::SDL_WINDOW_OPENGL;
        /// window partially or completely obscured by another window
        const Occluded          = sdl3_sys::video::SDL_WINDOW_OCCLUDED;
        /// window is not visible \
        /// it is neither mapped onto the desktop nor shown in the taskbar/dock/window list; [`Window::show()`] is required for it to become visible
        const Hidden            = sdl3_sys::video::SDL_WINDOW_HIDDEN;
        /// no window decoration
        const Borderless        = sdl3_sys::video::SDL_WINDOW_BORDERLESS;
        /// window can be resized
        const Resizable         = sdl3_sys::video::SDL_WINDOW_RESIZABLE;
        /// window is minimized
        const Minimized         = sdl3_sys::video::SDL_WINDOW_MINIMIZED;
        /// window is maximized
        const Maximized         = sdl3_sys::video::SDL_WINDOW_MAXIMIZED;
        /// window has grabbed mouse focus
        const MouseGrabbed      = sdl3_sys::video::SDL_WINDOW_MOUSE_GRABBED;
        /// window has input focus
        const InputFocus        = sdl3_sys::video::SDL_WINDOW_INPUT_FOCUS;
        /// window has mouse focus
        const MouseFocus        = sdl3_sys::video::SDL_WINDOW_MOUSE_FOCUS;
        /// window not created by SDL
        const External          = sdl3_sys::video::SDL_WINDOW_EXTERNAL;
        /// window is modal
        const Modal             = sdl3_sys::video::SDL_WINDOW_MODAL;
        /// window uses high pixel density back buffer if possible
        const HighPixelDensity  = sdl3_sys::video::SDL_WINDOW_HIGH_PIXEL_DENSITY;
        /// window has mouse captured (unrelated to `MouseGrabbed`)
        const MouseCapture      = sdl3_sys::video::SDL_WINDOW_MOUSE_CAPTURE;
        /// window has relative mode enabled
        const MouseRelativeMode = sdl3_sys::video::SDL_WINDOW_MOUSE_RELATIVE_MODE;
        /// window should always be above others
        const AlwaysOnTop       = sdl3_sys::video::SDL_WINDOW_ALWAYS_ON_TOP;
        /// window should be treated as a utility window, not showing in the task bar and window list
        const Utility           = sdl3_sys::video::SDL_WINDOW_UTILITY;
        /// window should be treated as a tooltip and does not get mouse or keyboard focus, requires a parent window
        const Tooltip           = sdl3_sys::video::SDL_WINDOW_TOOLTIP;
        /// window should be treated as a popup menu, requires a parent window
        const PopupMenu         = sdl3_sys::video::SDL_WINDOW_POPUP_MENU;
        /// window has grabbed keyboard input
        const KeyboardGrabbed   = sdl3_sys::video::SDL_WINDOW_KEYBOARD_GRABBED;
        /// window usable with a Vulkan instance
        const Vulkan            = sdl3_sys::video::SDL_WINDOW_VULKAN;
        /// window usable with a Metal instance
        const Metal             = sdl3_sys::video::SDL_WINDOW_METAL;
        /// window with transparent buffer
        const Transparent       = sdl3_sys::video::SDL_WINDOW_TRANSPARENT;
        /// window should not be focusable
        const NotFocusable      = sdl3_sys::video::SDL_WINDOW_NOT_FOCUSABLE;
    }
}

pub struct SdlHandle(());

impl Drop for SdlHandle {
    fn drop(&mut self) {
        unsafe { SDL_Quit(); }
    }
}

pub fn init(err_buf: &mut ErrorBuffer, flags: InitFlags) -> Result<SdlHandle, SdlError<'_>> {
    if unsafe { SDL_Init(flags.bits()) } {
        Ok(SdlHandle(()))
    } else {
        Err(err_buf.get())
    }
}

impl SdlHandle {
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
    /// If the window is created with any of the [`SDL_WINDOW_OPENGL`] or
    /// [`SDL_WINDOW_VULKAN`] flags, then the corresponding LoadLibrary function
    /// ([`SDL_GL_LoadLibrary`] or [`SDL_Vulkan_LoadLibrary`]) is called and the
    /// corresponding UnloadLibrary function is called by [`Window::drop()`].
    ///
    /// If [`SDL_WINDOW_VULKAN`] is specified and there isn't a working Vulkan driver,
    /// [`SdlHandle::create_window()`] will fail, because [`SDL_Vulkan_LoadLibrary()`] will fail.
    ///
    /// If [`SDL_WINDOW_METAL`] is specified on an OS that does not support Metal,
    /// [`SdlHandle::create_window()`] will fail.
    ///
    /// If you intend to use this window with an [`SDL_Renderer`], you should use
    /// [`SDL_CreateWindowAndRenderer()`] instead of this function, to avoid window
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
    /// - [`SdlHandle::create_window_and_renderer`]
    /// - [`Window::create_popup_window`]
    /// - [`SdlHandle::create_window_with_properties`]
    /// - [`Window::drop`]
    pub fn create_window<'a, 'b>(
        &'a self,
        err_buf: &'b mut ErrorBuffer,
        title: &CStr,
        w: u32,
        h: u32,
        flags: WindowFlags,
    ) -> Result<Window<'a>, SdlOrIntError<'b>> {
        let window = unsafe {
            SDL_CreateWindow(
                title.as_ptr(),
                w.try_into()?,
                h.try_into()?,
                flags.bits(),
            ).as_mut()
        };
        if let Some(window) = window {
            Ok(Window(window))
        } else {
            Err(err_buf.get().into())
        }
    }

    pub fn create_window_with_properties() {

    }
}

pub struct Window<'a>(&'a mut SDL_Window);

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
    /// ### Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ### See also
    /// - [`Window::create_popup_window`]
    /// - [`SdlHandle::create_window`]
    /// - [`SdlHandle::create_window_with_properties`]
    fn drop(&mut self) {
        unsafe { SDL_DestroyWindow(self.0); }
    }
}

impl<'a> Window<'a> {
    /// Create an OpenGL context for an OpenGL window, and make it current.
    ///
    /// ### Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ### See also
    /// - [`GlContext::destroy`]
    /// - [`GlContext::make_current`]
    pub fn gl_create_context<'b>(&mut self, err_buf: &'b mut ErrorBuffer) -> Result<GlContext<'a>, SdlError<'b>> {
        let ctx = unsafe { SDL_GL_CreateContext(self.0).as_mut() };
        if let Some(ctx) = ctx {
            Ok(GlContext(ctx, PhantomData))
        } else {
            Err(err_buf.get())
        }
    }

    /// Create a child popup window of the specified parent window.
    ///
    /// The flags parameter **must** contain at least one of the following:
    ///
    /// - [`WindowFlags::Tooltip`]\: The popup window is a tooltip and will not pass any
    ///   input events.
    /// - [`WindowFlags::PopupMenu`]\: The popup window is a popup menu. The topmost
    ///   popup menu will implicitly gain the keyboard focus.
    ///
    /// The following flags are not relevant to popup window creation and will be
    /// ignored:
    ///
    /// - [`WindowFlags::Minimized`]
    /// - [`WindowFlags::Maximized`]
    /// - [`WindowFlags::Fullscreen`]
    /// - [`WindowFlags::Borderless`]
    ///
    /// The following flags are incompatible with popup window creation and will
    /// cause it to fail:
    ///
    /// - [`WindowFlags::Utility`]
    /// - [`WindowFlags::Modal`]
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
    /// - [`SdlHandle::create_window`]
    /// - [`SdlHandle::create_window_with_properties`]
    /// - [`Window::drop`]
    /// - [`Window::parent`]
    pub fn create_popup_window<'b>(
        &mut self,
        err_buf: &'b mut ErrorBuffer,
        offset_x: i32,
        offset_y: i32,
        w: u32,
        h: u32,
        flags: WindowFlags,
    ) -> Result<Window<'a>, SdlOrIntError<'b>> {
        assert!(flags.intersects(WindowFlags::Tooltip.union(WindowFlags::PopupMenu)), "popup window flags must contain `Tooltip` and/or `PopupMenu`");
        assert!(!flags.intersects(WindowFlags::Utility.union(WindowFlags::Modal)), "popup window flags cannot contain `Utility` or `Modal`");
        #[cfg(debug_assertions)] {
            if flags.intersects(
                WindowFlags::Minimized
                    .union(WindowFlags::Maximized)
                    .union(WindowFlags::Fullscreen)
                    .union(WindowFlags::Borderless)
            ) {
                println!("warning: popup window is unaffected by the flags ");
            }
        }
        let popup = unsafe { SDL_CreatePopupWindow(
            &raw mut *self.0,
            offset_x,
            offset_y,
            w.try_into()?,
            h.try_into()?,
            flags.bits(),
        ).as_mut() };
        if let Some(popup) = popup {
            Ok(Window(popup))
        } else {
            Err(err_buf.get().into())
        }
    }

    /// Get parent of a window.
    ///
    /// Returns [`None`] if the window has no parent.
    ///
    /// ### Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ### See also
    /// - [`Window::create_popup_window`]
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
    /// - [`Window::create_popup_window`]
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
    pub fn destroy(self, err_buf: &mut ErrorBuffer) -> Result<(), SdlError<'_>> {
        let success = unsafe { SDL_GL_DestroyContext(self.0) };
        std::mem::forget(self);
        if success { Ok(()) } else { Err(err_buf.get()) }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use super::*;

    #[test]
    fn test0() {
        let mut err_buf = sdl_thread().unwrap();
        let err1 = err_buf.get();
        println!("error: {err1}");
        let err2 = err_buf.get();
        println!("error: {err2}");
        // err1;
    }

    #[test]
    fn test1() {
        let mut err_buf = sdl_thread().unwrap();
        let sdl = init(&mut err_buf, InitFlags::empty()).unwrap();
        let window = sdl.create_window(&mut err_buf, c"test", 1280, 720, WindowFlags::empty()).unwrap();
        std::thread::sleep(Duration::from_secs(2));
    }
}
