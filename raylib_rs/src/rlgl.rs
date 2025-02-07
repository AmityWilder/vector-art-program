use crate::{tracelog, Matrix};

/// Default number of batch buffers (multi-buffering)
const RL_DEFAULT_BATCH_BUFFERS: usize = 1;
/// Default number of batch draw calls (by state changes: mode, texture)
const RL_DEFAULT_BATCH_DRAWCALLS: usize = 256;
/// Maximum number of textures units that can be activated on batch drawing (SetShaderValueTexture())
const RL_DEFAULT_BATCH_MAX_TEXTURE_UNITS: usize = 4;

// Internal Matrix stack
/// Maximum size of Matrix stack
const RL_MAX_MATRIX_STACK_SIZE: usize = 32;

// Shader limits
/// Maximum number of shader locations supported
const RL_MAX_SHADER_LOCATIONS: usize = 32;

// Projection matrix culling
/// Default near cull distance
const RL_CULL_DISTANCE_NEAR: f32 = 0.01;
/// Default far cull distance
const RL_CULL_DISTANCE_FAR: f32 = 1000.0;

// Texture parameters (equivalent to OpenGL defines)
/// GL_TEXTURE_WRAP_S
const RL_TEXTURE_WRAP_S: i32 = 0x2802;
/// GL_TEXTURE_WRAP_T
const RL_TEXTURE_WRAP_T: i32 = 0x2803;
/// GL_TEXTURE_MAG_FILTER
const RL_TEXTURE_MAG_FILTER: i32 = 0x2800;
/// GL_TEXTURE_MIN_FILTER
const RL_TEXTURE_MIN_FILTER: i32 = 0x2801;

/// GL_NEAREST
const RL_TEXTURE_FILTER_NEAREST: i32 = 0x2600;
/// GL_LINEAR
const RL_TEXTURE_FILTER_LINEAR: i32 = 0x2601;
/// GL_NEAREST_MIPMAP_NEAREST
const RL_TEXTURE_FILTER_MIP_NEAREST: i32 = 0x2700;
/// GL_NEAREST_MIPMAP_LINEAR
const RL_TEXTURE_FILTER_NEAREST_MIP_LINEAR: i32 = 0x2702;
/// GL_LINEAR_MIPMAP_NEAREST
const RL_TEXTURE_FILTER_LINEAR_MIP_NEAREST: i32 = 0x2701;
/// GL_LINEAR_MIPMAP_LINEAR
const RL_TEXTURE_FILTER_MIP_LINEAR: i32 = 0x2703;
/// Anisotropic filter (custom identifier)
const RL_TEXTURE_FILTER_ANISOTROPIC: i32 = 0x3000;
/// Texture mipmap bias, percentage ratio (custom identifier)
const RL_TEXTURE_MIPMAP_BIAS_RATIO: i32 = 0x4000;

/// GL_REPEAT
const RL_TEXTURE_WRAP_REPEAT: i32 = 0x2901;
/// GL_CLAMP_TO_EDGE
const RL_TEXTURE_WRAP_CLAMP: i32 = 0x812F;
/// GL_MIRRORED_REPEAT
const RL_TEXTURE_WRAP_MIRROR_REPEAT: i32 = 0x8370;
/// GL_MIRROR_CLAMP_EXT
const RL_TEXTURE_WRAP_MIRROR_CLAMP: i32 = 0x8742;

// Matrix modes (equivalent to OpenGL)
/// GL_MODELVIEW
const RL_MODELVIEW: i32 = 0x1700;
/// GL_PROJECTION
const RL_PROJECTION: i32 = 0x1701;
/// GL_TEXTURE
const RL_TEXTURE: i32 = 0x1702;

// Primitive assembly draw modes
/// GL_LINES
const RL_LINES: i32 = 0x0001;
/// GL_TRIANGLES
const RL_TRIANGLES: i32 = 0x0004;
/// GL_QUADS
const RL_QUADS: i32 = 0x0007;

