#![allow(dead_code)]
use raylib::prelude::*;
use std::{ffi::CStr, num::{NonZeroUsize, TryFromIntError}, ops::{Deref, DerefMut}, os::raw::{c_void, c_uchar, c_uint}, ptr::null_mut};

pub mod tracking {
    use std::{os::raw::{c_int, c_uint}, marker::PhantomData};
    use raylib::prelude::*;

    macro_rules! define_gl_id_types {
        ($(
            {
                id:
                $(#[$id_meta:meta])*
                $TypeID:ident,

                sig:
                $(#[$sig_meta:meta])*
                $TypeHandle:ident,
            }
        ),+ $(,)?) => {$(
            $(#[$id_meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct $TypeID<'a>(c_uint, PhantomData<&'a $TypeHandle>);

            impl $TypeID<'_> {
                /// Designate the type of ID this is
                ///
                /// Safety: Programmer must ensure ID is of the stated type and survives the correct lifetime
                #[inline]
                pub const unsafe fn new(id: c_uint) -> Self {
                    Self(id, PhantomData)
                }

                /// Get the ID as an unsigned int for communicating with OpenGL
                ///
                /// Safety: Programmer must ensure ID is of the stated type and is still alive
                #[inline]
                pub const unsafe fn get(&self) -> c_uint {
                    self.0
                }
            }

            $(#[$sig_meta])*
            ///
            /// **Do not** drop without properly unloading the related resource
            #[repr(transparent)]
            pub struct $TypeHandle(c_uint);

            #[cfg(any(debug_assertions, test))]
            impl Drop for $TypeHandle {
                fn drop(&mut self) {
                    panic!(concat!(stringify!($TypeHandle), " dropped without being unloaded"));
                }
            }

            impl $TypeHandle {
                /// Get the ID of the signifier
                ///
                /// Safety: A signifier can only be created from the actual creation of the referred object,
                /// and should only be destroyed by the actual destruction of the referred object.
                #[inline]
                pub const fn id<'a>(&'a self) -> $TypeID<'a> {
                    unsafe { $TypeID::new(self.0) }
                }

                #[inline]
                pub(super) fn fuse(id: c_uint) -> Self {
                    Self(id)
                }

                #[inline]
                pub(super) fn defuse(self) -> c_uint {
                    let id = self.0;
                    std::mem::forget(self);
                    id
                }
            }
        )+};
    }

    define_gl_id_types!{
        {
            id:
                /// The ID of a Vertex Array Object
                VaoID,
            sig:
                /// A signifier of the ongoing validity of a Vertex Array Object
                VaoHandle,
        },
        {
            id:
                /// The ID of a Vertex Buffer Object
                VboID,
            sig:
                /// A signifier of the ongoing validity of a Vertex Buffer Object
                VboHandle,
        },
        {
            id:
                /// The ID of a Vertex Buffer Element
                VboElementID,
            sig:
                /// A signifier of the ongoing validity of a Vertex Buffer Element
                VboElementHandle,
        },
        {
            id:
                /// The ID of a Frame Buffer Object
                FboID,
            sig:
                /// A signifier of the ongoing validity of a Frame Buffer Object
                FboHandle,
        },
        {
            id:
                /// The ID of a shader
                ShaderID,
            sig:
                /// A signifier of the ongoing validity of a shader
                ShaderHandle,
        },
        {
            id:
                /// The ID of a texture
                TextureID,
            sig:
                /// A signifier of the ongoing validity of a texture
                TextureHandle,
        },
        {
            id:
                /// The ID of a Shader Storage Buffer Object
                SsboID,
            sig:
                /// A signifier of the ongoing validity of a Shader Storage Buffer Object
                SsboHandle,
        },
    }

    /// The ID of a shader Program
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct ShaderPrgmID<'a, 'b>(c_uint, PhantomData<&'a ShaderPrgmHandle<'b>>);

    impl ShaderPrgmID<'_, '_> {
        /// Designate the type of ID this is
        ///
        /// Safety: Programmer must ensure ID is of the stated type and survives the correct lifetime
        #[inline]
        pub const unsafe fn new(id: c_uint) -> Self {
            Self(id, PhantomData)
        }

        /// Get the ID as an unsigned int for communicating with OpenGL
        ///
        /// Safety: Programmer must ensure ID is of the stated type and is still alive
        #[inline]
        pub const unsafe fn get(&self) -> c_uint {
            self.0
        }
    }

    /// A signifier of the ongoing validity of a shader Program
    ///
    /// **Do not** drop without properly unloading the related resource
    #[repr(transparent)]
    pub struct ShaderPrgmHandle<'a>(c_uint, PhantomData<&'a ShaderHandle>);

    #[cfg(any(debug_assertions, test))]
    impl Drop for ShaderPrgmHandle<'_> {
        fn drop(&mut self) {
            panic!("ShaderPrgmHandle dropped without being unloaded");
        }
    }

    impl<'a> ShaderPrgmHandle<'a> {
        /// Get the ID of the signifier
        ///
        /// Safety: A signifier can only be created from the actual creation of the referred object,
        /// and should only be destroyed by the actual destruction of the referred object.
        #[inline]
        pub const fn id(&self) -> ShaderPrgmID<'_, 'a> {
            unsafe { ShaderPrgmID::new(self.0) }
        }

        #[inline]
        pub(super) fn fuse<const N: usize>(_: [ShaderID<'a>; N], id: c_uint) -> Self {
            Self(id, PhantomData)
        }

        #[inline]
        pub(super) fn defuse(self) -> c_uint {
            let id = self.0;
            std::mem::forget(self);
            id
        }
    }

    /// The location of a shader uniform
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ShaderUniformLoc<'a>(c_int, PhantomData<&'a ShaderHandle>);

    impl<'a> ShaderUniformLoc<'a> {
        #[inline]
        pub(super) const fn make(_: ShaderID<'a>, loc: c_int) -> Option<Self> {
            if loc != -1 { Some(Self(loc, PhantomData)) } else { None }
        }

        /// Get the ID as an int for communicating with OpenGL
        ///
        /// Safety: Programmer must ensure ID is of the stated type
        #[inline]
        pub const unsafe fn get(&self) -> c_int {
            self.0
        }
    }

    /// The location of a shader uniform
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ShaderAttribLoc<'a>(c_int, PhantomData<&'a ShaderHandle>);

    impl<'a> ShaderAttribLoc<'a> {
        #[inline]
        pub(super) const fn make(_: ShaderID<'a>, loc: c_int) -> Option<Self> {
            if loc != -1 { Some(unsafe { std::mem::transmute(loc) }) } else { None }
        }

        /// Get the ID as an int for communicating with OpenGL
        ///
        /// Safety: Programmer must ensure ID is of the stated type
        #[inline]
        pub const unsafe fn get(&self) -> c_int {
            self.0
        }
    }

    pub trait RlglID {
        type TypeID<'a> where Self: 'a;
        /// Get a safe wrapper for the ID of this type to communicate with RLGL
        fn id(&self) -> Self::TypeID<'_>;
    }

    impl RlglID for Texture2D {
        type TypeID<'a> = TextureID<'a> where Self: 'a;
        fn id(&self) -> Self::TypeID<'_> {
            unsafe { TextureID::new(self.id) }
        }
    }

    // 2025-02-19: As of writing, `RL_SHADER_LOC_MAP_BRDF` is the highest index
    const NUM_SHADER_LOCS: usize = ffi::rlShaderLocationIndex::RL_SHADER_LOC_MAP_BRDF as usize + 1;

    pub trait RlglShaderID {
        /// Get a safe wrapper for the ID of this type to communicate with RLGL
        fn id(&self) -> ShaderPrgmID<'_, '_>;
        fn locs(&self) -> &[ShaderUniformLoc];
        fn locs_mut(&self) -> &mut [ShaderUniformLoc<'_>];
    }

    impl RlglShaderID for Shader {
        fn id(&self) -> ShaderPrgmID<'_, '_> {
            unsafe { ShaderPrgmID::new(self.id) }
        }

        fn locs(&self) -> &[ShaderUniformLoc<'_>] {
            unsafe { std::slice::from_raw_parts(self.locs.cast(), NUM_SHADER_LOCS) }
        }

        fn locs_mut(&self) -> &mut [ShaderUniformLoc<'_>] {
            unsafe { std::slice::from_raw_parts_mut(self.locs.cast(), NUM_SHADER_LOCS) }
        }
    }
}
pub use tracking::*;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MatrixMode {
    Modelview = ffi::RL_MODELVIEW as i32,
    Projection = ffi::RL_PROJECTION as i32,
    Texture = ffi::RL_TEXTURE as i32,
}

