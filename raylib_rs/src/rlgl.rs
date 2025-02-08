use std::ptr::{null, null_mut};
use crate::{external::glad::*, tracelog, tracelogd, utils::TraceLogLevel::*, Matrix};

// Default internal render batch elements limits

#[cfg(any(graphics_api = "opengl_11", graphics_api = "opengl_33"))]
/// This is the maximum amount of elements (quads) per batch
/// NOTE: Be careful with text, every letter maps to a quad
const RL_DEFAULT_BATCH_BUFFER_ELEMENTS: usize = 8192;

#[cfg(graphics_api = "opengl_es2")]
/// We reduce memory sizes for embedded systems (RPI and HTML5)
/// NOTE: On HTML5 (emscripten) this is allocated on heap,
/// by default it's only 16MB!...just take care...
const RL_DEFAULT_BATCH_BUFFER_ELEMENTS: usize = 2048;

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

/// Matrix modes (equivalent to OpenGL)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum MatrixMode {
    /// GL_MODELVIEW
    #[default]
    Modelview = 0x1700,
    /// GL_PROJECTION
    Projection = 0x1701,
    /// GL_TEXTURE
    Texture = 0x1702,
}

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
/// GL_BLEND_EQUATION_RGB (Same as BLEND_EQUATION)
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

/// GL_READ_FRAMEBUFFER
const RL_READ_FRAMEBUFFER: i32 = 0x8CA8;
/// GL_DRAW_FRAMEBUFFER
const RL_DRAW_FRAMEBUFFER: i32 = 0x8CA9;

/// Default shader vertex attribute locations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum DefaultShaderAttribLocation {
    Position = 0,
    TexCoord = 1,
    Normal = 2,
    Color = 3,

    Tangent = 4,
    TexCoord2 = 5,
    Indices = 6,
    #[cfg(feature = "mesh_gpu_skinning")]
    BoneIds = 7,
    #[cfg(feature = "mesh_gpu_skinning")]
    BoneWeights = 8,
}