// GL equivalent data types
/// GL_UNSIGNED_BYTE
const RL_UNSIGNED_BYTE: i32 = 0x1401;
/// GL_FLOAT
const RL_FLOAT: i32 = 0x1406;

// GL buffer usage hint
/// GL_STREAM_DRAW
const RL_STREAM_DRAW: i32 = 0x88E0;
/// GL_STREAM_READ
const RL_STREAM_READ: i32 = 0x88E1;
/// GL_STREAM_COPY
const RL_STREAM_COPY: i32 = 0x88E2;
/// GL_STATIC_DRAW
const RL_STATIC_DRAW: i32 = 0x88E4;
/// GL_STATIC_READ
const RL_STATIC_READ: i32 = 0x88E5;
/// GL_STATIC_COPY
const RL_STATIC_COPY: i32 = 0x88E6;
/// GL_DYNAMIC_DRAW
const RL_DYNAMIC_DRAW: i32 = 0x88E8;
/// GL_DYNAMIC_READ
const RL_DYNAMIC_READ: i32 = 0x88E9;
/// GL_DYNAMIC_COPY
const RL_DYNAMIC_COPY: i32 = 0x88EA;

// GL Shader type
/// GL_FRAGMENT_SHADER
const RL_FRAGMENT_SHADER: i32 = 0x8B30;
/// GL_VERTEX_SHADER
const RL_VERTEX_SHADER: i32 = 0x8B31;
/// GL_COMPUTE_SHADER
const RL_COMPUTE_SHADER: i32 = 0x91B9;

// GL blending factors
/// GL_ZERO
const RL_ZERO: i32 = 0;
/// GL_ONE
const RL_ONE: i32 = 1;
/// GL_SRC_COLOR
const RL_SRC_COLOR: i32 = 0x0300;
/// GL_ONE_MINUS_SRC_COLOR
const RL_ONE_MINUS_SRC_COLOR: i32 = 0x0301;
/// GL_SRC_ALPHA
const RL_SRC_ALPHA: i32 = 0x0302;
/// GL_ONE_MINUS_SRC_ALPHA
const RL_ONE_MINUS_SRC_ALPHA: i32 = 0x0303;
/// GL_DST_ALPHA
const RL_DST_ALPHA: i32 = 0x0304;
/// GL_ONE_MINUS_DST_ALPHA
const RL_ONE_MINUS_DST_ALPHA: i32 = 0x0305;
/// GL_DST_COLOR
const RL_DST_COLOR: i32 = 0x0306;
/// GL_ONE_MINUS_DST_COLOR
const RL_ONE_MINUS_DST_COLOR: i32 = 0x0307;
/// GL_SRC_ALPHA_SATURATE
const RL_SRC_ALPHA_SATURATE: i32 = 0x0308;
/// GL_CONSTANT_COLOR
const RL_CONSTANT_COLOR: i32 = 0x8001;
/// GL_ONE_MINUS_CONSTANT_COLOR
const RL_ONE_MINUS_CONSTANT_COLOR: i32 = 0x8002;
/// GL_CONSTANT_ALPHA
const RL_CONSTANT_ALPHA: i32 = 0x8003;
/// GL_ONE_MINUS_CONSTANT_ALPHA
const RL_ONE_MINUS_CONSTANT_ALPHA: i32 = 0x8004;

// GL blending functions/equations
/// GL_FUNC_ADD
const RL_FUNC_ADD: i32 = 0x8006;
/// GL_MIN
const RL_MIN: i32 = 0x8007;
/// GL_MAX
const RL_MAX: i32 = 0x8008;
/// GL_FUNC_SUBTRACT
const RL_FUNC_SUBTRACT: i32 = 0x800A;
/// GL_FUNC_REVERSE_SUBTRACT
const RL_FUNC_REVERSE_SUBTRACT: i32 = 0x800B;
/// GL_BLEND_EQUATION
const RL_BLEND_EQUATION: i32 = 0x8009;
/// GL_BLEND_EQUATION_RGB   // (Same as BLEND_EQUATION)
const RL_BLEND_EQUATION_RGB: i32 = 0x8009;
/// GL_BLEND_EQUATION_ALPHA
const RL_BLEND_EQUATION_ALPHA: i32 = 0x883D;
/// GL_BLEND_DST_RGB
const RL_BLEND_DST_RGB: i32 = 0x80C8;
/// GL_BLEND_SRC_RGB
const RL_BLEND_SRC_RGB: i32 = 0x80C9;
/// GL_BLEND_DST_ALPHA
const RL_BLEND_DST_ALPHA: i32 = 0x80CA;
/// GL_BLEND_SRC_ALPHA
const RL_BLEND_SRC_ALPHA: i32 = 0x80CB;
/// GL_BLEND_COLOR
const RL_BLEND_COLOR: i32 = 0x8005;

