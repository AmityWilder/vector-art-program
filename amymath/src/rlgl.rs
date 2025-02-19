use raylib::prelude::*;
use std::{ffi::{c_void, CStr}, num::{NonZeroUsize, TryFromIntError}, ops::{Deref, DerefMut}, ptr::null_mut};

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
pub fn rl_enable_vertex_array(vao_id: u32) -> bool {
    unsafe { ffi::rlEnableVertexArray(vao_id) }
}

/// Disable vertex array (VAO, if supported)
#[inline]
pub fn rl_disable_vertex_array() {
    unsafe { ffi::rlDisableVertexArray(); }
}

/// Enable vertex buffer (VBO)
#[inline]
pub fn rl_enable_vertex_buffer(id: u32) {
    unsafe { ffi::rlEnableVertexBuffer(id); }
}

/// Disable vertex buffer (VBO)
#[inline]
pub fn rl_disable_vertex_buffer() {
    unsafe { ffi::rlDisableVertexBuffer(); }
}

/// Enable vertex buffer element (VBO element)
#[inline]
pub fn rl_enable_vertex_buffer_element(id: u32) {
    unsafe { ffi::rlEnableVertexBufferElement(id); }
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
pub fn rl_enable_texture(id: u32) {
    unsafe { ffi::rlEnableTexture(id); }
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
pub fn rl_texture_parameters(id: u32, param: i32, value: i32) {
    unsafe { ffi::rlTextureParameters(id, param, value); }
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
pub fn rl_enable_shader(id: u32) {
    unsafe { ffi::rlEnableShader(id); }
}

/// Disable shader program
pub fn rl_disable_shader() {
    unsafe { ffi::rlDisableShader(); }
}

// Framebuffer state

/// Enable render texture (fbo)
pub fn rl_enable_framebuffer(id: u32) {
    unsafe { ffi::rlEnableFramebuffer(id); }
}

/// Disable render texture (fbo), return to default framebuffer
pub fn rl_disable_framebuffer() {
    unsafe { ffi::rlDisableFramebuffer(); }
}

/// Activate multiple draw color buffers
pub fn rl_active_draw_buffers(count: i32) {
    unsafe { ffi::rlActiveDrawBuffers(count); }
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

/// Load a render batch system
pub fn rl_load_render_batch(num_buffers: i32, buffer_elements: i32) -> ffi::rlRenderBatch {
    unsafe { ffi::rlLoadRenderBatch(num_buffers, buffer_elements) }
}
/// Unload render batch system
pub fn rl_unload_render_batch(batch: ffi::rlRenderBatch) {
    unsafe { ffi::rlUnloadRenderBatch(batch); }
}
/// Draw render batch data (Update->Draw->Reset)
/// NOTE: We require a pointer to reset batch and increase current buffer (multi-buffer)
pub fn rl_draw_render_batch(mut batch: impl AsMut<ffi::rlRenderBatch>) {
    unsafe { ffi::rlDrawRenderBatch(batch.as_mut() as *mut ffi::rlRenderBatch); }
}
/// Set the active render batch for rlgl (NULL for default internal)
pub fn rl_set_render_batch_active(batch: Option<impl AsMut<ffi::rlRenderBatch>>) {
    unsafe { ffi::rlSetRenderBatchActive(batch.map_or(null_mut(), |mut batch| batch.as_mut() as *mut ffi::rlRenderBatch)); }
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
pub fn rl_set_texture(id: u32) {
    unsafe { ffi::rlSetTexture(id); }
}

//------------------------------------------------------------------------------------------------------------------------

// Vertex buffers management

/// Load vertex array (vao) if supported
pub fn rl_load_vertex_array() -> u32 {
    unsafe { ffi::rlLoadVertexArray() }
}

/// Load a vertex buffer attribute
pub fn rl_load_vertex_buffer<T>(buffer: &[T], dynamic: bool) -> Result<u32, TryFromIntError> {
    let range = buffer.as_ptr_range();
    let size = i32::try_from(unsafe { range.end.cast::<u8>().offset_from(range.start.cast::<u8>()) })?;
    Ok(unsafe { ffi::rlLoadVertexBuffer(range.start.cast(), size, dynamic) })
}

/// Load a new attributes element buffer
pub fn rl_load_vertex_buffer_element<T>(buffer: &[T], dynamic: bool) -> Result<u32, TryFromIntError> {
    let range = buffer.as_ptr_range();
    let size = i32::try_from(unsafe { range.end.cast::<u8>().offset_from(range.start.cast::<u8>()) })?;
    Ok(unsafe { ffi::rlLoadVertexBufferElement(range.start.cast(), size, dynamic) })
}

/// Update GPU buffer with new data
pub fn rl_update_vertex_buffer<T>(buffer_id: u32, data: &[T], offset: usize) -> Result<(), TryFromIntError> {
    let offset = i32::try_from(offset)?;
    let range = data.as_ptr_range();
    let size = i32::try_from(unsafe { range.end.cast::<u8>().offset_from(range.start.cast::<u8>()) })?;
    Ok(unsafe { ffi::rlUpdateVertexBuffer(buffer_id, range.start.cast(), size, offset); })
}

/// Update vertex buffer elements with new data
pub fn rl_update_vertex_buffer_elements<T>(id: u32, data: &[T], offset: usize) -> Result<(), TryFromIntError> {
    let offset = i32::try_from(offset)?;
    let range = data.as_ptr_range();
    let size = i32::try_from(unsafe { range.end.cast::<u8>().offset_from(range.start.cast::<u8>()) })?;
    Ok(unsafe { ffi::rlUpdateVertexBufferElements(id, range.start.cast(), size, offset); })
}

/// Unload vertex array object (VAO)
pub fn rl_unload_vertex_array(vao_id: u32) {
    unsafe { ffi::rlUnloadVertexArray(vao_id); }
}

// Unload vertex buffer (VBO)
pub fn rl_unload_vertex_buffer(vbo_id: u32) {
    unsafe { ffi::rlUnloadVertexBuffer(vbo_id); }
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
pub fn rl_set_vertex_attribute_default(loc_index: i32, value: *const c_void, attrib_type: i32, count: i32) {
    unsafe { ffi::rlSetVertexAttributeDefault(loc_index, value, attrib_type, count); }
}

/// Draw vertex array
pub fn rl_draw_vertex_array(offset: i32, count: i32) {
    unsafe { ffi::rlDrawVertexArray(offset, count); }
}

/// Draw vertex array elements
pub fn rl_draw_vertex_array_elements(offset: i32, count: i32, buffer: *const c_void) {
    // NOTE: Added pointer math separately from function to avoid UBSAN complaining
    unsafe { ffi::rlDrawVertexArrayElements(offset, count, buffer); }
}

/// Draw vertex array instanced
pub fn rl_draw_vertex_array_instanced(offset: i32, count: i32, instances: i32) {
    unsafe { ffi::rlDrawVertexArrayInstanced(offset, count, instances); }
}

/// Draw vertex array elements instanced
pub fn rl_draw_vertex_array_elements_instanced(offset: i32, count: i32, buffer: *const c_void, instances: i32) {
    // NOTE: Added pointer math separately from function to avoid UBSAN complaining
    unsafe { ffi::rlDrawVertexArrayElementsInstanced(offset, count, buffer, instances); }
}

// Textures management

/// Load texture in GPU
pub fn rl_load_texture(data: *const c_void, width: i32, height: i32, format: i32, mipmap_count: i32) -> u32 {
    unsafe { ffi::rlLoadTexture(data, width, height, format, mipmap_count) }
}

/// Load depth texture/renderbuffer (to be attached to fbo)
pub fn rl_load_texture_depth(width: i32, height: i32, use_render_buffer: bool) -> u32 {
    unsafe { ffi::rlLoadTextureDepth(width, height, use_render_buffer) }
}

/// Load texture cubemap
pub fn rl_load_texture_cubemap(data: *const c_void, size: i32, format: i32) -> u32 {
    unsafe { ffi::rlLoadTextureCubemap(data, size, format) }
}

/// Update GPU texture with new data
pub fn rl_update_texture(id: u32, offset_x: i32, offset_y: i32, width: i32, height: i32, format: i32, data: *const c_void) {
    unsafe { ffi::rlUpdateTexture(id, offset_x, offset_y, width, height, format, data); }
}

/// Get OpenGL internal formats
pub fn rl_get_gl_texture_formats(format: i32, gl_internal_format: *mut u32, gl_format: *mut u32, gl_type: *mut u32) {
    unsafe { ffi::rlGetGlTextureFormats(format, gl_internal_format, gl_format, gl_type); }
}

/// Get name string for pixel format
pub fn rl_get_pixel_format_name(format: ffi::rlPixelFormat) -> &'static str {
    // I just copied and pasted this in from the ffi version to prove to Rust that it's safe.
    match format {
        ffi::rlPixelFormat::RL_PIXELFORMAT_UNCOMPRESSED_GRAYSCALE => "GRAYSCALE",         // 8 bit per pixel (no alpha)
        ffi::rlPixelFormat::RL_PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA => "GRAY_ALPHA",       // 8*2 bpp (2 channels)
        ffi::rlPixelFormat::RL_PIXELFORMAT_UNCOMPRESSED_R5G6B5 => "R5G6B5",               // 16 bpp
        ffi::rlPixelFormat::RL_PIXELFORMAT_UNCOMPRESSED_R8G8B8 => "R8G8B8",               // 24 bpp
        ffi::rlPixelFormat::RL_PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 => "R5G5B5A1",           // 16 bpp (1 bit alpha)
        ffi::rlPixelFormat::RL_PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 => "R4G4B4A4",           // 16 bpp (4 bit alpha)
        ffi::rlPixelFormat::RL_PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 => "R8G8B8A8",           // 32 bpp
        ffi::rlPixelFormat::RL_PIXELFORMAT_UNCOMPRESSED_R32 => "R32",                     // 32 bpp (1 channel - float)
        ffi::rlPixelFormat::RL_PIXELFORMAT_UNCOMPRESSED_R32G32B32 => "R32G32B32",         // 32*3 bpp (3 channels - float)
        ffi::rlPixelFormat::RL_PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 => "R32G32B32A32",   // 32*4 bpp (4 channels - float)
        ffi::rlPixelFormat::RL_PIXELFORMAT_UNCOMPRESSED_R16 => "R16",                     // 16 bpp (1 channel - half float)
        ffi::rlPixelFormat::RL_PIXELFORMAT_UNCOMPRESSED_R16G16B16 => "R16G16B16",         // 16*3 bpp (3 channels - half float)
        ffi::rlPixelFormat::RL_PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 => "R16G16B16A16",   // 16*4 bpp (4 channels - half float)
        ffi::rlPixelFormat::RL_PIXELFORMAT_COMPRESSED_DXT1_RGB => "DXT1_RGB",             // 4 bpp (no alpha)
        ffi::rlPixelFormat::RL_PIXELFORMAT_COMPRESSED_DXT1_RGBA => "DXT1_RGBA",           // 4 bpp (1 bit alpha)
        ffi::rlPixelFormat::RL_PIXELFORMAT_COMPRESSED_DXT3_RGBA => "DXT3_RGBA",           // 8 bpp
        ffi::rlPixelFormat::RL_PIXELFORMAT_COMPRESSED_DXT5_RGBA => "DXT5_RGBA",           // 8 bpp
        ffi::rlPixelFormat::RL_PIXELFORMAT_COMPRESSED_ETC1_RGB => "ETC1_RGB",             // 4 bpp
        ffi::rlPixelFormat::RL_PIXELFORMAT_COMPRESSED_ETC2_RGB => "ETC2_RGB",             // 4 bpp
        ffi::rlPixelFormat::RL_PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA => "ETC2_RGBA",       // 8 bpp
        ffi::rlPixelFormat::RL_PIXELFORMAT_COMPRESSED_PVRT_RGB => "PVRT_RGB",             // 4 bpp
        ffi::rlPixelFormat::RL_PIXELFORMAT_COMPRESSED_PVRT_RGBA => "PVRT_RGBA",           // 4 bpp
        ffi::rlPixelFormat::RL_PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA => "ASTC_4x4_RGBA",   // 8 bpp
        ffi::rlPixelFormat::RL_PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA => "ASTC_8x8_RGBA",   // 2 bpp
        _ => "UNKNOWN",
    }
}

/// Unload texture from GPU memory
pub fn rl_unload_texture(id: u32) {
    unsafe { ffi::rlUnloadTexture(id); }
}

/// Generate mipmap data for selected texture
/// NOTE: Only supports GPU mipmap generation
pub fn rl_gen_texture_mipmaps(id: u32, width: i32, height: i32, format: ffi::rlPixelFormat) -> Option<NonZeroUsize> {
    let mut mipmaps: i32 = 0;
    unsafe { ffi::rlGenTextureMipmaps(id, width, height, format as i32, (&mut mipmaps) as *mut i32); }
    NonZeroUsize::new(usize::try_from(mipmaps).expect("rlGenTextureMipmaps should not return a negative"))
}

/// Read texture pixel data
pub fn rl_read_texture_pixels(id: u32, width: i32, height: i32, format: i32) -> *mut c_void {
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

    unsafe { ffi::rlReadTexturePixels(id, width, height, format) }
}

/// Read screen pixel data (color buffer)
pub fn rl_read_screen_pixels(width: i32, height: i32) -> *mut u8 {
    unsafe { ffi::rlReadScreenPixels(width, height) }
}

// Framebuffer management (fbo)
/// Load an empty framebuffer
pub fn rl_load_framebuffer(width: i32, height: i32) -> u32 {
    unsafe { ffi::rlLoadFramebuffer(width, height) }
}

/// Attach texture/renderbuffer to a framebuffer
pub fn rl_framebuffer_attach(fbo_id: u32, tex_id: u32, attach_type: i32, tex_type: i32, mip_level: i32) {
    unsafe { ffi::rlFramebufferAttach(fbo_id, tex_id, attach_type, tex_type, mip_level);}
}

/// Verify framebuffer is complete
pub fn rl_framebuffer_complete(id: u32) -> bool {
    unsafe { ffi::rlFramebufferComplete(id) }
}

/// Delete framebuffer from GPU
pub fn rl_unload_framebuffer(id: u32) {
    unsafe { ffi::rlUnloadFramebuffer(id); }
}

// Shaders management

/// Load shader from code strings
pub fn rl_load_shader_code(vs_code: &CStr, fs_code: &CStr) -> u32 {
    unsafe { ffi::rlLoadShaderCode(vs_code.as_ptr(), fs_code.as_ptr()) }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShaderType {
    Vertex   = ffi::RL_VERTEX_SHADER as i32,
    Fragment = ffi::RL_FRAGMENT_SHADER as i32,
    Compute  = ffi::RL_COMPUTE_SHADER as i32,
}

/// Compile custom shader and return shader id (type: `RL_VERTEX_SHADER`, `RL_FRAGMENT_SHADER`, `RL_COMPUTE_SHADER`)
pub fn rl_compile_shader(shader_code: &CStr, kind: ShaderType) -> u32 {
    unsafe { ffi::rlCompileShader(shader_code.as_ptr(), kind as i32) }
}

/// Load custom shader program
pub fn rl_load_shader_program(v_shader_id: u32, f_shader_id: u32) -> u32 {
    unsafe { ffi::rlLoadShaderProgram(v_shader_id, f_shader_id) }
}

/// Unload shader program
pub fn rl_unload_shader_program(id: u32) {
    unsafe { ffi::rlUnloadShaderProgram(id); }
}

/// Get shader location uniform
pub fn rl_get_location_uniform(shader_id: u32, uniform_name: &CStr) -> i32 {
    unsafe { ffi::rlGetLocationUniform(shader_id, uniform_name.as_ptr()) }
}

/// Get shader location attribute
pub fn rl_get_location_attrib(shader_id: u32, attrib_name: &CStr) -> i32 {
    unsafe { ffi::rlGetLocationAttrib(shader_id, attrib_name.as_ptr()) }
}

pub enum ShaderUniformData {
    RlShaderUniformFloat(f32),
    RlShaderUniformVec2([f32; 2]),
    RlShaderUniformVec3([f32; 3]),
    RlShaderUniformVec4([f32; 4]),
    RlShaderUniformInt(i32),
    RlShaderUniformIVec2([i32; 2]),
    RlShaderUniformIVec3([i32; 3]),
    RlShaderUniformIVec4([i32; 4]),
    RlShaderUniformSampler2d(),
}

/// Set shader value uniform
pub fn rl_set_uniform<T>(loc_index: i32, value: &[ShaderUniformData], uniform_type: i32) -> Result<(), TryFromIntError> {
    Ok(unsafe { ffi::rlSetUniform(loc_index, value.as_ptr().cast(), uniform_type, i32::try_from(value.len())?); })
}

/// Set shader value matrix
pub fn rl_set_uniform_matrix(loc_index: i32, mat: ffi::Matrix) {
    unsafe { ffi::rlSetUniformMatrix(loc_index, mat); }
}

/// Set shader value sampler
pub fn rl_set_uniform_sampler(loc_index: i32, texture_id: u32) {
    unsafe { ffi::rlSetUniformSampler(loc_index, texture_id); }
}

/// Set shader currently active (id and locations)
pub fn rl_set_shader(id: u32, locs: *mut i32) {
    unsafe { ffi::rlSetShader(id, locs); }
}

// Compute shader management

/// Load compute shader program
pub fn rl_load_compute_shader_program(shader_id: u32) -> u32 {
    unsafe { ffi::rlLoadComputeShaderProgram(shader_id) }
}

/// Dispatch compute shader (equivalent to *draw* for graphics pipeline)
pub fn rl_compute_shader_dispatch(group_x: u32, group_y: u32, group_z: u32) {
    unsafe { ffi::rlComputeShaderDispatch(group_x, group_y, group_z); }
}

// Shader buffer storage object management (ssbo)

/// Load shader storage buffer object (SSBO)
pub fn rl_load_shader_buffer(size: u32, data: *const c_void, usage_hint: i32) -> u32 {
    unsafe { ffi::rlLoadShaderBuffer(size, data, usage_hint) }
}

/// Unload shader storage buffer object (SSBO)
pub fn rl_unload_shader_buffer(ssbo_id: u32) {
    unsafe { ffi::rlUnloadShaderBuffer(ssbo_id); }
}

/// Update SSBO buffer data
pub fn rl_update_shader_buffer(id: u32, data: *const c_void, data_size: u32, offset: u32) {
    unsafe { ffi::rlUpdateShaderBuffer(id, data, data_size, offset); }
}

/// Bind SSBO buffer
pub fn rl_bind_shader_buffer(id: u32, index: u32) {
    unsafe { ffi::rlBindShaderBuffer(id, index); }
}

/// Read SSBO buffer data (GPU->CPU)
pub fn rl_read_shader_buffer(id: u32, dest: *mut c_void, count: u32, offset: u32) {
    unsafe { ffi::rlReadShaderBuffer(id, dest, count, offset); }
}

/// Copy SSBO data between buffers
pub fn rl_copy_shader_buffer(dest_id: u32, src_id: u32, dest_offset: u32, src_offset: u32, count: u32) {
    unsafe { ffi::rlCopyShaderBuffer(dest_id, src_id, dest_offset, src_offset, count); }
}

/// Get SSBO buffer size
pub fn rl_get_shader_buffer_size(id: u32) -> u32 {
    unsafe { ffi::rlGetShaderBufferSize(id) }
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


#[link(name = "raylib")]
unsafe extern "C" {
    static texShapes: ffi::Texture2D;
    static texShapesRec: ffi::Rectangle;
}

#[inline]
pub fn tex_shapes() -> &'static ffi::Texture2D {
    unsafe { &texShapes }
}

#[inline]
pub fn tex_shapes_size() -> (i32, i32) {
    unsafe { (texShapes.width, texShapes.height) }
}

#[inline]
pub fn tex_shapes_rec() -> Rectangle {
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

    /// Define four vertices (texture coordinate) - 2x4 float \
    /// and four vertices (position) - 2x4 float \
    /// NOTE: Texture coordinates are limited to QUADS only
    #[inline]
    pub fn rl_tex_quad2f(
        &mut self,
        coord0: (f32, f32), vert0: (f32, f32),
        coord1: (f32, f32), vert1: (f32, f32),
        coord2: (f32, f32), vert2: (f32, f32),
        coord3: (f32, f32), vert3: (f32, f32),
    ) {
        self.rl_tex_coord_vertex2f(coord0, vert0);
        self.rl_tex_coord_vertex2f(coord1, vert1);
        self.rl_tex_coord_vertex2f(coord2, vert2);
        self.rl_tex_coord_vertex2f(coord3, vert3);
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
                    let mut d = d.rl_set_texture(&texture);
                    let mut d = d.rl_begin_quads();

                    d.rl_color4ub(255, 255, 255, 255);
                    d.rl_normal3f(0.0, 0.0, 1.0);

                    d.rl_tex_quad2f(
                        (0.0, 0.0), ( 0.0,  0.0),
                        (0.0, 1.0), ( 0.0, 32.0),
                        (1.0, 1.0), (32.0, 32.0),
                        (1.0, 0.0), (32.0,  0.0),
                    );
                })
            })
        }).expect("rejected");
    }
}