/// Face culling mode
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CullMode {
    Front = ffi::rlCullMode::RL_CULL_FACE_FRONT as i32,
    Back  = ffi::rlCullMode::RL_CULL_FACE_BACK as i32,
}

pub struct RlglMatrixMode<'a, T: ?Sized>(&'a mut T);

impl<'a, T: ?Sized> Deref for RlglMatrixMode<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: ?Sized> DerefMut for RlglMatrixMode<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T: ?Sized> Drop for RlglMatrixMode<'a, T> {
    /// Pop latest inserted matrix from stack
    fn drop(&mut self) {
        unsafe { ffi::rlPopMatrix(); }
    }
}

pub trait Rlgl: RaylibDraw {
    // Matrix state

    /// Choose the current matrix to be transformed
    #[inline]
    fn rl_matrix_mode(&mut self, mode: MatrixMode) {
        unsafe { ffi::rlMatrixMode(mode as i32); }
    }

    /// Push the current matrix to stack
    #[inline]
    fn rl_push_matrix(&mut self) -> RlglMatrixMode<'_, Self> {
        unsafe { ffi::rlPushMatrix(); }
        RlglMatrixMode(self)
    }

    /// Reset current matrix to identity matrix
    #[inline]
    fn rl_load_identity(&mut self) {
        unsafe { ffi::rlLoadIdentity(); }
    }

    /// Multiply the current matrix by a translation matrix
    #[inline]
    fn rl_translatef(x: f32, y: f32, z: f32) {
        unsafe { ffi::rlTranslatef(x, y, z); }
    }

    /// Multiply the current matrix by a rotation matrix \
    /// NOTE: The provided angle must be in degrees
    #[inline]
    fn rl_rotatef(angle: f32, x: f32, y: f32, z: f32) {
        unsafe { ffi::rlRotatef(angle, x, y, z); }
    }

    /// Multiply the current matrix by a scaling matrix
    #[inline]
    fn rl_scalef(x: f32, y: f32, z: f32) {
        unsafe { ffi::rlScalef(x, y, z); }
    }

    /// Multiply the current matrix by another matrix
    #[inline]
    fn rl_mult_matrixf(matf: &[f32; 16]) {
        unsafe { ffi::rlMultMatrixf(matf as *const f32); }
    }

    /// Multiply the current matrix by a perspective matrix generated by parameters
    #[inline]
    fn rl_frustum(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
        unsafe { ffi::rlFrustum(left, right, bottom, top, znear, zfar); }
    }

    /// Multiply the current matrix by an orthographic matrix generated by parameters
    #[inline]
    fn rl_ortho(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
        // NOTE: If left-right and top-botton values are equal it could create a division by zero,
        // response to it is platform/compiler dependant
        unsafe { ffi::rlOrtho(left, right, bottom, top, znear, zfar); }
    }

    /// Set the viewport area
    /// NOTE: We store current viewport dimensions
    #[inline]
    fn rl_viewport(x: i32, y: i32, width: i32, height: i32) {
        unsafe { ffi::rlViewport(x, y, width, height); }
    }

    // Vertex state

    /// Set current texture to use
    #[inline]
    fn rl_set_texture(&mut self, id: u32) -> RlglTexture<'_, Self> {
        unsafe { ffi::rlSetTexture(id); }
        RlglTexture(self)
    }

    /// Set current texture to use with that of `texShapes`
    #[inline]
    fn rl_set_texture_tex_shapes(&mut self) -> RlglTexture<'_, Self> {
        unsafe { ffi::rlSetTexture(tex_shapes().id); }
        RlglTexture(self)
    }

    /// Initialize drawing mode (how to organize vertex)
    #[inline]
    fn rl_begin_quads(&mut self) -> RaylibRlglQuads<'_, Self> {
        unsafe { ffi::rlBegin(ffi::RL_QUADS as i32); }
        RaylibRlglQuads(self)
    }

    /// Initialize drawing mode (how to organize vertex)
    #[inline]
    fn rl_begin_lines(&mut self) -> RaylibRlglLines<'_, Self> {
        unsafe { ffi::rlBegin(ffi::RL_LINES as i32); }
        RaylibRlglLines(self)
    }

    /// Initialize drawing mode (how to organize vertex)
    #[inline]
    fn rl_begin_triangles(&mut self) -> RaylibRlglTriangles<'_, Self> {
        unsafe { ffi::rlBegin(ffi::RL_TRIANGLES as i32); }
        RaylibRlglTriangles(self)
    }
}

impl<D: RaylibDraw> Rlgl for D {}

// Vertex buffers state

/// Enable vertex array (VAO, if supported)
#[inline]
pub fn rl_enable_vertex_array(vao_id: VaoID) -> bool {
    unsafe { ffi::rlEnableVertexArray(vao_id.get()) }
}

/// Disable vertex array (VAO, if supported)
#[inline]
pub fn rl_disable_vertex_array() {
    unsafe { ffi::rlDisableVertexArray(); }
}

/// Enable vertex buffer (VBO)
#[inline]
pub fn rl_enable_vertex_buffer(id: VboID) {
    unsafe { ffi::rlEnableVertexBuffer(id.get()); }
}

/// Disable vertex buffer (VBO)
#[inline]
pub fn rl_disable_vertex_buffer() {
    unsafe { ffi::rlDisableVertexBuffer(); }
}