/// OpenGL version
pub enum rlGlVersion {
    /// OpenGL 1.1
    Opengl11 = 1,
    /// OpenGL 2.1 (GLSL 120)
    Opengl21,
    /// OpenGL 3.3 (GLSL 330)
    Opengl33,
    /// OpenGL 4.3 (using GLSL 330)
    Opengl43,
    /// OpenGL ES 2.0 (GLSL 100)
    OpenglEs20,
    /// OpenGL ES 3.0 (GLSL 300 es)
    OpenglEs30,
}

/// Texture pixel formats
/// NOTE: Support depends on OpenGL version
pub enum rlPixelFormat {
    /// 8 bit per pixel (no alpha)
    UncompressedGrayscale = 1,
    /// 8*2 bpp (2 channels)
    UncompressedGrayAlpha,
    /// 16 bpp
    UncompressedR5G6B5,
    /// 24 bpp
    UncompressedR8G8B8,
    /// 16 bpp (1 bit alpha)
    UncompressedR5G5B5A1,
    /// 16 bpp (4 bit alpha)
    UncompressedR4G4B4A4,
    /// 32 bpp
    UncompressedR8G8B8A8,
    /// 32 bpp (1 channel - float)
    UncompressedR32,
    /// 32*3 bpp (3 channels - float)
    UncompressedR32G32B32,
    /// 32*4 bpp (4 channels - float)
    UncompressedR32G32B32A32,
    /// 16 bpp (1 channel - half float)
    UncompressedR16,
    /// 16*3 bpp (3 channels - half float)
    UncompressedR16G16B16,
    /// 16*4 bpp (4 channels - half float)
    UncompressedR16G16B16A16,
    /// 4 bpp (no alpha)
    CompressedDXT1RGB,
    /// 4 bpp (1 bit alpha)
    CompressedDXT1RGBA,
    /// 8 bpp
    CompressedDXT3RGBA,
    /// 8 bpp
    CompressedDXT5RGBA,
    /// 4 bpp
    CompressedETC1RGB,
    /// 4 bpp
    CompressedETC2RGB,
    /// 8 bpp
    CompressedETC2EACRGBA,
    /// 4 bpp
    CompressedPVRTRGB,
    /// 4 bpp
    CompressedPVRTRGBA,
    /// 8 bpp
    CompressedASTC4x4RGBA,
    /// 2 bpp
    CompressedASTC8x8RGBA,
}

/// Texture parameters: filter mode
/// NOTE 1: Filtering considers mipmaps if available in the texture
/// NOTE 2: Filter is accordingly set for minification and magnification
pub enum rlTextureFilter {
    /// No filter, just pixel approximation
    Point,
    /// Linear filtering
    Bilinear,
    /// Trilinear filtering (linear with mipmaps)
    Trilinear,
    /// Anisotropic filtering 4x
    Anisotropic4x,
    /// Anisotropic filtering 8x
    Anisotropic8x,
    /// Anisotropic filtering 16x
    Anisotropic16x,
}

/// Color blending modes (pre-defined)
pub enum rlBlendMode {
    /// Blend textures considering alpha (default)
    Alpha,
    /// Blend textures adding colors
    Additive,
    /// Blend textures multiplying colors
    Multiplied,
    /// Blend textures adding colors (alternative)
    AddColors,
    /// Blend textures subtracting colors (alternative)
    SubtractColors,
    /// Blend premultiplied textures considering alpha
    AlphaPremultiply,
    /// Blend textures using custom src/dst factors (use rlSetBlendFactors())
    Custom,
    /// Blend textures using custom src/dst factors (use rlSetBlendFactorsSeparate())
    CustomSeparate,
}

