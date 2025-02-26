use std::{ffi::CStr, marker::PhantomData, mem::ManuallyDrop, ops::{Deref, DerefMut}};
use sdl3_sys::video::*;
use bitflags::bitflags;
use crate::{error::*, init::Sdl, properties::SdlPropertiesID};

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
    /// Get the size of a window's client area.
    ///
    /// The window pixel size may differ from its window coordinate size if the
    /// window is on a high pixel density display. Use [`Window::size_in_pixels()`]
    /// or [`Renderer::get_output_size()`] to get the real client area size in pixels.
    ///
    /// ### Thread safety
    /// This function should only be called on the main thread.
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
    /// ### Thread safety
    /// This function should only be called on the main thread.
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
    /// ### Thread safety
    /// This function should only be called on the main thread.
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
    /// ### Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.2.0.
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
    /// ### Thread safety
    /// This function should only be called on the main thread.
    ///
    /// ### See also
    /// - [`Window::hide`]
    /// - [`Window::raise`]
    pub fn show<'err>(&mut self, err_buf: &'err mut ErrorBuffer) -> Result<(), SdlError<'err>> {
        unsafe { SDL_ShowWindow(self.raw()) }.sdl_err(err_buf)
    }

    /// Hide a window.
    ///
    /// ### Thread safety
    /// This function should only be called on the main thread.
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
    ///
    /// ### Thread safety
    /// This function should only be called on the main thread.
    pub fn raise<'err>(&mut self, err_buf: &'err mut ErrorBuffer) -> Result<(), SdlError<'err>> {
        unsafe { SDL_RaiseWindow(self.raw()) }.sdl_err(err_buf)
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