/// OpenGL version
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GlVersion {
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

const GRAPHICS_API: GlVersion = cfg_match! {{
    graphics_api = "opengl_11"  => { GlVersion::Opengl11 }
    graphics_api = "opengl_21"  => { GlVersion::Opengl21 }
    graphics_api = "opengl_33"  => { GlVersion::Opengl33 }
    graphics_api = "opengl_43"  => { GlVersion::Opengl43 }
    graphics_api = "opengl_es2" => { GlVersion::OpenglEs20 }
    graphics_api = "opengl_es3" => { GlVersion::OpenglEs30 }
    _ => { panic!("invalid graphics_api") }
}};

/// Texture pixel formats
/// NOTE: Support depends on OpenGL version
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PixelFormat {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureFilter {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendMode {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderLocationIndex {
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
impl ShaderLocationIndex {
    pub const MapDiffuse: Self = Self::MapAlbedo;
    pub const MapSpecular: Self = Self::MapMetalness;
}

/// Shader uniform data type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderUniformDataType {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderAttributeDataType {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FramebufferAttachType {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FramebufferAttachTextureType {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CullMode {
    Front,
    Back,
}

const GL_SHADING_LANGUAGE_VERSION:         u32 = 0x8B8C;

const GL_COMPRESSED_RGB_S3TC_DXT1_EXT:     u32 = 0x83F0;
const GL_COMPRESSED_RGBA_S3TC_DXT1_EXT:    u32 = 0x83F1;
const GL_COMPRESSED_RGBA_S3TC_DXT3_EXT:    u32 = 0x83F2;
const GL_COMPRESSED_RGBA_S3TC_DXT5_EXT:    u32 = 0x83F3;
const GL_ETC1_RGB8_OES:                    u32 = 0x8D64;
const GL_COMPRESSED_RGB8_ETC2:             u32 = 0x9274;
const GL_COMPRESSED_RGBA8_ETC2_EAC:        u32 = 0x9278;
const GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG:  u32 = 0x8C00;
const GL_COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: u32 = 0x8C02;
#[allow(non_upper_case_globals)]
const GL_COMPRESSED_RGBA_ASTC_4x4_KHR:     u32 = 0x93b0;
#[allow(non_upper_case_globals)]
const GL_COMPRESSED_RGBA_ASTC_8x8_KHR:     u32 = 0x93b7;

const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT:   u32 = 0x84FF;
const GL_TEXTURE_MAX_ANISOTROPY_EXT:       u32 = 0x84FE;

const GL_PROGRAM_POINT_SIZE:               u32 = 0x8642;

const GL_LINE_WIDTH:                       u32 = 0x0B21;

#[cfg(graphics_api = "opengl_11")]
const GL_UNSIGNED_SHORT_5_6_5:   u32 = 0x8363;
#[cfg(graphics_api = "opengl_11")]
const GL_UNSIGNED_SHORT_5_5_5_1: u32 = 0x8034;
#[cfg(graphics_api = "opengl_11")]
const GL_UNSIGNED_SHORT_4_4_4_4: u32 = 0x8033;

#[cfg(graphics_api = "opengl_21")]
const GL_LUMINANCE: u32 = 0x1909;
#[cfg(graphics_api = "opengl_21")]
const GL_LUMINANCE_ALPHA: u32 = 0x190A;

#[cfg(graphics_api = "opengl_es2")]
use glClearDepthf as glClearDepth;
#[cfg(all(graphics_api = "opengl_es2", not(graphics_api = "opengl_es3")))]
const GL_READ_FRAMEBUFFER: u32 = GL_FRAMEBUFFER;
#[cfg(all(graphics_api = "opengl_es2", not(graphics_api = "opengl_es3")))]
const GL_DRAW_FRAMEBUFFER: u32 = GL_FRAMEBUFFER;

// Default shader vertex attribute names to set location points
/// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_NAME_POSITION
const RL_DEFAULT_SHADER_ATTRIB_NAME_POSITION:       &'static str = "vertexPosition";
/// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_NAME_TEXCOORD
const RL_DEFAULT_SHADER_ATTRIB_NAME_TEXCOORD:       &'static str = "vertexTexCoord";
/// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_NAME_NORMAL
const RL_DEFAULT_SHADER_ATTRIB_NAME_NORMAL:         &'static str = "vertexNormal";
/// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_NAME_COLOR
const RL_DEFAULT_SHADER_ATTRIB_NAME_COLOR:          &'static str = "vertexColor";
/// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_NAME_TANGENT
const RL_DEFAULT_SHADER_ATTRIB_NAME_TANGENT:        &'static str = "vertexTangent";
/// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_NAME_TEXCOORD2
const RL_DEFAULT_SHADER_ATTRIB_NAME_TEXCOORD2:      &'static str = "vertexTexCoord2";
/// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_NAME_BONEIDS
const RL_DEFAULT_SHADER_ATTRIB_NAME_BONEIDS:        &'static str = "vertexBoneIds";
/// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_NAME_BONEWEIGHTS
const RL_DEFAULT_SHADER_ATTRIB_NAME_BONEWEIGHTS:    &'static str = "vertexBoneWeights";

/// model-view-projection matrix
const RL_DEFAULT_SHADER_UNIFORM_NAME_MVP:           &'static str = "mvp";
/// view matrix
const RL_DEFAULT_SHADER_UNIFORM_NAME_VIEW:          &'static str = "matView";
/// projection matrix
const RL_DEFAULT_SHADER_UNIFORM_NAME_PROJECTION:    &'static str = "matProjection";
/// model matrix
const RL_DEFAULT_SHADER_UNIFORM_NAME_MODEL:         &'static str = "matModel";
/// normal matrix (transpose(inverse(matModelView))
const RL_DEFAULT_SHADER_UNIFORM_NAME_NORMAL:        &'static str = "matNormal";
/// color diffuse (base tint color, multiplied by texture color)
const RL_DEFAULT_SHADER_UNIFORM_NAME_COLOR:         &'static str = "colDiffuse";
/// bone matrices
const RL_DEFAULT_SHADER_UNIFORM_NAME_BONE_MATRICES: &'static str = "boneMatrices";
/// texture0 (texture slot active 0)
const RL_DEFAULT_SHADER_SAMPLER2D_NAME_TEXTURE0:    &'static str = "texture0";
/// texture1 (texture slot active 1)
const RL_DEFAULT_SHADER_SAMPLER2D_NAME_TEXTURE1:    &'static str = "texture1";
/// texture2 (texture slot active 2)
const RL_DEFAULT_SHADER_SAMPLER2D_NAME_TEXTURE2:    &'static str = "texture2";

/// Dynamic vertex buffers (position + texcoords + colors + indices arrays)
#[derive(Debug, Default)]
struct VertexBuffer<'a> {
    /// Number of elements in the buffer (QUADS)
    element_count: i32,

    /// Vertex position (XYZ - 3 components per vertex) (shader-location = 0)
    vertices: Option<&'a mut [f32]>,
    /// Vertex texture coordinates (UV - 2 components per vertex) (shader-location = 1)
    texcoords: Option<&'a mut [f32]>,
    /// Vertex colors (RGBA - 4 components per vertex) (shader-location = 3)
    colors: Option<&'a mut [u8]>,
    #[cfg(any(graphics_api = "opengl_11", graphics_api = "opengl_33"))]
    /// Vertex indices (in case vertex data comes indexed) (6 indices per quad)
    indices: Option<&'a mut [u32]>,
    #[cfg(graphics_api = "opengl_es2")]
    /// Vertex indices (in case vertex data comes indexed) (6 indices per quad)
    indices: Option<&'a mut [u16]>,
    /// OpenGL Vertex Array Object id
    vao_id: u32,
    /// OpenGL Vertex Buffer Objects id (4 types of vertex data)
    vbo_id: [u32; 4],
}

/// Draw call type
/// NOTE: Only texture changes register a new draw, other state-change-related elements are not
/// used at this moment (vaoId, shaderId, matrices), raylib just forces a batch draw call if any
/// of those state-change happens (this is done in core module)
#[derive(Debug, Default)]
struct DrawCall {
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
#[derive(Debug, Default)]
struct RenderBatch<'a> {
    /// Number of vertex buffers (multi-buffering support)
    buffer_count: i32,
    /// Current buffer tracking in case of multi-buffering
    current_buffer: i32,
    /// Dynamic buffer(s) for vertex data
    vertex_buffer: Option<&'a mut [VertexBuffer<'a>]>,

    /// Draw calls array, depends on textureId
    draws: Option<&'a mut [DrawCall]>,
    /// Draw calls counter
    draw_counter: i32,
    /// Current depth value for next draw
    current_depth: f32,
}

#[derive(Debug, Default)]
struct State<'a> {
    /// Current active render batch vertex counter (generic, used for all batches)
    vertex_counter: i32,
    /// Current active texture coordinate (added on glVertex*())
    texcoordx: f32, texcoordy: f32,
    /// Current active normal (added on glVertex*())
    normalx: f32, normaly: f32, normalz: f32,
    /// Current active color (added on glVertex*())
    colorr: u8, colorg: u8, colorb: u8, colora: u8,

    /// Current matrix mode
    current_matrix_mode: MatrixMode,
    /// Current matrix pointer
    current_matrix: Option<&'a mut Matrix>,
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
    default_shader_locs: Option<&'a mut [i32]>,
    /// Current shader id to be used on rendering (by default, defaultShaderId)
    current_shader_id: u32,
    /// Current shader locations pointer to be used on rendering (by default, defaultShaderLocs)
    current_shader_locs: Option<&'a mut [i32]>,

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
    framebuffer_width: u32,
    /// Current framebuffer height
    framebuffer_height: u32,
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
pub struct RlglData<'a> {
    /// Current render batch
    current_batch: Option<&'a mut RenderBatch<'a>>,
    /// Default internal render batch
    default_batch: RenderBatch<'a>,
    /// Renderer state
    state: State<'a>,
    /// Extensions supported flags
    ext_supported: ExtSupported,
}

impl Drop for RlglData<'_> {
    // Vertex Buffer Object deinitialization (memory free)
    fn drop(&mut self) {
        if cfg!(any(graphics_api = "opengl_33", graphics_api = "opengl_es2")) {
            self.unload_render_batch(self.default_batch);

            self.unload_shader_default();          // Unload default shader

            unsafe { glDeleteTextures(1, &mut self.state.default_texture_id); } // Unload default texture
            tracelog!(LogInfo, "TEXTURE: [ID {}] Default texture unloaded successfully", self.state.default_texture_id);
        }
    }
}