/// Shader location point type
pub enum rlShaderLocationIndex {
    /// Shader location: vertex attribute: position
    VertexPosition,
    /// Shader location: vertex attribute: texcoord01
    VertexTexcoord01,
    /// Shader location: vertex attribute: texcoord02
    VertexTexcoord02,
    /// Shader location: vertex attribute: normal
    VertexNormal,
    /// Shader location: vertex attribute: tangent
    VertexTangent,
    /// Shader location: vertex attribute: color
    VertexColor,
    /// Shader location: matrix uniform: model-view-projection
    MatrixMvp,
    /// Shader location: matrix uniform: view (camera transform)
    MatrixView,
    /// Shader location: matrix uniform: projection
    MatrixProjection,
    /// Shader location: matrix uniform: model (transform)
    MatrixModel,
    /// Shader location: matrix uniform: normal
    MatrixNormal,
    /// Shader location: vector uniform: view
    VectorView,
    /// Shader location: vector uniform: diffuse color
    ColorDiffuse,
    /// Shader location: vector uniform: specular color
    ColorSpecular,
    /// Shader location: vector uniform: ambient color
    ColorAmbient,
    /// Shader location: sampler2d texture: albedo (same as: MapDiffuse)
    MapAlbedo,
    /// Shader location: sampler2d texture: metalness (same as: MapSpecular)
    MapMetalness,
    /// Shader location: sampler2d texture: normal
    MapNormal,
    /// Shader location: sampler2d texture: roughness
    MapRoughness,
    /// Shader location: sampler2d texture: occlusion
    MapOcclusion,
    /// Shader location: sampler2d texture: emission
    MapEmission,
    /// Shader location: sampler2d texture: height
    MapHeight,
    /// Shader location: samplerCube texture: cubemap
    MapCubemap,
    /// Shader location: samplerCube texture: irradiance
    MapIrradiance,
    /// Shader location: samplerCube texture: prefilter
    MapPrefilter,
    /// Shader location: sampler2d texture: brdf
    MapBrdf,
}

#[allow(non_upper_case_globals)]
impl rlShaderLocationIndex {
    pub const MapDiffuse: Self = Self::MapAlbedo;
    pub const MapSpecular: Self = Self::MapMetalness;
}

/// Shader uniform data type
pub enum rlShaderUniformDataType {
    /// Shader uniform type: float
    Float,
    /// Shader uniform type: vec2 (2 float)
    Vec2,
    /// Shader uniform type: vec3 (3 float)
    Vec3,
    /// Shader uniform type: vec4 (4 float)
    Vec4,
    /// Shader uniform type: int
    Int,
    /// Shader uniform type: ivec2 (2 int)
    IVec2,
    /// Shader uniform type: ivec3 (3 int)
    IVec3,
    /// Shader uniform type: ivec4 (4 int)
    IVec4,
    /// Shader uniform type: sampler2d
    Sampler2D,
}

/// Shader attribute data types
pub enum rlShaderAttributeDataType {
    /// Shader attribute type: float
    Float,
    /// Shader attribute type: vec2 (2 float)
    Vec2,
    /// Shader attribute type: vec3 (3 float)
    Vec3,
    /// Shader attribute type: vec4 (4 float)
    Vec4,
}

/// Framebuffer attachment type
/// NOTE: By default up to 8 color channels defined, but it can be more
pub enum rlFramebufferAttachType {
    /// Framebuffer attachment type: color 0
    ColorChannel0 = 0,
    /// Framebuffer attachment type: color 1
    ColorChannel1 = 1,
    /// Framebuffer attachment type: color 2
    ColorChannel2 = 2,
    /// Framebuffer attachment type: color 3
    ColorChannel3 = 3,
    /// Framebuffer attachment type: color 4
    ColorChannel4 = 4,
    /// Framebuffer attachment type: color 5
    ColorChannel5 = 5,
    /// Framebuffer attachment type: color 6
    ColorChannel6 = 6,
    /// Framebuffer attachment type: color 7
    ColorChannel7 = 7,
    /// Framebuffer attachment type: depth
    Depth = 100,
    /// Framebuffer attachment type: stencil
    Stencil = 200,
}