/// Enable vertex buffer element (VBO element)
#[inline]
pub fn rl_enable_vertex_buffer_element(id: VboElementID) {
    unsafe { ffi::rlEnableVertexBufferElement(id.get()); }
}

/// Disable vertex buffer element (VBO element)
#[inline]
pub fn rl_disable_vertex_buffer_element() {
    unsafe { ffi::rlDisableVertexBufferElement(); }
}

/// Enable vertex attribute index
#[inline]
pub fn rl_enable_vertex_attribute(index: u32) {
    unsafe { ffi::rlEnableVertexAttribute(index); }
}

/// Disable vertex attribute index
#[inline]
pub fn rl_disable_vertex_attribute(index: u32) {
    unsafe { ffi::rlDisableVertexAttribute(index); }
}

// Textures state

/// Select and active a texture slot
#[inline]
pub fn rl_active_texture_slot(slot: i32) {
    unsafe { ffi::rlActiveTextureSlot(slot); }
}

/// Enable texture
#[inline]
pub fn rl_enable_texture(id: TextureID) {
    unsafe { ffi::rlEnableTexture(id.get()); }
}

/// Disable texture
#[inline]
pub fn rl_disable_texture() {
    unsafe { ffi::rlDisableTexture(); }
}

/// Enable texture cubemap
#[inline]
pub fn rl_enable_texture_cubemap(id: u32) {
    unsafe { ffi::rlEnableTextureCubemap(id); }
}

/// Disable texture cubemap
#[inline]
pub fn rl_disable_texture_cubemap() {
    unsafe { ffi::rlDisableTextureCubemap(); }
}

/// Set texture parameters (filter, wrap)
#[inline]
pub fn rl_texture_parameters(id: TextureID, param: i32, value: i32) {
    unsafe { ffi::rlTextureParameters(id.get(), param, value); }
}

/// Set cubemap parameters (filter, wrap)
#[inline]
pub fn rl_cubemap_parameters(id: u32, param: i32, value: i32) {
    unsafe { ffi::rlCubemapParameters(id, param, value); }
}

//------------------------------------------------------------------------------------
// Functions Declaration - OpenGL style functions (common to 1.1, 3.3+, ES2)
// NOTE: This functions are used to completely abstract raylib code from OpenGL layer,
// some of them are direct wrappers over OpenGL calls, some others are custom
//------------------------------------------------------------------------------------

// Shader state

/// Enable shader program
pub fn rl_enable_shader(id: ShaderID) {
    unsafe { ffi::rlEnableShader(id.get()); }
}

/// Disable shader program
pub fn rl_disable_shader() {
    unsafe { ffi::rlDisableShader(); }
}

// Framebuffer state

/// Enable render texture (fbo)
pub fn rl_enable_framebuffer(id: FboID) {
    unsafe { ffi::rlEnableFramebuffer(id.get()); }
}

/// Disable render texture (fbo), return to default framebuffer
pub fn rl_disable_framebuffer() {
    unsafe { ffi::rlDisableFramebuffer(); }
}

/// Activate multiple draw color buffers
pub fn rl_active_draw_buffers(count: usize) -> Result<(), TryFromIntError> {
    let count = i32::try_from(count)?;
    Ok(unsafe { ffi::rlActiveDrawBuffers(count); })
}

/// Blit active framebuffer to main framebuffer
pub fn rl_blit_framebuffer(
    src_x: i32,
    src_y: i32,
    src_width: i32,
    src_height: i32,
    dst_x: i32,
    dst_y: i32,
    dst_width: i32,
    dst_height: i32,
    buffer_mask: i32,
) {
    unsafe { ffi::rlBlitFramebuffer(src_x, src_y, src_width, src_height, dst_x, dst_y, dst_width, dst_height, buffer_mask); }
}

// General render state

/// Enable color blending
pub fn rl_enable_color_blend() {
    unsafe { ffi::rlEnableColorBlend(); }
}

/// Disable color blending
pub fn rl_disable_color_blend() {
    unsafe { ffi::rlDisableColorBlend(); }
}

/// Enable depth test
pub fn rl_enable_depth_test() {
    unsafe { ffi::rlEnableDepthTest(); }
}

/// Disable depth test
pub fn rl_disable_depth_test() {
    unsafe { ffi::rlDisableDepthTest(); }
}

/// Enable depth write
pub fn rl_enable_depth_mask() {
    unsafe { ffi::rlEnableDepthMask(); }
}

/// Disable depth write
pub fn rl_disable_depth_mask() {
    unsafe { ffi::rlDisableDepthMask(); }
}

/// Enable backface culling
pub fn rl_enable_backface_culling() {
    unsafe { ffi::rlEnableBackfaceCulling(); }
}

/// Disable backface culling
pub fn rl_disable_backface_culling() {
    unsafe { ffi::rlDisableBackfaceCulling(); }
}

/// Set face culling mode
pub fn rl_set_cull_face(mode: CullMode) {
    unsafe { ffi::rlSetCullFace(mode as i32); }
}

/// Enable scissor test
pub fn rl_enable_scissor_test() {
    unsafe { ffi::rlEnableScissorTest(); }
}

/// Disable scissor test
pub fn rl_disable_scissor_test() {
    unsafe { ffi::rlDisableScissorTest(); }
}

/// Scissor test
pub fn rl_scissor(x: i32, y: i32, width: i32, height: i32) {
    unsafe { ffi::rlScissor(x, y, width, height); }
}

/// Enable wire mode
pub fn rl_enable_wire_mode() {
    unsafe { ffi::rlEnableWireMode(); }
}

///  Enable point mode
pub fn rl_enable_point_mode() {
    unsafe { ffi::rlEnablePointMode(); }
}

/// Disable wire mode ( and point ) maybe rename
pub fn rl_disable_wire_mode() {
    unsafe { ffi::rlDisableWireMode(); }
}

/// Set the line drawing width
pub fn rl_set_line_width(width: f32) {
    unsafe { ffi::rlSetLineWidth(width); }
}

/// Get the line drawing width
pub fn rl_get_line_width() -> f32 {
    unsafe { ffi::rlGetLineWidth() }
}

/// Enable line aliasing
pub fn rl_enable_smooth_lines() {
    unsafe { ffi::rlEnableSmoothLines(); }
}

/// Disable line aliasing
pub fn rl_disable_smooth_lines() {
    unsafe { ffi::rlDisableSmoothLines(); }
}

/// Enable stereo rendering
pub fn rl_enable_stereo_render() {
    unsafe { ffi::rlEnableStereoRender(); }
}

/// Disable stereo rendering
pub fn rl_disable_stereo_render() {
    unsafe { ffi::rlDisableStereoRender(); }
}

/// Check if stereo render is enabled
pub fn rl_is_stereo_render_enabled() -> bool {
    unsafe { ffi::rlIsStereoRenderEnabled() }
}