impl<'a> RlglData<'a> {
    /// Initialize rlgl: OpenGL extensions, default buffers/shaders/textures, OpenGL states
    pub fn init(width: u32, height: u32) -> Self {
        let mut rlgl = Self::default();

        if cfg!(any(graphics_api = "opengl_33", graphics_api = "opengl_es2")) {
            // Init default white texture
            let pixels: [u8; 4] = [255, 255, 255, 255]; // 1 pixel RGBA (4 bytes)
            let default_texture_id = rlgl.load_texture(&mut pixels, 1, 1, PixelFormat::UncompressedR8G8B8A8, 1);

        } // GRAPHICS_API_OPENGL_33 || GRAPHICS_API_OPENGL_ES2

        todo!();

        rlgl
    }

    /// Textures data management
    ///-----------------------------------------------------------------------------------------
    /// Convert image data to OpenGL texture (returns OpenGL valid Id)
    pub fn load_texture(&mut self, data: Option<&[u8]>, width: u32, height: u32, format: PixelFormat, mipmap_count: u32) -> Option<u32> {
        unsafe { glBindTexture(GL_TEXTURE_2D, 0); } // Free any old binding

        // Check texture format support by OpenGL 1.1 (compressed textures not supported)
        if cfg!(graphics_api = "opengl_11") {
            if format >= PixelFormat::CompressedDXT1RGB {
                tracelog!(LogWarning, "GL: OpenGL 1.1 does not support GPU compressed texture formats");
                return None;
            }
        } else {
            if !self.ext_supported.tex_comp_dxt && matches!(format,
                | PixelFormat::CompressedDXT1RGB
                | PixelFormat::CompressedDXT1RGBA
                | PixelFormat::CompressedDXT3RGBA
                | PixelFormat::CompressedDXT5RGBA
            ) {
                tracelog!(LogWarning, "GL: DXT compressed texture format not supported");
                return None;
            }

            if cfg!(any(graphics_api = "opengl_33", graphics_api = "opengl_es2")) {
                if !self.ext_supported.tex_comp_etc1 && matches!(format, PixelFormat::CompressedETC1RGB) {
                    tracelog!(LogWarning, "GL: ETC1 compressed texture format not supported");
                    return None;
                }

                if !self.ext_supported.tex_comp_etc2 && matches!(format, PixelFormat::CompressedETC2RGB | PixelFormat::CompressedETC2EACRGBA) {
                    tracelog!(LogWarning, "GL: ETC2 compressed texture format not supported");
                    return None;
                }

                if !self.ext_supported.tex_comp_pvrt && matches!(format, PixelFormat::CompressedPVRTRGB | PixelFormat::CompressedPVRTRGBA) {
                    tracelog!(LogWarning, "GL: PVRT compressed texture format not supported");
                    return None;
                }

                if !self.ext_supported.tex_comp_astc && matches!(format, PixelFormat::CompressedASTC4x4RGBA | PixelFormat::CompressedASTC8x8RGBA) {
                    tracelog!(LogWarning, "GL: ASTC compressed texture format not supported");
                    return None;
                }
            }
        } // GRAPHICS_API_OPENGL_11

        let mut id: u32 = 0;
        unsafe {
            glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
            glGenTextures(1, &mut id); // Generate texture id
            glBindTexture(GL_TEXTURE_2D, id);
        }

        let mut mip_width = width;
        let mut mip_height = height;
        let mut mip_offset = 0; // Mipmap data offset, only used for tracelog

        // NOTE: Added pointer math separately from function to avoid UBSAN complaining
        let data_ptr = if let Some(data) = data { data.as_ptr() } else { null() };

        // Load the different mipmap levels
        for i in 0..mipmap_count {
            let mip_size = get_pixel_data_size(mip_width, mip_height, format);

            let (gl_internal_format, gl_format, gl_type) = get_gl_texture_formats(&self.ext_supported, format);

            tracelogd!("TEXTURE: Load mipmap level {} ({} x {}), size: {}, offset: {}", i, mip_width, mip_height, mip_size, mip_offset);

            if gl_internal_format != 0 {
                unsafe {
                    if format < PixelFormat::CompressedDXT1RGB { glTexImage2D(GL_TEXTURE_2D, i, gl_internal_format, mip_width, mip_height, 0, gl_format, gl_type, data_ptr.cast()); }
                    else if cfg!(graphics_api = "opengl_11") { glCompressedTexImage2D(GL_TEXTURE_2D, i, gl_internal_format, mip_width, mip_height, 0, mip_size, data_ptr.cast()); }

                    #[cfg(graphics_api = "opengl_33")] {
                        if format == PixelFormat::UncompressedGrayscale {
                            let swizzel_mask: [GLint; 4] = [GL_RED, GL_RED, GL_RED, GL_ONE];
                            glTexParameteriv(GL_TEXTURE_2D, GL_TEXTURE_SWIZZLE_RGBA, swizzle_mask);
                        } else if format == PixelFormat::UncompressedGrayAlpha {
                            let swizzel_mask: [GLint; 4] = if cfg!(graphics_api = "opengl_21") {
                                [GL_RED, GL_RED, GL_RED, GL_ALPHA]
                            } else {
                                [GL_RED, GL_RED, GL_RED, GL_GREEN]
                            };
                            glTexParameteriv(GL_TEXTURE_2D, GL_TEXTURE_SWIZZLE_RGBA, swizzle_mask);
                        }
                    }
                }
            }

            mip_width  /= 2;
            mip_height /= 2;
            mip_offset += mip_size; // Increment offset position to next mipmap
            if data.is_some() { data_ptr += mip_size; } // Increment data pointer to next mipmap

            // Security check for NPOT textures
            if mip_width  < 1 { mip_width  = 1; }
            if mip_height < 1 { mip_height = 1; }
        }