/// Framebuffer texture attachment type
pub enum rlFramebufferAttachTextureType {
    /// Framebuffer texture attachment type: cubemap, +X side
    CubemapPositiveX = 0,
    /// Framebuffer texture attachment type: cubemap, -X side
    CubemapNegativeX = 1,
    /// Framebuffer texture attachment type: cubemap, +Y side
    CubemapPositiveY = 2,
    /// Framebuffer texture attachment type: cubemap, -Y side
    CubemapNegativeY = 3,
    /// Framebuffer texture attachment type: cubemap, +Z side
    CubemapPositiveZ = 4,
    /// Framebuffer texture attachment type: cubemap, -Z side
    CubemapNegativeZ = 5,
    /// Framebuffer texture attachment type: texture2d
    Texture2d = 100,
    /// Framebuffer texture attachment type: renderbuffer
    Renderbuffer = 200,
}

/// Face culling mode
pub enum rlCullMode {
    Front,
    Back,
}

/// Dynamic vertex buffers (position + texcoords + colors + indices arrays)
struct rlVertexBuffer {
    /// Number of elements in the buffer (QUADS)
    element_count: i32,

    /// Vertex position (XYZ - 3 components per vertex) (shader-location = 0)
    vertices: *mut f32,
    /// Vertex texture coordinates (UV - 2 components per vertex) (shader-location = 1)
    texcoords: *mut f32,
    /// Vertex colors (RGBA - 4 components per vertex) (shader-location = 3)
    colors: *mut u8,
    #[cfg(any(feature = "graphics_api_opengl_11", feature = "graphics_api_opengl_33"))]
    /// Vertex indices (in case vertex data comes indexed) (6 indices per quad)
    indices: *mut u32,
    #[cfg(feature = "graphics_api_opengl_es2")]
    /// Vertex indices (in case vertex data comes indexed) (6 indices per quad)
    indices: *mut u16,
    /// OpenGL Vertex Array Object id
    vao_id: u32,
    /// OpenGL Vertex Buffer Objects id (4 types of vertex data)
    vbo_id: [u32; 4],
}

/// Draw call type
/// NOTE: Only texture changes register a new draw, other state-change-related elements are not
/// used at this moment (vaoId, shaderId, matrices), raylib just forces a batch draw call if any
/// of those state-change happens (this is done in core module)
struct rlDrawCall {
    /// Drawing mode: LINES, TRIANGLES, QUADS
    mode: i32,
    /// Number of vertex of the draw
    vertex_count: i32,
    /// Number of vertex required for index alignment (LINES, TRIANGLES)
    vertex_alignment: i32,
    // /// Vertex array id to be used on the draw -> Using RLGL.currentBatch->vertexBuffer.vaoId
    // vao_id: u32,
    // /// Shader id to be used on the draw -> Using RLGL.currentShaderId
    // shader_id: u32,
    /// Texture id to be used on the draw -> Use to create new draw call if changes
    texture_id: u32,

    // /// Projection matrix for this draw -> Using RLGL.projection by default
    // projection: Matrix,
    // /// Modelview matrix for this draw -> Using RLGL.modelview by default
    // modelview: Matrix,
}

// rlRenderBatch type
struct rlRenderBatch {
    /// Number of vertex buffers (multi-buffering support)
    buffer_count: i32,
    /// Current buffer tracking in case of multi-buffering
    current_buffer: i32,
    /// Dynamic buffer(s) for vertex data
    vertex_buffer: *mut rlVertexBuffer,

    /// Draw calls array, depends on textureId
    draws: *mut rlDrawCall,
    /// Draw calls counter
    draw_counter: i32,
    /// Current depth value for next draw
    current_depth: f32,
}