/// Clear color buffer with color
pub fn rl_clear_color(r: u8, g: u8, b: u8, a: u8) {
    unsafe { ffi::rlClearColor(r, g, b, a); }
}

/// Clear used screen buffers (color and depth)
pub fn rl_clear_screen_buffers() {
    unsafe { ffi::rlClearScreenBuffers(); }
}

/// Check and log OpenGL error codes
pub fn rl_check_errors() {
    unsafe { ffi::rlCheckErrors(); }
}

/// Set blending mode
pub fn rl_set_blend_mode(mode: BlendMode) {
    unsafe { ffi::rlSetBlendMode(mode as i32); }
}

/// Set blending mode factor and equation (using OpenGL factors)
pub fn rl_set_blend_factors(gl_src_factor: i32, gl_dst_factor: i32, gl_equation: i32) {
    unsafe { ffi::rlSetBlendFactors(gl_src_factor, gl_dst_factor, gl_equation); }
}

/// Set blending mode factors and equations separately (using OpenGL factors)
pub fn rl_set_blend_factors_separate(
    gl_src_rgb: i32,
    gl_dst_rgb: i32,
    gl_src_alpha: i32,
    gl_dst_alpha: i32,
    gl_eq_rgb: i32,
    gl_eq_alpha: i32,
) {
    unsafe { ffi::rlSetBlendFactorsSeparate(
        gl_src_rgb,
        gl_dst_rgb,
        gl_src_alpha,
        gl_dst_alpha,
        gl_eq_rgb,
        gl_eq_alpha,
    ); }
}

// Render batch management
// NOTE: rlgl provides a default render batch to behave like OpenGL 1.1 immediate mode
// but this render batch API is exposed in case of custom batches are required

#[derive(Debug, Clone)]
pub struct RenderBatch(ffi::rlRenderBatch);

impl Drop for RenderBatch {
    /// Unload render batch system
    fn drop(&mut self) {
        unsafe { ffi::rlUnloadRenderBatch(self.0); }
    }
}

impl RenderBatch {
    pub unsafe fn into_raw(self) -> ffi::rlRenderBatch {
        let batch = self.0;
        std::mem::forget(self);
        batch
    }

    pub unsafe fn from_raw(value: ffi::rlRenderBatch) -> Self {
        Self(value)
    }
}

/// Load a render batch system
pub fn rl_load_render_batch(num_buffers: usize, buffer_elements: i32) -> Result<RenderBatch, TryFromIntError> {
    let num_buffers = i32::try_from(num_buffers)?;
    Ok(unsafe { RenderBatch::from_raw(ffi::rlLoadRenderBatch(num_buffers, buffer_elements)) })
}

/// Draw render batch data (Update->Draw->Reset)
/// NOTE: We require a pointer to reset batch and increase current buffer (multi-buffer)
pub fn rl_draw_render_batch(batch: &mut RenderBatch) {
    unsafe { ffi::rlDrawRenderBatch((&mut batch.0) as *mut ffi::rlRenderBatch); }
}

/// Set the active render batch for rlgl (NULL for default internal)
pub fn rl_set_render_batch_active(batch: Option<&mut RenderBatch>) {
    unsafe { ffi::rlSetRenderBatchActive(batch.map_or(null_mut(), |batch| (&mut batch.0) as *mut ffi::rlRenderBatch)); }
}

/// Update and draw internal render batch
pub fn rl_draw_render_batch_active() {
    unsafe { ffi::rlDrawRenderBatchActive(); }
}

/// Check internal buffer overflow for a given number of vertex
/// and force a [`ffi::rlRenderBatch`] draw call if required
pub fn rl_check_render_batch_limit(v_count: i32) -> bool {
    unsafe { ffi::rlCheckRenderBatchLimit(v_count) }
}

/// Set current texture for render batch and check buffers limits
pub fn rl_set_texture(id: TextureID) {
    unsafe { ffi::rlSetTexture(id.get()); }
}

//------------------------------------------------------------------------------------------------------------------------

// Vertex buffers management

/// Load vertex array (vao) if supported
pub fn rl_load_vertex_array() -> VaoHandle {
    unsafe { VaoHandle::fuse(ffi::rlLoadVertexArray()) }
}

/// Load a vertex buffer attribute
pub fn rl_load_vertex_buffer<T>(buffer: &[T], dynamic: bool) -> Result<VboHandle, TryFromIntError> {
    let range = buffer.as_ptr_range();
    let size = i32::try_from(unsafe { range.end.cast::<u8>().offset_from(range.start.cast::<u8>()) })?;
    Ok(unsafe { VboHandle::fuse(ffi::rlLoadVertexBuffer(range.start.cast(), size, dynamic)) })
}

/// Load a new attributes element buffer
pub fn rl_load_vertex_buffer_element<T>(buffer: &[T], dynamic: bool) -> Result<VboElementHandle, TryFromIntError> {
    let range = buffer.as_ptr_range();
    let size = i32::try_from(unsafe { range.end.cast::<u8>().offset_from(range.start.cast::<u8>()) })?;
    Ok(unsafe { VboElementHandle::fuse(ffi::rlLoadVertexBufferElement(range.start.cast(), size, dynamic)) })
}

/// Update GPU buffer with new data
pub fn rl_update_vertex_buffer<T>(buffer_id: VboID, data: &[T], offset: usize) -> Result<(), TryFromIntError> {
    let offset = i32::try_from(offset)?;
    let range = data.as_ptr_range();
    let size = i32::try_from(unsafe { range.end.cast::<u8>().offset_from(range.start.cast::<u8>()) })?;
    Ok(unsafe { ffi::rlUpdateVertexBuffer(buffer_id.get(), range.start.cast(), size, offset); })
}

/// Update vertex buffer elements with new data
pub fn rl_update_vertex_buffer_elements<T>(id: VboElementID, data: &[T], offset: usize) -> Result<(), TryFromIntError> {
    let offset = i32::try_from(offset)?;
    let range = data.as_ptr_range();
    let size = i32::try_from(unsafe { range.end.cast::<u8>().offset_from(range.start.cast::<u8>()) })?;
    Ok(unsafe { ffi::rlUpdateVertexBufferElements(id.get(), range.start.cast(), size, offset); })
}

/// Unload vertex array object (VAO)
pub fn rl_unload_vertex_array(vao_id: VaoHandle) {
    unsafe { ffi::rlUnloadVertexArray(vao_id.defuse()); }
}