        todo!()
    }


    // Choose the current matrix to be transformed
    pub fn matrix_mode(&'a mut self, mode: MatrixMode) {
        let rlgl = self;
        match mode {
            MatrixMode::Projection => rlgl.state.current_matrix = Some(&mut rlgl.state.projection),
            MatrixMode::Modelview  => rlgl.state.current_matrix = Some(&mut rlgl.state.modelview ),
            MatrixMode::Texture => unimplemented!("Not supported"),
        }

        rlgl.state.current_matrix_mode = mode;
    }
}

/// Get OpenGL internal formats and data type from raylib PixelFormat
///
/// Returns `(gl_internal_format, gl_format, gl_type)`
pub const fn get_gl_texture_formats(ext_supported: &ExtSupported, format: PixelFormat) -> (u32, u32, u32) {
    use {PixelFormat::*, GlVersion::*};
    match (GRAPHICS_API, format) {
        // NOTE: on OpenGL ES 2.0 (WebGL), internalFormat must match format and options allowed are: GL_LUMINANCE, GL_RGB, GL_RGBA
        #[cfg(any(graphics_api = "opengl_11", graphics_api = "opengl_21", graphics_api = "opengl_es2"))] UncompressedGrayscale    =>                                  return (GL_LUMINANCE,                        GL_LUMINANCE,       GL_UNSIGNED_BYTE         ),
        #[cfg(any(graphics_api = "opengl_11", graphics_api = "opengl_21", graphics_api = "opengl_es2"))] UncompressedGrayAlpha    =>                                  return (GL_LUMINANCE_ALPHA,                  GL_LUMINANCE_ALPHA, GL_UNSIGNED_BYTE         ),
        #[cfg(any(graphics_api = "opengl_11", graphics_api = "opengl_21", graphics_api = "opengl_es2"))] UncompressedR5G6B5       =>                                  return (GL_RGB,                              GL_RGB,             GL_UNSIGNED_SHORT_5_6_5  ),
        #[cfg(any(graphics_api = "opengl_11", graphics_api = "opengl_21", graphics_api = "opengl_es2"))] UncompressedR8G8B8       =>                                  return (GL_RGB,                              GL_RGB,             GL_UNSIGNED_BYTE         ),
        #[cfg(any(graphics_api = "opengl_11", graphics_api = "opengl_21", graphics_api = "opengl_es2"))] UncompressedR5G5B5A1     =>                                  return (GL_RGBA,                             GL_RGBA,            GL_UNSIGNED_SHORT_5_5_5_1),
        #[cfg(any(graphics_api = "opengl_11", graphics_api = "opengl_21", graphics_api = "opengl_es2"))] UncompressedR4G4B4A4     =>                                  return (GL_RGBA,                             GL_RGBA,            GL_UNSIGNED_SHORT_4_4_4_4),
        #[cfg(any(graphics_api = "opengl_11", graphics_api = "opengl_21", graphics_api = "opengl_es2"))] UncompressedR8G8B8A8     =>                                  return (GL_RGBA,                             GL_RGBA,            GL_UNSIGNED_BYTE         ),
        #[cfg(    graphics_api = "opengl_es3"                                                         )] UncompressedR32          => if ext_supported.tex_float32   { return (GL_R32F_EXT,                         GL_RED_EXT,         GL_FLOAT                 ) },
        #[cfg(    graphics_api = "opengl_es3"                                                         )] UncompressedR32G32B32    => if ext_supported.tex_float32   { return (GL_RGB32F_EXT,                       GL_RGB,             GL_FLOAT                 ) },
        #[cfg(    graphics_api = "opengl_es3"                                                         )] UncompressedR32G32B32A32 => if ext_supported.tex_float32   { return (GL_RGBA32F_EXT,                      GL_RGBA,            GL_FLOAT                 ) },
        #[cfg(    graphics_api = "opengl_es3"                                                         )] UncompressedR16          => if ext_supported.tex_float16   { return (GL_R16F_EXT,                         GL_RED_EXT,         GL_HALF_FLOAT            ) },
        #[cfg(    graphics_api = "opengl_es3"                                                         )] UncompressedR16G16B16    => if ext_supported.tex_float16   { return (GL_RGB16F_EXT,                       GL_RGB,             GL_HALF_FLOAT            ) },
        #[cfg(    graphics_api = "opengl_es3"                                                         )] UncompressedR16G16B16A16 => if ext_supported.tex_float16   { return (GL_RGBA16F_EXT,                      GL_RGBA,            GL_HALF_FLOAT            ) },
        #[cfg(any(graphics_api = "opengl_21", graphics_api = "opengl_es2")                            )] UncompressedR32          => if ext_supported.tex_float32   { return (GL_LUMINANCE,                        GL_LUMINANCE,       GL_FLOAT                 ) }, // NOTE: Requires extension OES_texture_float
        #[cfg(any(graphics_api = "opengl_21", graphics_api = "opengl_es2")                            )] UncompressedR32G32B32    => if ext_supported.tex_float32   { return (GL_RGB,                              GL_RGB,             GL_FLOAT                 ) }, // NOTE: Requires extension OES_texture_float
        #[cfg(any(graphics_api = "opengl_21", graphics_api = "opengl_es2")                            )] UncompressedR32G32B32A32 => if ext_supported.tex_float32   { return (GL_RGBA,                             GL_RGBA,            GL_FLOAT                 ) }, // NOTE: Requires extension OES_texture_float
        #[cfg(    graphics_api = "opengl_21"                                                          )] UncompressedR16          => if ext_supported.tex_float16   { return (GL_LUMINANCE,                        GL_LUMINANCE,       GL_HALF_FLOAT_ARB        ) },
        #[cfg(    graphics_api = "opengl_21"                                                          )] UncompressedR16G16B16    => if ext_supported.tex_float16   { return (GL_RGB,                              GL_RGB,             GL_HALF_FLOAT_ARB        ) },
        #[cfg(    graphics_api = "opengl_21"                                                          )] UncompressedR16G16B16A16 => if ext_supported.tex_float16   { return (GL_RGBA,                             GL_RGBA,            GL_HALF_FLOAT_ARB        ) },
        #[cfg(    graphics_api = "opengl_es2"                                                         )] UncompressedR16          => if ext_supported.tex_float16   { return (GL_LUMINANCE,                        GL_LUMINANCE,       GL_HALF_FLOAT_OES        ) }, // NOTE: Requires extension OES_texture_half_float
        #[cfg(    graphics_api = "opengl_es2"                                                         )] UncompressedR16G16B16    => if ext_supported.tex_float16   { return (GL_RGB,                              GL_RGB,             GL_HALF_FLOAT_OES        ) }, // NOTE: Requires extension OES_texture_half_float
        #[cfg(    graphics_api = "opengl_es2"                                                         )] UncompressedR16G16B16A16 => if ext_supported.tex_float16   { return (GL_RGBA,                             GL_RGBA,            GL_HALF_FLOAT_OES        ) }, // NOTE: Requires extension OES_texture_half_float
        #[cfg(    graphics_api = "opengl_33"                                                          )] UncompressedGrayscale    =>                                  return (GL_R8,                               GL_RED,             GL_UNSIGNED_BYTE         ),
        #[cfg(    graphics_api = "opengl_33"                                                          )] UncompressedGrayAlpha    =>                                  return (GL_RG8,                              GL_RG,              GL_UNSIGNED_BYTE         ),
        #[cfg(    graphics_api = "opengl_33"                                                          )] UncompressedR5G6B5       =>                                  return (GL_RGB565,                           GL_RGB,             GL_UNSIGNED_SHORT_5_6_5  ),
        #[cfg(    graphics_api = "opengl_33"                                                          )] UncompressedR8G8B8       =>                                  return (GL_RGB8,                             GL_RGB,             GL_UNSIGNED_BYTE         ),
        #[cfg(    graphics_api = "opengl_33"                                                          )] UncompressedR5G5B5A1     =>                                  return (GL_RGB5_A1,                          GL_RGBA,            GL_UNSIGNED_SHORT_5_5_5_1),
        #[cfg(    graphics_api = "opengl_33"                                                          )] UncompressedR4G4B4A4     =>                                  return (GL_RGBA4,                            GL_RGBA,            GL_UNSIGNED_SHORT_4_4_4_4),
        #[cfg(    graphics_api = "opengl_33"                                                          )] UncompressedR8G8B8A8     =>                                  return (GL_RGBA8,                            GL_RGBA,            GL_UNSIGNED_BYTE         ),
        #[cfg(    graphics_api = "opengl_33"                                                          )] UncompressedR32          => if ext_supported.tex_float32   { return (GL_R32F,                             GL_RED,             GL_FLOAT                 ) },
        #[cfg(    graphics_api = "opengl_33"                                                          )] UncompressedR32G32B32    => if ext_supported.tex_float32   { return (GL_RGB32F,                           GL_RGB,             GL_FLOAT                 ) },
        #[cfg(    graphics_api = "opengl_33"                                                          )] UncompressedR32G32B32A32 => if ext_supported.tex_float32   { return (GL_RGBA32F,                          GL_RGBA,            GL_FLOAT                 ) },
        #[cfg(    graphics_api = "opengl_33"                                                          )] UncompressedR16          => if ext_supported.tex_float16   { return (GL_R16F,                             GL_RED,             GL_HALF_FLOAT            ) },
        #[cfg(    graphics_api = "opengl_33"                                                          )] UncompressedR16G16B16    => if ext_supported.tex_float16   { return (GL_RGB16F,                           GL_RGB,             GL_HALF_FLOAT            ) },
        #[cfg(    graphics_api = "opengl_33"                                                          )] UncompressedR16G16B16A16 => if ext_supported.tex_float16   { return (GL_RGBA16F,                          GL_RGBA,            GL_HALF_FLOAT            ) },
        #[cfg(not(graphics_api = "opengl_11")                                                         )] CompressedDXT1RGB        => if ext_supported.tex_comp_dxt  { return (GL_COMPRESSED_RGB_S3TC_DXT1_EXT,     0,                  0                        ) },
        #[cfg(not(graphics_api = "opengl_11")                                                         )] CompressedDXT1RGBA       => if ext_supported.tex_comp_dxt  { return (GL_COMPRESSED_RGBA_S3TC_DXT1_EXT,    0,                  0                        ) },
        #[cfg(not(graphics_api = "opengl_11")                                                         )] CompressedDXT3RGBA       => if ext_supported.tex_comp_dxt  { return (GL_COMPRESSED_RGBA_S3TC_DXT3_EXT,    0,                  0                        ) },
        #[cfg(not(graphics_api = "opengl_11")                                                         )] CompressedDXT5RGBA       => if ext_supported.tex_comp_dxt  { return (GL_COMPRESSED_RGBA_S3TC_DXT5_EXT,    0,                  0                        ) },
        #[cfg(not(graphics_api = "opengl_11")                                                         )] CompressedETC1RGB        => if ext_supported.tex_comp_etc1 { return (GL_ETC1_RGB8_OES,                    0,                  0                        ) }, // NOTE: Requires OpenGL ES 2.0 or OpenGL 4.3
        #[cfg(not(graphics_api = "opengl_11")                                                         )] CompressedETC2RGB        => if ext_supported.tex_comp_etc2 { return (GL_COMPRESSED_RGB8_ETC2,             0,                  0                        ) }, // NOTE: Requires OpenGL ES 3.0 or OpenGL 4.3
        #[cfg(not(graphics_api = "opengl_11")                                                         )] CompressedETC2EACRGBA    => if ext_supported.tex_comp_etc2 { return (GL_COMPRESSED_RGBA8_ETC2_EAC,        0,                  0                        ) }, // NOTE: Requires OpenGL ES 3.0 or OpenGL 4.3
        #[cfg(not(graphics_api = "opengl_11")                                                         )] CompressedPVRTRGB        => if ext_supported.tex_comp_pvrt { return (GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG,  0,                  0                        ) }, // NOTE: Requires PowerVR GPU
        #[cfg(not(graphics_api = "opengl_11")                                                         )] CompressedPVRTRGBA       => if ext_supported.tex_comp_pvrt { return (GL_COMPRESSED_RGBA_PVRTC_4BPPV1_IMG, 0,                  0                        ) }, // NOTE: Requires PowerVR GPU
        #[cfg(not(graphics_api = "opengl_11")                                                         )] CompressedASTC4x4RGBA    => if ext_supported.tex_comp_astc { return (GL_COMPRESSED_RGBA_ASTC_4x4_KHR,     0,                  0                        ) }, // NOTE: Requires OpenGL ES 3.1 or OpenGL 4.3
        #[cfg(not(graphics_api = "opengl_11")                                                         )] CompressedASTC8x8RGBA    => if ext_supported.tex_comp_astc { return (GL_COMPRESSED_RGBA_ASTC_8x8_KHR,     0,                  0                        ) }, // NOTE: Requires OpenGL ES 3.1 or OpenGL 4.3
        _ => tracelog!(LogWarning, "TEXTURE: Current format not supported ({})", format),
    }
    (0, 0, 0)
}