#[derive(Debug, Default)]
struct State {
    /// Current active render batch vertex counter (generic, used for all batches)
    vertex_counter: i32,
    /// Current active texture coordinate (added on glVertex*())
    texcoordx: f32, texcoordy: f32,
    /// Current active normal (added on glVertex*())
    normalx: f32, normaly: f32, normalz: f32,
    /// Current active color (added on glVertex*())
    colorr: u8, colorg: u8, colorb: u8, colora: u8,

    /// Current matrix mode
    current_matrix_mode: i32,
    /// Current matrix pointer
    current_matrix: *mut Matrix,
    /// Default modelview matrix
    modelview: Matrix,
    /// Default projection matrix
    projection: Matrix,
    /// Transform matrix to be used with rlTranslate, rlRotate, rlScale
    transform: Matrix,
    /// Require transform matrix application to current draw-call vertex (if required)
    transform_required: bool,
    /// Matrix stack for push/pop
    stack: Vec<Matrix>,

    /// Default texture used on shapes/poly drawing (required by shader)
    default_texture_id: u32,
    /// Active texture ids to be enabled on batch drawing (0 active by default)
    active_texture_id: [u32; RL_DEFAULT_BATCH_MAX_TEXTURE_UNITS],
    /// Default vertex shader id (used by default shader program)
    default_v_shader_id: u32,
    /// Default fragment shader id (used by default shader program)
    default_f_shader_id: u32,
    /// Default shader program id, supports vertex color and diffuse texture
    default_shader_id: u32,
    /// Default shader locations pointer to be used on rendering
    default_shader_locs: *mut i32,
    /// Current shader id to be used on rendering (by default, defaultShaderId)
    current_shader_id: u32,
    /// Current shader locations pointer to be used on rendering (by default, defaultShaderLocs)
    current_shader_locs: *mut i32,

    /// Stereo rendering flag
    stereo_render: bool,
    /// VR stereo rendering eyes projection matrices
    projection_stereo: [Matrix; 2],
    /// VR stereo rendering eyes view offset matrices
    view_offset_stereo: [Matrix; 2],

    // Blending variables
    /// Blending mode active
    current_blend_mode: i32,
    /// Blending source factor
    gl_blend_src_factor: i32,
    /// Blending destination factor
    gl_blend_dst_factor: i32,
    /// Blending equation
    gl_blend_equation: i32,
    /// Blending source RGB factor
    gl_blend_src_factor_rgb: i32,
    /// Blending destination RGB factor
    gl_blend_dest_factor_rgb: i32,
    /// Blending source alpha factor
    gl_blend_src_factor_alpha: i32,
    /// Blending destination alpha factor
    gl_blend_dest_factor_alpha: i32,
    /// Blending equation for RGB
    gl_blend_equation_rgb: i32,
    /// Blending equation for alpha
    gl_blend_equation_alpha: i32,
    /// Custom blending factor and equation modification status
    gl_custom_blend_mode_modified: bool,

    /// Current framebuffer width
    framebuffer_width: i32,
    /// Current framebuffer height
    framebuffer_height: i32,

}

