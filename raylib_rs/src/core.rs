/*!********************************************************************************************
*
*   rcore - Window/display management, Graphic device/context management and input management
*
*   PLATFORMS SUPPORTED:
*       > PLATFORM_DESKTOP (GLFW backend):
*           - Windows (Win32, Win64)
*           - Linux (X11/Wayland desktop mode)
*           - macOS/OSX (x64, arm64)
*           - FreeBSD, OpenBSD, NetBSD, DragonFly (X11 desktop)
*       > PLATFORM_DESKTOP_SDL (SDL backend):
*           - Windows (Win32, Win64)
*           - Linux (X11/Wayland desktop mode)
*           - Others (not tested)
*       > PLATFORM_WEB:
*           - HTML5 (WebAssembly)
*       > PLATFORM_DRM:
*           - Raspberry Pi 0-5 (DRM/KMS)
*           - Linux DRM subsystem (KMS mode)
*       > PLATFORM_ANDROID:
*           - Android (ARM, ARM64)
*
*   CONFIGURATION:
*       #define SUPPORT_DEFAULT_FONT (default)
*           Default font is loaded on window initialization to be available for the user to render simple text.
*           NOTE: If enabled, uses external module functions to load default raylib font (module: text)
*
*       #define SUPPORT_CAMERA_SYSTEM
*           Camera module is included (rcamera.h) and multiple predefined cameras are available:
*               free, 1st/3rd person, orbital, custom
*
*       #define SUPPORT_GESTURES_SYSTEM
*           Gestures module is included (rgestures.h) to support gestures detection: tap, hold, swipe, drag
*
*       #define SUPPORT_MOUSE_GESTURES
*           Mouse gestures are directly mapped like touches and processed by gestures system.
*
*       #define SUPPORT_BUSY_WAIT_LOOP
*           Use busy wait loop for timing sync, if not defined, a high-resolution timer is setup and used
*
*       #define SUPPORT_PARTIALBUSY_WAIT_LOOP
*           Use a partial-busy wait loop, in this case frame sleeps for most of the time and runs a busy-wait-loop at the end
*
*       #define SUPPORT_SCREEN_CAPTURE
*           Allow automatic screen capture of current screen pressing F12, defined in KeyCallback()
*
*       #define SUPPORT_GIF_RECORDING
*           Allow automatic gif recording of current screen pressing CTRL+F12, defined in KeyCallback()
*
*       #define SUPPORT_COMPRESSION_API
*           Support CompressData() and DecompressData() functions, those functions use zlib implementation
*           provided by stb_image and stb_image_write libraries, so, those libraries must be enabled on textures module
*           for linkage
*
*       #define SUPPORT_AUTOMATION_EVENTS
*           Support automatic events recording and playing, useful for automated testing systems or AI based game playing
*
*   DEPENDENCIES:
*       raymath  - 3D math functionality (Vector2, Vector3, Matrix, Quaternion)
*       camera   - Multiple 3D camera modes (free, orbital, 1st person, 3rd person)
*       gestures - Gestures system for touch-ready devices (or simulated from mouse inputs)
*
*
*   LICENSE: zlib/libpng
*
*   Copyright (c) 2013-2023 Ramon Santamaria (@raysan5) and contributors
*
*   This software is provided "as-is", without any express or implied warranty. In no event
*   will the authors be held liable for any damages arising from the use of this software.
*
*   Permission is granted to anyone to use this software for any purpose, including commercial
*   applications, and to alter it and redistribute it freely, subject to the following restrictions:
*
*     1. The origin of this software must not be misrepresented; you must not claim that you
*     wrote the original software. If you use this software in a product, an acknowledgment
*     in the product documentation would be appreciated but is not required.
*
*     2. Altered source versions must be plainly marked as such, and must not be misrepresented
*     as being the original software.
*
*     3. This notice may not be removed or altered from any source distribution.
*
**********************************************************************************************/

use crate::{tracelog, utils::TraceLogLevel::*};