// Unload vertex buffer (VBO)
pub fn rl_unload_vertex_buffer(vbo_id: VboHandle) {
    unsafe { ffi::rlUnloadVertexBuffer(vbo_id.defuse()); }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompSize {
    One   = 1,
    Two   = 2,
    Three = 3,
    Four  = 4,
}

/// GL equivalent data types
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VertexAttributeType {
    /// GL_UNSIGNED_BYTE
    UnsignedByte = ffi::RL_UNSIGNED_BYTE as i32,
    /// GL_FLOAT
    Float = ffi::RL_FLOAT as i32,
}

/// Set vertex attribute
pub fn rl_set_vertex_attribute<T>(
    index: u32,
    comp_size: CompSize,
    kind: VertexAttributeType,
    normalized: bool,
    stride: u32,
    pointer: usize,
) {
    unsafe { ffi::rlSetVertexAttribute(index, comp_size as i32, kind as i32, normalized, stride as i32, pointer as *const c_void); }
}

/// Set vertex attribute divisor
pub fn rl_set_vertex_attribute_divisor(index: u32, divisor: i32) {
    unsafe { ffi::rlSetVertexAttributeDivisor(index, divisor); }
}

/// Set vertex attribute default value
pub unsafe fn rl_set_vertex_attribute_default(loc_index: i32, value: *const c_void, attrib_type: i32, count: i32) {
    ffi::rlSetVertexAttributeDefault(loc_index, value, attrib_type, count);
}

/// Draw vertex array
pub fn rl_draw_vertex_array(offset: i32, count: i32) {
    unsafe { ffi::rlDrawVertexArray(offset, count); }
}

/// Draw vertex array elements
pub unsafe fn rl_draw_vertex_array_elements(offset: i32, count: i32, buffer: *const c_void) {
    // NOTE: Added pointer math separately from function to avoid UBSAN complaining
    ffi::rlDrawVertexArrayElements(offset, count, buffer);
}

/// Draw vertex array instanced
pub fn rl_draw_vertex_array_instanced(offset: i32, count: i32, instances: i32) {
    unsafe { ffi::rlDrawVertexArrayInstanced(offset, count, instances); }
}

/// Draw vertex array elements instanced
pub unsafe fn rl_draw_vertex_array_elements_instanced(offset: i32, count: i32, buffer: *const c_void, instances: i32) {
    // NOTE: Added pointer math separately from function to avoid UBSAN complaining
    ffi::rlDrawVertexArrayElementsInstanced(offset, count, buffer, instances);
}

// Textures management

/// Load texture in GPU
pub unsafe fn rl_load_texture(data: *const c_void, width: i32, height: i32, format: i32, mipmap_count: i32) -> TextureHandle {
    TextureHandle::fuse(ffi::rlLoadTexture(data, width, height, format, mipmap_count))
}

/// Load depth texture/renderbuffer (to be attached to fbo)
pub fn rl_load_texture_depth(width: i32, height: i32, use_render_buffer: bool) -> u32 {
    unsafe { ffi::rlLoadTextureDepth(width, height, use_render_buffer) }
}

/// Load texture cubemap
pub unsafe fn rl_load_texture_cubemap(data: *const c_void, size: i32, format: i32) -> u32 {
    ffi::rlLoadTextureCubemap(data, size, format)
}

/// Update GPU texture with new data
pub unsafe fn rl_update_texture(id: TextureID, offset_x: i32, offset_y: i32, width: i32, height: i32, format: i32, data: *const c_void) {
    ffi::rlUpdateTexture(id.get(), offset_x, offset_y, width, height, format, data);
}

/// Get OpenGL internal formats \
/// Returns: (`gl_internal_format`, `gl_format`, `gl_type`)
pub fn rl_get_gl_texture_formats(format: i32) -> (u32, u32, u32) {
    let mut gl_internal_format: c_uint = 0;
    let mut gl_format: c_uint = 0;
    let mut gl_type: c_uint = 0;
    unsafe { ffi::rlGetGlTextureFormats(format, (&mut gl_internal_format) as *mut _, (&mut gl_format) as *mut _, (&mut gl_type) as *mut _); }
    (gl_internal_format, gl_format, gl_type)
}

/// Get name string for pixel format
pub fn rl_get_pixel_format_name(format: PixelFormat) -> &'static CStr {
    // I just copied and pasted this in from the ffi version to prove to Rust that it's safe.
    match format {
        PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAYSCALE => c"GRAYSCALE",         // 8 bit per pixel (no alpha)
        PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA => c"GRAY_ALPHA",       // 8*2 bpp (2 channels)
        PixelFormat::PIXELFORMAT_UNCOMPRESSED_R5G6B5 => c"R5G6B5",               // 16 bpp
        PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8 => c"R8G8B8",               // 24 bpp
        PixelFormat::PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 => c"R5G5B5A1",           // 16 bpp (1 bit alpha)
        PixelFormat::PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 => c"R4G4B4A4",           // 16 bpp (4 bit alpha)
        PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 => c"R8G8B8A8",           // 32 bpp
        PixelFormat::PIXELFORMAT_UNCOMPRESSED_R32 => c"R32",                     // 32 bpp (1 channel - float)
        PixelFormat::PIXELFORMAT_UNCOMPRESSED_R32G32B32 => c"R32G32B32",         // 32*3 bpp (3 channels - float)
        PixelFormat::PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 => c"R32G32B32A32",   // 32*4 bpp (4 channels - float)
        PixelFormat::PIXELFORMAT_UNCOMPRESSED_R16 => c"R16",                     // 16 bpp (1 channel - half float)
        PixelFormat::PIXELFORMAT_UNCOMPRESSED_R16G16B16 => c"R16G16B16",         // 16*3 bpp (3 channels - half float)
        PixelFormat::PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 => c"R16G16B16A16",   // 16*4 bpp (4 channels - half float)
        PixelFormat::PIXELFORMAT_COMPRESSED_DXT1_RGB => c"DXT1_RGB",             // 4 bpp (no alpha)
        PixelFormat::PIXELFORMAT_COMPRESSED_DXT1_RGBA => c"DXT1_RGBA",           // 4 bpp (1 bit alpha)
        PixelFormat::PIXELFORMAT_COMPRESSED_DXT3_RGBA => c"DXT3_RGBA",           // 8 bpp
        PixelFormat::PIXELFORMAT_COMPRESSED_DXT5_RGBA => c"DXT5_RGBA",           // 8 bpp
        PixelFormat::PIXELFORMAT_COMPRESSED_ETC1_RGB => c"ETC1_RGB",             // 4 bpp
        PixelFormat::PIXELFORMAT_COMPRESSED_ETC2_RGB => c"ETC2_RGB",             // 4 bpp
        PixelFormat::PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA => c"ETC2_RGBA",       // 8 bpp
        PixelFormat::PIXELFORMAT_COMPRESSED_PVRT_RGB => c"PVRT_RGB",             // 4 bpp
        PixelFormat::PIXELFORMAT_COMPRESSED_PVRT_RGBA => c"PVRT_RGBA",           // 4 bpp
        PixelFormat::PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA => c"ASTC_4x4_RGBA",   // 8 bpp
        PixelFormat::PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA => c"ASTC_8x8_RGBA",   // 2 bpp
        // _ => c"UNKNOWN",
    }
}

/// Unload texture from GPU memory
pub fn rl_unload_texture(id: TextureHandle) {
    unsafe { ffi::rlUnloadTexture(id.defuse()); }
}