#[derive(Debug, Default)]
struct ExtSupported {
    /// VAO support (OpenGL ES2 could not support VAO extension) (GL_ARB_vertex_array_object)
    vao: bool,
    /// Instancing supported (GL_ANGLE_instanced_arrays, GL_EXT_draw_instanced + GL_EXT_instanced_arrays)
    instancing: bool,
    /// NPOT textures full support (GL_ARB_texture_non_power_of_two, GL_OES_texture_npot)
    tex_npot: bool,
    /// Depth textures supported (GL_ARB_depth_texture, GL_OES_depth_texture)
    tex_depth: bool,
    /// Depth textures supported WebGL specific (GL_WEBGL_depth_texture)
    tex_depth_web_gl: bool,
    /// float textures support (32 bit per channel) (GL_OES_texture_float)
    tex_float32: bool,
    /// half float textures support (16 bit per channel) (GL_OES_texture_half_float)
    tex_float16: bool,
    /// DDS texture compression support (GL_EXT_texture_compression_s3tc, GL_WEBGL_compressed_texture_s3tc, GL_WEBKIT_WEBGL_compressed_texture_s3tc)
    tex_comp_dxt: bool,
    /// ETC1 texture compression support (GL_OES_compressed_ETC1_RGB8_texture, GL_WEBGL_compressed_texture_etc1)
    tex_comp_etc1: bool,
    /// ETC2/EAC texture compression support (GL_ARB_ES3_compatibility)
    tex_comp_etc2: bool,
    /// PVR texture compression support (GL_IMG_texture_compression_pvrtc)
    tex_comp_pvrt: bool,
    /// ASTC texture compression support (GL_KHR_texture_compression_astc_hdr, GL_KHR_texture_compression_astc_ldr)
    tex_comp_astc: bool,
    /// Clamp mirror wrap mode supported (GL_EXT_texture_mirror_clamp)
    tex_mirror_clamp: bool,
    /// Anisotropic texture filtering support (GL_EXT_texture_filter_anisotropic)
    tex_aniso_filter: bool,
    /// Compute shaders support (GL_ARB_compute_shader)
    compute_shader: bool,
    /// Shader storage buffer object support (GL_ARB_shader_storage_buffer_object)
    ssbo: bool,

    /// Maximum anisotropy level supported (minimum is 2.0f)
    max_anisotropy_level: f32,
    /// Maximum bits for depth component
    max_depth_bits: i32,

}

#[derive(Debug, Default)]
struct rlglData {
    /// Current render batch
    current_batch: *mut rlRenderBatch,
    /// Default internal render batch
    default_batch: rlRenderBatch,
    /// Renderer state
    state: State,
    /// Extensions supported flags
    ext_supported: ExtSupported,
}