mod ffi {
    #[link(name = "raylib")]
    unsafe extern "C" {
        /// Initialize platform (graphics, inputs and more)
        pub fn InitPlatform() -> i32;

        /// Close platform
        pub fn ClosePlatform();

        /// Initialize rlgl: OpenGL extensions, default buffers/shaders/textures, OpenGL states
        pub fn rlglInit(width: i32, height: i32);


        /// Set the viewport area (transformation from normalized device coordinates to window coordinates)
        /// NOTE: We store current viewport dimensions
        pub fn rlViewport(x: i32, y: i32, width: i32, height: i32);

        /// Get window scale DPI factor for current monitor
        pub fn GetWindowScaleDPI() -> super::Vector2;
    }
}

/// Maximum file paths capacity
const MAX_FILEPATH_CAPACITY: usize = 8192;
/// Maximum length for filepaths (Linux PATH_MAX default value)
const MAX_FILEPATH_LENGTH: usize = 4096;

/// Maximum number of keyboard keys supported
const MAX_KEYBOARD_KEYS: usize = 512;
/// Maximum number of mouse buttons supported
const MAX_MOUSE_BUTTONS: usize = 8;
/// Maximum number of gamepads supported
const MAX_GAMEPADS: usize = 4;
/// Maximum number of axis supported (per gamepad)
const MAX_GAMEPAD_AXIS: usize = 8;
/// Maximum number of buttons supported (per gamepad)
const MAX_GAMEPAD_BUTTONS: usize = 32;
/// Maximum number of touch points supported
const MAX_TOUCH_POINTS: usize = 8;
/// Maximum number of keys in the key input queue
const MAX_KEY_PRESSED_QUEUE: usize = 16;
/// Maximum number of characters in the char input queue
const MAX_CHAR_PRESSED_QUEUE: usize = 16;

/// Max size allocated for decompression in MB
const MAX_DECOMPRESSION_SIZE: usize = 64;

/// Maximum number of automation events to record
const MAX_AUTOMATION_EVENTS: usize = 16384;

#[derive(Debug, Clone, Copy, Default)]
struct Matrix {} // todo

#[derive(Debug, Clone, Copy, Default)]
struct Vector2 { x: f32, y: f32 } // todo

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KeyboardKey {} // todo

#[derive(Debug, Clone, Copy, Default)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, Default)]
struct Size {
    width: u32,
    height: u32,
}

#[derive(Debug, Default)]
struct Window<'a> {
    /// Window text title const pointer
    title: &'a str,
    /// Configuration flags (bit based), keeps window state
    flags: u32,
    /// Check if window has been initialized successfully
    ready: bool,
    /// Check if fullscreen mode is enabled
    fullscreen: bool,
    /// Check if window set for closing
    should_close: bool,
    /// Check if window has been resized last frame
    resized_last_frame: bool,
    /// Wait for events before ending frame
    event_waiting: bool,
    /// Using FBO (RenderTexture) for rendering instead of default framebuffer
    using_fbo: bool,

    /// Window position (required on fullscreen toggle)
    position: Point,
    /// Window previous position (required on borderless windowed toggle)
    previous_position: Point,
    /// Display width and height (monitor, device-screen, LCD, ...)
    display: Size,
    /// Screen width and height (used render area)
    screen: Size,
    /// Screen previous width and height (required on borderless windowed toggle)
    previous_screen: Size,
    /// Current render width and height (depends on active fbo)
    current_fbo: Size,
    /// Framebuffer width and height (render area, including black bars if required)
    render: Size,
    /// Offset from render area (must be divided by 2)
    render_offset: Point,
    /// Screen minimum width and height (for resizable window)
    screen_min: Size,
    /// Screen maximum width and height (for resizable window)
    screen_max: Size,
    /// Matrix to scale screen (framebuffer rendering)
    screen_scale: Matrix,

    /// Store dropped files paths pointers (provided by GLFW)
    drop_filepaths: Option<&'a mut [Option<&'a mut str>]>,
}

#[derive(Debug, Default)]
struct Storage<'a> {
    /// Base path for data storage
    base_path: Option<&'a str>,
}

#[derive(Debug)]
struct Keyboard {
    /// Default exit key
    exit_key: Option<KeyboardKey>,
    /// Registers current frame key state
    current_key_state: [u8; MAX_KEYBOARD_KEYS],
    /// Registers previous frame key state
    previous_key_state: [u8; MAX_KEYBOARD_KEYS],