/// Generate mipmap data for selected texture
/// NOTE: Only supports GPU mipmap generation
pub fn rl_gen_texture_mipmaps(id: TextureID, width: i32, height: i32, format: ffi::rlPixelFormat) -> Option<NonZeroUsize> {
    let mut mipmaps: i32 = 0;
    unsafe { ffi::rlGenTextureMipmaps(id.get(), width, height, format as i32, (&mut mipmaps) as *mut i32); }
    NonZeroUsize::new(usize::try_from(mipmaps).expect("rlGenTextureMipmaps should not return a negative"))
}

/// Read texture pixel data
pub fn rl_read_texture_pixels(id: TextureID, width: i32, height: i32, format: i32) -> *mut c_void {
    // NOTE: Using texture id, we can retrieve some texture info (but not on OpenGL ES 2.0)
    // Possible texture info: GL_TEXTURE_RED_SIZE, GL_TEXTURE_GREEN_SIZE, GL_TEXTURE_BLUE_SIZE, GL_TEXTURE_ALPHA_SIZE
    //int width, height, format;
    //glGetTexLevelParameteriv(GL_TEXTURE_2D, 0, GL_TEXTURE_WIDTH, &width);
    //glGetTexLevelParameteriv(GL_TEXTURE_2D, 0, GL_TEXTURE_HEIGHT, &height);
    //glGetTexLevelParameteriv(GL_TEXTURE_2D, 0, GL_TEXTURE_INTERNAL_FORMAT, &format);

    // NOTE: Each row written to or read from by OpenGL pixel operations like glGetTexImage are aligned to a 4 byte boundary by default, which may add some padding.
    // Use glPixelStorei to modify padding with the GL_[UN]PACK_ALIGNMENT setting.
    // GL_PACK_ALIGNMENT affects operations that read from OpenGL memory (glReadPixels, glGetTexImage, etc.)
    // GL_UNPACK_ALIGNMENT affects operations that write to OpenGL memory (glTexImage, etc.)


    // glGetTexImage() is not available on OpenGL ES 2.0
    // Texture width and height are required on OpenGL ES 2.0. There is no way to get it from texture id.
    // Two possible Options:
    // 1 - Bind texture to color fbo attachment and glReadPixels()
    // 2 - Create an fbo, activate it, render quad with texture, glReadPixels()
    // We are using Option 1, just need to care for texture format on retrieval
    // NOTE: This behaviour could be conditioned by graphic driver...

    unsafe { ffi::rlReadTexturePixels(id.get(), width, height, format) }
}

/// Read screen pixel data (color buffer)
pub fn rl_read_screen_pixels(width: i32, height: i32) -> *mut c_uchar {
    unsafe { ffi::rlReadScreenPixels(width, height) }
}

// Framebuffer management (fbo)
/// Load an empty framebuffer
pub fn rl_load_framebuffer(width: i32, height: i32) -> FboHandle {
    unsafe { FboHandle::fuse(ffi::rlLoadFramebuffer(width, height)) }
}

/// Attach texture/renderbuffer to a framebuffer
pub fn rl_framebuffer_attach(fbo_id: FboID, tex_id: TextureID, attach_type: i32, tex_type: i32, mip_level: i32) {
    unsafe { ffi::rlFramebufferAttach(fbo_id.get(), tex_id.get(), attach_type, tex_type, mip_level);}
}

/// Verify framebuffer is complete
pub fn rl_framebuffer_complete(id: FboID) -> bool {
    unsafe { ffi::rlFramebufferComplete(id.get()) }
}

/// Delete framebuffer from GPU
pub fn rl_unload_framebuffer(id: FboHandle) {
    unsafe { ffi::rlUnloadFramebuffer(id.defuse()); }
}

// Shaders management

/// Load shader from code strings
pub fn rl_load_shader_code(vs_code: &CStr, fs_code: &CStr) -> ShaderHandle {
    unsafe { ShaderHandle::fuse(ffi::rlLoadShaderCode(vs_code.as_ptr(), fs_code.as_ptr())) }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShaderType {
    Vertex   = ffi::RL_VERTEX_SHADER as i32,
    Fragment = ffi::RL_FRAGMENT_SHADER as i32,
    Compute  = ffi::RL_COMPUTE_SHADER as i32,
}

/// Compile custom shader and return shader id (type: `RL_VERTEX_SHADER`, `RL_FRAGMENT_SHADER`, `RL_COMPUTE_SHADER`)
pub fn rl_compile_shader(shader_code: &CStr, kind: ShaderType) -> ShaderHandle {
    unsafe { ShaderHandle::fuse(ffi::rlCompileShader(shader_code.as_ptr(), kind as i32)) }
}

/// Load custom shader program
pub fn rl_load_shader_program<'a>(v_shader_id: ShaderID<'a>, f_shader_id: ShaderID<'a>) -> ShaderPrgmHandle<'a> {
    unsafe { ShaderPrgmHandle::fuse([v_shader_id, f_shader_id], ffi::rlLoadShaderProgram(v_shader_id.get(), f_shader_id.get())) }
}

/// Unload shader program
pub fn rl_unload_shader_program(id: ShaderPrgmHandle) {
    unsafe { ffi::rlUnloadShaderProgram(id.defuse()); }
}

/// Get shader location uniform
pub fn rl_get_location_uniform<'a>(shader_id: ShaderID<'a>, uniform_name: &CStr) -> Option<ShaderUniformLoc<'a>> {
    unsafe { ShaderUniformLoc::make(shader_id, ffi::rlGetLocationUniform(shader_id.get(), uniform_name.as_ptr())) }
}

/// Get shader location attribute
pub fn rl_get_location_attrib<'a>(shader_id: ShaderID<'a>, attrib_name: &CStr) -> Option<ShaderAttribLoc<'a>> {
    unsafe { ShaderAttribLoc::make(shader_id, ffi::rlGetLocationAttrib(shader_id.get(), attrib_name.as_ptr())) }
}