/// Initialize rlgl: OpenGL extensions, default buffers/shaders/textures, OpenGL states
fn rlglInit(width: u32, height: u32) -> rlglData {
    let mut rlgl = rlglData::default();

    // Enable OpenGL debug context if required
    if cfg!(all(feature = "rlgl_enable_opengl_debug_context", feature = "graphics_api_opengl_43")) {
        if glDebugMessageCallback.is_some() && glDebugMessageControl.is_some() {
            glDebugMessageCallback(rlDebugMessageCallback, 0);
            // glDebugMessageControl(GL_DEBUG_SOURCE_API, GL_DEBUG_TYPE_ERROR, GL_DEBUG_SEVERITY_HIGH, 0, 0, GL_TRUE);

            // Debug context options:
            //  - GL_DEBUG_OUTPUT - Faster version but not useful for breakpoints
            //  - GL_DEBUG_OUTPUT_SYNCHRONUS - Callback is in sync with errors, so a breakpoint can be placed on the callback in order to get a stacktrace for the GL error
            glEnable(GL_DEBUG_OUTPUT);
            glEnable(GL_DEBUG_OUTPUT_SYNCHRONOUS);
        }
    }

    if cfg!(any(feature = "graphics_api_opengl_33", feature = "graphics_api_opengl_es2")) {
        // Init default white texture
        let pixels = [255, 255, 255, 255];   // 1 pixel RGBA (4 bytes)
        rlgl.state.default_texture_id = rlLoadTexture(pixels, 1, 1, RL_PIXELFORMAT_UNCOMPRESSED_R8G8B8A8, 1);

        if rlgl.state.default_texture_id != 0 { tracelog!(RL_LOG_INFO, "TEXTURE: [ID %i] Default texture loaded successfully", rlgl.state.default_texture_id); }
        else { tracelog!(RL_LOG_WARNING, "TEXTURE: Failed to load default texture"); }

        // Init default Shader (customized for GL 3.3 and ES2)
        // Loaded: rlgl.state.defaultShaderId + rlgl.state.defaultShaderLocs
        rlLoadShaderDefault();
        rlgl.state.current_shader_id = rlgl.state.default_shader_id;
        rlgl.state.current_shader_locs = rlgl.state.default_shader_locs;

        // Init default vertex arrays buffers
        rlgl.sefault_batch = rlLoadRenderBatch(RL_DEFAULT_BATCH_BUFFERS, RL_DEFAULT_BATCH_BUFFER_ELEMENTS);
        rlgl.surrent_batch = &rlgl.sefault_batch;

        // Init stack matrices (emulating OpenGL 1.1)
        for i in 0..RL_MAX_MATRIX_STACK_SIZE { rlgl.state.stack[i] = rlMatrixIdentity(); }

        // Init internal matrices
        rlgl.state.transform = rlMatrixIdentity();
        rlgl.state.projection = rlMatrixIdentity();
        rlgl.state.modelview = rlMatrixIdentity();
        rlgl.state.current_matrix = &rlgl.state.modelview;
    } // GRAPHICS_API_OPENGL_33 || GRAPHICS_API_OPENGL_ES2

    // Initialize OpenGL default states
    //----------------------------------------------------------
    // Init state: Depth test
    glDepthFunc(GL_LEQUAL);                                 // Type of depth testing to apply
    glDisable(GL_DEPTH_TEST);                               // Disable depth testing for 2D (only used for 3D)

    // Init state: Blending mode
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);      // Color blending function (how colors are mixed)
    glEnable(GL_BLEND);                                     // Enable color blending (required to work with transparencies)

    // Init state: Culling
    // NOTE: All shapes/models triangles are drawn CCW
    glCullFace(GL_BACK);                                    // Cull the back face (default)
    glFrontFace(GL_CCW);                                    // Front face are defined counter clockwise (default)
    glEnable(GL_CULL_FACE);                                 // Enable backface culling

    // Init state: Cubemap seamless
    if cfg!(feature = "graphics_api_opengl_33") {
        glEnable(GL_TEXTURE_CUBE_MAP_SEAMLESS);             // Seamless cubemaps (not supported on OpenGL ES 2.0)
    }

    if cfg!(feature = "graphics_api_opengl_11") {
        // Init state: Color hints (deprecated in OpenGL 3.0+)
        glHint(GL_PERSPECTIVE_CORRECTION_HINT, GL_NICEST);  // Improve quality of color and texture coordinate interpolation
        glShadeModel(GL_SMOOTH);                            // Smooth shading between vertex (vertex colors interpolation)
    }

    if cfg!(feature = "GRAPHICS_API_OPENGL_33", feature = "GRAPHICS_API_OPENGL_ES2") {
        // Store screen size into global variables
        rlgl.state.framebuffer_width = width;
        rlgl.state.framebuffer_height = height;

        tracelog!(RL_LOG_INFO, "RLGL: Default OpenGL state initialized successfully");
        //----------------------------------------------------------
    }

    // Init state: Color/Depth buffers clear
    glClearColor(0.0f, 0.0f, 0.0f, 1.0f);                   // Set clear color (black)
    glClearDepth(1.0f);                                     // Set clear depth value (default)
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);     // Clear color and depth buffers (depth buffer required for 3D)

    rlgl
}

// Vertex Buffer Object deinitialization (memory free)
fn rlglClose() {
    if cfg!(any(feature = "graphics_api_opengl_33", feature = "graphics_api_opengl_es2")) {
        rlUnloadRenderBatch(rlgl.default_batch);

        rlUnloadShaderDefault();          // Unload default shader

        glDeleteTextures(1, &rlgl.State.defaultTextureId); // Unload default texture
        tracelog!(RL_LOG_INFO, "TEXTURE: [ID %i] Default texture unloaded successfully", rlgl.State.defaultTextureId);
    }
}

// Choose the current matrix to be transformed
fn rlMatrixMode(rlgl: &mut rlglData, mode: i32) {
    if mode == RL_PROJECTION { rlgl.state.current_matrix = &mut rlgl.state.projection as *mut _; }
    else if mode == RL_MODELVIEW { rlgl.state.current_matrix = &mut rlgl.state.modelview as *mut _; }
    // else if mode == RL_TEXTURE // Not supported

    rlgl.state.current_matrix_mode = mode;
}