    // NOTE: Since key press logic involves comparing prev vs cur key state, we need to handle key repeats specially
    /// Registers key repeats for current frame.
    key_repeat_in_frame: [u8; MAX_KEYBOARD_KEYS],

    /// Input keys queue
    key_pressed_queue: Vec<i32>,

    /// Input characters queue (unicode)
    char_pressed_queue: Vec<char>,
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            exit_key: None,
            current_key_state:   [0; MAX_KEYBOARD_KEYS],
            previous_key_state:  [0; MAX_KEYBOARD_KEYS],
            key_repeat_in_frame: [0; MAX_KEYBOARD_KEYS],
            key_pressed_queue:  Vec::with_capacity(MAX_KEY_PRESSED_QUEUE),
            char_pressed_queue: Vec::with_capacity(MAX_CHAR_PRESSED_QUEUE),
        }
    }
}

#[derive(Debug, Default)]
struct Mouse {
    /// Mouse offset
    offset: Vector2,
    /// Mouse scaling
    scale: Vector2,
    /// Mouse position on screen
    current_position: Vector2,
    /// Previous mouse position
    previous_position: Vector2,

    /// Tracks current mouse cursor
    cursor: i32,
    /// Track if cursor is hidden
    cursor_hidden: bool,
    /// Tracks if cursor is inside client area
    cursor_on_screen: bool,

    /// Registers current mouse button state
    current_button_state: [u8; MAX_MOUSE_BUTTONS],
    /// Registers previous mouse button state
    previous_button_state: [u8; MAX_MOUSE_BUTTONS],
    /// Registers current mouse wheel variation
    current_wheel_move: Vector2,
    /// Registers previous mouse wheel variation
    previous_wheel_move: Vector2,
}

#[derive(Debug, Default)]
struct Touch {
    /// Number of touch points active
    point_count: i32,
    /// Point identifiers
    point_id: [i32; MAX_TOUCH_POINTS],
    /// Touch position on screen
    position: [Vector2; MAX_TOUCH_POINTS],
    /// Registers current touch state
    current_touch_state: [u8; MAX_TOUCH_POINTS],
    /// Registers previous touch state
    previous_touch_state: [u8; MAX_TOUCH_POINTS],
}

#[derive(Debug, Default)]
struct Gamepad<'a> {
    /// Register last gamepad button pressed
    last_button_pressed: i32,
    /// Register number of available gamepad axis
    axis_count: [i32; MAX_GAMEPADS],
    /// Flag to know if gamepad is ready
    ready: [bool; MAX_GAMEPADS],
    /// Gamepad name holder
    name: [&'a str; MAX_GAMEPADS],
    /// Current gamepad buttons state
    current_button_state: [[u8; MAX_GAMEPAD_BUTTONS]; MAX_GAMEPADS],
    /// Previous gamepad buttons state
    previous_button_state: [[u8; MAX_GAMEPAD_BUTTONS]; MAX_GAMEPADS],
    /// Gamepad axis state
    axis_state: [[f32; MAX_GAMEPAD_AXIS]; MAX_GAMEPADS],
}

#[derive(Debug, Default)]
struct Input<'a> {
    keyboard: Keyboard,
    mouse: Mouse,
    touch: Touch,
    gamepad: Gamepad<'a>,
}

#[derive(Debug, Default)]
struct Time {
    /// Current time measure
    current: f64,
    /// Previous time measure
    previous: f64,
    /// Time measure for frame update
    update: f64,
    /// Time measure for frame draw
    draw: f64,
    /// Time measure for one frame
    frame: f64,
    /// Desired time for one frame, if 0 not applied
    target: f64,
    /// Base time measure for hi-res timer (PLATFORM_ANDROID, PLATFORM_DRM)
    base: u64,
    /// Frame counter
    frame_counter: u32,
}

// Core global state context data
#[derive(Debug, Default)]
struct CoreData<'a> {
    window: Window<'a>,
    storage: Storage<'a>,
    input: Input<'a>,
    time: Time,
}

pub struct RaylibHandle<'a>(CoreData<'a>);