pub enum ShaderUniformData<'a, 'b> {
    RlShaderUniformFloat(&'a [f32]),
    RlShaderUniformVec2(&'a [[f32; 2]]),
    RlShaderUniformVec3(&'a [[f32; 3]]),
    RlShaderUniformVec4(&'a [[f32; 4]]),
    RlShaderUniformInt(&'a [i32]),
    RlShaderUniformIVec2(&'a [[i32; 2]]),
    RlShaderUniformIVec3(&'a [[i32; 3]]),
    RlShaderUniformIVec4(&'a [[i32; 4]]),
    RlShaderUniformSampler2D(&'a [TextureID<'b>]),
}

impl<'a, 'b> ShaderUniformData<'a, 'b> {
    #[inline]
    const fn into_raw_parts(&self) -> (i32, *const std::os::raw::c_void, usize) {
        match *self {
            ShaderUniformData::RlShaderUniformFloat    (items) => (ffi::rlShaderUniformDataType::RL_SHADER_UNIFORM_FLOAT     as i32, items.as_ptr().cast(), items.len()),
            ShaderUniformData::RlShaderUniformVec2     (items) => (ffi::rlShaderUniformDataType::RL_SHADER_UNIFORM_VEC2      as i32, items.as_ptr().cast(), items.len()),
            ShaderUniformData::RlShaderUniformVec3     (items) => (ffi::rlShaderUniformDataType::RL_SHADER_UNIFORM_VEC3      as i32, items.as_ptr().cast(), items.len()),
            ShaderUniformData::RlShaderUniformVec4     (items) => (ffi::rlShaderUniformDataType::RL_SHADER_UNIFORM_VEC4      as i32, items.as_ptr().cast(), items.len()),
            ShaderUniformData::RlShaderUniformInt      (items) => (ffi::rlShaderUniformDataType::RL_SHADER_UNIFORM_INT       as i32, items.as_ptr().cast(), items.len()),
            ShaderUniformData::RlShaderUniformIVec2    (items) => (ffi::rlShaderUniformDataType::RL_SHADER_UNIFORM_IVEC2     as i32, items.as_ptr().cast(), items.len()),
            ShaderUniformData::RlShaderUniformIVec3    (items) => (ffi::rlShaderUniformDataType::RL_SHADER_UNIFORM_IVEC3     as i32, items.as_ptr().cast(), items.len()),
            ShaderUniformData::RlShaderUniformIVec4    (items) => (ffi::rlShaderUniformDataType::RL_SHADER_UNIFORM_IVEC4     as i32, items.as_ptr().cast(), items.len()),
            ShaderUniformData::RlShaderUniformSampler2D(items) => (ffi::rlShaderUniformDataType::RL_SHADER_UNIFORM_SAMPLER2D as i32, items.as_ptr().cast(), items.len()),
        }
    }
}

/// Set shader value uniform
pub fn rl_set_uniform(loc_index: ShaderUniformLoc, data: ShaderUniformData) -> Result<(), TryFromIntError> {
    let (uniform_type, value, count) = data.into_raw_parts();
    Ok(unsafe { ffi::rlSetUniform(loc_index.get(), value, uniform_type, i32::try_from(count)?); })
}

/// Set shader value matrix
pub fn rl_set_uniform_matrix(loc_index: ShaderUniformLoc, mat: ffi::Matrix) {
    unsafe { ffi::rlSetUniformMatrix(loc_index.get(), mat); }
}

/// Set shader value sampler
pub fn rl_set_uniform_sampler(loc_index: ShaderUniformLoc, texture_id: TextureID) {
    unsafe { ffi::rlSetUniformSampler(loc_index.get(), texture_id.get()); }
}

/// Set shader currently active (id and locations)
pub fn rl_set_shader(id: ShaderID, locs: &mut [ShaderUniformLoc]) {
    unsafe { ffi::rlSetShader(id.get(), locs.as_mut_ptr().cast()); }
}

// Compute shader management

/// Load compute shader program
pub fn rl_load_compute_shader_program<'a>(shader_id: ShaderID<'a>) -> ShaderPrgmHandle<'a> {
    unsafe { ShaderPrgmHandle::fuse([shader_id], ffi::rlLoadComputeShaderProgram(shader_id.get())) }
}

/// Dispatch compute shader (equivalent to *draw* for graphics pipeline)
pub fn rl_compute_shader_dispatch(group_x: u32, group_y: u32, group_z: u32) {
    unsafe { ffi::rlComputeShaderDispatch(group_x, group_y, group_z); }
}

// Shader buffer storage object management (ssbo)

/// Load shader storage buffer object (SSBO)
pub unsafe fn rl_load_shader_buffer(size: u32, data: *const c_void, usage_hint: i32) -> SsboHandle {
    SsboHandle::fuse(ffi::rlLoadShaderBuffer(size, data, usage_hint))
}

/// Unload shader storage buffer object (SSBO)
pub fn rl_unload_shader_buffer(ssbo_id: SsboHandle) {
    unsafe { ffi::rlUnloadShaderBuffer(ssbo_id.defuse()); }
}

/// Update SSBO buffer data
pub unsafe fn rl_update_shader_buffer(id: SsboID, data: *const c_void, data_size: u32, offset: u32) {
    ffi::rlUpdateShaderBuffer(id.get(), data, data_size, offset);
}

/// Bind SSBO buffer
pub fn rl_bind_shader_buffer(id: SsboID, index: u32) {
    unsafe { ffi::rlBindShaderBuffer(id.get(), index); }
}

/// Read SSBO buffer data (GPU->CPU)
pub unsafe fn rl_read_shader_buffer(id: SsboID, dest: *mut c_void, count: u32, offset: u32) {
    ffi::rlReadShaderBuffer(id.get(), dest, count, offset);
}

/// Copy SSBO data between buffers
pub fn rl_copy_shader_buffer(dest_id: SsboID, src_id: SsboID, dest_offset: u32, src_offset: u32, count: u32) {
    unsafe { ffi::rlCopyShaderBuffer(dest_id.get(), src_id.get(), dest_offset, src_offset, count); }
}

/// Get SSBO buffer size
pub fn rl_get_shader_buffer_size(id: SsboID) -> u32 {
    unsafe { ffi::rlGetShaderBufferSize(id.get()) }
}

// Buffer management

/// Bind image texture
pub fn rl_bind_image_texture(id: u32, index: u32, format: i32, readonly: bool) {
    unsafe { ffi::rlBindImageTexture(id, index, format, readonly); }
}

// Matrix state management

/// Get internal modelview matrix
pub fn rl_get_matrix_modelview() -> ffi::Matrix {
    unsafe { ffi::rlGetMatrixModelview() }
}

/// Get internal projection matrix
pub fn rl_get_matrix_projection() -> ffi::Matrix {
    unsafe { ffi::rlGetMatrixProjection() }
}

/// Get internal accumulated transform matrix
pub fn rl_get_matrix_transform() -> ffi::Matrix {
    unsafe { ffi::rlGetMatrixTransform() }
}

/// Get internal projection matrix for stereo render (selected eye)
pub fn rl_get_matrix_projection_stereo(eye: i32) -> ffi::Matrix {
    unsafe { ffi::rlGetMatrixProjectionStereo(eye) }
}

/// Get internal view offset matrix for stereo render (selected eye)
pub fn rl_get_matrix_view_offset_stereo(eye: i32) -> ffi::Matrix {
    unsafe { ffi::rlGetMatrixViewOffsetStereo(eye) }
}

/// Set a custom projection matrix (replaces internal projection matrix)
pub fn rl_set_matrix_projection(proj: ffi::Matrix) {
    unsafe { ffi::rlSetMatrixProjection(proj); }
}

/// Set a custom modelview matrix (replaces internal modelview matrix)
pub fn rl_set_matrix_modelview(view: ffi::Matrix) {
    unsafe { ffi::rlSetMatrixModelview(view); }
}

/// Set eyes projection matrices for stereo rendering
pub fn rl_set_matrix_projection_stereo(right: ffi::Matrix, left: ffi::Matrix) {
    unsafe { ffi::rlSetMatrixProjectionStereo(right, left); }
}

/// Set eyes view offsets matrices for stereo rendering
pub fn rl_set_matrix_view_offset_stereo(right: ffi::Matrix, left: ffi::Matrix) {
    unsafe { ffi::rlSetMatrixViewOffsetStereo(right, left); }
}

// Quick and dirty cube/quad buffers load->draw->unload

/// Load and draw a cube
pub fn rl_load_draw_cube() {
    unsafe { ffi::rlLoadDrawCube(); }
}

/// Load and draw a quad
pub fn rl_load_draw_quad() {
    unsafe { ffi::rlLoadDrawQuad(); }
}

#[inline]
pub fn tex_shapes() -> &'static ffi::Texture2D {
    #[link(name = "raylib")]
    unsafe extern "C" {
        static texShapes: ffi::Texture2D;
    }
    unsafe { &texShapes }
}

#[inline]
pub fn tex_shapes_rec() -> Rectangle {
    #[link(name = "raylib")]
    unsafe extern "C" {
        static texShapesRec: ffi::Rectangle;
    }
    unsafe { Rectangle::from(texShapesRec) }
}

pub struct RlglTexture<'a, D: ?Sized>(&'a mut D);

impl<D: ?Sized> Drop for RlglTexture<'_, D> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::rlSetTexture(0); }
    }
}

impl<D: ?Sized> RlglTexture<'_, D> {
    /// Initialize drawing mode (how to organize vertex)
    #[inline]
    pub fn rl_begin_quads(&mut self) -> RaylibRlglQuads<'_, Self> {
        unsafe { ffi::rlBegin(ffi::RL_QUADS as i32); }
        RaylibRlglQuads(self)
    }
}