/// Get pixel data size in bytes (image or texture)
/// NOTE: Size depends on pixel format
pub const fn get_pixel_data_size(width: u32, height: u32, format: PixelFormat) -> u32 {
    const COMPRESSED_DXT1RGB:     isize = PixelFormat::CompressedDXT1RGB     as isize;
    const COMPRESSED_DXT3RGBA:    isize = PixelFormat::CompressedDXT3RGBA    as isize;
    const COMPRESSED_ASTC8X8RGBA: isize = PixelFormat::CompressedASTC8x8RGBA as isize;

    // Bits per pixel
    let bpp: u32 = match format {
        PixelFormat::UncompressedGrayscale => 8,
        PixelFormat::UncompressedGrayAlpha |
        PixelFormat::UncompressedR5G6B5 |
        PixelFormat::UncompressedR5G5B5A1 |
        PixelFormat::UncompressedR4G4B4A4 => 16,
        PixelFormat::UncompressedR8G8B8A8 => 32,
        PixelFormat::UncompressedR8G8B8 => 24,
        PixelFormat::UncompressedR32 => 32,
        PixelFormat::UncompressedR32G32B32 => 32*3,
        PixelFormat::UncompressedR32G32B32A32 => 32*4,
        PixelFormat::UncompressedR16 => 16,
        PixelFormat::UncompressedR16G16B16 => 16*3,
        PixelFormat::UncompressedR16G16B16A16 => 16*4,
        PixelFormat::CompressedDXT1RGB |
        PixelFormat::CompressedDXT1RGBA |
        PixelFormat::CompressedETC1RGB |
        PixelFormat::CompressedETC2RGB |
        PixelFormat::CompressedPVRTRGB |
        PixelFormat::CompressedPVRTRGBA => 4,
        PixelFormat::CompressedDXT3RGBA |
        PixelFormat::CompressedDXT5RGBA |
        PixelFormat::CompressedETC2EACRGBA |
        PixelFormat::CompressedASTC4x4RGBA => 8,
        PixelFormat::CompressedASTC8x8RGBA => 2,
    };

    let bytes_per_pixel: f64 = bpp as f64 / 8.0;
    // Size in bytes
    let mut data_size = (bytes_per_pixel * width as f64 * height as f64) as u32; // Total data size in bytes

    // Most compressed formats works on 4x4 blocks,
    // if texture is smaller, minimum dataSize is 8 or 16
    if width < 4 && height < 4 {
        match format as isize {
            COMPRESSED_DXT1RGB ..COMPRESSED_DXT3RGBA    => data_size =  8,
            COMPRESSED_DXT3RGBA..COMPRESSED_ASTC8X8RGBA => data_size = 16,
            _ => (),
        }
    }

    data_size
}