impl Drop for RaylibHandle<'_> {
    fn drop(&mut self) {
        unsafe { ffi::ClosePlatform() };
    }
}

pub fn init_window<'a>(width: u32, height: u32, title: &'a str) -> RaylibHandle<'a> {
    let mut core = CoreData::default();

    // todo!("initialize core");

    // Initialize platform
    //--------------------------------------------------------------
    _ = unsafe { ffi::InitPlatform() };
    //--------------------------------------------------------------

    // Initialize rlgl default data (buffers and shaders)
    // NOTE: core.window.currentFbo.width and core.window.currentFbo.height not used, just stored as globals in rlgl
    unsafe { ffi::rlglInit(width.try_into().unwrap(), height.try_into().unwrap()) }; // todo: make this safer

    // Setup default viewport
    setup_viewport(&mut core, core.window.current_fbo.width, core.window.current_fbo.height);

//     #if defined(SUPPORT_MODULE_RTEXT) && defined(SUPPORT_DEFAULT_FONT)
//     // Load default font
//     // WARNING: External function: Module required: rtext
//     LoadFontDefault();
//     #if defined(SUPPORT_MODULE_RSHAPES)
//     // Set font white rectangle for shapes drawing, so shapes and text can be batched together
//     // WARNING: rshapes module is required, if not available, default internal white rectangle is used
//     Rectangle rec = GetFontDefault().recs[95];
//     if (core.window.flags & FLAG_MSAA_4X_HINT)
//     {
//         // NOTE: We try to maxime rec padding to avoid pixel bleeding on MSAA filtering
//         SetShapesTexture(GetFontDefault().texture, (Rectangle){ rec.x + 2, rec.y + 2, 1, 1 });
//     }
//     else
//     {
//         // NOTE: We set up a 1px padding on char rectangle to avoid pixel bleeding
//         SetShapesTexture(GetFontDefault().texture, (Rectangle){ rec.x + 1, rec.y + 1, rec.width - 2, rec.height - 2 });
//     }
//     #endif
// #else
//     #if defined(SUPPORT_MODULE_RSHAPES)
//     // Set default texture and rectangle to be used for shapes drawing
//     // NOTE: rlgl default texture is a 1x1 pixel UNCOMPRESSED_R8G8B8A8
//     Texture2D texture = { rlGetTextureIdDefault(), 1, 1, 1, PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 };
//     SetShapesTexture(texture, (Rectangle){ 0.0f, 0.0f, 1.0f, 1.0f });    // WARNING: Module required: rshapes
//     #endif
// #endif
// #if defined(SUPPORT_MODULE_RTEXT) && defined(SUPPORT_DEFAULT_FONT)
//     if ((core.window.flags & FLAG_WINDOW_HIGHDPI) > 0)
//     {
//         // Set default font texture filter for HighDPI (blurry)
//         // RL_TEXTURE_FILTER_LINEAR - tex filter: BILINEAR, no mipmaps
//         rlTextureParameters(GetFontDefault().texture.id, RL_TEXTURE_MIN_FILTER, RL_TEXTURE_FILTER_LINEAR);
//         rlTextureParameters(GetFontDefault().texture.id, RL_TEXTURE_MAG_FILTER, RL_TEXTURE_FILTER_LINEAR);
//     }
// #endif

//     core.Time.frameCounter = 0;
//     core.window.shouldClose = false;

//     // Initialize random seed
//     SetRandomSeed((unsigned int)time(NULL));

    RaylibHandle(core)
}

// Set viewport for a provided width and height
fn setup_viewport<'a>(core: &mut CoreData<'a>, width: u32, height: u32)
{
    core.window.render.width = width;
    core.window.render.height = height;

    // Set viewport width and height
    // NOTE: We consider render size (scaled) and offset in case black bars are required and
    // render area does not match full display area (this situation is only applicable on fullscreen mode)
    if cfg!(any(target_os = "macos", target_os = "ios")) {
        let scale = unsafe { ffi::GetWindowScaleDPI() };
        rlViewport(core.window.render_offset.x / 2*scale.x, core.window.render_offset.y/2*scale.y, (core.window.render.width)*scale.x, (core.window.render.height)*scale.y);
    } else {
        rlViewport(core.window.render_offset.x / 2, core.window.render_offset.y/2, core.window.render.width, core.window.render.height);
    }

    rlMatrixMode(RL_PROJECTION);        // Switch to projection matrix
    rlLoadIdentity();                   // Reset current matrix (projection)

    // Set orthographic projection to current framebuffer size
    // NOTE: Configured top-left corner as (0, 0)
    rlOrtho(0, core.window.render.width, core.window.render.height, 0, 0.0, 1.0);

    rlMatrixMode(RL_MODELVIEW);         // Switch back to modelview matrix
    rlLoadIdentity();                   // Reset current matrix (modelview)
}