pub struct RaylibRlglLines<'a, D: ?Sized>(&'a mut D);

impl<D: ?Sized> Drop for RaylibRlglLines<'_, D> {
    /// Finish vertex providing
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::rlEnd(); }
    }
}

pub struct RaylibRlglTriangles<'a, D: ?Sized>(&'a mut D);

impl<D: ?Sized> Drop for RaylibRlglTriangles<'_, D> {
    /// Finish vertex providing
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::rlEnd(); }
    }
}

impl<D: ?Sized> RaylibRlglTriangles<'_, D> {
    /// Define three vertices (position) - 2x3 float
    #[inline]
    pub fn rl_triangle2f(&mut self, p0: (f32, f32), p1: (f32, f32), p2: (f32, f32)) {
        self.rl_vertex2f(p0.0, p0.1);
        self.rl_vertex2f(p1.0, p1.1);
        self.rl_vertex2f(p2.0, p2.1);
    }

    /// Define one vertex (normal) - 3 float \
    /// NOTE: Normals limited to TRIANGLES only?
    #[inline]
    pub fn rl_normal3f(&mut self, x: f32, y: f32, z: f32) {
        unsafe { ffi::rlNormal3f(x, y, z); }
    }
}

pub struct RaylibRlglQuads<'a, D: ?Sized>(&'a mut D);

impl<D: ?Sized> Drop for RaylibRlglQuads<'_, D> {
    /// Finish vertex providing
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::rlEnd(); }
    }
}

impl<D: ?Sized> RaylibRlglQuads<'_, D> {
    /// Define one vertex (texture coordinate) - 2 float \
    /// NOTE: Texture coordinates are limited to QUADS only
    #[inline]
    pub fn rl_tex_coord2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::rlTexCoord2f(x, y); }
    }

    /// Define one vertex (texture coordinate) - 2 float \
    /// and one vertex (position) - 2 float \
    /// NOTE: Texture coordinates are limited to QUADS only
    #[inline]
    pub fn rl_tex_coord_vertex2f(&mut self, coord: (f32, f32), vert: (f32, f32)) {
        self.rl_tex_coord2f(coord.0, coord.1);
        self.rl_vertex2f(vert.0, vert.1);
    }

    /// Define one vertex (normal) - 3 float \
    /// NOTE: Normals limited to TRIANGLES only?
    #[inline]
    pub fn rl_normal3f(&mut self, x: f32, y: f32, z: f32) {
        unsafe { ffi::rlNormal3f(x, y, z); }
    }
}

pub trait RaylibRlglDraw {
    /// Define one vertex (position) - 2 int
    #[inline]
    fn rl_vertex2i(&mut self, x: i32, y: i32) {
        unsafe { ffi::rlVertex2i(x, y); }
    }

    /// Define one vertex (position) - 2 float
    #[inline]
    fn rl_vertex2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::rlVertex2f(x, y); }
    }

    /// Define one vertex (position) - 3 float
    #[inline]
    fn rl_vertex3f(&mut self, x: f32, y: f32, z: f32) {
        unsafe { ffi::rlVertex3f(x, y, z); }
    }

    /// Define one vertex (color) - 4 byte
    #[inline]
    fn rl_color4ub(&mut self, r: u8, g: u8, b: u8, a: u8) {
        unsafe { ffi::rlColor4ub(r, g, b, a); }
    }

    /// Define one vertex (color) - 3 float
    #[inline]
    fn rl_color3f(&mut self, r: f32, g: f32, b: f32) {
        unsafe { ffi::rlColor3f(r, g, b); }
    }

    /// Define one vertex (color) - 4 float
    #[inline]
    fn rl_color4f(&mut self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { ffi::rlColor4f(r, g, b, a); }
    }
}

impl<D: ?Sized> RaylibRlglDraw for RaylibRlglLines<'_, D> {}
impl<D: ?Sized> RaylibRlglDraw for RaylibRlglTriangles<'_, D> {}
impl<D: ?Sized> RaylibRlglDraw for RaylibRlglQuads<'_, D> {}

#[cfg(test)]
mod tests {
    use raylib::prelude::*;
    use rltest::*;
    use super::*;

    #[test]
    fn test0() {
        rl_test("test0", 640, 480, 60, |rl| {
            let texture = RaylibTestWrapper::load_texture_from_image(rl, &Image::gen_image_gradient_square(32, 32, 0.5, Color::RED, Color::BLUE))?;
            rl.run(|rl| {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    success!()
                }
                rl.begin_drawing(Color::BLACK, |d| {
                    let mut d = d.rl_set_texture(texture.id);
                    let mut d = d.rl_begin_quads();

                    d.rl_color4ub(255, 255, 255, 255);
                    d.rl_normal3f(0.0, 0.0, 1.0);

                    d.rl_tex_coord2f(0.0, 0.0); d.rl_vertex2f( 0.0,  0.0);
                    d.rl_tex_coord2f(0.0, 1.0); d.rl_vertex2f( 0.0, 32.0);
                    d.rl_tex_coord2f(1.0, 1.0); d.rl_vertex2f(32.0, 32.0);
                    d.rl_tex_coord2f(1.0, 0.0); d.rl_vertex2f(32.0,  0.0);
                })
            })
        }).expect("rejected");
    }
}