// Compute framebuffer size relative to screen size and display size
// NOTE: Global variables core.window.render.width/core.window.render.height and core.window.render_offset.x/core.window.render_offset.y can be modified
fn setup_frame_buffer(width: u32, height: u32)
{
    // Calculate core.window.render.width and core.window.render.height, we have the display size (input params) and the desired screen size (global var)
    if ((core.window.screen.width > core.window.display.width) || (core.window.screen.height > core.window.display.height))
    {
        tracelog!(LogInfo, "DISPLAY: Downscaling required: Screen size (%ix%i) is bigger than display size (%ix%i)", core.window.screen.width, core.window.screen.height, core.window.display.width, core.window.display.height);

        // Downscaling to fit display with border-bars
        float width_ratio = (float)core.window.display.width/(float)core.window.screen.width;
        float height_ratio = (float)core.window.display.height/(float)core.window.screen.height;

        if (width_ratio <= height_ratio)
        {
            core.window.render.width = core.window.display.width;
            core.window.render.height = (core.window.screen.height as f32 * width_ratio).round() as u32;
            core.window.render_offset.x = 0;
            core.window.render_offset.y = core.window.display.height - core.window.render.height;
        }
        else
        {
            core.window.render.width = (core.window.screen.width as f32 * height_ratio).round() as u32;
            core.window.render.height = core.window.display.height;
            core.window.render_offset.x = core.window.display.width - core.window.render.width;
            core.window.render_offset.y = 0;
        }

        // Screen scaling required
        let scale_ratio = core.window.render.width as f32 / core.window.screen.width as f32;
        core.window.screen_scale = MatrixScale(scale_ratio, scale_ratio, 1.0f);

        // NOTE: We render to full display resolution!
        // We just need to calculate above parameters for downscale matrix and offsets
        core.window.render.width = core.window.display.width;
        core.window.render.height = core.window.display.height;

        tracelog!(LogInfo, "DISPLAY: Downscale matrix generated, content will be rendered at ({}x{})", core.window.render.width, core.window.render.height);
    }
    else if ((core.window.screen.width < core.window.display.width) || (core.window.screen.height < core.window.display.height))
    {
        // Required screen size is smaller than display size
        tracelog!(LogInfo, "DISPLAY: Upscaling required: Screen size ({}x{}) smaller than display size ({}x{})", core.window.screen.width, core.window.screen.height, core.window.display.width, core.window.display.height);

        if ((core.window.screen.width == 0) || (core.window.screen.height == 0))
        {
            core.window.screen.width = core.window.display.width;
            core.window.screen.height = core.window.display.height;
        }

        // Upscaling to fit display with border-bars
        float displayRatio = (float)core.window.display.width/(float)core.window.display.height;
        float screenRatio = (float)core.window.screen.width/(float)core.window.screen.height;

        if (displayRatio <= screenRatio)
        {
            core.window.render.width = core.window.screen.width;
            core.window.render.height = (int)round((float)core.window.screen.width/displayRatio);
            core.window.render_offset.x = 0;
            core.window.render_offset.y = (core.window.render.height - core.window.screen.height);
        }
        else
        {
            core.window.render.width = (int)round((float)core.window.screen.height*displayRatio);
            core.window.render.height = core.window.screen.height;
            core.window.render_offset.x = (core.window.render.width - core.window.screen.width);
            core.window.render_offset.y = 0;
        }
    }
    else
    {
        core.window.render.width = core.window.screen.width;
        core.window.render.height = core.window.screen.height;
        core.window.render_offset.x = 0;
        core.window.render_offset.y = 0;
    }
}
