#![allow(nonstandard_style)]
use std::ffi::*;

mod GLAD_PLATFORM_H_ {
    use std::ffi::*;

    #[macro_export]
    macro_rules! GLAD_MAKE_VERSION {
        ($major:expr, minor:expr) => {{ $major * 10000 + $minor }};
    }
    #[macro_export]
    macro_rules! GLAD_VERSION_MAJOR {
        ($version:expr) => {{ $version / 10000 }};
    }
    #[macro_export]
    macro_rules! GLAD_VERSION_MINOR {
        ($version:expr) => {{ $version % 10000 }};
    }

    pub const GLAD_GENERATOR_VERSION: &CStr = c"2.0.0-beta";

    pub type GLADapiproc = unsafe extern "C" fn();

    pub type GLADloadfunc = unsafe extern "C" fn(name: *const c_char) -> Option<GLADapiproc>;
    pub type GLADuserptrloadfunc = unsafe extern "C" fn(userptr: *mut c_void, name: *const c_char) -> Option<GLADapiproc>;

    pub type GLADprecallback = unsafe extern "C" fn(name: *const c_char, apiproc: Option<GLADapiproc>, len_args: c_int, ...);
    pub type GLADpostcallback = unsafe extern "C" fn(ret: *mut c_void, name: *const c_char, apiproc: Option<GLADapiproc>, len_args: c_int, ...);
}
pub use GLAD_PLATFORM_H_::*;
use crate::{
    // GLAD_MAKE_VERSION,
    GLAD_VERSION_MAJOR,
    // GLAD_VERSION_MINOR,
};

pub const GL_ACTIVE_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92D9;
pub const GL_ACTIVE_ATTRIBUTES: c_uint = 0x8B89;
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: c_uint = 0x8B8A;
pub const GL_ACTIVE_PROGRAM: c_uint = 0x8259;
pub const GL_ACTIVE_RESOURCES: c_uint = 0x92F5;
pub const GL_ACTIVE_SUBROUTINES: c_uint = 0x8DE5;
pub const GL_ACTIVE_SUBROUTINE_MAX_LENGTH: c_uint = 0x8E48;
pub const GL_ACTIVE_SUBROUTINE_UNIFORMS: c_uint = 0x8DE6;
pub const GL_ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: c_uint = 0x8E47;
pub const GL_ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: c_uint = 0x8E49;
pub const GL_ACTIVE_TEXTURE: c_uint = 0x84E0;
pub const GL_ACTIVE_TEXTURE_ARB: c_uint = 0x84E0;
pub const GL_ACTIVE_UNIFORMS: c_uint = 0x8B86;
pub const GL_ACTIVE_UNIFORM_BLOCKS: c_uint = 0x8A36;
pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: c_uint = 0x8A35;
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: c_uint = 0x8B87;
pub const GL_ACTIVE_VARIABLES: c_uint = 0x9305;
pub const GL_ALIASED_LINE_WIDTH_RANGE: c_uint = 0x846E;
pub const GL_ALL_BARRIER_BITS: c_uint = 0xFFFFFFFF;
pub const GL_ALL_SHADER_BITS: c_uint = 0xFFFFFFFF;
pub const GL_ALPHA: c_uint = 0x1906;
pub const GL_ALPHA16F_ARB: c_uint = 0x881C;
pub const GL_ALPHA32F_ARB: c_uint = 0x8816;
pub const GL_ALREADY_SIGNALED: c_uint = 0x911A;
pub const GL_ALWAYS: c_uint = 0x0207;
pub const GL_AND: c_uint = 0x1501;
pub const GL_AND_INVERTED: c_uint = 0x1504;
pub const GL_AND_REVERSE: c_uint = 0x1502;
pub const GL_ANY_SAMPLES_PASSED: c_uint = 0x8C2F;
pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE: c_uint = 0x8D6A;
pub const GL_ARRAY_BUFFER: c_uint = 0x8892;
pub const GL_ARRAY_BUFFER_ARB: c_uint = 0x8892;
pub const GL_ARRAY_BUFFER_BINDING: c_uint = 0x8894;
pub const GL_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8894;
pub const GL_ARRAY_SIZE: c_uint = 0x92FB;
pub const GL_ARRAY_STRIDE: c_uint = 0x92FE;
pub const GL_ATOMIC_COUNTER_BARRIER_BIT: c_uint = 0x00001000;
pub const GL_ATOMIC_COUNTER_BUFFER: c_uint = 0x92C0;
pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: c_uint = 0x92C5;
pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: c_uint = 0x92C6;
pub const GL_ATOMIC_COUNTER_BUFFER_BINDING: c_uint = 0x92C1;
pub const GL_ATOMIC_COUNTER_BUFFER_DATA_SIZE: c_uint = 0x92C4;
pub const GL_ATOMIC_COUNTER_BUFFER_INDEX: c_uint = 0x9301;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: c_uint = 0x90ED;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: c_uint = 0x92CB;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: c_uint = 0x92CA;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: c_uint = 0x92C8;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: c_uint = 0x92C9;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: c_uint = 0x92C7;
pub const GL_ATOMIC_COUNTER_BUFFER_SIZE: c_uint = 0x92C3;
pub const GL_ATOMIC_COUNTER_BUFFER_START: c_uint = 0x92C2;
pub const GL_ATTACHED_SHADERS: c_uint = 0x8B85;
pub const GL_AUTO_GENERATE_MIPMAP: c_uint = 0x8295;
pub const GL_BACK: c_uint = 0x0405;
pub const GL_BACK_LEFT: c_uint = 0x0402;
pub const GL_BACK_RIGHT: c_uint = 0x0403;
pub const GL_BGR: c_uint = 0x80E0;
pub const GL_BGRA: c_uint = 0x80E1;
pub const GL_BGRA_INTEGER: c_uint = 0x8D9B;
pub const GL_BGR_INTEGER: c_uint = 0x8D9A;
pub const GL_BLEND: c_uint = 0x0BE2;
pub const GL_BLEND_COLOR: c_uint = 0x8005;
pub const GL_BLEND_DST: c_uint = 0x0BE0;
pub const GL_BLEND_DST_ALPHA: c_uint = 0x80CA;
pub const GL_BLEND_DST_RGB: c_uint = 0x80C8;
pub const GL_BLEND_EQUATION: c_uint = 0x8009;
pub const GL_BLEND_EQUATION_ALPHA: c_uint = 0x883D;
pub const GL_BLEND_EQUATION_RGB: c_uint = 0x8009;
pub const GL_BLEND_SRC: c_uint = 0x0BE1;
pub const GL_BLEND_SRC_ALPHA: c_uint = 0x80CB;
pub const GL_BLEND_SRC_RGB: c_uint = 0x80C9;
pub const GL_BLOCK_INDEX: c_uint = 0x92FD;
pub const GL_BLUE: c_uint = 0x1905;
pub const GL_BLUE_INTEGER: c_uint = 0x8D96;
pub const GL_BOOL: c_uint = 0x8B56;
pub const GL_BOOL_ARB: c_uint = 0x8B56;
pub const GL_BOOL_VEC2: c_uint = 0x8B57;
pub const GL_BOOL_VEC2_ARB: c_uint = 0x8B57;
pub const GL_BOOL_VEC3: c_uint = 0x8B58;
pub const GL_BOOL_VEC3_ARB: c_uint = 0x8B58;
pub const GL_BOOL_VEC4: c_uint = 0x8B59;
pub const GL_BOOL_VEC4_ARB: c_uint = 0x8B59;
pub const GL_BUFFER: c_uint = 0x82E0;
pub const GL_BUFFER_ACCESS: c_uint = 0x88BB;
pub const GL_BUFFER_ACCESS_ARB: c_uint = 0x88BB;
pub const GL_BUFFER_ACCESS_FLAGS: c_uint = 0x911F;
pub const GL_BUFFER_BINDING: c_uint = 0x9302;
pub const GL_BUFFER_DATA_SIZE: c_uint = 0x9303;
pub const GL_BUFFER_IMMUTABLE_STORAGE: c_uint = 0x821F;
pub const GL_BUFFER_MAPPED: c_uint = 0x88BC;
pub const GL_BUFFER_MAPPED_ARB: c_uint = 0x88BC;
pub const GL_BUFFER_MAP_LENGTH: c_uint = 0x9120;
pub const GL_BUFFER_MAP_OFFSET: c_uint = 0x9121;
pub const GL_BUFFER_MAP_POINTER: c_uint = 0x88BD;
pub const GL_BUFFER_MAP_POINTER_ARB: c_uint = 0x88BD;
pub const GL_BUFFER_SIZE: c_uint = 0x8764;
pub const GL_BUFFER_SIZE_ARB: c_uint = 0x8764;
pub const GL_BUFFER_STORAGE_FLAGS: c_uint = 0x8220;
pub const GL_BUFFER_UPDATE_BARRIER_BIT: c_uint = 0x00000200;
pub const GL_BUFFER_USAGE: c_uint = 0x8765;
pub const GL_BUFFER_USAGE_ARB: c_uint = 0x8765;
pub const GL_BUFFER_VARIABLE: c_uint = 0x92E5;
pub const GL_BYTE: c_uint = 0x1400;
pub const GL_CAVEAT_SUPPORT: c_uint = 0x82B8;
pub const GL_CCW: c_uint = 0x0901;
pub const GL_CLAMP_FRAGMENT_COLOR_ARB: c_uint = 0x891B;
pub const GL_CLAMP_READ_COLOR: c_uint = 0x891C;
pub const GL_CLAMP_READ_COLOR_ARB: c_uint = 0x891C;
pub const GL_CLAMP_TO_BORDER: c_uint = 0x812D;
pub const GL_CLAMP_TO_BORDER_ARB: c_uint = 0x812D;
pub const GL_CLAMP_TO_EDGE: c_uint = 0x812F;
pub const GL_CLAMP_VERTEX_COLOR_ARB: c_uint = 0x891A;
pub const GL_CLEAR: c_uint = 0x1500;
pub const GL_CLEAR_BUFFER: c_uint = 0x82B4;
pub const GL_CLEAR_TEXTURE: c_uint = 0x9365;
pub const GL_CLIENT_ACTIVE_TEXTURE_ARB: c_uint = 0x84E1;
pub const GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT: c_uint = 0x00004000;
pub const GL_CLIENT_STORAGE_BIT: c_uint = 0x0200;
pub const GL_CLIPPING_INPUT_PRIMITIVES: c_uint = 0x82F6;
pub const GL_CLIPPING_INPUT_PRIMITIVES_ARB: c_uint = 0x82F6;
pub const GL_CLIPPING_OUTPUT_PRIMITIVES: c_uint = 0x82F7;
pub const GL_CLIPPING_OUTPUT_PRIMITIVES_ARB: c_uint = 0x82F7;
pub const GL_CLIP_DISTANCE0: c_uint = 0x3000;
pub const GL_CLIP_DISTANCE1: c_uint = 0x3001;
pub const GL_CLIP_DISTANCE2: c_uint = 0x3002;
pub const GL_CLIP_DISTANCE3: c_uint = 0x3003;
pub const GL_CLIP_DISTANCE4: c_uint = 0x3004;
pub const GL_CLIP_DISTANCE5: c_uint = 0x3005;
pub const GL_CLIP_DISTANCE6: c_uint = 0x3006;
pub const GL_CLIP_DISTANCE7: c_uint = 0x3007;
pub const GL_COLOR: c_uint = 0x1800;
pub const GL_COLOR_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8898;
pub const GL_COLOR_ATTACHMENT0: c_uint = 0x8CE0;
pub const GL_COLOR_ATTACHMENT0_EXT: c_uint = 0x8CE0;
pub const GL_COLOR_ATTACHMENT1: c_uint = 0x8CE1;
pub const GL_COLOR_ATTACHMENT10: c_uint = 0x8CEA;
pub const GL_COLOR_ATTACHMENT10_EXT: c_uint = 0x8CEA;
pub const GL_COLOR_ATTACHMENT11: c_uint = 0x8CEB;
pub const GL_COLOR_ATTACHMENT11_EXT: c_uint = 0x8CEB;
pub const GL_COLOR_ATTACHMENT12: c_uint = 0x8CEC;
pub const GL_COLOR_ATTACHMENT12_EXT: c_uint = 0x8CEC;
pub const GL_COLOR_ATTACHMENT13: c_uint = 0x8CED;
pub const GL_COLOR_ATTACHMENT13_EXT: c_uint = 0x8CED;
pub const GL_COLOR_ATTACHMENT14: c_uint = 0x8CEE;
pub const GL_COLOR_ATTACHMENT14_EXT: c_uint = 0x8CEE;
pub const GL_COLOR_ATTACHMENT15: c_uint = 0x8CEF;
pub const GL_COLOR_ATTACHMENT15_EXT: c_uint = 0x8CEF;
pub const GL_COLOR_ATTACHMENT16: c_uint = 0x8CF0;
pub const GL_COLOR_ATTACHMENT17: c_uint = 0x8CF1;
pub const GL_COLOR_ATTACHMENT18: c_uint = 0x8CF2;
pub const GL_COLOR_ATTACHMENT19: c_uint = 0x8CF3;
pub const GL_COLOR_ATTACHMENT1_EXT: c_uint = 0x8CE1;
pub const GL_COLOR_ATTACHMENT2: c_uint = 0x8CE2;
pub const GL_COLOR_ATTACHMENT20: c_uint = 0x8CF4;
pub const GL_COLOR_ATTACHMENT21: c_uint = 0x8CF5;
pub const GL_COLOR_ATTACHMENT22: c_uint = 0x8CF6;
pub const GL_COLOR_ATTACHMENT23: c_uint = 0x8CF7;
pub const GL_COLOR_ATTACHMENT24: c_uint = 0x8CF8;
pub const GL_COLOR_ATTACHMENT25: c_uint = 0x8CF9;
pub const GL_COLOR_ATTACHMENT26: c_uint = 0x8CFA;
pub const GL_COLOR_ATTACHMENT27: c_uint = 0x8CFB;
pub const GL_COLOR_ATTACHMENT28: c_uint = 0x8CFC;
pub const GL_COLOR_ATTACHMENT29: c_uint = 0x8CFD;
pub const GL_COLOR_ATTACHMENT2_EXT: c_uint = 0x8CE2;
pub const GL_COLOR_ATTACHMENT3: c_uint = 0x8CE3;
pub const GL_COLOR_ATTACHMENT30: c_uint = 0x8CFE;
pub const GL_COLOR_ATTACHMENT31: c_uint = 0x8CFF;
pub const GL_COLOR_ATTACHMENT3_EXT: c_uint = 0x8CE3;
pub const GL_COLOR_ATTACHMENT4: c_uint = 0x8CE4;
pub const GL_COLOR_ATTACHMENT4_EXT: c_uint = 0x8CE4;
pub const GL_COLOR_ATTACHMENT5: c_uint = 0x8CE5;
pub const GL_COLOR_ATTACHMENT5_EXT: c_uint = 0x8CE5;
pub const GL_COLOR_ATTACHMENT6: c_uint = 0x8CE6;
pub const GL_COLOR_ATTACHMENT6_EXT: c_uint = 0x8CE6;
pub const GL_COLOR_ATTACHMENT7: c_uint = 0x8CE7;
pub const GL_COLOR_ATTACHMENT7_EXT: c_uint = 0x8CE7;
pub const GL_COLOR_ATTACHMENT8: c_uint = 0x8CE8;
pub const GL_COLOR_ATTACHMENT8_EXT: c_uint = 0x8CE8;
pub const GL_COLOR_ATTACHMENT9: c_uint = 0x8CE9;
pub const GL_COLOR_ATTACHMENT9_EXT: c_uint = 0x8CE9;
pub const GL_COLOR_BUFFER_BIT: c_uint = 0x00004000;
pub const GL_COLOR_CLEAR_VALUE: c_uint = 0x0C22;
pub const GL_COLOR_COMPONENTS: c_uint = 0x8283;
pub const GL_COLOR_ENCODING: c_uint = 0x8296;
pub const GL_COLOR_LOGIC_OP: c_uint = 0x0BF2;
pub const GL_COLOR_RENDERABLE: c_uint = 0x8286;
pub const GL_COLOR_SUM_ARB: c_uint = 0x8458;
pub const GL_COLOR_WRITEMASK: c_uint = 0x0C23;
pub const GL_COMMAND_BARRIER_BIT: c_uint = 0x00000040;
pub const GL_COMPARE_REF_TO_TEXTURE: c_uint = 0x884E;
pub const GL_COMPATIBLE_SUBROUTINES: c_uint = 0x8E4B;
pub const GL_COMPILE_STATUS: c_uint = 0x8B81;
pub const GL_COMPRESSED_ALPHA_ARB: c_uint = 0x84E9;
pub const GL_COMPRESSED_INTENSITY_ARB: c_uint = 0x84EC;
pub const GL_COMPRESSED_LUMINANCE_ALPHA_ARB: c_uint = 0x84EB;
pub const GL_COMPRESSED_LUMINANCE_ARB: c_uint = 0x84EA;
pub const GL_COMPRESSED_R11_EAC: c_uint = 0x9270;
pub const GL_COMPRESSED_RED: c_uint = 0x8225;
pub const GL_COMPRESSED_RED_RGTC1: c_uint = 0x8DBB;
pub const GL_COMPRESSED_RG: c_uint = 0x8226;
pub const GL_COMPRESSED_RG11_EAC: c_uint = 0x9272;
pub const GL_COMPRESSED_RGB: c_uint = 0x84ED;
pub const GL_COMPRESSED_RGB8_ETC2: c_uint = 0x9274;
pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: c_uint = 0x9276;
pub const GL_COMPRESSED_RGBA: c_uint = 0x84EE;
pub const GL_COMPRESSED_RGBA8_ETC2_EAC: c_uint = 0x9278;
pub const GL_COMPRESSED_RGBA_ARB: c_uint = 0x84EE;
pub const GL_COMPRESSED_RGBA_ASTC_10x10_KHR: c_uint = 0x93BB;
pub const GL_COMPRESSED_RGBA_ASTC_10x5_KHR: c_uint = 0x93B8;
pub const GL_COMPRESSED_RGBA_ASTC_10x6_KHR: c_uint = 0x93B9;
pub const GL_COMPRESSED_RGBA_ASTC_10x8_KHR: c_uint = 0x93BA;
pub const GL_COMPRESSED_RGBA_ASTC_12x10_KHR: c_uint = 0x93BC;
pub const GL_COMPRESSED_RGBA_ASTC_12x12_KHR: c_uint = 0x93BD;
pub const GL_COMPRESSED_RGBA_ASTC_4x4_KHR: c_uint = 0x93B0;
pub const GL_COMPRESSED_RGBA_ASTC_5x4_KHR: c_uint = 0x93B1;
pub const GL_COMPRESSED_RGBA_ASTC_5x5_KHR: c_uint = 0x93B2;
pub const GL_COMPRESSED_RGBA_ASTC_6x5_KHR: c_uint = 0x93B3;
pub const GL_COMPRESSED_RGBA_ASTC_6x6_KHR: c_uint = 0x93B4;
pub const GL_COMPRESSED_RGBA_ASTC_8x5_KHR: c_uint = 0x93B5;
pub const GL_COMPRESSED_RGBA_ASTC_8x6_KHR: c_uint = 0x93B6;
pub const GL_COMPRESSED_RGBA_ASTC_8x8_KHR: c_uint = 0x93B7;
pub const GL_COMPRESSED_RGBA_BPTC_UNORM: c_uint = 0x8E8C;
pub const GL_COMPRESSED_RGBA_S3TC_DXT1_EXT: c_uint = 0x83F1;
pub const GL_COMPRESSED_RGBA_S3TC_DXT3_EXT: c_uint = 0x83F2;
pub const GL_COMPRESSED_RGBA_S3TC_DXT5_EXT: c_uint = 0x83F3;
pub const GL_COMPRESSED_RGB_ARB: c_uint = 0x84ED;
pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT: c_uint = 0x8E8E;
pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: c_uint = 0x8E8F;
pub const GL_COMPRESSED_RGB_S3TC_DXT1_EXT: c_uint = 0x83F0;
pub const GL_COMPRESSED_RG_RGTC2: c_uint = 0x8DBD;
pub const GL_COMPRESSED_SIGNED_R11_EAC: c_uint = 0x9271;
pub const GL_COMPRESSED_SIGNED_RED_RGTC1: c_uint = 0x8DBC;
pub const GL_COMPRESSED_SIGNED_RG11_EAC: c_uint = 0x9273;
pub const GL_COMPRESSED_SIGNED_RG_RGTC2: c_uint = 0x8DBE;
pub const GL_COMPRESSED_SRGB: c_uint = 0x8C48;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR: c_uint = 0x93DB;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR: c_uint = 0x93D8;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR: c_uint = 0x93D9;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR: c_uint = 0x93DA;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR: c_uint = 0x93DC;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR: c_uint = 0x93DD;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR: c_uint = 0x93D0;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR: c_uint = 0x93D1;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR: c_uint = 0x93D2;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR: c_uint = 0x93D3;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR: c_uint = 0x93D4;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR: c_uint = 0x93D5;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR: c_uint = 0x93D6;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR: c_uint = 0x93D7;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: c_uint = 0x9279;
pub const GL_COMPRESSED_SRGB8_ETC2: c_uint = 0x9275;
pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: c_uint = 0x9277;
pub const GL_COMPRESSED_SRGB_ALPHA: c_uint = 0x8C49;
pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM: c_uint = 0x8E8D;
pub const GL_COMPRESSED_TEXTURE_FORMATS: c_uint = 0x86A3;
pub const GL_COMPRESSED_TEXTURE_FORMATS_ARB: c_uint = 0x86A3;
pub const GL_COMPUTE_SHADER: c_uint = 0x91B9;
pub const GL_COMPUTE_SHADER_BIT: c_uint = 0x00000020;
pub const GL_COMPUTE_SHADER_INVOCATIONS: c_uint = 0x82F5;
pub const GL_COMPUTE_SHADER_INVOCATIONS_ARB: c_uint = 0x82F5;
pub const GL_COMPUTE_SUBROUTINE: c_uint = 0x92ED;
pub const GL_COMPUTE_SUBROUTINE_UNIFORM: c_uint = 0x92F3;
pub const GL_COMPUTE_TEXTURE: c_uint = 0x82A0;
pub const GL_COMPUTE_WORK_GROUP_SIZE: c_uint = 0x8267;
pub const GL_CONDITION_SATISFIED: c_uint = 0x911C;
pub const GL_CONSTANT_ALPHA: c_uint = 0x8003;
pub const GL_CONSTANT_COLOR: c_uint = 0x8001;
pub const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT: c_uint = 0x00000002;
pub const GL_CONTEXT_CORE_PROFILE_BIT: c_uint = 0x00000001;
pub const GL_CONTEXT_FLAGS: c_uint = 0x821E;
pub const GL_CONTEXT_FLAG_DEBUG_BIT: c_uint = 0x00000002;
pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: c_uint = 0x00000001;
pub const GL_CONTEXT_PROFILE_MASK: c_uint = 0x9126;
pub const GL_COPY: c_uint = 0x1503;
pub const GL_COPY_INVERTED: c_uint = 0x150C;
pub const GL_COPY_READ_BUFFER: c_uint = 0x8F36;
pub const GL_COPY_READ_BUFFER_BINDING: c_uint = 0x8F36;
pub const GL_COPY_WRITE_BUFFER: c_uint = 0x8F37;
pub const GL_COPY_WRITE_BUFFER_BINDING: c_uint = 0x8F37;
pub const GL_CULL_FACE: c_uint = 0x0B44;
pub const GL_CULL_FACE_MODE: c_uint = 0x0B45;
pub const GL_CURRENT_FOG_COORDINATE_EXT: c_uint = 0x8453;
pub const GL_CURRENT_MATRIX_ARB: c_uint = 0x8641;
pub const GL_CURRENT_MATRIX_STACK_DEPTH_ARB: c_uint = 0x8640;
pub const GL_CURRENT_PROGRAM: c_uint = 0x8B8D;
pub const GL_CURRENT_QUERY: c_uint = 0x8865;
pub const GL_CURRENT_QUERY_ARB: c_uint = 0x8865;
pub const GL_CURRENT_VERTEX_ATTRIB: c_uint = 0x8626;
pub const GL_CURRENT_VERTEX_ATTRIB_ARB: c_uint = 0x8626;
pub const GL_CW: c_uint = 0x0900;
pub const GL_DEBUG_CALLBACK_FUNCTION: c_uint = 0x8244;
pub const GL_DEBUG_CALLBACK_FUNCTION_ARB: c_uint = 0x8244;
pub const GL_DEBUG_CALLBACK_USER_PARAM: c_uint = 0x8245;
pub const GL_DEBUG_CALLBACK_USER_PARAM_ARB: c_uint = 0x8245;
pub const GL_DEBUG_GROUP_STACK_DEPTH: c_uint = 0x826D;
pub const GL_DEBUG_LOGGED_MESSAGES: c_uint = 0x9145;
pub const GL_DEBUG_LOGGED_MESSAGES_ARB: c_uint = 0x9145;
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: c_uint = 0x8243;
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_ARB: c_uint = 0x8243;
pub const GL_DEBUG_OUTPUT: c_uint = 0x92E0;
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS: c_uint = 0x8242;
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS_ARB: c_uint = 0x8242;
pub const GL_DEBUG_SEVERITY_HIGH: c_uint = 0x9146;
pub const GL_DEBUG_SEVERITY_HIGH_ARB: c_uint = 0x9146;
pub const GL_DEBUG_SEVERITY_LOW: c_uint = 0x9148;
pub const GL_DEBUG_SEVERITY_LOW_ARB: c_uint = 0x9148;
pub const GL_DEBUG_SEVERITY_MEDIUM: c_uint = 0x9147;
pub const GL_DEBUG_SEVERITY_MEDIUM_ARB: c_uint = 0x9147;
pub const GL_DEBUG_SEVERITY_NOTIFICATION: c_uint = 0x826B;
pub const GL_DEBUG_SOURCE_API: c_uint = 0x8246;
pub const GL_DEBUG_SOURCE_API_ARB: c_uint = 0x8246;
pub const GL_DEBUG_SOURCE_APPLICATION: c_uint = 0x824A;
pub const GL_DEBUG_SOURCE_APPLICATION_ARB: c_uint = 0x824A;
pub const GL_DEBUG_SOURCE_OTHER: c_uint = 0x824B;
pub const GL_DEBUG_SOURCE_OTHER_ARB: c_uint = 0x824B;
pub const GL_DEBUG_SOURCE_SHADER_COMPILER: c_uint = 0x8248;
pub const GL_DEBUG_SOURCE_SHADER_COMPILER_ARB: c_uint = 0x8248;
pub const GL_DEBUG_SOURCE_THIRD_PARTY: c_uint = 0x8249;
pub const GL_DEBUG_SOURCE_THIRD_PARTY_ARB: c_uint = 0x8249;
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM: c_uint = 0x8247;
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM_ARB: c_uint = 0x8247;
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: c_uint = 0x824D;
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_ARB: c_uint = 0x824D;
pub const GL_DEBUG_TYPE_ERROR: c_uint = 0x824C;
pub const GL_DEBUG_TYPE_ERROR_ARB: c_uint = 0x824C;
pub const GL_DEBUG_TYPE_MARKER: c_uint = 0x8268;
pub const GL_DEBUG_TYPE_OTHER: c_uint = 0x8251;
pub const GL_DEBUG_TYPE_OTHER_ARB: c_uint = 0x8251;
pub const GL_DEBUG_TYPE_PERFORMANCE: c_uint = 0x8250;
pub const GL_DEBUG_TYPE_PERFORMANCE_ARB: c_uint = 0x8250;
pub const GL_DEBUG_TYPE_POP_GROUP: c_uint = 0x826A;
pub const GL_DEBUG_TYPE_PORTABILITY: c_uint = 0x824F;
pub const GL_DEBUG_TYPE_PORTABILITY_ARB: c_uint = 0x824F;
pub const GL_DEBUG_TYPE_PUSH_GROUP: c_uint = 0x8269;
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: c_uint = 0x824E;
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_ARB: c_uint = 0x824E;
pub const GL_DECR: c_uint = 0x1E03;
pub const GL_DECR_WRAP: c_uint = 0x8508;
pub const GL_DELETE_STATUS: c_uint = 0x8B80;
pub const GL_DEPTH: c_uint = 0x1801;
pub const GL_DEPTH24_STENCIL8: c_uint = 0x88F0;
pub const GL_DEPTH32F_STENCIL8: c_uint = 0x8CAD;
pub const GL_DEPTH_ATTACHMENT: c_uint = 0x8D00;
pub const GL_DEPTH_ATTACHMENT_EXT: c_uint = 0x8D00;
pub const GL_DEPTH_BUFFER_BIT: c_uint = 0x00000100;
pub const GL_DEPTH_CLAMP: c_uint = 0x864F;
pub const GL_DEPTH_CLEAR_VALUE: c_uint = 0x0B73;
pub const GL_DEPTH_COMPONENT: c_uint = 0x1902;
pub const GL_DEPTH_COMPONENT16: c_uint = 0x81A5;
pub const GL_DEPTH_COMPONENT16_ARB: c_uint = 0x81A5;
pub const GL_DEPTH_COMPONENT24: c_uint = 0x81A6;
pub const GL_DEPTH_COMPONENT24_ARB: c_uint = 0x81A6;
pub const GL_DEPTH_COMPONENT32: c_uint = 0x81A7;
pub const GL_DEPTH_COMPONENT32F: c_uint = 0x8CAC;
pub const GL_DEPTH_COMPONENT32_ARB: c_uint = 0x81A7;
pub const GL_DEPTH_COMPONENTS: c_uint = 0x8284;
pub const GL_DEPTH_FUNC: c_uint = 0x0B74;
pub const GL_DEPTH_RANGE: c_uint = 0x0B70;
pub const GL_DEPTH_RENDERABLE: c_uint = 0x8287;
pub const GL_DEPTH_STENCIL: c_uint = 0x84F9;
pub const GL_DEPTH_STENCIL_ATTACHMENT: c_uint = 0x821A;
pub const GL_DEPTH_STENCIL_TEXTURE_MODE: c_uint = 0x90EA;
pub const GL_DEPTH_TEST: c_uint = 0x0B71;
pub const GL_DEPTH_TEXTURE_MODE_ARB: c_uint = 0x884B;
pub const GL_DEPTH_WRITEMASK: c_uint = 0x0B72;
pub const GL_DISPATCH_INDIRECT_BUFFER: c_uint = 0x90EE;
pub const GL_DISPATCH_INDIRECT_BUFFER_BINDING: c_uint = 0x90EF;
pub const GL_DITHER: c_uint = 0x0BD0;
pub const GL_DONT_CARE: c_uint = 0x1100;
pub const GL_DOUBLE: c_uint = 0x140A;
pub const GL_DOUBLEBUFFER: c_uint = 0x0C32;
pub const GL_DOUBLE_MAT2: c_uint = 0x8F46;
pub const GL_DOUBLE_MAT2x3: c_uint = 0x8F49;
pub const GL_DOUBLE_MAT2x4: c_uint = 0x8F4A;
pub const GL_DOUBLE_MAT3: c_uint = 0x8F47;
pub const GL_DOUBLE_MAT3x2: c_uint = 0x8F4B;
pub const GL_DOUBLE_MAT3x4: c_uint = 0x8F4C;
pub const GL_DOUBLE_MAT4: c_uint = 0x8F48;
pub const GL_DOUBLE_MAT4x2: c_uint = 0x8F4D;
pub const GL_DOUBLE_MAT4x3: c_uint = 0x8F4E;
pub const GL_DOUBLE_VEC2: c_uint = 0x8FFC;
pub const GL_DOUBLE_VEC3: c_uint = 0x8FFD;
pub const GL_DOUBLE_VEC4: c_uint = 0x8FFE;
pub const GL_DRAW_BUFFER: c_uint = 0x0C01;
pub const GL_DRAW_BUFFER0: c_uint = 0x8825;
pub const GL_DRAW_BUFFER0_ARB: c_uint = 0x8825;
pub const GL_DRAW_BUFFER1: c_uint = 0x8826;
pub const GL_DRAW_BUFFER10: c_uint = 0x882F;
pub const GL_DRAW_BUFFER10_ARB: c_uint = 0x882F;
pub const GL_DRAW_BUFFER11: c_uint = 0x8830;
pub const GL_DRAW_BUFFER11_ARB: c_uint = 0x8830;
pub const GL_DRAW_BUFFER12: c_uint = 0x8831;
pub const GL_DRAW_BUFFER12_ARB: c_uint = 0x8831;
pub const GL_DRAW_BUFFER13: c_uint = 0x8832;
pub const GL_DRAW_BUFFER13_ARB: c_uint = 0x8832;
pub const GL_DRAW_BUFFER14: c_uint = 0x8833;
pub const GL_DRAW_BUFFER14_ARB: c_uint = 0x8833;
pub const GL_DRAW_BUFFER15: c_uint = 0x8834;
pub const GL_DRAW_BUFFER15_ARB: c_uint = 0x8834;
pub const GL_DRAW_BUFFER1_ARB: c_uint = 0x8826;
pub const GL_DRAW_BUFFER2: c_uint = 0x8827;
pub const GL_DRAW_BUFFER2_ARB: c_uint = 0x8827;
pub const GL_DRAW_BUFFER3: c_uint = 0x8828;
pub const GL_DRAW_BUFFER3_ARB: c_uint = 0x8828;
pub const GL_DRAW_BUFFER4: c_uint = 0x8829;
pub const GL_DRAW_BUFFER4_ARB: c_uint = 0x8829;
pub const GL_DRAW_BUFFER5: c_uint = 0x882A;
pub const GL_DRAW_BUFFER5_ARB: c_uint = 0x882A;
pub const GL_DRAW_BUFFER6: c_uint = 0x882B;
pub const GL_DRAW_BUFFER6_ARB: c_uint = 0x882B;
pub const GL_DRAW_BUFFER7: c_uint = 0x882C;
pub const GL_DRAW_BUFFER7_ARB: c_uint = 0x882C;
pub const GL_DRAW_BUFFER8: c_uint = 0x882D;
pub const GL_DRAW_BUFFER8_ARB: c_uint = 0x882D;
pub const GL_DRAW_BUFFER9: c_uint = 0x882E;
pub const GL_DRAW_BUFFER9_ARB: c_uint = 0x882E;
pub const GL_DRAW_FRAMEBUFFER: c_uint = 0x8CA9;
pub const GL_DRAW_FRAMEBUFFER_BINDING: c_uint = 0x8CA6;
pub const GL_DRAW_FRAMEBUFFER_BINDING_EXT: c_uint = 0x8CA6;
pub const GL_DRAW_FRAMEBUFFER_EXT: c_uint = 0x8CA9;
pub const GL_DRAW_INDIRECT_BUFFER: c_uint = 0x8F3F;
pub const GL_DRAW_INDIRECT_BUFFER_BINDING: c_uint = 0x8F43;
pub const GL_DST_ALPHA: c_uint = 0x0304;
pub const GL_DST_COLOR: c_uint = 0x0306;
pub const GL_DYNAMIC_COPY: c_uint = 0x88EA;
pub const GL_DYNAMIC_COPY_ARB: c_uint = 0x88EA;
pub const GL_DYNAMIC_DRAW: c_uint = 0x88E8;
pub const GL_DYNAMIC_DRAW_ARB: c_uint = 0x88E8;
pub const GL_DYNAMIC_READ: c_uint = 0x88E9;
pub const GL_DYNAMIC_READ_ARB: c_uint = 0x88E9;
pub const GL_DYNAMIC_STORAGE_BIT: c_uint = 0x0100;
pub const GL_EDGE_FLAG_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889B;
pub const GL_ELEMENT_ARRAY_BARRIER_BIT: c_uint = 0x00000002;
pub const GL_ELEMENT_ARRAY_BUFFER: c_uint = 0x8893;
pub const GL_ELEMENT_ARRAY_BUFFER_ARB: c_uint = 0x8893;
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: c_uint = 0x8895;
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8895;
pub const GL_EQUAL: c_uint = 0x0202;
pub const GL_EQUIV: c_uint = 0x1509;
pub const GL_EXTENSIONS: c_uint = 0x1F03;
pub const GL_FALSE: c_uint = 0;
pub const GL_FASTEST: c_uint = 0x1101;
pub const GL_FILL: c_uint = 0x1B02;
pub const GL_FILTER: c_uint = 0x829A;
pub const GL_FIRST_VERTEX_CONVENTION: c_uint = 0x8E4D;
pub const GL_FIXED: c_uint = 0x140C;
pub const GL_FIXED_OES: c_uint = 0x140C;
pub const GL_FIXED_ONLY: c_uint = 0x891D;
pub const GL_FIXED_ONLY_ARB: c_uint = 0x891D;
pub const GL_FLOAT: c_uint = 0x1406;
pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: c_uint = 0x8DAD;
pub const GL_FLOAT_MAT2: c_uint = 0x8B5A;
pub const GL_FLOAT_MAT2_ARB: c_uint = 0x8B5A;
pub const GL_FLOAT_MAT2x3: c_uint = 0x8B65;
pub const GL_FLOAT_MAT2x4: c_uint = 0x8B66;
pub const GL_FLOAT_MAT3: c_uint = 0x8B5B;
pub const GL_FLOAT_MAT3_ARB: c_uint = 0x8B5B;
pub const GL_FLOAT_MAT3x2: c_uint = 0x8B67;
pub const GL_FLOAT_MAT3x4: c_uint = 0x8B68;
pub const GL_FLOAT_MAT4: c_uint = 0x8B5C;
pub const GL_FLOAT_MAT4_ARB: c_uint = 0x8B5C;
pub const GL_FLOAT_MAT4x2: c_uint = 0x8B69;
pub const GL_FLOAT_MAT4x3: c_uint = 0x8B6A;
pub const GL_FLOAT_VEC2: c_uint = 0x8B50;
pub const GL_FLOAT_VEC2_ARB: c_uint = 0x8B50;
pub const GL_FLOAT_VEC3: c_uint = 0x8B51;
pub const GL_FLOAT_VEC3_ARB: c_uint = 0x8B51;
pub const GL_FLOAT_VEC4: c_uint = 0x8B52;
pub const GL_FLOAT_VEC4_ARB: c_uint = 0x8B52;
pub const GL_FOG_COORDINATE_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889D;
pub const GL_FOG_COORDINATE_ARRAY_EXT: c_uint = 0x8457;
pub const GL_FOG_COORDINATE_ARRAY_POINTER_EXT: c_uint = 0x8456;
pub const GL_FOG_COORDINATE_ARRAY_STRIDE_EXT: c_uint = 0x8455;
pub const GL_FOG_COORDINATE_ARRAY_TYPE_EXT: c_uint = 0x8454;
pub const GL_FOG_COORDINATE_EXT: c_uint = 0x8451;
pub const GL_FOG_COORDINATE_SOURCE_EXT: c_uint = 0x8450;
pub const GL_FRACTIONAL_EVEN: c_uint = 0x8E7C;
pub const GL_FRACTIONAL_ODD: c_uint = 0x8E7B;
pub const GL_FRAGMENT_DEPTH_EXT: c_uint = 0x8452;
pub const GL_FRAGMENT_INTERPOLATION_OFFSET_BITS: c_uint = 0x8E5D;
pub const GL_FRAGMENT_PROGRAM_ARB: c_uint = 0x8804;
pub const GL_FRAGMENT_SHADER: c_uint = 0x8B30;
pub const GL_FRAGMENT_SHADER_ARB: c_uint = 0x8B30;
pub const GL_FRAGMENT_SHADER_BIT: c_uint = 0x00000002;
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: c_uint = 0x8B8B;
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT_ARB: c_uint = 0x8B8B;
pub const GL_FRAGMENT_SHADER_INVOCATIONS: c_uint = 0x82F4;
pub const GL_FRAGMENT_SHADER_INVOCATIONS_ARB: c_uint = 0x82F4;
pub const GL_FRAGMENT_SUBROUTINE: c_uint = 0x92EC;
pub const GL_FRAGMENT_SUBROUTINE_UNIFORM: c_uint = 0x92F2;
pub const GL_FRAGMENT_TEXTURE: c_uint = 0x829F;
pub const GL_FRAMEBUFFER: c_uint = 0x8D40;
pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: c_uint = 0x8215;
pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: c_uint = 0x8214;
pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: c_uint = 0x8210;
pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: c_uint = 0x8211;
pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: c_uint = 0x8216;
pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: c_uint = 0x8213;
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED: c_uint = 0x8DA7;
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED_ARB: c_uint = 0x8DA7;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: c_uint = 0x8CD1;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME_EXT: c_uint = 0x8CD1;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: c_uint = 0x8CD0;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE_EXT: c_uint = 0x8CD0;
pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: c_uint = 0x8212;
pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: c_uint = 0x8217;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_3D_ZOFFSET_EXT: c_uint = 0x8CD4;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: c_uint = 0x8CD3;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE_EXT: c_uint = 0x8CD3;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: c_uint = 0x8CD4;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: c_uint = 0x8CD2;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL_EXT: c_uint = 0x8CD2;
pub const GL_FRAMEBUFFER_BARRIER_BIT: c_uint = 0x00000400;
pub const GL_FRAMEBUFFER_BINDING: c_uint = 0x8CA6;
pub const GL_FRAMEBUFFER_BINDING_EXT: c_uint = 0x8CA6;
pub const GL_FRAMEBUFFER_BLEND: c_uint = 0x828B;
pub const GL_FRAMEBUFFER_COMPLETE: c_uint = 0x8CD5;
pub const GL_FRAMEBUFFER_COMPLETE_EXT: c_uint = 0x8CD5;
pub const GL_FRAMEBUFFER_DEFAULT: c_uint = 0x8218;
pub const GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: c_uint = 0x9314;
pub const GL_FRAMEBUFFER_DEFAULT_HEIGHT: c_uint = 0x9311;
pub const GL_FRAMEBUFFER_DEFAULT_LAYERS: c_uint = 0x9312;
pub const GL_FRAMEBUFFER_DEFAULT_SAMPLES: c_uint = 0x9313;
pub const GL_FRAMEBUFFER_DEFAULT_WIDTH: c_uint = 0x9310;
pub const GL_FRAMEBUFFER_EXT: c_uint = 0x8D40;
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: c_uint = 0x8CD6;
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT_EXT: c_uint = 0x8CD6;
pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS_EXT: c_uint = 0x8CD9;
pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: c_uint = 0x8CDB;
pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER_EXT: c_uint = 0x8CDB;
pub const GL_FRAMEBUFFER_INCOMPLETE_FORMATS_EXT: c_uint = 0x8CDA;
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_COUNT_ARB: c_uint = 0x8DA9;
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: c_uint = 0x8DA8;
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS_ARB: c_uint = 0x8DA8;
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: c_uint = 0x8CD7;
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT_EXT: c_uint = 0x8CD7;
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: c_uint = 0x8D56;
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_EXT: c_uint = 0x8D56;
pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER: c_uint = 0x8CDC;
pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER_EXT: c_uint = 0x8CDC;
pub const GL_FRAMEBUFFER_PROGRAMMABLE_SAMPLE_LOCATIONS_ARB: c_uint = 0x9342;
pub const GL_FRAMEBUFFER_RENDERABLE: c_uint = 0x8289;
pub const GL_FRAMEBUFFER_RENDERABLE_LAYERED: c_uint = 0x828A;
pub const GL_FRAMEBUFFER_SAMPLE_LOCATION_PIXEL_GRID_ARB: c_uint = 0x9343;
pub const GL_FRAMEBUFFER_SRGB: c_uint = 0x8DB9;
pub const GL_FRAMEBUFFER_SRGB_CAPABLE_EXT: c_uint = 0x8DBA;
pub const GL_FRAMEBUFFER_SRGB_EXT: c_uint = 0x8DB9;
pub const GL_FRAMEBUFFER_UNDEFINED: c_uint = 0x8219;
pub const GL_FRAMEBUFFER_UNSUPPORTED: c_uint = 0x8CDD;
pub const GL_FRAMEBUFFER_UNSUPPORTED_EXT: c_uint = 0x8CDD;
pub const GL_FRONT: c_uint = 0x0404;
pub const GL_FRONT_AND_BACK: c_uint = 0x0408;
pub const GL_FRONT_FACE: c_uint = 0x0B46;
pub const GL_FRONT_LEFT: c_uint = 0x0400;
pub const GL_FRONT_RIGHT: c_uint = 0x0401;
pub const GL_FULL_SUPPORT: c_uint = 0x82B7;
pub const GL_FUNC_ADD: c_uint = 0x8006;
pub const GL_FUNC_REVERSE_SUBTRACT: c_uint = 0x800B;
pub const GL_FUNC_SUBTRACT: c_uint = 0x800A;
pub const GL_GEOMETRY_INPUT_TYPE: c_uint = 0x8917;
pub const GL_GEOMETRY_INPUT_TYPE_ARB: c_uint = 0x8DDB;
pub const GL_GEOMETRY_OUTPUT_TYPE: c_uint = 0x8918;
pub const GL_GEOMETRY_OUTPUT_TYPE_ARB: c_uint = 0x8DDC;
pub const GL_GEOMETRY_SHADER: c_uint = 0x8DD9;
pub const GL_GEOMETRY_SHADER_ARB: c_uint = 0x8DD9;
pub const GL_GEOMETRY_SHADER_BIT: c_uint = 0x00000004;
pub const GL_GEOMETRY_SHADER_INVOCATIONS: c_uint = 0x887F;
pub const GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED: c_uint = 0x82F3;
pub const GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED_ARB: c_uint = 0x82F3;
pub const GL_GEOMETRY_SUBROUTINE: c_uint = 0x92EB;
pub const GL_GEOMETRY_SUBROUTINE_UNIFORM: c_uint = 0x92F1;
pub const GL_GEOMETRY_TEXTURE: c_uint = 0x829E;
pub const GL_GEOMETRY_VERTICES_OUT: c_uint = 0x8916;
pub const GL_GEOMETRY_VERTICES_OUT_ARB: c_uint = 0x8DDA;
pub const GL_GEQUAL: c_uint = 0x0206;
pub const GL_GET_TEXTURE_IMAGE_FORMAT: c_uint = 0x8291;
pub const GL_GET_TEXTURE_IMAGE_TYPE: c_uint = 0x8292;
pub const GL_GREATER: c_uint = 0x0204;
pub const GL_GREEN: c_uint = 0x1904;
pub const GL_GREEN_INTEGER: c_uint = 0x8D95;
pub const GL_HALF_FLOAT: c_uint = 0x140B;
pub const GL_HALF_FLOAT_ARB: c_uint = 0x140B;
pub const GL_HIGH_FLOAT: c_uint = 0x8DF2;
pub const GL_HIGH_INT: c_uint = 0x8DF5;
pub const GL_IMAGE_1D: c_uint = 0x904C;
pub const GL_IMAGE_1D_ARRAY: c_uint = 0x9052;
pub const GL_IMAGE_2D: c_uint = 0x904D;
pub const GL_IMAGE_2D_ARRAY: c_uint = 0x9053;
pub const GL_IMAGE_2D_MULTISAMPLE: c_uint = 0x9055;
pub const GL_IMAGE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9056;
pub const GL_IMAGE_2D_RECT: c_uint = 0x904F;
pub const GL_IMAGE_3D: c_uint = 0x904E;
pub const GL_IMAGE_BINDING_ACCESS: c_uint = 0x8F3E;
pub const GL_IMAGE_BINDING_FORMAT: c_uint = 0x906E;
pub const GL_IMAGE_BINDING_LAYER: c_uint = 0x8F3D;
pub const GL_IMAGE_BINDING_LAYERED: c_uint = 0x8F3C;
pub const GL_IMAGE_BINDING_LEVEL: c_uint = 0x8F3B;
pub const GL_IMAGE_BINDING_NAME: c_uint = 0x8F3A;
pub const GL_IMAGE_BUFFER: c_uint = 0x9051;
pub const GL_IMAGE_CLASS_10_10_10_2: c_uint = 0x82C3;
pub const GL_IMAGE_CLASS_11_11_10: c_uint = 0x82C2;
pub const GL_IMAGE_CLASS_1_X_16: c_uint = 0x82BE;
pub const GL_IMAGE_CLASS_1_X_32: c_uint = 0x82BB;
pub const GL_IMAGE_CLASS_1_X_8: c_uint = 0x82C1;
pub const GL_IMAGE_CLASS_2_X_16: c_uint = 0x82BD;
pub const GL_IMAGE_CLASS_2_X_32: c_uint = 0x82BA;
pub const GL_IMAGE_CLASS_2_X_8: c_uint = 0x82C0;
pub const GL_IMAGE_CLASS_4_X_16: c_uint = 0x82BC;
pub const GL_IMAGE_CLASS_4_X_32: c_uint = 0x82B9;
pub const GL_IMAGE_CLASS_4_X_8: c_uint = 0x82BF;
pub const GL_IMAGE_COMPATIBILITY_CLASS: c_uint = 0x82A8;
pub const GL_IMAGE_CUBE: c_uint = 0x9050;
pub const GL_IMAGE_CUBE_MAP_ARRAY: c_uint = 0x9054;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: c_uint = 0x90C9;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: c_uint = 0x90C8;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_TYPE: c_uint = 0x90C7;
pub const GL_IMAGE_PIXEL_FORMAT: c_uint = 0x82A9;
pub const GL_IMAGE_PIXEL_TYPE: c_uint = 0x82AA;
pub const GL_IMAGE_TEXEL_SIZE: c_uint = 0x82A7;
pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: c_uint = 0x8B9B;
pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: c_uint = 0x8B9A;
pub const GL_INCR: c_uint = 0x1E02;
pub const GL_INCR_WRAP: c_uint = 0x8507;
pub const GL_INDEX_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8899;
pub const GL_INFO_LOG_LENGTH: c_uint = 0x8B84;
pub const GL_INT: c_uint = 0x1404;
pub const GL_INT64_ARB: c_uint = 0x140E;
pub const GL_INT64_VEC2_ARB: c_uint = 0x8FE9;
pub const GL_INT64_VEC3_ARB: c_uint = 0x8FEA;
pub const GL_INT64_VEC4_ARB: c_uint = 0x8FEB;
pub const GL_INTENSITY16F_ARB: c_uint = 0x881D;
pub const GL_INTENSITY32F_ARB: c_uint = 0x8817;
pub const GL_INTERLEAVED_ATTRIBS: c_uint = 0x8C8C;
pub const GL_INTERNALFORMAT_ALPHA_SIZE: c_uint = 0x8274;
pub const GL_INTERNALFORMAT_ALPHA_TYPE: c_uint = 0x827B;
pub const GL_INTERNALFORMAT_BLUE_SIZE: c_uint = 0x8273;
pub const GL_INTERNALFORMAT_BLUE_TYPE: c_uint = 0x827A;
pub const GL_INTERNALFORMAT_DEPTH_SIZE: c_uint = 0x8275;
pub const GL_INTERNALFORMAT_DEPTH_TYPE: c_uint = 0x827C;
pub const GL_INTERNALFORMAT_GREEN_SIZE: c_uint = 0x8272;
pub const GL_INTERNALFORMAT_GREEN_TYPE: c_uint = 0x8279;
pub const GL_INTERNALFORMAT_PREFERRED: c_uint = 0x8270;
pub const GL_INTERNALFORMAT_RED_SIZE: c_uint = 0x8271;
pub const GL_INTERNALFORMAT_RED_TYPE: c_uint = 0x8278;
pub const GL_INTERNALFORMAT_SHARED_SIZE: c_uint = 0x8277;
pub const GL_INTERNALFORMAT_STENCIL_SIZE: c_uint = 0x8276;
pub const GL_INTERNALFORMAT_STENCIL_TYPE: c_uint = 0x827D;
pub const GL_INTERNALFORMAT_SUPPORTED: c_uint = 0x826F;
pub const GL_INT_2_10_10_10_REV: c_uint = 0x8D9F;
pub const GL_INT_IMAGE_1D: c_uint = 0x9057;
pub const GL_INT_IMAGE_1D_ARRAY: c_uint = 0x905D;
pub const GL_INT_IMAGE_2D: c_uint = 0x9058;
pub const GL_INT_IMAGE_2D_ARRAY: c_uint = 0x905E;
pub const GL_INT_IMAGE_2D_MULTISAMPLE: c_uint = 0x9060;
pub const GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9061;
pub const GL_INT_IMAGE_2D_RECT: c_uint = 0x905A;
pub const GL_INT_IMAGE_3D: c_uint = 0x9059;
pub const GL_INT_IMAGE_BUFFER: c_uint = 0x905C;
pub const GL_INT_IMAGE_CUBE: c_uint = 0x905B;
pub const GL_INT_IMAGE_CUBE_MAP_ARRAY: c_uint = 0x905F;
pub const GL_INT_SAMPLER_1D: c_uint = 0x8DC9;
pub const GL_INT_SAMPLER_1D_ARRAY: c_uint = 0x8DCE;
pub const GL_INT_SAMPLER_2D: c_uint = 0x8DCA;
pub const GL_INT_SAMPLER_2D_ARRAY: c_uint = 0x8DCF;
pub const GL_INT_SAMPLER_2D_MULTISAMPLE: c_uint = 0x9109;
pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: c_uint = 0x910C;
pub const GL_INT_SAMPLER_2D_RECT: c_uint = 0x8DCD;
pub const GL_INT_SAMPLER_3D: c_uint = 0x8DCB;
pub const GL_INT_SAMPLER_BUFFER: c_uint = 0x8DD0;
pub const GL_INT_SAMPLER_CUBE: c_uint = 0x8DCC;
pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY: c_uint = 0x900E;
pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY_ARB: c_uint = 0x900E;
pub const GL_INT_VEC2: c_uint = 0x8B53;
pub const GL_INT_VEC2_ARB: c_uint = 0x8B53;
pub const GL_INT_VEC3: c_uint = 0x8B54;
pub const GL_INT_VEC3_ARB: c_uint = 0x8B54;
pub const GL_INT_VEC4: c_uint = 0x8B55;
pub const GL_INT_VEC4_ARB: c_uint = 0x8B55;
pub const GL_INVALID_ENUM: c_uint = 0x0500;
pub const GL_INVALID_FRAMEBUFFER_OPERATION: c_uint = 0x0506;
pub const GL_INVALID_FRAMEBUFFER_OPERATION_EXT: c_uint = 0x0506;
pub const GL_INVALID_INDEX: c_uint = 0xFFFFFFFF;
pub const GL_INVALID_OPERATION: c_uint = 0x0502;
pub const GL_INVALID_VALUE: c_uint = 0x0501;
pub const GL_INVERT: c_uint = 0x150A;
pub const GL_ISOLINES: c_uint = 0x8E7A;
pub const GL_IS_PER_PATCH: c_uint = 0x92E7;
pub const GL_IS_ROW_MAJOR: c_uint = 0x9300;
pub const GL_KEEP: c_uint = 0x1E00;
pub const GL_LAST_VERTEX_CONVENTION: c_uint = 0x8E4E;
pub const GL_LAYER_PROVOKING_VERTEX: c_uint = 0x825E;
pub const GL_LEFT: c_uint = 0x0406;
pub const GL_LEQUAL: c_uint = 0x0203;
pub const GL_LESS: c_uint = 0x0201;
pub const GL_LINE: c_uint = 0x1B01;
pub const GL_LINEAR: c_uint = 0x2601;
pub const GL_LINEAR_MIPMAP_LINEAR: c_uint = 0x2703;
pub const GL_LINEAR_MIPMAP_NEAREST: c_uint = 0x2701;
pub const GL_LINES: c_uint = 0x0001;
pub const GL_LINES_ADJACENCY: c_uint = 0x000A;
pub const GL_LINES_ADJACENCY_ARB: c_uint = 0x000A;
pub const GL_LINE_LOOP: c_uint = 0x0002;
pub const GL_LINE_SMOOTH: c_uint = 0x0B20;
pub const GL_LINE_SMOOTH_HINT: c_uint = 0x0C52;
pub const GL_LINE_STRIP: c_uint = 0x0003;
pub const GL_LINE_STRIP_ADJACENCY: c_uint = 0x000B;
pub const GL_LINE_STRIP_ADJACENCY_ARB: c_uint = 0x000B;
pub const GL_LINE_WIDTH: c_uint = 0x0B21;
pub const GL_LINE_WIDTH_GRANULARITY: c_uint = 0x0B23;
pub const GL_LINE_WIDTH_RANGE: c_uint = 0x0B22;
pub const GL_LINK_STATUS: c_uint = 0x8B82;
pub const GL_LOCATION: c_uint = 0x930E;
pub const GL_LOCATION_COMPONENT: c_uint = 0x934A;
pub const GL_LOCATION_INDEX: c_uint = 0x930F;
pub const GL_LOGIC_OP_MODE: c_uint = 0x0BF0;
pub const GL_LOWER_LEFT: c_uint = 0x8CA1;
pub const GL_LOW_FLOAT: c_uint = 0x8DF0;
pub const GL_LOW_INT: c_uint = 0x8DF3;
pub const GL_LUMINANCE16F_ARB: c_uint = 0x881E;
pub const GL_LUMINANCE32F_ARB: c_uint = 0x8818;
pub const GL_LUMINANCE_ALPHA16F_ARB: c_uint = 0x881F;
pub const GL_LUMINANCE_ALPHA32F_ARB: c_uint = 0x8819;
pub const GL_MAJOR_VERSION: c_uint = 0x821B;
pub const GL_MANUAL_GENERATE_MIPMAP: c_uint = 0x8294;
pub const GL_MAP_COHERENT_BIT: c_uint = 0x0080;
pub const GL_MAP_FLUSH_EXPLICIT_BIT: c_uint = 0x0010;
pub const GL_MAP_INVALIDATE_BUFFER_BIT: c_uint = 0x0008;
pub const GL_MAP_INVALIDATE_RANGE_BIT: c_uint = 0x0004;
pub const GL_MAP_PERSISTENT_BIT: c_uint = 0x0040;
pub const GL_MAP_READ_BIT: c_uint = 0x0001;
pub const GL_MAP_UNSYNCHRONIZED_BIT: c_uint = 0x0020;
pub const GL_MAP_WRITE_BIT: c_uint = 0x0002;
pub const GL_MATRIX0_ARB: c_uint = 0x88C0;
pub const GL_MATRIX10_ARB: c_uint = 0x88CA;
pub const GL_MATRIX11_ARB: c_uint = 0x88CB;
pub const GL_MATRIX12_ARB: c_uint = 0x88CC;
pub const GL_MATRIX13_ARB: c_uint = 0x88CD;
pub const GL_MATRIX14_ARB: c_uint = 0x88CE;
pub const GL_MATRIX15_ARB: c_uint = 0x88CF;
pub const GL_MATRIX16_ARB: c_uint = 0x88D0;
pub const GL_MATRIX17_ARB: c_uint = 0x88D1;
pub const GL_MATRIX18_ARB: c_uint = 0x88D2;
pub const GL_MATRIX19_ARB: c_uint = 0x88D3;
pub const GL_MATRIX1_ARB: c_uint = 0x88C1;
pub const GL_MATRIX20_ARB: c_uint = 0x88D4;
pub const GL_MATRIX21_ARB: c_uint = 0x88D5;
pub const GL_MATRIX22_ARB: c_uint = 0x88D6;
pub const GL_MATRIX23_ARB: c_uint = 0x88D7;
pub const GL_MATRIX24_ARB: c_uint = 0x88D8;
pub const GL_MATRIX25_ARB: c_uint = 0x88D9;
pub const GL_MATRIX26_ARB: c_uint = 0x88DA;
pub const GL_MATRIX27_ARB: c_uint = 0x88DB;
pub const GL_MATRIX28_ARB: c_uint = 0x88DC;
pub const GL_MATRIX29_ARB: c_uint = 0x88DD;
pub const GL_MATRIX2_ARB: c_uint = 0x88C2;
pub const GL_MATRIX30_ARB: c_uint = 0x88DE;
pub const GL_MATRIX31_ARB: c_uint = 0x88DF;
pub const GL_MATRIX3_ARB: c_uint = 0x88C3;
pub const GL_MATRIX4_ARB: c_uint = 0x88C4;
pub const GL_MATRIX5_ARB: c_uint = 0x88C5;
pub const GL_MATRIX6_ARB: c_uint = 0x88C6;
pub const GL_MATRIX7_ARB: c_uint = 0x88C7;
pub const GL_MATRIX8_ARB: c_uint = 0x88C8;
pub const GL_MATRIX9_ARB: c_uint = 0x88C9;
pub const GL_MATRIX_STRIDE: c_uint = 0x92FF;
pub const GL_MAX: c_uint = 0x8008;
pub const GL_MAX_3D_TEXTURE_SIZE: c_uint = 0x8073;
pub const GL_MAX_ARRAY_TEXTURE_LAYERS: c_uint = 0x88FF;
pub const GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: c_uint = 0x92DC;
pub const GL_MAX_ATOMIC_COUNTER_BUFFER_SIZE: c_uint = 0x92D8;
pub const GL_MAX_CLIP_DISTANCES: c_uint = 0x0D32;
pub const GL_MAX_COLOR_ATTACHMENTS: c_uint = 0x8CDF;
pub const GL_MAX_COLOR_ATTACHMENTS_EXT: c_uint = 0x8CDF;
pub const GL_MAX_COLOR_TEXTURE_SAMPLES: c_uint = 0x910E;
pub const GL_MAX_COMBINED_ATOMIC_COUNTERS: c_uint = 0x92D7;
pub const GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92D1;
pub const GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: c_uint = 0x8266;
pub const GL_MAX_COMBINED_DIMENSIONS: c_uint = 0x8282;
pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: c_uint = 0x8A33;
pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: c_uint = 0x8A32;
pub const GL_MAX_COMBINED_IMAGE_UNIFORMS: c_uint = 0x90CF;
pub const GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: c_uint = 0x8F39;
pub const GL_MAX_COMBINED_SHADER_OUTPUT_RESOURCES: c_uint = 0x8F39;
pub const GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS: c_uint = 0x90DC;
pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: c_uint = 0x8E1E;
pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: c_uint = 0x8E1F;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4D;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS_ARB: c_uint = 0x8B4D;
pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: c_uint = 0x8A2E;
pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: c_uint = 0x8A31;
pub const GL_MAX_COMPUTE_ATOMIC_COUNTERS: c_uint = 0x8265;
pub const GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: c_uint = 0x8264;
pub const GL_MAX_COMPUTE_FIXED_GROUP_INVOCATIONS_ARB: c_uint = 0x90EB;
pub const GL_MAX_COMPUTE_FIXED_GROUP_SIZE_ARB: c_uint = 0x91BF;
pub const GL_MAX_COMPUTE_IMAGE_UNIFORMS: c_uint = 0x91BD;
pub const GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS: c_uint = 0x90DB;
pub const GL_MAX_COMPUTE_SHARED_MEMORY_SIZE: c_uint = 0x8262;
pub const GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS: c_uint = 0x91BC;
pub const GL_MAX_COMPUTE_UNIFORM_BLOCKS: c_uint = 0x91BB;
pub const GL_MAX_COMPUTE_UNIFORM_COMPONENTS: c_uint = 0x8263;
pub const GL_MAX_COMPUTE_VARIABLE_GROUP_INVOCATIONS_ARB: c_uint = 0x9344;
pub const GL_MAX_COMPUTE_VARIABLE_GROUP_SIZE_ARB: c_uint = 0x9345;
pub const GL_MAX_COMPUTE_WORK_GROUP_COUNT: c_uint = 0x91BE;
pub const GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS: c_uint = 0x90EB;
pub const GL_MAX_COMPUTE_WORK_GROUP_SIZE: c_uint = 0x91BF;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: c_uint = 0x851C;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE_ARB: c_uint = 0x851C;
pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH: c_uint = 0x826C;
pub const GL_MAX_DEBUG_LOGGED_MESSAGES: c_uint = 0x9144;
pub const GL_MAX_DEBUG_LOGGED_MESSAGES_ARB: c_uint = 0x9144;
pub const GL_MAX_DEBUG_MESSAGE_LENGTH: c_uint = 0x9143;
pub const GL_MAX_DEBUG_MESSAGE_LENGTH_ARB: c_uint = 0x9143;
pub const GL_MAX_DEPTH: c_uint = 0x8280;
pub const GL_MAX_DEPTH_TEXTURE_SAMPLES: c_uint = 0x910F;
pub const GL_MAX_DRAW_BUFFERS: c_uint = 0x8824;
pub const GL_MAX_DRAW_BUFFERS_ARB: c_uint = 0x8824;
pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS: c_uint = 0x88FC;
pub const GL_MAX_ELEMENTS_INDICES: c_uint = 0x80E9;
pub const GL_MAX_ELEMENTS_VERTICES: c_uint = 0x80E8;
pub const GL_MAX_ELEMENT_INDEX: c_uint = 0x8D6B;
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTERS: c_uint = 0x92D6;
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92D0;
pub const GL_MAX_FRAGMENT_IMAGE_UNIFORMS: c_uint = 0x90CE;
pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: c_uint = 0x9125;
pub const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET: c_uint = 0x8E5C;
pub const GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: c_uint = 0x90DA;
pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: c_uint = 0x8A2D;
pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: c_uint = 0x8B49;
pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS_ARB: c_uint = 0x8B49;
pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS: c_uint = 0x8DFD;
pub const GL_MAX_FRAMEBUFFER_HEIGHT: c_uint = 0x9316;
pub const GL_MAX_FRAMEBUFFER_LAYERS: c_uint = 0x9317;
pub const GL_MAX_FRAMEBUFFER_SAMPLES: c_uint = 0x9318;
pub const GL_MAX_FRAMEBUFFER_WIDTH: c_uint = 0x9315;
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS: c_uint = 0x92D5;
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92CF;
pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS: c_uint = 0x90CD;
pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS: c_uint = 0x9123;
pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: c_uint = 0x9124;
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES: c_uint = 0x8DE0;
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES_ARB: c_uint = 0x8DE0;
pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS: c_uint = 0x8E5A;
pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: c_uint = 0x90D7;
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: c_uint = 0x8C29;
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS_ARB: c_uint = 0x8C29;
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: c_uint = 0x8DE1;
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS_ARB: c_uint = 0x8DE1;
pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS: c_uint = 0x8A2C;
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: c_uint = 0x8DDF;
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS_ARB: c_uint = 0x8DDF;
pub const GL_MAX_GEOMETRY_VARYING_COMPONENTS_ARB: c_uint = 0x8DDD;
pub const GL_MAX_HEIGHT: c_uint = 0x827F;
pub const GL_MAX_IMAGE_SAMPLES: c_uint = 0x906D;
pub const GL_MAX_IMAGE_UNITS: c_uint = 0x8F38;
pub const GL_MAX_INTEGER_SAMPLES: c_uint = 0x9110;
pub const GL_MAX_LABEL_LENGTH: c_uint = 0x82E8;
pub const GL_MAX_LAYERS: c_uint = 0x8281;
pub const GL_MAX_NAME_LENGTH: c_uint = 0x92F6;
pub const GL_MAX_NUM_ACTIVE_VARIABLES: c_uint = 0x92F7;
pub const GL_MAX_NUM_COMPATIBLE_SUBROUTINES: c_uint = 0x92F8;
pub const GL_MAX_PATCH_VERTICES: c_uint = 0x8E7D;
pub const GL_MAX_PROGRAM_ADDRESS_REGISTERS_ARB: c_uint = 0x88B1;
pub const GL_MAX_PROGRAM_ALU_INSTRUCTIONS_ARB: c_uint = 0x880B;
pub const GL_MAX_PROGRAM_ATTRIBS_ARB: c_uint = 0x88AD;
pub const GL_MAX_PROGRAM_ENV_PARAMETERS_ARB: c_uint = 0x88B5;
pub const GL_MAX_PROGRAM_INSTRUCTIONS_ARB: c_uint = 0x88A1;
pub const GL_MAX_PROGRAM_LOCAL_PARAMETERS_ARB: c_uint = 0x88B4;
pub const GL_MAX_PROGRAM_MATRICES_ARB: c_uint = 0x862F;
pub const GL_MAX_PROGRAM_MATRIX_STACK_DEPTH_ARB: c_uint = 0x862E;
pub const GL_MAX_PROGRAM_NATIVE_ADDRESS_REGISTERS_ARB: c_uint = 0x88B3;
pub const GL_MAX_PROGRAM_NATIVE_ALU_INSTRUCTIONS_ARB: c_uint = 0x880E;
pub const GL_MAX_PROGRAM_NATIVE_ATTRIBS_ARB: c_uint = 0x88AF;
pub const GL_MAX_PROGRAM_NATIVE_INSTRUCTIONS_ARB: c_uint = 0x88A3;
pub const GL_MAX_PROGRAM_NATIVE_PARAMETERS_ARB: c_uint = 0x88AB;
pub const GL_MAX_PROGRAM_NATIVE_TEMPORARIES_ARB: c_uint = 0x88A7;
pub const GL_MAX_PROGRAM_NATIVE_TEX_INDIRECTIONS_ARB: c_uint = 0x8810;
pub const GL_MAX_PROGRAM_NATIVE_TEX_INSTRUCTIONS_ARB: c_uint = 0x880F;
pub const GL_MAX_PROGRAM_PARAMETERS_ARB: c_uint = 0x88A9;
pub const GL_MAX_PROGRAM_TEMPORARIES_ARB: c_uint = 0x88A5;
pub const GL_MAX_PROGRAM_TEXEL_OFFSET: c_uint = 0x8905;
pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET: c_uint = 0x8E5F;
pub const GL_MAX_PROGRAM_TEX_INDIRECTIONS_ARB: c_uint = 0x880D;
pub const GL_MAX_PROGRAM_TEX_INSTRUCTIONS_ARB: c_uint = 0x880C;
pub const GL_MAX_RECTANGLE_TEXTURE_SIZE: c_uint = 0x84F8;
pub const GL_MAX_RENDERBUFFER_SIZE: c_uint = 0x84E8;
pub const GL_MAX_RENDERBUFFER_SIZE_EXT: c_uint = 0x84E8;
pub const GL_MAX_SAMPLES: c_uint = 0x8D57;
pub const GL_MAX_SAMPLES_EXT: c_uint = 0x8D57;
pub const GL_MAX_SAMPLE_MASK_WORDS: c_uint = 0x8E59;
pub const GL_MAX_SERVER_WAIT_TIMEOUT: c_uint = 0x9111;
pub const GL_MAX_SHADER_STORAGE_BLOCK_SIZE: c_uint = 0x90DE;
pub const GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS: c_uint = 0x90DD;
pub const GL_MAX_SUBROUTINES: c_uint = 0x8DE7;
pub const GL_MAX_SUBROUTINE_UNIFORM_LOCATIONS: c_uint = 0x8DE8;
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS: c_uint = 0x92D3;
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92CD;
pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS: c_uint = 0x90CB;
pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS: c_uint = 0x886C;
pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS: c_uint = 0x8E83;
pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: c_uint = 0x90D8;
pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: c_uint = 0x8E81;
pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: c_uint = 0x8E85;
pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS: c_uint = 0x8E89;
pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS: c_uint = 0x8E7F;
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS: c_uint = 0x92D4;
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92CE;
pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS: c_uint = 0x90CC;
pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS: c_uint = 0x886D;
pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: c_uint = 0x8E86;
pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: c_uint = 0x90D9;
pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: c_uint = 0x8E82;
pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS: c_uint = 0x8E8A;
pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: c_uint = 0x8E80;
pub const GL_MAX_TESS_GEN_LEVEL: c_uint = 0x8E7E;
pub const GL_MAX_TESS_PATCH_COMPONENTS: c_uint = 0x8E84;
pub const GL_MAX_TEXTURE_BUFFER_SIZE: c_uint = 0x8C2B;
pub const GL_MAX_TEXTURE_COORDS_ARB: c_uint = 0x8871;
pub const GL_MAX_TEXTURE_IMAGE_UNITS: c_uint = 0x8872;
pub const GL_MAX_TEXTURE_IMAGE_UNITS_ARB: c_uint = 0x8872;
pub const GL_MAX_TEXTURE_LOD_BIAS: c_uint = 0x84FD;
pub const GL_MAX_TEXTURE_MAX_ANISOTROPY: c_uint = 0x84FF;
pub const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: c_uint = 0x84FF;
pub const GL_MAX_TEXTURE_SIZE: c_uint = 0x0D33;
pub const GL_MAX_TEXTURE_UNITS_ARB: c_uint = 0x84E2;
pub const GL_MAX_TRANSFORM_FEEDBACK_BUFFERS: c_uint = 0x8E70;
pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: c_uint = 0x8C8A;
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: c_uint = 0x8C8B;
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: c_uint = 0x8C80;
pub const GL_MAX_UNIFORM_BLOCK_SIZE: c_uint = 0x8A30;
pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: c_uint = 0x8A2F;
pub const GL_MAX_UNIFORM_LOCATIONS: c_uint = 0x826E;
pub const GL_MAX_VARYING_COMPONENTS: c_uint = 0x8B4B;
pub const GL_MAX_VARYING_FLOATS: c_uint = 0x8B4B;
pub const GL_MAX_VARYING_FLOATS_ARB: c_uint = 0x8B4B;
pub const GL_MAX_VARYING_VECTORS: c_uint = 0x8DFC;
pub const GL_MAX_VERTEX_ATOMIC_COUNTERS: c_uint = 0x92D2;
pub const GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92CC;
pub const GL_MAX_VERTEX_ATTRIBS: c_uint = 0x8869;
pub const GL_MAX_VERTEX_ATTRIBS_ARB: c_uint = 0x8869;
pub const GL_MAX_VERTEX_ATTRIB_BINDINGS: c_uint = 0x82DA;
pub const GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: c_uint = 0x82D9;
pub const GL_MAX_VERTEX_IMAGE_UNIFORMS: c_uint = 0x90CA;
pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: c_uint = 0x9122;
pub const GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS: c_uint = 0x90D6;
pub const GL_MAX_VERTEX_STREAMS: c_uint = 0x8E71;
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4C;
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS_ARB: c_uint = 0x8B4C;
pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: c_uint = 0x8A2B;
pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: c_uint = 0x8B4A;
pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS_ARB: c_uint = 0x8B4A;
pub const GL_MAX_VERTEX_UNIFORM_VECTORS: c_uint = 0x8DFB;
pub const GL_MAX_VERTEX_VARYING_COMPONENTS_ARB: c_uint = 0x8DDE;
pub const GL_MAX_VIEWPORTS: c_uint = 0x825B;
pub const GL_MAX_VIEWPORT_DIMS: c_uint = 0x0D3A;
pub const GL_MAX_WIDTH: c_uint = 0x827E;
pub const GL_MEDIUM_FLOAT: c_uint = 0x8DF1;
pub const GL_MEDIUM_INT: c_uint = 0x8DF4;
pub const GL_MIN: c_uint = 0x8007;
pub const GL_MINOR_VERSION: c_uint = 0x821C;
pub const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET: c_uint = 0x8E5B;
pub const GL_MIN_MAP_BUFFER_ALIGNMENT: c_uint = 0x90BC;
pub const GL_MIN_PROGRAM_TEXEL_OFFSET: c_uint = 0x8904;
pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET: c_uint = 0x8E5E;
pub const GL_MIN_SAMPLE_SHADING_VALUE: c_uint = 0x8C37;
pub const GL_MIN_SAMPLE_SHADING_VALUE_ARB: c_uint = 0x8C37;
pub const GL_MIPMAP: c_uint = 0x8293;
pub const GL_MIRRORED_REPEAT: c_uint = 0x8370;
pub const GL_MIRRORED_REPEAT_ARB: c_uint = 0x8370;
pub const GL_MIRROR_CLAMP_EXT: c_uint = 0x8742;
pub const GL_MIRROR_CLAMP_TO_BORDER_EXT: c_uint = 0x8912;
pub const GL_MIRROR_CLAMP_TO_EDGE: c_uint = 0x8743;
pub const GL_MIRROR_CLAMP_TO_EDGE_EXT: c_uint = 0x8743;
pub const GL_MULTISAMPLE: c_uint = 0x809D;
pub const GL_MULTISAMPLE_ARB: c_uint = 0x809D;
pub const GL_MULTISAMPLE_BIT_ARB: c_uint = 0x20000000;
pub const GL_MULTISAMPLE_LINE_WIDTH_GRANULARITY_ARB: c_uint = 0x9382;
pub const GL_MULTISAMPLE_LINE_WIDTH_RANGE_ARB: c_uint = 0x9381;
pub const GL_NAMED_STRING_LENGTH_ARB: c_uint = 0x8DE9;
pub const GL_NAMED_STRING_TYPE_ARB: c_uint = 0x8DEA;
pub const GL_NAME_LENGTH: c_uint = 0x92F9;
pub const GL_NAND: c_uint = 0x150E;
pub const GL_NEAREST: c_uint = 0x2600;
pub const GL_NEAREST_MIPMAP_LINEAR: c_uint = 0x2702;
pub const GL_NEAREST_MIPMAP_NEAREST: c_uint = 0x2700;
pub const GL_NEVER: c_uint = 0x0200;
pub const GL_NICEST: c_uint = 0x1102;
pub const GL_NONE: c_uint = 0;
pub const GL_NOOP: c_uint = 0x1505;
pub const GL_NOR: c_uint = 0x1508;
pub const GL_NORMAL_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8897;
pub const GL_NORMAL_MAP_ARB: c_uint = 0x8511;
pub const GL_NOTEQUAL: c_uint = 0x0205;
pub const GL_NO_ERROR: c_uint = 0;
pub const GL_NUM_ACTIVE_VARIABLES: c_uint = 0x9304;
pub const GL_NUM_COMPATIBLE_SUBROUTINES: c_uint = 0x8E4A;
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: c_uint = 0x86A2;
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS_ARB: c_uint = 0x86A2;
pub const GL_NUM_EXTENSIONS: c_uint = 0x821D;
pub const GL_NUM_PROGRAM_BINARY_FORMATS: c_uint = 0x87FE;
pub const GL_NUM_SAMPLE_COUNTS: c_uint = 0x9380;
pub const GL_NUM_SHADER_BINARY_FORMATS: c_uint = 0x8DF9;
pub const GL_NUM_SHADING_LANGUAGE_VERSIONS: c_uint = 0x82E9;
pub const GL_NUM_SPIR_V_EXTENSIONS: c_uint = 0x9554;
pub const GL_OBJECT_ACTIVE_ATTRIBUTES_ARB: c_uint = 0x8B89;
pub const GL_OBJECT_ACTIVE_ATTRIBUTE_MAX_LENGTH_ARB: c_uint = 0x8B8A;
pub const GL_OBJECT_ACTIVE_UNIFORMS_ARB: c_uint = 0x8B86;
pub const GL_OBJECT_ACTIVE_UNIFORM_MAX_LENGTH_ARB: c_uint = 0x8B87;
pub const GL_OBJECT_ATTACHED_OBJECTS_ARB: c_uint = 0x8B85;
pub const GL_OBJECT_COMPILE_STATUS_ARB: c_uint = 0x8B81;
pub const GL_OBJECT_DELETE_STATUS_ARB: c_uint = 0x8B80;
pub const GL_OBJECT_INFO_LOG_LENGTH_ARB: c_uint = 0x8B84;
pub const GL_OBJECT_LINK_STATUS_ARB: c_uint = 0x8B82;
pub const GL_OBJECT_SHADER_SOURCE_LENGTH_ARB: c_uint = 0x8B88;
pub const GL_OBJECT_SUBTYPE_ARB: c_uint = 0x8B4F;
pub const GL_OBJECT_TYPE: c_uint = 0x9112;
pub const GL_OBJECT_TYPE_ARB: c_uint = 0x8B4E;
pub const GL_OBJECT_VALIDATE_STATUS_ARB: c_uint = 0x8B83;
pub const GL_OFFSET: c_uint = 0x92FC;
pub const GL_ONE: c_uint = 1;
pub const GL_ONE_MINUS_CONSTANT_ALPHA: c_uint = 0x8004;
pub const GL_ONE_MINUS_CONSTANT_COLOR: c_uint = 0x8002;
pub const GL_ONE_MINUS_DST_ALPHA: c_uint = 0x0305;
pub const GL_ONE_MINUS_DST_COLOR: c_uint = 0x0307;
pub const GL_ONE_MINUS_SRC1_ALPHA: c_uint = 0x88FB;
pub const GL_ONE_MINUS_SRC1_COLOR: c_uint = 0x88FA;
pub const GL_ONE_MINUS_SRC_ALPHA: c_uint = 0x0303;
pub const GL_ONE_MINUS_SRC_COLOR: c_uint = 0x0301;
pub const GL_OR: c_uint = 0x1507;
pub const GL_OR_INVERTED: c_uint = 0x150D;
pub const GL_OR_REVERSE: c_uint = 0x150B;
pub const GL_OUT_OF_MEMORY: c_uint = 0x0505;
pub const GL_PACK_ALIGNMENT: c_uint = 0x0D05;
pub const GL_PACK_COMPRESSED_BLOCK_DEPTH: c_uint = 0x912D;
pub const GL_PACK_COMPRESSED_BLOCK_HEIGHT: c_uint = 0x912C;
pub const GL_PACK_COMPRESSED_BLOCK_SIZE: c_uint = 0x912E;
pub const GL_PACK_COMPRESSED_BLOCK_WIDTH: c_uint = 0x912B;
pub const GL_PACK_IMAGE_HEIGHT: c_uint = 0x806C;
pub const GL_PACK_LSB_FIRST: c_uint = 0x0D01;
pub const GL_PACK_ROW_LENGTH: c_uint = 0x0D02;
pub const GL_PACK_SKIP_IMAGES: c_uint = 0x806B;
pub const GL_PACK_SKIP_PIXELS: c_uint = 0x0D04;
pub const GL_PACK_SKIP_ROWS: c_uint = 0x0D03;
pub const GL_PACK_SWAP_BYTES: c_uint = 0x0D00;
pub const GL_PALETTE4_R5_G6_B5_OES: c_uint = 0x8B92;
pub const GL_PALETTE4_RGB5_A1_OES: c_uint = 0x8B94;
pub const GL_PALETTE4_RGB8_OES: c_uint = 0x8B90;
pub const GL_PALETTE4_RGBA4_OES: c_uint = 0x8B93;
pub const GL_PALETTE4_RGBA8_OES: c_uint = 0x8B91;
pub const GL_PALETTE8_R5_G6_B5_OES: c_uint = 0x8B97;
pub const GL_PALETTE8_RGB5_A1_OES: c_uint = 0x8B99;
pub const GL_PALETTE8_RGB8_OES: c_uint = 0x8B95;
pub const GL_PALETTE8_RGBA4_OES: c_uint = 0x8B98;
pub const GL_PALETTE8_RGBA8_OES: c_uint = 0x8B96;
pub const GL_PATCHES: c_uint = 0x000E;
pub const GL_PATCH_DEFAULT_INNER_LEVEL: c_uint = 0x8E73;
pub const GL_PATCH_DEFAULT_OUTER_LEVEL: c_uint = 0x8E74;
pub const GL_PATCH_VERTICES: c_uint = 0x8E72;
pub const GL_PIXEL_BUFFER_BARRIER_BIT: c_uint = 0x00000080;
pub const GL_PIXEL_PACK_BUFFER: c_uint = 0x88EB;
pub const GL_PIXEL_PACK_BUFFER_BINDING: c_uint = 0x88ED;
pub const GL_PIXEL_UNPACK_BUFFER: c_uint = 0x88EC;
pub const GL_PIXEL_UNPACK_BUFFER_BINDING: c_uint = 0x88EF;
pub const GL_POINT: c_uint = 0x1B00;
pub const GL_POINTS: c_uint = 0x0000;
pub const GL_POINT_FADE_THRESHOLD_SIZE: c_uint = 0x8128;
pub const GL_POINT_SIZE: c_uint = 0x0B11;
pub const GL_POINT_SIZE_GRANULARITY: c_uint = 0x0B13;
pub const GL_POINT_SIZE_RANGE: c_uint = 0x0B12;
pub const GL_POINT_SPRITE_COORD_ORIGIN: c_uint = 0x8CA0;
pub const GL_POLYGON_MODE: c_uint = 0x0B40;
pub const GL_POLYGON_OFFSET_FACTOR: c_uint = 0x8038;
pub const GL_POLYGON_OFFSET_FILL: c_uint = 0x8037;
pub const GL_POLYGON_OFFSET_LINE: c_uint = 0x2A02;
pub const GL_POLYGON_OFFSET_POINT: c_uint = 0x2A01;
pub const GL_POLYGON_OFFSET_UNITS: c_uint = 0x2A00;
pub const GL_POLYGON_SMOOTH: c_uint = 0x0B41;
pub const GL_POLYGON_SMOOTH_HINT: c_uint = 0x0C53;
pub const GL_PRIMITIVES_GENERATED: c_uint = 0x8C87;
pub const GL_PRIMITIVES_SUBMITTED: c_uint = 0x82EF;
pub const GL_PRIMITIVES_SUBMITTED_ARB: c_uint = 0x82EF;
pub const GL_PRIMITIVE_BOUNDING_BOX_ARB: c_uint = 0x92BE;
pub const GL_PRIMITIVE_RESTART: c_uint = 0x8F9D;
pub const GL_PRIMITIVE_RESTART_FIXED_INDEX: c_uint = 0x8D69;
pub const GL_PRIMITIVE_RESTART_INDEX: c_uint = 0x8F9E;
pub const GL_PROGRAM: c_uint = 0x82E2;
pub const GL_PROGRAMMABLE_SAMPLE_LOCATION_ARB: c_uint = 0x9341;
pub const GL_PROGRAMMABLE_SAMPLE_LOCATION_TABLE_SIZE_ARB: c_uint = 0x9340;
pub const GL_PROGRAM_ADDRESS_REGISTERS_ARB: c_uint = 0x88B0;
pub const GL_PROGRAM_ALU_INSTRUCTIONS_ARB: c_uint = 0x8805;
pub const GL_PROGRAM_ATTRIBS_ARB: c_uint = 0x88AC;
pub const GL_PROGRAM_BINARY_FORMATS: c_uint = 0x87FF;
pub const GL_PROGRAM_BINARY_LENGTH: c_uint = 0x8741;
pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT: c_uint = 0x8257;
pub const GL_PROGRAM_BINDING_ARB: c_uint = 0x8677;
pub const GL_PROGRAM_ERROR_POSITION_ARB: c_uint = 0x864B;
pub const GL_PROGRAM_ERROR_STRING_ARB: c_uint = 0x8874;
pub const GL_PROGRAM_FORMAT_ARB: c_uint = 0x8876;
pub const GL_PROGRAM_FORMAT_ASCII_ARB: c_uint = 0x8875;
pub const GL_PROGRAM_INPUT: c_uint = 0x92E3;
pub const GL_PROGRAM_INSTRUCTIONS_ARB: c_uint = 0x88A0;
pub const GL_PROGRAM_LENGTH_ARB: c_uint = 0x8627;
pub const GL_PROGRAM_NATIVE_ADDRESS_REGISTERS_ARB: c_uint = 0x88B2;
pub const GL_PROGRAM_NATIVE_ALU_INSTRUCTIONS_ARB: c_uint = 0x8808;
pub const GL_PROGRAM_NATIVE_ATTRIBS_ARB: c_uint = 0x88AE;
pub const GL_PROGRAM_NATIVE_INSTRUCTIONS_ARB: c_uint = 0x88A2;
pub const GL_PROGRAM_NATIVE_PARAMETERS_ARB: c_uint = 0x88AA;
pub const GL_PROGRAM_NATIVE_TEMPORARIES_ARB: c_uint = 0x88A6;
pub const GL_PROGRAM_NATIVE_TEX_INDIRECTIONS_ARB: c_uint = 0x880A;
pub const GL_PROGRAM_NATIVE_TEX_INSTRUCTIONS_ARB: c_uint = 0x8809;
pub const GL_PROGRAM_OBJECT_ARB: c_uint = 0x8B40;
pub const GL_PROGRAM_OUTPUT: c_uint = 0x92E4;
pub const GL_PROGRAM_PARAMETERS_ARB: c_uint = 0x88A8;
pub const GL_PROGRAM_PIPELINE: c_uint = 0x82E4;
pub const GL_PROGRAM_PIPELINE_BINDING: c_uint = 0x825A;
pub const GL_PROGRAM_POINT_SIZE: c_uint = 0x8642;
pub const GL_PROGRAM_POINT_SIZE_ARB: c_uint = 0x8642;
pub const GL_PROGRAM_SEPARABLE: c_uint = 0x8258;
pub const GL_PROGRAM_STRING_ARB: c_uint = 0x8628;
pub const GL_PROGRAM_TEMPORARIES_ARB: c_uint = 0x88A4;
pub const GL_PROGRAM_TEX_INDIRECTIONS_ARB: c_uint = 0x8807;
pub const GL_PROGRAM_TEX_INSTRUCTIONS_ARB: c_uint = 0x8806;
pub const GL_PROGRAM_UNDER_NATIVE_LIMITS_ARB: c_uint = 0x88B6;
pub const GL_PROVOKING_VERTEX: c_uint = 0x8E4F;
pub const GL_PROXY_TEXTURE_1D: c_uint = 0x8063;
pub const GL_PROXY_TEXTURE_1D_ARRAY: c_uint = 0x8C19;
pub const GL_PROXY_TEXTURE_2D: c_uint = 0x8064;
pub const GL_PROXY_TEXTURE_2D_ARRAY: c_uint = 0x8C1B;
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE: c_uint = 0x9101;
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9103;
pub const GL_PROXY_TEXTURE_3D: c_uint = 0x8070;
pub const GL_PROXY_TEXTURE_CUBE_MAP: c_uint = 0x851B;
pub const GL_PROXY_TEXTURE_CUBE_MAP_ARB: c_uint = 0x851B;
pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY: c_uint = 0x900B;
pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY_ARB: c_uint = 0x900B;
pub const GL_PROXY_TEXTURE_RECTANGLE: c_uint = 0x84F7;
pub const GL_QUADS: c_uint = 0x0007;
pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: c_uint = 0x8E4C;
pub const GL_QUERY: c_uint = 0x82E3;
pub const GL_QUERY_BUFFER: c_uint = 0x9192;
pub const GL_QUERY_BUFFER_BARRIER_BIT: c_uint = 0x00008000;
pub const GL_QUERY_BUFFER_BINDING: c_uint = 0x9193;
pub const GL_QUERY_BY_REGION_NO_WAIT: c_uint = 0x8E16;
pub const GL_QUERY_BY_REGION_WAIT: c_uint = 0x8E15;
pub const GL_QUERY_COUNTER_BITS: c_uint = 0x8864;
pub const GL_QUERY_COUNTER_BITS_ARB: c_uint = 0x8864;
pub const GL_QUERY_NO_WAIT: c_uint = 0x8E14;
pub const GL_QUERY_RESULT: c_uint = 0x8866;
pub const GL_QUERY_RESULT_ARB: c_uint = 0x8866;
pub const GL_QUERY_RESULT_AVAILABLE: c_uint = 0x8867;
pub const GL_QUERY_RESULT_AVAILABLE_ARB: c_uint = 0x8867;
pub const GL_QUERY_RESULT_NO_WAIT: c_uint = 0x9194;
pub const GL_QUERY_TARGET: c_uint = 0x82EA;
pub const GL_QUERY_WAIT: c_uint = 0x8E13;
pub const GL_R11F_G11F_B10F: c_uint = 0x8C3A;
pub const GL_R16: c_uint = 0x822A;
pub const GL_R16F: c_uint = 0x822D;
pub const GL_R16I: c_uint = 0x8233;
pub const GL_R16UI: c_uint = 0x8234;
pub const GL_R16_SNORM: c_uint = 0x8F98;
pub const GL_R32F: c_uint = 0x822E;
pub const GL_R32I: c_uint = 0x8235;
pub const GL_R32UI: c_uint = 0x8236;
pub const GL_R3_G3_B2: c_uint = 0x2A10;
pub const GL_R8: c_uint = 0x8229;
pub const GL_R8I: c_uint = 0x8231;
pub const GL_R8UI: c_uint = 0x8232;
pub const GL_R8_SNORM: c_uint = 0x8F94;
pub const GL_RASTERIZER_DISCARD: c_uint = 0x8C89;
pub const GL_READ_BUFFER: c_uint = 0x0C02;
pub const GL_READ_FRAMEBUFFER: c_uint = 0x8CA8;
pub const GL_READ_FRAMEBUFFER_BINDING: c_uint = 0x8CAA;
pub const GL_READ_FRAMEBUFFER_BINDING_EXT: c_uint = 0x8CAA;
pub const GL_READ_FRAMEBUFFER_EXT: c_uint = 0x8CA8;
pub const GL_READ_ONLY: c_uint = 0x88B8;
pub const GL_READ_ONLY_ARB: c_uint = 0x88B8;
pub const GL_READ_PIXELS: c_uint = 0x828C;
pub const GL_READ_PIXELS_FORMAT: c_uint = 0x828D;
pub const GL_READ_PIXELS_TYPE: c_uint = 0x828E;
pub const GL_READ_WRITE: c_uint = 0x88BA;
pub const GL_READ_WRITE_ARB: c_uint = 0x88BA;
pub const GL_RED: c_uint = 0x1903;
pub const GL_RED_INTEGER: c_uint = 0x8D94;
pub const GL_REFERENCED_BY_COMPUTE_SHADER: c_uint = 0x930B;
pub const GL_REFERENCED_BY_FRAGMENT_SHADER: c_uint = 0x930A;
pub const GL_REFERENCED_BY_GEOMETRY_SHADER: c_uint = 0x9309;
pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER: c_uint = 0x9307;
pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER: c_uint = 0x9308;
pub const GL_REFERENCED_BY_VERTEX_SHADER: c_uint = 0x9306;
pub const GL_REFLECTION_MAP_ARB: c_uint = 0x8512;
pub const GL_RENDERBUFFER: c_uint = 0x8D41;
pub const GL_RENDERBUFFER_ALPHA_SIZE: c_uint = 0x8D53;
pub const GL_RENDERBUFFER_ALPHA_SIZE_EXT: c_uint = 0x8D53;
pub const GL_RENDERBUFFER_BINDING: c_uint = 0x8CA7;
pub const GL_RENDERBUFFER_BINDING_EXT: c_uint = 0x8CA7;
pub const GL_RENDERBUFFER_BLUE_SIZE: c_uint = 0x8D52;
pub const GL_RENDERBUFFER_BLUE_SIZE_EXT: c_uint = 0x8D52;
pub const GL_RENDERBUFFER_DEPTH_SIZE: c_uint = 0x8D54;
pub const GL_RENDERBUFFER_DEPTH_SIZE_EXT: c_uint = 0x8D54;
pub const GL_RENDERBUFFER_EXT: c_uint = 0x8D41;
pub const GL_RENDERBUFFER_GREEN_SIZE: c_uint = 0x8D51;
pub const GL_RENDERBUFFER_GREEN_SIZE_EXT: c_uint = 0x8D51;
pub const GL_RENDERBUFFER_HEIGHT: c_uint = 0x8D43;
pub const GL_RENDERBUFFER_HEIGHT_EXT: c_uint = 0x8D43;
pub const GL_RENDERBUFFER_INTERNAL_FORMAT: c_uint = 0x8D44;
pub const GL_RENDERBUFFER_INTERNAL_FORMAT_EXT: c_uint = 0x8D44;
pub const GL_RENDERBUFFER_RED_SIZE: c_uint = 0x8D50;
pub const GL_RENDERBUFFER_RED_SIZE_EXT: c_uint = 0x8D50;
pub const GL_RENDERBUFFER_SAMPLES: c_uint = 0x8CAB;
pub const GL_RENDERBUFFER_SAMPLES_EXT: c_uint = 0x8CAB;
pub const GL_RENDERBUFFER_STENCIL_SIZE: c_uint = 0x8D55;
pub const GL_RENDERBUFFER_STENCIL_SIZE_EXT: c_uint = 0x8D55;
pub const GL_RENDERBUFFER_WIDTH: c_uint = 0x8D42;
pub const GL_RENDERBUFFER_WIDTH_EXT: c_uint = 0x8D42;
pub const GL_RENDERER: c_uint = 0x1F01;
pub const GL_REPEAT: c_uint = 0x2901;
pub const GL_REPLACE: c_uint = 0x1E01;
pub const GL_RG: c_uint = 0x8227;
pub const GL_RG16: c_uint = 0x822C;
pub const GL_RG16F: c_uint = 0x822F;
pub const GL_RG16I: c_uint = 0x8239;
pub const GL_RG16UI: c_uint = 0x823A;
pub const GL_RG16_SNORM: c_uint = 0x8F99;
pub const GL_RG32F: c_uint = 0x8230;
pub const GL_RG32I: c_uint = 0x823B;
pub const GL_RG32UI: c_uint = 0x823C;
pub const GL_RG8: c_uint = 0x822B;
pub const GL_RG8I: c_uint = 0x8237;
pub const GL_RG8UI: c_uint = 0x8238;
pub const GL_RG8_SNORM: c_uint = 0x8F95;
pub const GL_RGB: c_uint = 0x1907;
pub const GL_RGB10: c_uint = 0x8052;
pub const GL_RGB10_A2: c_uint = 0x8059;
pub const GL_RGB10_A2UI: c_uint = 0x906F;
pub const GL_RGB12: c_uint = 0x8053;
pub const GL_RGB16: c_uint = 0x8054;
pub const GL_RGB16F: c_uint = 0x881B;
pub const GL_RGB16F_ARB: c_uint = 0x881B;
pub const GL_RGB16I: c_uint = 0x8D89;
pub const GL_RGB16UI: c_uint = 0x8D77;
pub const GL_RGB16_SNORM: c_uint = 0x8F9A;
pub const GL_RGB32F: c_uint = 0x8815;
pub const GL_RGB32F_ARB: c_uint = 0x8815;
pub const GL_RGB32I: c_uint = 0x8D83;
pub const GL_RGB32UI: c_uint = 0x8D71;
pub const GL_RGB4: c_uint = 0x804F;
pub const GL_RGB5: c_uint = 0x8050;
pub const GL_RGB565: c_uint = 0x8D62;
pub const GL_RGB5_A1: c_uint = 0x8057;
pub const GL_RGB8: c_uint = 0x8051;
pub const GL_RGB8I: c_uint = 0x8D8F;
pub const GL_RGB8UI: c_uint = 0x8D7D;
pub const GL_RGB8_SNORM: c_uint = 0x8F96;
pub const GL_RGB9_E5: c_uint = 0x8C3D;
pub const GL_RGBA: c_uint = 0x1908;
pub const GL_RGBA12: c_uint = 0x805A;
pub const GL_RGBA16: c_uint = 0x805B;
pub const GL_RGBA16F: c_uint = 0x881A;
pub const GL_RGBA16F_ARB: c_uint = 0x881A;
pub const GL_RGBA16I: c_uint = 0x8D88;
pub const GL_RGBA16UI: c_uint = 0x8D76;
pub const GL_RGBA16_SNORM: c_uint = 0x8F9B;
pub const GL_RGBA2: c_uint = 0x8055;
pub const GL_RGBA32F: c_uint = 0x8814;
pub const GL_RGBA32F_ARB: c_uint = 0x8814;
pub const GL_RGBA32I: c_uint = 0x8D82;
pub const GL_RGBA32UI: c_uint = 0x8D70;
pub const GL_RGBA4: c_uint = 0x8056;
pub const GL_RGBA8: c_uint = 0x8058;
pub const GL_RGBA8I: c_uint = 0x8D8E;
pub const GL_RGBA8UI: c_uint = 0x8D7C;
pub const GL_RGBA8_SNORM: c_uint = 0x8F97;
pub const GL_RGBA_FLOAT_MODE_ARB: c_uint = 0x8820;
pub const GL_RGBA_INTEGER: c_uint = 0x8D99;
pub const GL_RGB_INTEGER: c_uint = 0x8D98;
pub const GL_RG_INTEGER: c_uint = 0x8228;
pub const GL_RIGHT: c_uint = 0x0407;
pub const GL_SAMPLER: c_uint = 0x82E6;
pub const GL_SAMPLER_1D: c_uint = 0x8B5D;
pub const GL_SAMPLER_1D_ARB: c_uint = 0x8B5D;
pub const GL_SAMPLER_1D_ARRAY: c_uint = 0x8DC0;
pub const GL_SAMPLER_1D_ARRAY_SHADOW: c_uint = 0x8DC3;
pub const GL_SAMPLER_1D_SHADOW: c_uint = 0x8B61;
pub const GL_SAMPLER_1D_SHADOW_ARB: c_uint = 0x8B61;
pub const GL_SAMPLER_2D: c_uint = 0x8B5E;
pub const GL_SAMPLER_2D_ARB: c_uint = 0x8B5E;
pub const GL_SAMPLER_2D_ARRAY: c_uint = 0x8DC1;
pub const GL_SAMPLER_2D_ARRAY_SHADOW: c_uint = 0x8DC4;
pub const GL_SAMPLER_2D_MULTISAMPLE: c_uint = 0x9108;
pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: c_uint = 0x910B;
pub const GL_SAMPLER_2D_RECT: c_uint = 0x8B63;
pub const GL_SAMPLER_2D_RECT_ARB: c_uint = 0x8B63;
pub const GL_SAMPLER_2D_RECT_SHADOW: c_uint = 0x8B64;
pub const GL_SAMPLER_2D_RECT_SHADOW_ARB: c_uint = 0x8B64;
pub const GL_SAMPLER_2D_SHADOW: c_uint = 0x8B62;
pub const GL_SAMPLER_2D_SHADOW_ARB: c_uint = 0x8B62;
pub const GL_SAMPLER_3D: c_uint = 0x8B5F;
pub const GL_SAMPLER_3D_ARB: c_uint = 0x8B5F;
pub const GL_SAMPLER_BINDING: c_uint = 0x8919;
pub const GL_SAMPLER_BUFFER: c_uint = 0x8DC2;
pub const GL_SAMPLER_CUBE: c_uint = 0x8B60;
pub const GL_SAMPLER_CUBE_ARB: c_uint = 0x8B60;
pub const GL_SAMPLER_CUBE_MAP_ARRAY: c_uint = 0x900C;
pub const GL_SAMPLER_CUBE_MAP_ARRAY_ARB: c_uint = 0x900C;
pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW: c_uint = 0x900D;
pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW_ARB: c_uint = 0x900D;
pub const GL_SAMPLER_CUBE_SHADOW: c_uint = 0x8DC5;
pub const GL_SAMPLES: c_uint = 0x80A9;
pub const GL_SAMPLES_ARB: c_uint = 0x80A9;
pub const GL_SAMPLES_PASSED: c_uint = 0x8914;
pub const GL_SAMPLES_PASSED_ARB: c_uint = 0x8914;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: c_uint = 0x809E;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE_ARB: c_uint = 0x809E;
pub const GL_SAMPLE_ALPHA_TO_ONE: c_uint = 0x809F;
pub const GL_SAMPLE_ALPHA_TO_ONE_ARB: c_uint = 0x809F;
pub const GL_SAMPLE_BUFFERS: c_uint = 0x80A8;
pub const GL_SAMPLE_BUFFERS_ARB: c_uint = 0x80A8;
pub const GL_SAMPLE_COVERAGE: c_uint = 0x80A0;
pub const GL_SAMPLE_COVERAGE_ARB: c_uint = 0x80A0;
pub const GL_SAMPLE_COVERAGE_INVERT: c_uint = 0x80AB;
pub const GL_SAMPLE_COVERAGE_INVERT_ARB: c_uint = 0x80AB;
pub const GL_SAMPLE_COVERAGE_VALUE: c_uint = 0x80AA;
pub const GL_SAMPLE_COVERAGE_VALUE_ARB: c_uint = 0x80AA;
pub const GL_SAMPLE_LOCATION_ARB: c_uint = 0x8E50;
pub const GL_SAMPLE_LOCATION_PIXEL_GRID_HEIGHT_ARB: c_uint = 0x933F;
pub const GL_SAMPLE_LOCATION_PIXEL_GRID_WIDTH_ARB: c_uint = 0x933E;
pub const GL_SAMPLE_LOCATION_SUBPIXEL_BITS_ARB: c_uint = 0x933D;
pub const GL_SAMPLE_MASK: c_uint = 0x8E51;
pub const GL_SAMPLE_MASK_VALUE: c_uint = 0x8E52;
pub const GL_SAMPLE_POSITION: c_uint = 0x8E50;
pub const GL_SAMPLE_SHADING: c_uint = 0x8C36;
pub const GL_SAMPLE_SHADING_ARB: c_uint = 0x8C36;
pub const GL_SCISSOR_BOX: c_uint = 0x0C10;
pub const GL_SCISSOR_TEST: c_uint = 0x0C11;
pub const GL_SECONDARY_COLOR_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889C;
pub const GL_SEPARATE_ATTRIBS: c_uint = 0x8C8D;
pub const GL_SET: c_uint = 0x150F;
pub const GL_SHADER: c_uint = 0x82E1;
pub const GL_SHADER_BINARY_FORMATS: c_uint = 0x8DF8;
pub const GL_SHADER_BINARY_FORMAT_SPIR_V: c_uint = 0x9551;
pub const GL_SHADER_BINARY_FORMAT_SPIR_V_ARB: c_uint = 0x9551;
pub const GL_SHADER_COMPILER: c_uint = 0x8DFA;
pub const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT: c_uint = 0x00000020;
pub const GL_SHADER_IMAGE_ATOMIC: c_uint = 0x82A6;
pub const GL_SHADER_IMAGE_LOAD: c_uint = 0x82A4;
pub const GL_SHADER_IMAGE_STORE: c_uint = 0x82A5;
pub const GL_SHADER_INCLUDE_ARB: c_uint = 0x8DAE;
pub const GL_SHADER_OBJECT_ARB: c_uint = 0x8B48;
pub const GL_SHADER_SOURCE_LENGTH: c_uint = 0x8B88;
pub const GL_SHADER_STORAGE_BARRIER_BIT: c_uint = 0x00002000;
pub const GL_SHADER_STORAGE_BLOCK: c_uint = 0x92E6;
pub const GL_SHADER_STORAGE_BUFFER: c_uint = 0x90D2;
pub const GL_SHADER_STORAGE_BUFFER_BINDING: c_uint = 0x90D3;
pub const GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: c_uint = 0x90DF;
pub const GL_SHADER_STORAGE_BUFFER_SIZE: c_uint = 0x90D5;
pub const GL_SHADER_STORAGE_BUFFER_START: c_uint = 0x90D4;
pub const GL_SHADER_TYPE: c_uint = 0x8B4F;
pub const GL_SHADING_LANGUAGE_VERSION: c_uint = 0x8B8C;
pub const GL_SHADING_LANGUAGE_VERSION_ARB: c_uint = 0x8B8C;
pub const GL_SHORT: c_uint = 0x1402;
pub const GL_SIGNALED: c_uint = 0x9119;
pub const GL_SIGNED_NORMALIZED: c_uint = 0x8F9C;
pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: c_uint = 0x82AC;
pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: c_uint = 0x82AE;
pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: c_uint = 0x82AD;
pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: c_uint = 0x82AF;
pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY: c_uint = 0x0B23;
pub const GL_SMOOTH_LINE_WIDTH_RANGE: c_uint = 0x0B22;
pub const GL_SMOOTH_POINT_SIZE_GRANULARITY: c_uint = 0x0B13;
pub const GL_SMOOTH_POINT_SIZE_RANGE: c_uint = 0x0B12;
pub const GL_SOURCE1_ALPHA: c_uint = 0x8589;
pub const GL_SPIR_V_BINARY: c_uint = 0x9552;
pub const GL_SPIR_V_BINARY_ARB: c_uint = 0x9552;
pub const GL_SPIR_V_EXTENSIONS: c_uint = 0x9553;
pub const GL_SRC1_ALPHA: c_uint = 0x8589;
pub const GL_SRC1_COLOR: c_uint = 0x88F9;
pub const GL_SRC_ALPHA: c_uint = 0x0302;
pub const GL_SRC_ALPHA_SATURATE: c_uint = 0x0308;
pub const GL_SRC_COLOR: c_uint = 0x0300;
pub const GL_SRGB: c_uint = 0x8C40;
pub const GL_SRGB8: c_uint = 0x8C41;
pub const GL_SRGB8_ALPHA8: c_uint = 0x8C43;
pub const GL_SRGB_ALPHA: c_uint = 0x8C42;
pub const GL_SRGB_DECODE_ARB: c_uint = 0x8299;
pub const GL_SRGB_READ: c_uint = 0x8297;
pub const GL_SRGB_WRITE: c_uint = 0x8298;
pub const GL_STACK_OVERFLOW: c_uint = 0x0503;
pub const GL_STACK_UNDERFLOW: c_uint = 0x0504;
pub const GL_STATIC_COPY: c_uint = 0x88E6;
pub const GL_STATIC_COPY_ARB: c_uint = 0x88E6;
pub const GL_STATIC_DRAW: c_uint = 0x88E4;
pub const GL_STATIC_DRAW_ARB: c_uint = 0x88E4;
pub const GL_STATIC_READ: c_uint = 0x88E5;
pub const GL_STATIC_READ_ARB: c_uint = 0x88E5;
pub const GL_STENCIL: c_uint = 0x1802;
pub const GL_STENCIL_ATTACHMENT: c_uint = 0x8D20;
pub const GL_STENCIL_ATTACHMENT_EXT: c_uint = 0x8D20;
pub const GL_STENCIL_BACK_FAIL: c_uint = 0x8801;
pub const GL_STENCIL_BACK_FUNC: c_uint = 0x8800;
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: c_uint = 0x8802;
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: c_uint = 0x8803;
pub const GL_STENCIL_BACK_REF: c_uint = 0x8CA3;
pub const GL_STENCIL_BACK_VALUE_MASK: c_uint = 0x8CA4;
pub const GL_STENCIL_BACK_WRITEMASK: c_uint = 0x8CA5;
pub const GL_STENCIL_BUFFER_BIT: c_uint = 0x00000400;
pub const GL_STENCIL_CLEAR_VALUE: c_uint = 0x0B91;
pub const GL_STENCIL_COMPONENTS: c_uint = 0x8285;
pub const GL_STENCIL_FAIL: c_uint = 0x0B94;
pub const GL_STENCIL_FUNC: c_uint = 0x0B92;
pub const GL_STENCIL_INDEX: c_uint = 0x1901;
pub const GL_STENCIL_INDEX1: c_uint = 0x8D46;
pub const GL_STENCIL_INDEX16: c_uint = 0x8D49;
pub const GL_STENCIL_INDEX16_EXT: c_uint = 0x8D49;
pub const GL_STENCIL_INDEX1_EXT: c_uint = 0x8D46;
pub const GL_STENCIL_INDEX4: c_uint = 0x8D47;
pub const GL_STENCIL_INDEX4_EXT: c_uint = 0x8D47;
pub const GL_STENCIL_INDEX8: c_uint = 0x8D48;
pub const GL_STENCIL_INDEX8_EXT: c_uint = 0x8D48;
pub const GL_STENCIL_PASS_DEPTH_FAIL: c_uint = 0x0B95;
pub const GL_STENCIL_PASS_DEPTH_PASS: c_uint = 0x0B96;
pub const GL_STENCIL_REF: c_uint = 0x0B97;
pub const GL_STENCIL_RENDERABLE: c_uint = 0x8288;
pub const GL_STENCIL_TEST: c_uint = 0x0B90;
pub const GL_STENCIL_VALUE_MASK: c_uint = 0x0B93;
pub const GL_STENCIL_WRITEMASK: c_uint = 0x0B98;
pub const GL_STEREO: c_uint = 0x0C33;
pub const GL_STREAM_COPY: c_uint = 0x88E2;
pub const GL_STREAM_COPY_ARB: c_uint = 0x88E2;
pub const GL_STREAM_DRAW: c_uint = 0x88E0;
pub const GL_STREAM_DRAW_ARB: c_uint = 0x88E0;
pub const GL_STREAM_READ: c_uint = 0x88E1;
pub const GL_STREAM_READ_ARB: c_uint = 0x88E1;
pub const GL_SUBPIXEL_BITS: c_uint = 0x0D50;
pub const GL_SYNC_CONDITION: c_uint = 0x9113;
pub const GL_SYNC_FENCE: c_uint = 0x9116;
pub const GL_SYNC_FLAGS: c_uint = 0x9115;
pub const GL_SYNC_FLUSH_COMMANDS_BIT: c_uint = 0x00000001;
pub const GL_SYNC_GPU_COMMANDS_COMPLETE: c_uint = 0x9117;
pub const GL_SYNC_STATUS: c_uint = 0x9114;
pub const GL_TESS_CONTROL_OUTPUT_VERTICES: c_uint = 0x8E75;
pub const GL_TESS_CONTROL_SHADER: c_uint = 0x8E88;
pub const GL_TESS_CONTROL_SHADER_BIT: c_uint = 0x00000008;
pub const GL_TESS_CONTROL_SHADER_PATCHES: c_uint = 0x82F1;
pub const GL_TESS_CONTROL_SHADER_PATCHES_ARB: c_uint = 0x82F1;
pub const GL_TESS_CONTROL_SUBROUTINE: c_uint = 0x92E9;
pub const GL_TESS_CONTROL_SUBROUTINE_UNIFORM: c_uint = 0x92EF;
pub const GL_TESS_CONTROL_TEXTURE: c_uint = 0x829C;
pub const GL_TESS_EVALUATION_SHADER: c_uint = 0x8E87;
pub const GL_TESS_EVALUATION_SHADER_BIT: c_uint = 0x00000010;
pub const GL_TESS_EVALUATION_SHADER_INVOCATIONS: c_uint = 0x82F2;
pub const GL_TESS_EVALUATION_SHADER_INVOCATIONS_ARB: c_uint = 0x82F2;
pub const GL_TESS_EVALUATION_SUBROUTINE: c_uint = 0x92EA;
pub const GL_TESS_EVALUATION_SUBROUTINE_UNIFORM: c_uint = 0x92F0;
pub const GL_TESS_EVALUATION_TEXTURE: c_uint = 0x829D;
pub const GL_TESS_GEN_MODE: c_uint = 0x8E76;
pub const GL_TESS_GEN_POINT_MODE: c_uint = 0x8E79;
pub const GL_TESS_GEN_SPACING: c_uint = 0x8E77;
pub const GL_TESS_GEN_VERTEX_ORDER: c_uint = 0x8E78;
pub const GL_TEXTURE: c_uint = 0x1702;
pub const GL_TEXTURE0: c_uint = 0x84C0;
pub const GL_TEXTURE0_ARB: c_uint = 0x84C0;
pub const GL_TEXTURE1: c_uint = 0x84C1;
pub const GL_TEXTURE10: c_uint = 0x84CA;
pub const GL_TEXTURE10_ARB: c_uint = 0x84CA;
pub const GL_TEXTURE11: c_uint = 0x84CB;
pub const GL_TEXTURE11_ARB: c_uint = 0x84CB;
pub const GL_TEXTURE12: c_uint = 0x84CC;
pub const GL_TEXTURE12_ARB: c_uint = 0x84CC;
pub const GL_TEXTURE13: c_uint = 0x84CD;
pub const GL_TEXTURE13_ARB: c_uint = 0x84CD;
pub const GL_TEXTURE14: c_uint = 0x84CE;
pub const GL_TEXTURE14_ARB: c_uint = 0x84CE;
pub const GL_TEXTURE15: c_uint = 0x84CF;
pub const GL_TEXTURE15_ARB: c_uint = 0x84CF;
pub const GL_TEXTURE16: c_uint = 0x84D0;
pub const GL_TEXTURE16_ARB: c_uint = 0x84D0;
pub const GL_TEXTURE17: c_uint = 0x84D1;
pub const GL_TEXTURE17_ARB: c_uint = 0x84D1;
pub const GL_TEXTURE18: c_uint = 0x84D2;
pub const GL_TEXTURE18_ARB: c_uint = 0x84D2;
pub const GL_TEXTURE19: c_uint = 0x84D3;
pub const GL_TEXTURE19_ARB: c_uint = 0x84D3;
pub const GL_TEXTURE1_ARB: c_uint = 0x84C1;
pub const GL_TEXTURE2: c_uint = 0x84C2;
pub const GL_TEXTURE20: c_uint = 0x84D4;
pub const GL_TEXTURE20_ARB: c_uint = 0x84D4;
pub const GL_TEXTURE21: c_uint = 0x84D5;
pub const GL_TEXTURE21_ARB: c_uint = 0x84D5;
pub const GL_TEXTURE22: c_uint = 0x84D6;
pub const GL_TEXTURE22_ARB: c_uint = 0x84D6;
pub const GL_TEXTURE23: c_uint = 0x84D7;
pub const GL_TEXTURE23_ARB: c_uint = 0x84D7;
pub const GL_TEXTURE24: c_uint = 0x84D8;
pub const GL_TEXTURE24_ARB: c_uint = 0x84D8;
pub const GL_TEXTURE25: c_uint = 0x84D9;
pub const GL_TEXTURE25_ARB: c_uint = 0x84D9;
pub const GL_TEXTURE26: c_uint = 0x84DA;
pub const GL_TEXTURE26_ARB: c_uint = 0x84DA;
pub const GL_TEXTURE27: c_uint = 0x84DB;
pub const GL_TEXTURE27_ARB: c_uint = 0x84DB;
pub const GL_TEXTURE28: c_uint = 0x84DC;
pub const GL_TEXTURE28_ARB: c_uint = 0x84DC;
pub const GL_TEXTURE29: c_uint = 0x84DD;
pub const GL_TEXTURE29_ARB: c_uint = 0x84DD;
pub const GL_TEXTURE2_ARB: c_uint = 0x84C2;
pub const GL_TEXTURE3: c_uint = 0x84C3;
pub const GL_TEXTURE30: c_uint = 0x84DE;
pub const GL_TEXTURE30_ARB: c_uint = 0x84DE;
pub const GL_TEXTURE31: c_uint = 0x84DF;
pub const GL_TEXTURE31_ARB: c_uint = 0x84DF;
pub const GL_TEXTURE3_ARB: c_uint = 0x84C3;
pub const GL_TEXTURE4: c_uint = 0x84C4;
pub const GL_TEXTURE4_ARB: c_uint = 0x84C4;
pub const GL_TEXTURE5: c_uint = 0x84C5;
pub const GL_TEXTURE5_ARB: c_uint = 0x84C5;
pub const GL_TEXTURE6: c_uint = 0x84C6;
pub const GL_TEXTURE6_ARB: c_uint = 0x84C6;
pub const GL_TEXTURE7: c_uint = 0x84C7;
pub const GL_TEXTURE7_ARB: c_uint = 0x84C7;
pub const GL_TEXTURE8: c_uint = 0x84C8;
pub const GL_TEXTURE8_ARB: c_uint = 0x84C8;
pub const GL_TEXTURE9: c_uint = 0x84C9;
pub const GL_TEXTURE9_ARB: c_uint = 0x84C9;
pub const GL_TEXTURE_1D: c_uint = 0x0DE0;
pub const GL_TEXTURE_1D_ARRAY: c_uint = 0x8C18;
pub const GL_TEXTURE_2D: c_uint = 0x0DE1;
pub const GL_TEXTURE_2D_ARRAY: c_uint = 0x8C1A;
pub const GL_TEXTURE_2D_MULTISAMPLE: c_uint = 0x9100;
pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9102;
pub const GL_TEXTURE_3D: c_uint = 0x806F;
pub const GL_TEXTURE_ALPHA_SIZE: c_uint = 0x805F;
pub const GL_TEXTURE_ALPHA_TYPE: c_uint = 0x8C13;
pub const GL_TEXTURE_ALPHA_TYPE_ARB: c_uint = 0x8C13;
pub const GL_TEXTURE_BASE_LEVEL: c_uint = 0x813C;
pub const GL_TEXTURE_BINDING_1D: c_uint = 0x8068;
pub const GL_TEXTURE_BINDING_1D_ARRAY: c_uint = 0x8C1C;
pub const GL_TEXTURE_BINDING_2D: c_uint = 0x8069;
pub const GL_TEXTURE_BINDING_2D_ARRAY: c_uint = 0x8C1D;
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE: c_uint = 0x9104;
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: c_uint = 0x9105;
pub const GL_TEXTURE_BINDING_3D: c_uint = 0x806A;
pub const GL_TEXTURE_BINDING_BUFFER: c_uint = 0x8C2C;
pub const GL_TEXTURE_BINDING_CUBE_MAP: c_uint = 0x8514;
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARB: c_uint = 0x8514;
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY: c_uint = 0x900A;
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY_ARB: c_uint = 0x900A;
pub const GL_TEXTURE_BINDING_RECTANGLE: c_uint = 0x84F6;
pub const GL_TEXTURE_BLUE_SIZE: c_uint = 0x805E;
pub const GL_TEXTURE_BLUE_TYPE: c_uint = 0x8C12;
pub const GL_TEXTURE_BLUE_TYPE_ARB: c_uint = 0x8C12;
pub const GL_TEXTURE_BORDER_COLOR: c_uint = 0x1004;
pub const GL_TEXTURE_BUFFER: c_uint = 0x8C2A;
pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING: c_uint = 0x8C2D;
pub const GL_TEXTURE_BUFFER_OFFSET: c_uint = 0x919D;
pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT: c_uint = 0x919F;
pub const GL_TEXTURE_BUFFER_SIZE: c_uint = 0x919E;
pub const GL_TEXTURE_COMPARE_FUNC: c_uint = 0x884D;
pub const GL_TEXTURE_COMPARE_MODE: c_uint = 0x884C;
pub const GL_TEXTURE_COMPRESSED: c_uint = 0x86A1;
pub const GL_TEXTURE_COMPRESSED_ARB: c_uint = 0x86A1;
pub const GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT: c_uint = 0x82B2;
pub const GL_TEXTURE_COMPRESSED_BLOCK_SIZE: c_uint = 0x82B3;
pub const GL_TEXTURE_COMPRESSED_BLOCK_WIDTH: c_uint = 0x82B1;
pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE: c_uint = 0x86A0;
pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE_ARB: c_uint = 0x86A0;
pub const GL_TEXTURE_COMPRESSION_HINT: c_uint = 0x84EF;
pub const GL_TEXTURE_COMPRESSION_HINT_ARB: c_uint = 0x84EF;
pub const GL_TEXTURE_COORD_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889A;
pub const GL_TEXTURE_CUBE_MAP: c_uint = 0x8513;
pub const GL_TEXTURE_CUBE_MAP_ARB: c_uint = 0x8513;
pub const GL_TEXTURE_CUBE_MAP_ARRAY: c_uint = 0x9009;
pub const GL_TEXTURE_CUBE_MAP_ARRAY_ARB: c_uint = 0x9009;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: c_uint = 0x8516;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X_ARB: c_uint = 0x8516;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: c_uint = 0x8518;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y_ARB: c_uint = 0x8518;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: c_uint = 0x851A;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z_ARB: c_uint = 0x851A;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: c_uint = 0x8515;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X_ARB: c_uint = 0x8515;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: c_uint = 0x8517;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y_ARB: c_uint = 0x8517;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: c_uint = 0x8519;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z_ARB: c_uint = 0x8519;
pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: c_uint = 0x884F;
pub const GL_TEXTURE_DEPTH: c_uint = 0x8071;
pub const GL_TEXTURE_DEPTH_SIZE: c_uint = 0x884A;
pub const GL_TEXTURE_DEPTH_SIZE_ARB: c_uint = 0x884A;
pub const GL_TEXTURE_DEPTH_TYPE: c_uint = 0x8C16;
pub const GL_TEXTURE_DEPTH_TYPE_ARB: c_uint = 0x8C16;
pub const GL_TEXTURE_FETCH_BARRIER_BIT: c_uint = 0x00000008;
pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS: c_uint = 0x9107;
pub const GL_TEXTURE_GATHER: c_uint = 0x82A2;
pub const GL_TEXTURE_GATHER_SHADOW: c_uint = 0x82A3;
pub const GL_TEXTURE_GREEN_SIZE: c_uint = 0x805D;
pub const GL_TEXTURE_GREEN_TYPE: c_uint = 0x8C11;
pub const GL_TEXTURE_GREEN_TYPE_ARB: c_uint = 0x8C11;
pub const GL_TEXTURE_HEIGHT: c_uint = 0x1001;
pub const GL_TEXTURE_IMAGE_FORMAT: c_uint = 0x828F;
pub const GL_TEXTURE_IMAGE_TYPE: c_uint = 0x8290;
pub const GL_TEXTURE_IMMUTABLE_FORMAT: c_uint = 0x912F;
pub const GL_TEXTURE_IMMUTABLE_LEVELS: c_uint = 0x82DF;
pub const GL_TEXTURE_INTENSITY_TYPE_ARB: c_uint = 0x8C15;
pub const GL_TEXTURE_INTERNAL_FORMAT: c_uint = 0x1003;
pub const GL_TEXTURE_LOD_BIAS: c_uint = 0x8501;
pub const GL_TEXTURE_LUMINANCE_TYPE_ARB: c_uint = 0x8C14;
pub const GL_TEXTURE_MAG_FILTER: c_uint = 0x2800;
pub const GL_TEXTURE_MAX_ANISOTROPY: c_uint = 0x84FE;
pub const GL_TEXTURE_MAX_ANISOTROPY_EXT: c_uint = 0x84FE;
pub const GL_TEXTURE_MAX_LEVEL: c_uint = 0x813D;
pub const GL_TEXTURE_MAX_LOD: c_uint = 0x813B;
pub const GL_TEXTURE_MIN_FILTER: c_uint = 0x2801;
pub const GL_TEXTURE_MIN_LOD: c_uint = 0x813A;
pub const GL_TEXTURE_RECTANGLE: c_uint = 0x84F5;
pub const GL_TEXTURE_REDUCTION_MODE_ARB: c_uint = 0x9366;
pub const GL_TEXTURE_RED_SIZE: c_uint = 0x805C;
pub const GL_TEXTURE_RED_TYPE: c_uint = 0x8C10;
pub const GL_TEXTURE_RED_TYPE_ARB: c_uint = 0x8C10;
pub const GL_TEXTURE_SAMPLES: c_uint = 0x9106;
pub const GL_TEXTURE_SHADOW: c_uint = 0x82A1;
pub const GL_TEXTURE_SHARED_SIZE: c_uint = 0x8C3F;
pub const GL_TEXTURE_STENCIL_SIZE: c_uint = 0x88F1;
pub const GL_TEXTURE_SWIZZLE_A: c_uint = 0x8E45;
pub const GL_TEXTURE_SWIZZLE_B: c_uint = 0x8E44;
pub const GL_TEXTURE_SWIZZLE_G: c_uint = 0x8E43;
pub const GL_TEXTURE_SWIZZLE_R: c_uint = 0x8E42;
pub const GL_TEXTURE_SWIZZLE_RGBA: c_uint = 0x8E46;
pub const GL_TEXTURE_TARGET: c_uint = 0x1006;
pub const GL_TEXTURE_UPDATE_BARRIER_BIT: c_uint = 0x00000100;
pub const GL_TEXTURE_VIEW: c_uint = 0x82B5;
pub const GL_TEXTURE_VIEW_MIN_LAYER: c_uint = 0x82DD;
pub const GL_TEXTURE_VIEW_MIN_LEVEL: c_uint = 0x82DB;
pub const GL_TEXTURE_VIEW_NUM_LAYERS: c_uint = 0x82DE;
pub const GL_TEXTURE_VIEW_NUM_LEVELS: c_uint = 0x82DC;
pub const GL_TEXTURE_WIDTH: c_uint = 0x1000;
pub const GL_TEXTURE_WRAP_R: c_uint = 0x8072;
pub const GL_TEXTURE_WRAP_S: c_uint = 0x2802;
pub const GL_TEXTURE_WRAP_T: c_uint = 0x2803;
pub const GL_TIMEOUT_EXPIRED: c_uint = 0x911B;
pub const GL_TIMEOUT_IGNORED: c_uint = 0xFFFFFFFFFFFFFFFF;
pub const GL_TIMESTAMP: c_uint = 0x8E28;
pub const GL_TIME_ELAPSED: c_uint = 0x88BF;
pub const GL_TOP_LEVEL_ARRAY_SIZE: c_uint = 0x930C;
pub const GL_TOP_LEVEL_ARRAY_STRIDE: c_uint = 0x930D;
pub const GL_TRANSFORM_FEEDBACK: c_uint = 0x8E22;
pub const GL_TRANSFORM_FEEDBACK_ACTIVE: c_uint = 0x8E24;
pub const GL_TRANSFORM_FEEDBACK_BARRIER_BIT: c_uint = 0x00000800;
pub const GL_TRANSFORM_FEEDBACK_BINDING: c_uint = 0x8E25;
pub const GL_TRANSFORM_FEEDBACK_BUFFER: c_uint = 0x8C8E;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE: c_uint = 0x8E24;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: c_uint = 0x8C8F;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_INDEX: c_uint = 0x934B;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: c_uint = 0x8C7F;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED: c_uint = 0x8E23;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: c_uint = 0x8C85;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: c_uint = 0x8C84;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_STRIDE: c_uint = 0x934C;
pub const GL_TRANSFORM_FEEDBACK_PAUSED: c_uint = 0x8E23;
pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: c_uint = 0x8C88;
pub const GL_TRANSFORM_FEEDBACK_VARYING: c_uint = 0x92F4;
pub const GL_TRANSFORM_FEEDBACK_VARYINGS: c_uint = 0x8C83;
pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: c_uint = 0x8C76;
pub const GL_TRANSPOSE_COLOR_MATRIX_ARB: c_uint = 0x84E6;
pub const GL_TRANSPOSE_CURRENT_MATRIX_ARB: c_uint = 0x88B7;
pub const GL_TRANSPOSE_MODELVIEW_MATRIX_ARB: c_uint = 0x84E3;
pub const GL_TRANSPOSE_PROJECTION_MATRIX_ARB: c_uint = 0x84E4;
pub const GL_TRANSPOSE_TEXTURE_MATRIX_ARB: c_uint = 0x84E5;
pub const GL_TRIANGLES: c_uint = 0x0004;
pub const GL_TRIANGLES_ADJACENCY: c_uint = 0x000C;
pub const GL_TRIANGLES_ADJACENCY_ARB: c_uint = 0x000C;
pub const GL_TRIANGLE_FAN: c_uint = 0x0006;
pub const GL_TRIANGLE_STRIP: c_uint = 0x0005;
pub const GL_TRIANGLE_STRIP_ADJACENCY: c_uint = 0x000D;
pub const GL_TRIANGLE_STRIP_ADJACENCY_ARB: c_uint = 0x000D;
pub const GL_TRUE: c_uint = 1;
pub const GL_TYPE: c_uint = 0x92FA;
pub const GL_UNDEFINED_VERTEX: c_uint = 0x8260;
pub const GL_UNIFORM: c_uint = 0x92E1;
pub const GL_UNIFORM_ARRAY_STRIDE: c_uint = 0x8A3C;
pub const GL_UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: c_uint = 0x92DA;
pub const GL_UNIFORM_BARRIER_BIT: c_uint = 0x00000004;
pub const GL_UNIFORM_BLOCK: c_uint = 0x92E2;
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: c_uint = 0x8A42;
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: c_uint = 0x8A43;
pub const GL_UNIFORM_BLOCK_BINDING: c_uint = 0x8A3F;
pub const GL_UNIFORM_BLOCK_DATA_SIZE: c_uint = 0x8A40;
pub const GL_UNIFORM_BLOCK_INDEX: c_uint = 0x8A3A;
pub const GL_UNIFORM_BLOCK_NAME_LENGTH: c_uint = 0x8A41;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: c_uint = 0x90EC;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: c_uint = 0x8A46;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: c_uint = 0x8A45;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: c_uint = 0x84F0;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: c_uint = 0x84F1;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: c_uint = 0x8A44;
pub const GL_UNIFORM_BUFFER: c_uint = 0x8A11;
pub const GL_UNIFORM_BUFFER_BINDING: c_uint = 0x8A28;
pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: c_uint = 0x8A34;
pub const GL_UNIFORM_BUFFER_SIZE: c_uint = 0x8A2A;
pub const GL_UNIFORM_BUFFER_START: c_uint = 0x8A29;
pub const GL_UNIFORM_IS_ROW_MAJOR: c_uint = 0x8A3E;
pub const GL_UNIFORM_MATRIX_STRIDE: c_uint = 0x8A3D;
pub const GL_UNIFORM_NAME_LENGTH: c_uint = 0x8A39;
pub const GL_UNIFORM_OFFSET: c_uint = 0x8A3B;
pub const GL_UNIFORM_SIZE: c_uint = 0x8A38;
pub const GL_UNIFORM_TYPE: c_uint = 0x8A37;
pub const GL_UNPACK_ALIGNMENT: c_uint = 0x0CF5;
pub const GL_UNPACK_COMPRESSED_BLOCK_DEPTH: c_uint = 0x9129;
pub const GL_UNPACK_COMPRESSED_BLOCK_HEIGHT: c_uint = 0x9128;
pub const GL_UNPACK_COMPRESSED_BLOCK_SIZE: c_uint = 0x912A;
pub const GL_UNPACK_COMPRESSED_BLOCK_WIDTH: c_uint = 0x9127;
pub const GL_UNPACK_IMAGE_HEIGHT: c_uint = 0x806E;
pub const GL_UNPACK_LSB_FIRST: c_uint = 0x0CF1;
pub const GL_UNPACK_ROW_LENGTH: c_uint = 0x0CF2;
pub const GL_UNPACK_SKIP_IMAGES: c_uint = 0x806D;
pub const GL_UNPACK_SKIP_PIXELS: c_uint = 0x0CF4;
pub const GL_UNPACK_SKIP_ROWS: c_uint = 0x0CF3;
pub const GL_UNPACK_SWAP_BYTES: c_uint = 0x0CF0;
pub const GL_UNSIGNALED: c_uint = 0x9118;
pub const GL_UNSIGNED_BYTE: c_uint = 0x1401;
pub const GL_UNSIGNED_BYTE_2_3_3_REV: c_uint = 0x8362;
pub const GL_UNSIGNED_BYTE_3_3_2: c_uint = 0x8032;
pub const GL_UNSIGNED_INT: c_uint = 0x1405;
pub const GL_UNSIGNED_INT64_ARB: c_uint = 0x140F;
pub const GL_UNSIGNED_INT64_VEC2_ARB: c_uint = 0x8FF5;
pub const GL_UNSIGNED_INT64_VEC3_ARB: c_uint = 0x8FF6;
pub const GL_UNSIGNED_INT64_VEC4_ARB: c_uint = 0x8FF7;
pub const GL_UNSIGNED_INT_10F_11F_11F_REV: c_uint = 0x8C3B;
pub const GL_UNSIGNED_INT_10_10_10_2: c_uint = 0x8036;
pub const GL_UNSIGNED_INT_24_8: c_uint = 0x84FA;
pub const GL_UNSIGNED_INT_2_10_10_10_REV: c_uint = 0x8368;
pub const GL_UNSIGNED_INT_5_9_9_9_REV: c_uint = 0x8C3E;
pub const GL_UNSIGNED_INT_8_8_8_8: c_uint = 0x8035;
pub const GL_UNSIGNED_INT_8_8_8_8_REV: c_uint = 0x8367;
pub const GL_UNSIGNED_INT_ATOMIC_COUNTER: c_uint = 0x92DB;
pub const GL_UNSIGNED_INT_IMAGE_1D: c_uint = 0x9062;
pub const GL_UNSIGNED_INT_IMAGE_1D_ARRAY: c_uint = 0x9068;
pub const GL_UNSIGNED_INT_IMAGE_2D: c_uint = 0x9063;
pub const GL_UNSIGNED_INT_IMAGE_2D_ARRAY: c_uint = 0x9069;
pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: c_uint = 0x906B;
pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: c_uint = 0x906C;
pub const GL_UNSIGNED_INT_IMAGE_2D_RECT: c_uint = 0x9065;
pub const GL_UNSIGNED_INT_IMAGE_3D: c_uint = 0x9064;
pub const GL_UNSIGNED_INT_IMAGE_BUFFER: c_uint = 0x9067;
pub const GL_UNSIGNED_INT_IMAGE_CUBE: c_uint = 0x9066;
pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: c_uint = 0x906A;
pub const GL_UNSIGNED_INT_SAMPLER_1D: c_uint = 0x8DD1;
pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY: c_uint = 0x8DD6;
pub const GL_UNSIGNED_INT_SAMPLER_2D: c_uint = 0x8DD2;
pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: c_uint = 0x8DD7;
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: c_uint = 0x910A;
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: c_uint = 0x910D;
pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT: c_uint = 0x8DD5;
pub const GL_UNSIGNED_INT_SAMPLER_3D: c_uint = 0x8DD3;
pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: c_uint = 0x8DD8;
pub const GL_UNSIGNED_INT_SAMPLER_CUBE: c_uint = 0x8DD4;
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: c_uint = 0x900F;
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY_ARB: c_uint = 0x900F;
pub const GL_UNSIGNED_INT_VEC2: c_uint = 0x8DC6;
pub const GL_UNSIGNED_INT_VEC3: c_uint = 0x8DC7;
pub const GL_UNSIGNED_INT_VEC4: c_uint = 0x8DC8;
pub const GL_UNSIGNED_NORMALIZED: c_uint = 0x8C17;
pub const GL_UNSIGNED_NORMALIZED_ARB: c_uint = 0x8C17;
pub const GL_UNSIGNED_SHORT: c_uint = 0x1403;
pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: c_uint = 0x8366;
pub const GL_UNSIGNED_SHORT_4_4_4_4: c_uint = 0x8033;
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: c_uint = 0x8365;
pub const GL_UNSIGNED_SHORT_5_5_5_1: c_uint = 0x8034;
pub const GL_UNSIGNED_SHORT_5_6_5: c_uint = 0x8363;
pub const GL_UNSIGNED_SHORT_5_6_5_REV: c_uint = 0x8364;
pub const GL_UPPER_LEFT: c_uint = 0x8CA2;
pub const GL_VALIDATE_STATUS: c_uint = 0x8B83;
pub const GL_VENDOR: c_uint = 0x1F00;
pub const GL_VERSION: c_uint = 0x1F02;
pub const GL_VERTEX_ARRAY: c_uint = 0x8074;
pub const GL_VERTEX_ARRAY_BINDING: c_uint = 0x85B5;
pub const GL_VERTEX_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8896;
pub const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT: c_uint = 0x00000001;
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: c_uint = 0x889F;
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889F;
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: c_uint = 0x88FE;
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR_ARB: c_uint = 0x88FE;
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: c_uint = 0x8622;
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED_ARB: c_uint = 0x8622;
pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: c_uint = 0x88FD;
pub const GL_VERTEX_ATTRIB_ARRAY_LONG: c_uint = 0x874E;
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: c_uint = 0x886A;
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED_ARB: c_uint = 0x886A;
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: c_uint = 0x8645;
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER_ARB: c_uint = 0x8645;
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: c_uint = 0x8623;
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE_ARB: c_uint = 0x8623;
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: c_uint = 0x8624;
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE_ARB: c_uint = 0x8624;
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: c_uint = 0x8625;
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE_ARB: c_uint = 0x8625;
pub const GL_VERTEX_ATTRIB_BINDING: c_uint = 0x82D4;
pub const GL_VERTEX_ATTRIB_RELATIVE_OFFSET: c_uint = 0x82D5;
pub const GL_VERTEX_BINDING_BUFFER: c_uint = 0x8F4F;
pub const GL_VERTEX_BINDING_DIVISOR: c_uint = 0x82D6;
pub const GL_VERTEX_BINDING_OFFSET: c_uint = 0x82D7;
pub const GL_VERTEX_BINDING_STRIDE: c_uint = 0x82D8;
pub const GL_VERTEX_PROGRAM_ARB: c_uint = 0x8620;
pub const GL_VERTEX_PROGRAM_POINT_SIZE: c_uint = 0x8642;
pub const GL_VERTEX_PROGRAM_POINT_SIZE_ARB: c_uint = 0x8642;
pub const GL_VERTEX_PROGRAM_TWO_SIDE_ARB: c_uint = 0x8643;
pub const GL_VERTEX_SHADER: c_uint = 0x8B31;
pub const GL_VERTEX_SHADER_ARB: c_uint = 0x8B31;
pub const GL_VERTEX_SHADER_BIT: c_uint = 0x00000001;
pub const GL_VERTEX_SHADER_INVOCATIONS: c_uint = 0x82F0;
pub const GL_VERTEX_SHADER_INVOCATIONS_ARB: c_uint = 0x82F0;
pub const GL_VERTEX_SUBROUTINE: c_uint = 0x92E8;
pub const GL_VERTEX_SUBROUTINE_UNIFORM: c_uint = 0x92EE;
pub const GL_VERTEX_TEXTURE: c_uint = 0x829B;
pub const GL_VERTICES_SUBMITTED: c_uint = 0x82EE;
pub const GL_VERTICES_SUBMITTED_ARB: c_uint = 0x82EE;
pub const GL_VIEWPORT: c_uint = 0x0BA2;
pub const GL_VIEWPORT_BOUNDS_RANGE: c_uint = 0x825D;
pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX: c_uint = 0x825F;
pub const GL_VIEWPORT_SUBPIXEL_BITS: c_uint = 0x825C;
pub const GL_VIEW_CLASS_128_BITS: c_uint = 0x82C4;
pub const GL_VIEW_CLASS_16_BITS: c_uint = 0x82CA;
pub const GL_VIEW_CLASS_24_BITS: c_uint = 0x82C9;
pub const GL_VIEW_CLASS_32_BITS: c_uint = 0x82C8;
pub const GL_VIEW_CLASS_48_BITS: c_uint = 0x82C7;
pub const GL_VIEW_CLASS_64_BITS: c_uint = 0x82C6;
pub const GL_VIEW_CLASS_8_BITS: c_uint = 0x82CB;
pub const GL_VIEW_CLASS_96_BITS: c_uint = 0x82C5;
pub const GL_VIEW_CLASS_ASTC_10x10_RGBA: c_uint = 0x9393;
pub const GL_VIEW_CLASS_ASTC_10x5_RGBA: c_uint = 0x9390;
pub const GL_VIEW_CLASS_ASTC_10x6_RGBA: c_uint = 0x9391;
pub const GL_VIEW_CLASS_ASTC_10x8_RGBA: c_uint = 0x9392;
pub const GL_VIEW_CLASS_ASTC_12x10_RGBA: c_uint = 0x9394;
pub const GL_VIEW_CLASS_ASTC_12x12_RGBA: c_uint = 0x9395;
pub const GL_VIEW_CLASS_ASTC_4x4_RGBA: c_uint = 0x9388;
pub const GL_VIEW_CLASS_ASTC_5x4_RGBA: c_uint = 0x9389;
pub const GL_VIEW_CLASS_ASTC_5x5_RGBA: c_uint = 0x938A;
pub const GL_VIEW_CLASS_ASTC_6x5_RGBA: c_uint = 0x938B;
pub const GL_VIEW_CLASS_ASTC_6x6_RGBA: c_uint = 0x938C;
pub const GL_VIEW_CLASS_ASTC_8x5_RGBA: c_uint = 0x938D;
pub const GL_VIEW_CLASS_ASTC_8x6_RGBA: c_uint = 0x938E;
pub const GL_VIEW_CLASS_ASTC_8x8_RGBA: c_uint = 0x938F;
pub const GL_VIEW_CLASS_BPTC_FLOAT: c_uint = 0x82D3;
pub const GL_VIEW_CLASS_BPTC_UNORM: c_uint = 0x82D2;
pub const GL_VIEW_CLASS_EAC_R11: c_uint = 0x9383;
pub const GL_VIEW_CLASS_EAC_RG11: c_uint = 0x9384;
pub const GL_VIEW_CLASS_ETC2_EAC_RGBA: c_uint = 0x9387;
pub const GL_VIEW_CLASS_ETC2_RGB: c_uint = 0x9385;
pub const GL_VIEW_CLASS_ETC2_RGBA: c_uint = 0x9386;
pub const GL_VIEW_CLASS_RGTC1_RED: c_uint = 0x82D0;
pub const GL_VIEW_CLASS_RGTC2_RG: c_uint = 0x82D1;
pub const GL_VIEW_CLASS_S3TC_DXT1_RGB: c_uint = 0x82CC;
pub const GL_VIEW_CLASS_S3TC_DXT1_RGBA: c_uint = 0x82CD;
pub const GL_VIEW_CLASS_S3TC_DXT3_RGBA: c_uint = 0x82CE;
pub const GL_VIEW_CLASS_S3TC_DXT5_RGBA: c_uint = 0x82CF;
pub const GL_VIEW_COMPATIBILITY_CLASS: c_uint = 0x82B6;
pub const GL_WAIT_FAILED: c_uint = 0x911D;
pub const GL_WEIGHTED_AVERAGE_ARB: c_uint = 0x9367;
pub const GL_WEIGHT_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889E;
pub const GL_WRITE_ONLY: c_uint = 0x88B9;
pub const GL_WRITE_ONLY_ARB: c_uint = 0x88B9;
pub const GL_XOR: c_uint = 0x1506;
pub const GL_ZERO: c_uint = 0;

mod __khrplatform_h_ {
    use std::ffi::*;

    /*
    ** Copyright (c) 2008-2018 The Khronos Group Inc.
    **
    ** Permission is hereby granted, free of charge, to any person obtaining a
    ** copy of this software and/or associated documentation files (the
    ** "Materials"), to deal in the Materials without restriction, including
    ** without limitation the rights to use, copy, modify, merge, publish,
    ** distribute, sublicense, and/or sell copies of the Materials, and to
    ** permit persons to whom the Materials are furnished to do so, subject to
    ** the following conditions:
    **
    ** The above copyright notice and this permission notice shall be included
    ** in all copies or substantial portions of the Materials.
    **
    ** THE MATERIALS ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
    ** EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
    ** MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
    ** IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
    ** CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
    ** TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
    ** MATERIALS OR THE USE OR OTHER DEALINGS IN THE MATERIALS.
    */

    /* Khronos platform-specific types and definitions.
    *
    * The master copy of khrplatform.h is maintained in the Khronos EGL
    * Registry repository at https://github.com/KhronosGroup/EGL-Registry
    * The last semantic modification to khrplatform.h was at commit ID:
    *      67a3e0864c2d75ea5287b9f3d2eb74a745936692
    *
    * Adopters may modify this file to suit their platform. Adopters are
    * encouraged to submit platform specific modifications to the Khronos
    * group so that they can be included in future versions of this file.
    * Please submit changes by filing pull requests or issues on
    * the EGL Registry repository linked above.
    *
    *
    * See the Implementer's Guidelines for information about where this file
    * should be located on your system and for more details of its use:
    *    http://www.khronos.org/registry/implementers_guide.pdf
    *
    * This file should be included as
    *        #include <KHR/khrplatform.h>
    * by Khronos client API header files that use its types and defines.
    *
    * The types in khrplatform.h should only be used to define API-specific types.
    *
    * Types defined in khrplatform.h:
    *    khronos_int8_t              signed   8  bit
    *    khronos_uint8_t             unsigned 8  bit
    *    khronos_int16_t             signed   16 bit
    *    khronos_uint16_t            unsigned 16 bit
    *    khronos_int32_t             signed   32 bit
    *    khronos_uint32_t            unsigned 32 bit
    *    khronos_int64_t             signed   64 bit
    *    khronos_uint64_t            unsigned 64 bit
    *    khronos_intptr_t            signed   same number of bits as a pointer
    *    khronos_uintptr_t           unsigned same number of bits as a pointer
    *    khronos_ssize_t             signed   size
    *    khronos_usize_t             unsigned size
    *    khronos_float_t             signed   32 bit floating point
    *    khronos_time_ns_t           unsigned 64 bit time in nanoseconds
    *    khronos_utime_nanoseconds_t unsigned time interval or absolute time in
    *                                         nanoseconds
    *    khronos_stime_nanoseconds_t signed time interval in nanoseconds
    *    khronos_boolean_enum_t      enumerated boolean type. This should
    *      only be used as a base type when a client API's boolean type is
    *      an enum. Client APIs which use an integer or other type for
    *      booleans cannot use this as the base type for their boolean.
    *
    * Tokens defined in khrplatform.h:
    *
    *    KHRONOS_FALSE, KHRONOS_TRUE Enumerated boolean false/true values.
    *
    *    KHRONOS_SUPPORT_INT64 is 1 if 64 bit integers are supported; otherwise 0.
    *    KHRONOS_SUPPORT_FLOAT is 1 if floats are supported; otherwise 0.
    *
    * Calling convention macros defined in this file:
    *    KHRONOS_APICALL
    *    KHRONOS_GLAD_API_PTR
    *    KHRONOS_APIATTRIBUTES
    *
    * These may be used in function prototypes as:
    *
    *      KHRONOS_APICALL void KHRONOS_GLAD_API_PTR funcname(
    *                                  int arg1,
    *                                  int arg2) KHRONOS_APIATTRIBUTES;
    */

    /*-------------------------------------------------------------------------
    * basic type definitions
    *-----------------------------------------------------------------------*/

    pub type khronos_int32_t  = i32;
    pub type khronos_uint32_t = u32;
    pub type khronos_int64_t  = i64;
    pub type khronos_uint64_t = u64;

    /*
    * Types that are (so far) the same on all platforms
    */
    pub type khronos_int8_t   = c_schar;
    pub type khronos_uint8_t  = c_uchar;
    pub type khronos_int16_t  = c_short;
    pub type khronos_uint16_t = c_ushort;

    /*
    * Types that differ between LLP64 and LP64 architectures - in LLP64,
    * pointers are 64 bits, but 'long' is still 32 bits. Win64 appears
    * to be the only LLP64 architecture in current use.
    */
    #[cfg(KHRONOS_USE_INTPTR_T)] pub type khronos_intptr_t  = libc::intptr_t;
    #[cfg(KHRONOS_USE_INTPTR_T)] pub type khronos_uintptr_t = libc::uintptr_t;

    #[cfg(all(not(KHRONOS_USE_INTPTR_T), _WIN64))] pub type khronos_intptr_t  = c_longlong;
    #[cfg(all(not(KHRONOS_USE_INTPTR_T), _WIN64))] pub type khronos_uintptr_t = c_ulonglong;

    #[cfg(all(not(KHRONOS_USE_INTPTR_T), not(_WIN64)))] pub type khronos_intptr_t  = c_long;
    #[cfg(all(not(KHRONOS_USE_INTPTR_T), not(_WIN64)))] pub type khronos_uintptr_t = c_ulong;

    #[cfg(_WIN64)] pub type khronos_ssize_t = c_longlong;
    #[cfg(_WIN64)] pub type khronos_usize_t = c_ulonglong;

    #[cfg(not(_WIN64))] pub type khronos_ssize_t = c_long;
    #[cfg(not(_WIN64))] pub type khronos_usize_t = c_ulong;

    /*
    * Float type
    */
    pub type khronos_float_t = c_float;

    /* Time types
    *
    * These types can be used to represent a time interval in nanoseconds or
    * an absolute Unadjusted System Time.  Unadjusted System Time is the number
    * of nanoseconds since some arbitrary system event (e.g. since the last
    * time the system booted).  The Unadjusted System Time is an unsigned
    * 64 bit value that wraps back to 0 every 584 years.  Time intervals
    * may be either signed or unsigned.
    */
    #[cfg(KHRONOS_SUPPORT_INT64)] pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
    #[cfg(KHRONOS_SUPPORT_INT64)] pub type khronos_stime_nanoseconds_t = khronos_int64_t;

    /*
    * Dummy value used to pad enum types to 32 bits.
    */
    pub const KHRONOS_MAX_ENUM: isize = 0x7FFFFFFF;

    /*
    * Enumerated boolean type
    *
    * Values other than zero should be considered to be true.  Therefore
    * comparisons should not be made against KHRONOS_TRUE.
    */
    #[repr(C)]
    pub enum khronos_boolean_enum_t {
        KHRONOS_FALSE = 0,
        KHRONOS_TRUE  = 1,
        KHRONOS_BOOLEAN_ENUM_FORCE_SIZE = KHRONOS_MAX_ENUM,
    }
}
pub use __khrplatform_h_::*;

pub type GLenum = c_uint;
pub type GLboolean = c_uchar;
pub type GLbitfield = c_uint;
pub type GLvoid = c_void;
pub type GLbyte = khronos_int8_t;
pub type GLubyte = khronos_uint8_t;
pub type GLshort = khronos_int16_t;
pub type GLushort = khronos_uint16_t;
pub type GLint = c_int;
pub type GLuint = c_uint;
pub type GLclampx = khronos_int32_t;
pub type GLsizei = c_int;
pub type GLfloat = khronos_float_t;
pub type GLclampf = khronos_float_t;
pub type GLdouble = c_double;
pub type GLclampd = c_double;
pub type GLeglClientBufferEXT = *mut c_void;
pub type GLeglImageOES = *mut c_void;
pub type GLchar = char;
pub type GLcharARB = char;
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub type GLhandleARB = *mut c_void;
#[cfg(not(any(target_os = "ios", target_os = "macos")))]
pub type GLhandleARB = c_uint;
pub type GLhalf = khronos_uint16_t;
pub type GLhalfARB = khronos_uint16_t;
pub type GLfixed = khronos_int32_t;
pub type GLintptr = khronos_intptr_t;
pub type GLintptrARB = khronos_intptr_t;
pub type GLsizeiptr = khronos_ssize_t;
pub type GLsizeiptrARB = khronos_ssize_t;
pub type GLint64 = khronos_int64_t;
pub type GLint64EXT = khronos_int64_t;
pub type GLuint64 = khronos_uint64_t;
pub type GLuint64EXT = khronos_uint64_t;
type __GLsync = (); // externally defined opaque type
pub type GLsync = *mut __GLsync;
pub type GLDEBUGPROC      = fn(source: GLenum, r#type: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *const c_void);
pub type GLDEBUGPROCARB   = fn(source: GLenum, r#type: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *const c_void);
pub type GLDEBUGPROCKHR   = fn(source: GLenum, r#type: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *const c_void);
pub type GLDEBUGPROCAMD   = fn(id: GLuint, category: GLenum, severity: GLenum,               length: GLsizei, message: *const GLchar, userParam: *mut   c_void);
pub type GLhalfNV         = c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;
pub type GLVULKANPROCNV   = fn();

struct GladCompat {
    GL_VERSION_1_0:                          bool,
    GL_VERSION_1_1:                          bool,
    GL_VERSION_1_2:                          bool,
    GL_VERSION_1_3:                          bool,
    GL_VERSION_1_4:                          bool,
    GL_VERSION_1_5:                          bool,
    GL_VERSION_2_0:                          bool,
    GL_VERSION_2_1:                          bool,
    GL_VERSION_3_0:                          bool,
    GL_VERSION_3_1:                          bool,
    GL_VERSION_3_2:                          bool,
    GL_VERSION_3_3:                          bool,
    GL_VERSION_4_0:                          bool,
    GL_VERSION_4_1:                          bool,
    GL_VERSION_4_2:                          bool,
    GL_VERSION_4_3:                          bool,
    GL_ARB_ES2_compatibility:                bool,
    GL_ARB_ES3_1_compatibility:              bool,
    GL_ARB_ES3_2_compatibility:              bool,
    GL_ARB_ES3_compatibility:                bool,
    GL_ARB_blend_func_extended:              bool,
    GL_ARB_buffer_storage:                   bool,
    GL_ARB_clear_buffer_object:              bool,
    GL_ARB_clear_texture:                    bool,
    GL_ARB_color_buffer_float:               bool,
    GL_ARB_compatibility:                    bool,
    GL_ARB_compressed_texture_pixel_storage: bool,
    GL_ARB_compute_shader:                   bool,
    GL_ARB_compute_variable_group_size:      bool,
    GL_ARB_copy_buffer:                      bool,
    GL_ARB_copy_image:                       bool,
    GL_ARB_debug_output:                     bool,
    GL_ARB_depth_buffer_float:               bool,
    GL_ARB_depth_clamp:                      bool,
    GL_ARB_depth_texture:                    bool,
    GL_ARB_direct_state_access:              bool,
    GL_ARB_draw_buffers:                     bool,
    GL_ARB_draw_buffers_blend:               bool,
    GL_ARB_draw_elements_base_vertex:        bool,
    GL_ARB_draw_indirect:                    bool,
    GL_ARB_draw_instanced:                   bool,
    GL_ARB_enhanced_layouts:                 bool,
    GL_ARB_explicit_attrib_location:         bool,
    GL_ARB_explicit_uniform_location:        bool,
    GL_ARB_fragment_coord_conventions:       bool,
    GL_ARB_fragment_layer_viewport:          bool,
    GL_ARB_fragment_program:                 bool,
    GL_ARB_fragment_program_shadow:          bool,
    GL_ARB_fragment_shader:                  bool,
    GL_ARB_fragment_shader_interlock:        bool,
    GL_ARB_framebuffer_no_attachments:       bool,
    GL_ARB_framebuffer_object:               bool,
    GL_ARB_framebuffer_sRGB:                 bool,
    GL_ARB_geometry_shader4:                 bool,
    GL_ARB_get_program_binary:               bool,
    GL_ARB_get_texture_sub_image:            bool,
    GL_ARB_gl_spirv:                         bool,
    GL_ARB_gpu_shader5:                      bool,
    GL_ARB_gpu_shader_fp64:                  bool,
    GL_ARB_gpu_shader_int64:                 bool,
    GL_ARB_half_float_pixel:                 bool,
    GL_ARB_half_float_vertex:                bool,
    GL_ARB_instanced_arrays:                 bool,
    GL_ARB_internalformat_query:             bool,
    GL_ARB_internalformat_query2:            bool,
    GL_ARB_map_buffer_range:                 bool,
    GL_ARB_multi_bind:                       bool,
    GL_ARB_multi_draw_indirect:              bool,
    GL_ARB_multisample:                      bool,
    GL_ARB_multitexture:                     bool,
    GL_ARB_occlusion_query:                  bool,
    GL_ARB_occlusion_query2:                 bool,
    GL_ARB_pipeline_statistics_query:        bool,
    GL_ARB_query_buffer_object:              bool,
    GL_ARB_sample_locations:                 bool,
    GL_ARB_sample_shading:                   bool,
    GL_ARB_seamless_cube_map:                bool,
    GL_ARB_seamless_cubemap_per_texture:     bool,
    GL_ARB_shader_atomic_counter_ops:        bool,
    GL_ARB_shader_atomic_counters:           bool,
    GL_ARB_shader_bit_encoding:              bool,
    GL_ARB_shader_clock:                     bool,
    GL_ARB_shader_image_load_store:          bool,
    GL_ARB_shader_image_size:                bool,
    GL_ARB_shader_objects:                   bool,
    GL_ARB_shader_storage_buffer_object:     bool,
    GL_ARB_shader_texture_lod:               bool,
    GL_ARB_shading_language_100:             bool,
    GL_ARB_shading_language_420pack:         bool,
    GL_ARB_shading_language_include:         bool,
    GL_ARB_shading_language_packing:         bool,
    GL_ARB_spirv_extensions:                 bool,
    GL_ARB_tessellation_shader:              bool,
    GL_ARB_texture_border_clamp:             bool,
    GL_ARB_texture_buffer_object_rgb32:      bool,
    GL_ARB_texture_compression:              bool,
    GL_ARB_texture_cube_map:                 bool,
    GL_ARB_texture_cube_map_array:           bool,
    GL_ARB_texture_env_add:                  bool,
    GL_ARB_texture_filter_anisotropic:       bool,
    GL_ARB_texture_filter_minmax:            bool,
    GL_ARB_texture_float:                    bool,
    GL_ARB_texture_mirror_clamp_to_edge:     bool,
    GL_ARB_texture_mirrored_repeat:          bool,
    GL_ARB_texture_multisample:              bool,
    GL_ARB_texture_non_power_of_two:         bool,
    GL_ARB_texture_rg:                       bool,
    GL_ARB_texture_storage:                  bool,
    GL_ARB_texture_swizzle:                  bool,
    GL_ARB_texture_view:                     bool,
    GL_ARB_timer_query:                      bool,
    GL_ARB_transpose_matrix:                 bool,
    GL_ARB_uniform_buffer_object:            bool,
    GL_ARB_vertex_array_bgra:                bool,
    GL_ARB_vertex_array_object:              bool,
    GL_ARB_vertex_attrib_binding:            bool,
    GL_ARB_vertex_buffer_object:             bool,
    GL_ARB_vertex_program:                   bool,
    GL_ARB_vertex_shader:                    bool,
    GL_EXT_draw_instanced:                   bool,
    GL_EXT_fog_coord:                        bool,
    GL_EXT_framebuffer_blit:                 bool,
    GL_EXT_framebuffer_multisample:          bool,
    GL_EXT_framebuffer_object:               bool,
    GL_EXT_framebuffer_sRGB:                 bool,
    GL_EXT_texture_compression_s3tc:         bool,
    GL_EXT_texture_filter_anisotropic:       bool,
    GL_EXT_texture_mirror_clamp:             bool,
    GL_KHR_texture_compression_astc_hdr:     bool,
    GL_KHR_texture_compression_astc_ldr:     bool,
    GL_OES_compressed_paletted_texture:      bool,
    GL_OES_fixed_point:                      bool,
}

pub type PFNGLACCUMXOESPROC                                   = unsafe extern "C" fn(op: GLenum, value: GLfixed);
pub type PFNGLACTIVESHADERPROGRAMPROC                         = unsafe extern "C" fn(pipeline: GLuint, program: GLuint);
pub type PFNGLACTIVETEXTUREPROC                               = unsafe extern "C" fn(texture: GLenum);
pub type PFNGLACTIVETEXTUREARBPROC                            = unsafe extern "C" fn(texture: GLenum);
pub type PFNGLALPHAFUNCXOESPROC                               = unsafe extern "C" fn(func: GLenum, r#ref: GLfixed);
pub type PFNGLATTACHOBJECTARBPROC                             = unsafe extern "C" fn(containerObj: GLhandleARB, obj: GLhandleARB);
pub type PFNGLATTACHSHADERPROC                                = unsafe extern "C" fn(program: GLuint, shader: GLuint);
pub type PFNGLBEGINCONDITIONALRENDERPROC                      = unsafe extern "C" fn(id: GLuint, mode: GLenum);
pub type PFNGLBEGINQUERYPROC                                  = unsafe extern "C" fn(target: GLenum, id: GLuint);
pub type PFNGLBEGINQUERYARBPROC                               = unsafe extern "C" fn(target: GLenum, id: GLuint);
pub type PFNGLBEGINQUERYINDEXEDPROC                           = unsafe extern "C" fn(target: GLenum, index: GLuint, id: GLuint);
pub type PFNGLBEGINTRANSFORMFEEDBACKPROC                      = unsafe extern "C" fn(primitiveMode: GLenum);
pub type PFNGLBINDATTRIBLOCATIONPROC                          = unsafe extern "C" fn(program: GLuint, index: GLuint, name: *const GLchar);
pub type PFNGLBINDATTRIBLOCATIONARBPROC                       = unsafe extern "C" fn(programObj: GLhandleARB, index: GLuint, name: *const GLcharARB);
pub type PFNGLBINDBUFFERPROC                                  = unsafe extern "C" fn(target: GLenum, buffer: GLuint);
pub type PFNGLBINDBUFFERARBPROC                               = unsafe extern "C" fn(target: GLenum, buffer: GLuint);
pub type PFNGLBINDBUFFERBASEPROC                              = unsafe extern "C" fn(target: GLenum, index: GLuint, buffer: GLuint);
pub type PFNGLBINDBUFFERRANGEPROC                             = unsafe extern "C" fn(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
pub type PFNGLBINDBUFFERSBASEPROC                             = unsafe extern "C" fn(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint);
pub type PFNGLBINDBUFFERSRANGEPROC                            = unsafe extern "C" fn(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr);
pub type PFNGLBINDFRAGDATALOCATIONPROC                        = unsafe extern "C" fn(program: GLuint, color: GLuint, name: *const GLchar);
pub type PFNGLBINDFRAGDATALOCATIONINDEXEDPROC                 = unsafe extern "C" fn(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar);
pub type PFNGLBINDFRAMEBUFFERPROC                             = unsafe extern "C" fn(target: GLenum, framebuffer: GLuint);
pub type PFNGLBINDFRAMEBUFFEREXTPROC                          = unsafe extern "C" fn(target: GLenum, framebuffer: GLuint);
pub type PFNGLBINDIMAGETEXTUREPROC                            = unsafe extern "C" fn(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum);
pub type PFNGLBINDIMAGETEXTURESPROC                           = unsafe extern "C" fn(first: GLuint, count: GLsizei, textures: *const GLuint);
pub type PFNGLBINDPROGRAMARBPROC                              = unsafe extern "C" fn(target: GLenum, program: GLuint);
pub type PFNGLBINDPROGRAMPIPELINEPROC                         = unsafe extern "C" fn(pipeline: GLuint);
pub type PFNGLBINDRENDERBUFFERPROC                            = unsafe extern "C" fn(target: GLenum, renderbuffer: GLuint);
pub type PFNGLBINDRENDERBUFFEREXTPROC                         = unsafe extern "C" fn(target: GLenum, renderbuffer: GLuint);
pub type PFNGLBINDSAMPLERPROC                                 = unsafe extern "C" fn(unit: GLuint, sampler: GLuint);
pub type PFNGLBINDSAMPLERSPROC                                = unsafe extern "C" fn(first: GLuint, count: GLsizei, samplers: *const GLuint);
pub type PFNGLBINDTEXTUREPROC                                 = unsafe extern "C" fn(target: GLenum, texture: GLuint);
pub type PFNGLBINDTEXTUREUNITPROC                             = unsafe extern "C" fn(unit: GLuint, texture: GLuint);
pub type PFNGLBINDTEXTURESPROC                                = unsafe extern "C" fn(first: GLuint, count: GLsizei, textures: *const GLuint);
pub type PFNGLBINDTRANSFORMFEEDBACKPROC                       = unsafe extern "C" fn(target: GLenum, id: GLuint);
pub type PFNGLBINDVERTEXARRAYPROC                             = unsafe extern "C" fn(array: GLuint);
pub type PFNGLBINDVERTEXBUFFERPROC                            = unsafe extern "C" fn(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);
pub type PFNGLBINDVERTEXBUFFERSPROC                           = unsafe extern "C" fn(first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);
pub type PFNGLBITMAPXOESPROC                                  = unsafe extern "C" fn(width: GLsizei, height: GLsizei, xorig: GLfixed, yorig: GLfixed, xmove: GLfixed, ymove: GLfixed, bitmap: *const GLubyte);
pub type PFNGLBLENDCOLORPROC                                  = unsafe extern "C" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
pub type PFNGLBLENDCOLORXOESPROC                              = unsafe extern "C" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);
pub type PFNGLBLENDEQUATIONPROC                               = unsafe extern "C" fn(mode: GLenum);
pub type PFNGLBLENDEQUATIONSEPARATEPROC                       = unsafe extern "C" fn(modeRGB: GLenum, modeAlpha: GLenum);
pub type PFNGLBLENDEQUATIONSEPARATEIPROC                      = unsafe extern "C" fn(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);
pub type PFNGLBLENDEQUATIONSEPARATEIARBPROC                   = unsafe extern "C" fn(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);
pub type PFNGLBLENDEQUATIONIPROC                              = unsafe extern "C" fn(buf: GLuint, mode: GLenum);
pub type PFNGLBLENDEQUATIONIARBPROC                           = unsafe extern "C" fn(buf: GLuint, mode: GLenum);
pub type PFNGLBLENDFUNCPROC                                   = unsafe extern "C" fn(sfactor: GLenum, dfactor: GLenum);
pub type PFNGLBLENDFUNCSEPARATEPROC                           = unsafe extern "C" fn(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum);
pub type PFNGLBLENDFUNCSEPARATEIPROC                          = unsafe extern "C" fn(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);
pub type PFNGLBLENDFUNCSEPARATEIARBPROC                       = unsafe extern "C" fn(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);
pub type PFNGLBLENDFUNCIPROC                                  = unsafe extern "C" fn(buf: GLuint, src: GLenum, dst: GLenum);
pub type PFNGLBLENDFUNCIARBPROC                               = unsafe extern "C" fn(buf: GLuint, src: GLenum, dst: GLenum);
pub type PFNGLBLITFRAMEBUFFERPROC                             = unsafe extern "C" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);
pub type PFNGLBLITFRAMEBUFFEREXTPROC                          = unsafe extern "C" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);
pub type PFNGLBLITNAMEDFRAMEBUFFERPROC                        = unsafe extern "C" fn(readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);
pub type PFNGLBUFFERDATAPROC                                  = unsafe extern "C" fn(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);
pub type PFNGLBUFFERDATAARBPROC                               = unsafe extern "C" fn(target: GLenum, size: GLsizeiptrARB, data: *const c_void, usage: GLenum);
pub type PFNGLBUFFERSTORAGEPROC                               = unsafe extern "C" fn(target: GLenum, size: GLsizeiptr, data: *const c_void, flags: GLbitfield);
pub type PFNGLBUFFERSUBDATAPROC                               = unsafe extern "C" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void);
pub type PFNGLBUFFERSUBDATAARBPROC                            = unsafe extern "C" fn(target: GLenum, offset: GLintptrARB, size: GLsizeiptrARB, data: *const c_void);
pub type PFNGLCHECKFRAMEBUFFERSTATUSPROC                      = unsafe extern "C" fn(target: GLenum) -> GLenum;
pub type PFNGLCHECKFRAMEBUFFERSTATUSEXTPROC                   = unsafe extern "C" fn(target: GLenum) -> GLenum;
pub type PFNGLCHECKNAMEDFRAMEBUFFERSTATUSPROC                 = unsafe extern "C" fn(framebuffer: GLuint, target: GLenum) -> GLenum;
pub type PFNGLCLAMPCOLORPROC                                  = unsafe extern "C" fn(target: GLenum, clamp: GLenum);
pub type PFNGLCLAMPCOLORARBPROC                               = unsafe extern "C" fn(target: GLenum, clamp: GLenum);
pub type PFNGLCLEARPROC                                       = unsafe extern "C" fn(mask: GLbitfield);
pub type PFNGLCLEARACCUMXOESPROC                              = unsafe extern "C" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);
pub type PFNGLCLEARBUFFERDATAPROC                             = unsafe extern "C" fn(target: GLenum, internalformat: GLenum, format: GLenum, r#type: GLenum, data: *const c_void);
pub type PFNGLCLEARBUFFERSUBDATAPROC                          = unsafe extern "C" fn(target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, r#type: GLenum, data: *const c_void);
pub type PFNGLCLEARBUFFERFIPROC                               = unsafe extern "C" fn(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
pub type PFNGLCLEARBUFFERFVPROC                               = unsafe extern "C" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
pub type PFNGLCLEARBUFFERIVPROC                               = unsafe extern "C" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLint);
pub type PFNGLCLEARBUFFERUIVPROC                              = unsafe extern "C" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
pub type PFNGLCLEARCOLORPROC                                  = unsafe extern "C" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
pub type PFNGLCLEARCOLORXOESPROC                              = unsafe extern "C" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);
pub type PFNGLCLEARDEPTHPROC                                  = unsafe extern "C" fn(depth: GLdouble);
pub type PFNGLCLEARDEPTHFPROC                                 = unsafe extern "C" fn(d: GLfloat);
pub type PFNGLCLEARDEPTHXOESPROC                              = unsafe extern "C" fn(depth: GLfixed);
pub type PFNGLCLEARNAMEDBUFFERDATAPROC                        = unsafe extern "C" fn(buffer: GLuint, internalformat: GLenum, format: GLenum, r#type: GLenum, data: *const c_void);
pub type PFNGLCLEARNAMEDBUFFERSUBDATAPROC                     = unsafe extern "C" fn(buffer: GLuint, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, r#type: GLenum, data: *const c_void);
pub type PFNGLCLEARNAMEDFRAMEBUFFERFIPROC                     = unsafe extern "C" fn(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
pub type PFNGLCLEARNAMEDFRAMEBUFFERFVPROC                     = unsafe extern "C" fn(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
pub type PFNGLCLEARNAMEDFRAMEBUFFERIVPROC                     = unsafe extern "C" fn(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLint);
pub type PFNGLCLEARNAMEDFRAMEBUFFERUIVPROC                    = unsafe extern "C" fn(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
pub type PFNGLCLEARSTENCILPROC                                = unsafe extern "C" fn(s: GLint);
pub type PFNGLCLEARTEXIMAGEPROC                               = unsafe extern "C" fn(texture: GLuint, level: GLint, format: GLenum, r#type: GLenum, data: *const c_void);
pub type PFNGLCLEARTEXSUBIMAGEPROC                            = unsafe extern "C" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, r#type: GLenum, data: *const c_void);
pub type PFNGLCLIENTACTIVETEXTUREARBPROC                      = unsafe extern "C" fn(texture: GLenum);
pub type PFNGLCLIENTWAITSYNCPROC                              = unsafe extern "C" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum;
pub type PFNGLCLIPPLANEXOESPROC                               = unsafe extern "C" fn(plane: GLenum, equation: *const GLfixed);
pub type PFNGLCOLOR3XOESPROC                                  = unsafe extern "C" fn(red: GLfixed, green: GLfixed, blue: GLfixed);
pub type PFNGLCOLOR3XVOESPROC                                 = unsafe extern "C" fn(components: *const GLfixed);
pub type PFNGLCOLOR4XOESPROC                                  = unsafe extern "C" fn(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);
pub type PFNGLCOLOR4XVOESPROC                                 = unsafe extern "C" fn(components: *const GLfixed);
pub type PFNGLCOLORMASKPROC                                   = unsafe extern "C" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
pub type PFNGLCOLORMASKIPROC                                  = unsafe extern "C" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);
pub type PFNGLCOMPILESHADERPROC                               = unsafe extern "C" fn(shader: GLuint);
pub type PFNGLCOMPILESHADERARBPROC                            = unsafe extern "C" fn(shaderObj: GLhandleARB);
pub type PFNGLCOMPILESHADERINCLUDEARBPROC                     = unsafe extern "C" fn(shader: GLuint, count: GLsizei, path: *const *const GLchar, length: *const GLint);
pub type PFNGLCOMPRESSEDTEXIMAGE1DPROC                        = unsafe extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
pub type PFNGLCOMPRESSEDTEXIMAGE1DARBPROC                     = unsafe extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
pub type PFNGLCOMPRESSEDTEXIMAGE2DPROC                        = unsafe extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
pub type PFNGLCOMPRESSEDTEXIMAGE2DARBPROC                     = unsafe extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
pub type PFNGLCOMPRESSEDTEXIMAGE3DPROC                        = unsafe extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
pub type PFNGLCOMPRESSEDTEXIMAGE3DARBPROC                     = unsafe extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
pub type PFNGLCOMPRESSEDTEXSUBIMAGE1DPROC                     = unsafe extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
pub type PFNGLCOMPRESSEDTEXSUBIMAGE1DARBPROC                  = unsafe extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
pub type PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC                     = unsafe extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
pub type PFNGLCOMPRESSEDTEXSUBIMAGE2DARBPROC                  = unsafe extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
pub type PFNGLCOMPRESSEDTEXSUBIMAGE3DPROC                     = unsafe extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
pub type PFNGLCOMPRESSEDTEXSUBIMAGE3DARBPROC                  = unsafe extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
pub type PFNGLCOMPRESSEDTEXTURESUBIMAGE1DPROC                 = unsafe extern "C" fn(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
pub type PFNGLCOMPRESSEDTEXTURESUBIMAGE2DPROC                 = unsafe extern "C" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
pub type PFNGLCOMPRESSEDTEXTURESUBIMAGE3DPROC                 = unsafe extern "C" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
pub type PFNGLCONVOLUTIONPARAMETERXOESPROC                    = unsafe extern "C" fn(target: GLenum, pname: GLenum, param: GLfixed);
pub type PFNGLCONVOLUTIONPARAMETERXVOESPROC                   = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *const GLfixed);
pub type PFNGLCOPYBUFFERSUBDATAPROC                           = unsafe extern "C" fn(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);
pub type PFNGLCOPYIMAGESUBDATAPROC                            = unsafe extern "C" fn(srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);
pub type PFNGLCOPYNAMEDBUFFERSUBDATAPROC                      = unsafe extern "C" fn(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);
pub type PFNGLCOPYTEXIMAGE1DPROC                              = unsafe extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint);
pub type PFNGLCOPYTEXIMAGE2DPROC                              = unsafe extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);
pub type PFNGLCOPYTEXSUBIMAGE1DPROC                           = unsafe extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
pub type PFNGLCOPYTEXSUBIMAGE2DPROC                           = unsafe extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
pub type PFNGLCOPYTEXSUBIMAGE3DPROC                           = unsafe extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
pub type PFNGLCOPYTEXTURESUBIMAGE1DPROC                       = unsafe extern "C" fn(texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
pub type PFNGLCOPYTEXTURESUBIMAGE2DPROC                       = unsafe extern "C" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
pub type PFNGLCOPYTEXTURESUBIMAGE3DPROC                       = unsafe extern "C" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
pub type PFNGLCREATEBUFFERSPROC                               = unsafe extern "C" fn(n: GLsizei, buffers: *mut GLuint);
pub type PFNGLCREATEFRAMEBUFFERSPROC                          = unsafe extern "C" fn(n: GLsizei, framebuffers: *mut GLuint);
pub type PFNGLCREATEPROGRAMPROC                               = unsafe extern "C" fn() -> GLuint;
pub type PFNGLCREATEPROGRAMOBJECTARBPROC                      = unsafe extern "C" fn() -> GLhandleARB;
pub type PFNGLCREATEPROGRAMPIPELINESPROC                      = unsafe extern "C" fn(n: GLsizei, pipelines: *mut GLuint);
pub type PFNGLCREATEQUERIESPROC                               = unsafe extern "C" fn(target: GLenum, n: GLsizei, ids: *mut GLuint);
pub type PFNGLCREATERENDERBUFFERSPROC                         = unsafe extern "C" fn(n: GLsizei, renderbuffers: *mut GLuint);
pub type PFNGLCREATESAMPLERSPROC                              = unsafe extern "C" fn(n: GLsizei, samplers: *mut GLuint);
pub type PFNGLCREATESHADERPROC                                = unsafe extern "C" fn(r#type: GLenum) -> GLuint;
pub type PFNGLCREATESHADEROBJECTARBPROC                       = unsafe extern "C" fn(shaderType: GLenum) -> GLhandleARB;
pub type PFNGLCREATESHADERPROGRAMVPROC                        = unsafe extern "C" fn(r#type: GLenum, count: GLsizei, strings: *const *const GLchar) -> GLuint;
pub type PFNGLCREATETEXTURESPROC                              = unsafe extern "C" fn(target: GLenum, n: GLsizei, textures: *mut GLuint);
pub type PFNGLCREATETRANSFORMFEEDBACKSPROC                    = unsafe extern "C" fn(n: GLsizei, ids: *mut GLuint);
pub type PFNGLCREATEVERTEXARRAYSPROC                          = unsafe extern "C" fn(n: GLsizei, arrays: *mut GLuint);
pub type PFNGLCULLFACEPROC                                    = unsafe extern "C" fn(mode: GLenum);
pub type PFNGLDEBUGMESSAGECALLBACKPROC                        = unsafe extern "C" fn(callback: GLDEBUGPROC, userParam: *const c_void);
pub type PFNGLDEBUGMESSAGECALLBACKARBPROC                     = unsafe extern "C" fn(callback: GLDEBUGPROCARB, userParam: *const c_void);
pub type PFNGLDEBUGMESSAGECONTROLPROC                         = unsafe extern "C" fn(source: GLenum, r#type: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean);
pub type PFNGLDEBUGMESSAGECONTROLARBPROC                      = unsafe extern "C" fn(source: GLenum, r#type: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean);
pub type PFNGLDEBUGMESSAGEINSERTPROC                          = unsafe extern "C" fn(source: GLenum, r#type: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar);
pub type PFNGLDEBUGMESSAGEINSERTARBPROC                       = unsafe extern "C" fn(source: GLenum, r#type: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar);
pub type PFNGLDELETEBUFFERSPROC                               = unsafe extern "C" fn(n: GLsizei, buffers: *const GLuint);
pub type PFNGLDELETEBUFFERSARBPROC                            = unsafe extern "C" fn(n: GLsizei, buffers: *const GLuint);
pub type PFNGLDELETEFRAMEBUFFERSPROC                          = unsafe extern "C" fn(n: GLsizei, framebuffers: *const GLuint);
pub type PFNGLDELETEFRAMEBUFFERSEXTPROC                       = unsafe extern "C" fn(n: GLsizei, framebuffers: *const GLuint);
pub type PFNGLDELETENAMEDSTRINGARBPROC                        = unsafe extern "C" fn(namelen: GLint, name: *const GLchar);
pub type PFNGLDELETEOBJECTARBPROC                             = unsafe extern "C" fn(obj: GLhandleARB);
pub type PFNGLDELETEPROGRAMPROC                               = unsafe extern "C" fn(program: GLuint);
pub type PFNGLDELETEPROGRAMPIPELINESPROC                      = unsafe extern "C" fn(n: GLsizei, pipelines: *const GLuint);
pub type PFNGLDELETEPROGRAMSARBPROC                           = unsafe extern "C" fn(n: GLsizei, programs: *const GLuint);
pub type PFNGLDELETEQUERIESPROC                               = unsafe extern "C" fn(n: GLsizei, ids: *const GLuint);
pub type PFNGLDELETEQUERIESARBPROC                            = unsafe extern "C" fn(n: GLsizei, ids: *const GLuint);
pub type PFNGLDELETERENDERBUFFERSPROC                         = unsafe extern "C" fn(n: GLsizei, renderbuffers: *const GLuint);
pub type PFNGLDELETERENDERBUFFERSEXTPROC                      = unsafe extern "C" fn(n: GLsizei, renderbuffers: *const GLuint);
pub type PFNGLDELETESAMPLERSPROC                              = unsafe extern "C" fn(count: GLsizei, samplers: *const GLuint);
pub type PFNGLDELETESHADERPROC                                = unsafe extern "C" fn(shader: GLuint);
pub type PFNGLDELETESYNCPROC                                  = unsafe extern "C" fn(sync: GLsync);
pub type PFNGLDELETETEXTURESPROC                              = unsafe extern "C" fn(n: GLsizei, textures: *const GLuint);
pub type PFNGLDELETETRANSFORMFEEDBACKSPROC                    = unsafe extern "C" fn(n: GLsizei, ids: *const GLuint);
pub type PFNGLDELETEVERTEXARRAYSPROC                          = unsafe extern "C" fn(n: GLsizei, arrays: *const GLuint);
pub type PFNGLDEPTHFUNCPROC                                   = unsafe extern "C" fn(func: GLenum);
pub type PFNGLDEPTHMASKPROC                                   = unsafe extern "C" fn(flag: GLboolean);
pub type PFNGLDEPTHRANGEPROC                                  = unsafe extern "C" fn(n: GLdouble, f: GLdouble);
pub type PFNGLDEPTHRANGEARRAYVPROC                            = unsafe extern "C" fn(first: GLuint, count: GLsizei, v: *const GLdouble);
pub type PFNGLDEPTHRANGEINDEXEDPROC                           = unsafe extern "C" fn(index: GLuint, n: GLdouble, f: GLdouble);
pub type PFNGLDEPTHRANGEFPROC                                 = unsafe extern "C" fn(n: GLfloat, f: GLfloat);
pub type PFNGLDEPTHRANGEXOESPROC                              = unsafe extern "C" fn(n: GLfixed, f: GLfixed);
pub type PFNGLDETACHOBJECTARBPROC                             = unsafe extern "C" fn(containerObj: GLhandleARB, attachedObj: GLhandleARB);
pub type PFNGLDETACHSHADERPROC                                = unsafe extern "C" fn(program: GLuint, shader: GLuint);
pub type PFNGLDISABLEPROC                                     = unsafe extern "C" fn(cap: GLenum);
pub type PFNGLDISABLEVERTEXARRAYATTRIBPROC                    = unsafe extern "C" fn(vaobj: GLuint, index: GLuint);
pub type PFNGLDISABLEVERTEXATTRIBARRAYPROC                    = unsafe extern "C" fn(index: GLuint);
pub type PFNGLDISABLEVERTEXATTRIBARRAYARBPROC                 = unsafe extern "C" fn(index: GLuint);
pub type PFNGLDISABLEIPROC                                    = unsafe extern "C" fn(target: GLenum, index: GLuint);
pub type PFNGLDISPATCHCOMPUTEPROC                             = unsafe extern "C" fn(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint);
pub type PFNGLDISPATCHCOMPUTEGROUPSIZEARBPROC                 = unsafe extern "C" fn(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint, group_size_x: GLuint, group_size_y: GLuint, group_size_z: GLuint);
pub type PFNGLDISPATCHCOMPUTEINDIRECTPROC                     = unsafe extern "C" fn(indirect: GLintptr);
pub type PFNGLDRAWARRAYSPROC                                  = unsafe extern "C" fn(mode: GLenum, first: GLint, count: GLsizei);
pub type PFNGLDRAWARRAYSINDIRECTPROC                          = unsafe extern "C" fn(mode: GLenum, indirect: *const c_void);
pub type PFNGLDRAWARRAYSINSTANCEDPROC                         = unsafe extern "C" fn(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei);
pub type PFNGLDRAWARRAYSINSTANCEDARBPROC                      = unsafe extern "C" fn(mode: GLenum, first: GLint, count: GLsizei, primcount: GLsizei);
pub type PFNGLDRAWARRAYSINSTANCEDBASEINSTANCEPROC             = unsafe extern "C" fn(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint);
pub type PFNGLDRAWARRAYSINSTANCEDEXTPROC                      = unsafe extern "C" fn(mode: GLenum, start: GLint, count: GLsizei, primcount: GLsizei);
pub type PFNGLDRAWBUFFERPROC                                  = unsafe extern "C" fn(buf: GLenum);
pub type PFNGLDRAWBUFFERSPROC                                 = unsafe extern "C" fn(n: GLsizei, bufs: *const GLenum);
pub type PFNGLDRAWBUFFERSARBPROC                              = unsafe extern "C" fn(n: GLsizei, bufs: *const GLenum);
pub type PFNGLDRAWELEMENTSPROC                                = unsafe extern "C" fn(mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const c_void);
pub type PFNGLDRAWELEMENTSBASEVERTEXPROC                      = unsafe extern "C" fn(mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const c_void, basevertex: GLint);
pub type PFNGLDRAWELEMENTSINDIRECTPROC                        = unsafe extern "C" fn(mode: GLenum, r#type: GLenum, indirect: *const c_void);
pub type PFNGLDRAWELEMENTSINSTANCEDPROC                       = unsafe extern "C" fn(mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const c_void, instancecount: GLsizei);
pub type PFNGLDRAWELEMENTSINSTANCEDARBPROC                    = unsafe extern "C" fn(mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const c_void, primcount: GLsizei);
pub type PFNGLDRAWELEMENTSINSTANCEDBASEINSTANCEPROC           = unsafe extern "C" fn(mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const c_void, instancecount: GLsizei, baseinstance: GLuint);
pub type PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC             = unsafe extern "C" fn(mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint);
pub type PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXBASEINSTANCEPROC = unsafe extern "C" fn(mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint);
pub type PFNGLDRAWELEMENTSINSTANCEDEXTPROC                    = unsafe extern "C" fn(mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const c_void, primcount: GLsizei);
pub type PFNGLDRAWRANGEELEMENTSPROC                           = unsafe extern "C" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, r#type: GLenum, indices: *const c_void);
pub type PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC                 = unsafe extern "C" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, r#type: GLenum, indices: *const c_void, basevertex: GLint);
pub type PFNGLDRAWTRANSFORMFEEDBACKPROC                       = unsafe extern "C" fn(mode: GLenum, id: GLuint);
pub type PFNGLDRAWTRANSFORMFEEDBACKINSTANCEDPROC              = unsafe extern "C" fn(mode: GLenum, id: GLuint, instancecount: GLsizei);
pub type PFNGLDRAWTRANSFORMFEEDBACKSTREAMPROC                 = unsafe extern "C" fn(mode: GLenum, id: GLuint, stream: GLuint);
pub type PFNGLDRAWTRANSFORMFEEDBACKSTREAMINSTANCEDPROC        = unsafe extern "C" fn(mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei);
pub type PFNGLENABLEPROC                                      = unsafe extern "C" fn(cap: GLenum);
pub type PFNGLENABLEVERTEXARRAYATTRIBPROC                     = unsafe extern "C" fn(vaobj: GLuint, index: GLuint);
pub type PFNGLENABLEVERTEXATTRIBARRAYPROC                     = unsafe extern "C" fn(index: GLuint);
pub type PFNGLENABLEVERTEXATTRIBARRAYARBPROC                  = unsafe extern "C" fn(index: GLuint);
pub type PFNGLENABLEIPROC                                     = unsafe extern "C" fn(target: GLenum, index: GLuint);
pub type PFNGLENDCONDITIONALRENDERPROC                        = unsafe extern "C" fn();
pub type PFNGLENDQUERYPROC                                    = unsafe extern "C" fn(target: GLenum);
pub type PFNGLENDQUERYARBPROC                                 = unsafe extern "C" fn(target: GLenum);
pub type PFNGLENDQUERYINDEXEDPROC                             = unsafe extern "C" fn(target: GLenum, index: GLuint);
pub type PFNGLENDTRANSFORMFEEDBACKPROC                        = unsafe extern "C" fn();
pub type PFNGLEVALCOORD1XOESPROC                              = unsafe extern "C" fn(u: GLfixed);
pub type PFNGLEVALCOORD1XVOESPROC                             = unsafe extern "C" fn(coords: *const GLfixed);
pub type PFNGLEVALCOORD2XOESPROC                              = unsafe extern "C" fn(u: GLfixed, v: GLfixed);
pub type PFNGLEVALCOORD2XVOESPROC                             = unsafe extern "C" fn(coords: *const GLfixed);
pub type PFNGLEVALUATEDEPTHVALUESARBPROC                      = unsafe extern "C" fn();
pub type PFNGLFEEDBACKBUFFERXOESPROC                          = unsafe extern "C" fn(n: GLsizei, r#type: GLenum, buffer: *const GLfixed);
pub type PFNGLFENCESYNCPROC                                   = unsafe extern "C" fn(condition: GLenum, flags: GLbitfield) -> GLsync;
pub type PFNGLFINISHPROC                                      = unsafe extern "C" fn();
pub type PFNGLFLUSHPROC                                       = unsafe extern "C" fn();
pub type PFNGLFLUSHMAPPEDBUFFERRANGEPROC                      = unsafe extern "C" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr);
pub type PFNGLFLUSHMAPPEDNAMEDBUFFERRANGEPROC                 = unsafe extern "C" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
pub type PFNGLFOGCOORDPOINTEREXTPROC                          = unsafe extern "C" fn(r#type: GLenum, stride: GLsizei, pointer: *const c_void);
pub type PFNGLFOGCOORDDEXTPROC                                = unsafe extern "C" fn(coord: GLdouble);
pub type PFNGLFOGCOORDDVEXTPROC                               = unsafe extern "C" fn(coord: *const GLdouble);
pub type PFNGLFOGCOORDFEXTPROC                                = unsafe extern "C" fn(coord: GLfloat);
pub type PFNGLFOGCOORDFVEXTPROC                               = unsafe extern "C" fn(coord: *const GLfloat);
pub type PFNGLFOGXOESPROC                                     = unsafe extern "C" fn(pname: GLenum, param: GLfixed);
pub type PFNGLFOGXVOESPROC                                    = unsafe extern "C" fn(pname: GLenum, param: *const GLfixed);
pub type PFNGLFRAMEBUFFERPARAMETERIPROC                       = unsafe extern "C" fn(target: GLenum, pname: GLenum, param: GLint);
pub type PFNGLFRAMEBUFFERRENDERBUFFERPROC                     = unsafe extern "C" fn(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);
pub type PFNGLFRAMEBUFFERRENDERBUFFEREXTPROC                  = unsafe extern "C" fn(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);
pub type PFNGLFRAMEBUFFERSAMPLELOCATIONSFVARBPROC             = unsafe extern "C" fn(target: GLenum, start: GLuint, count: GLsizei, v: *const GLfloat);
pub type PFNGLFRAMEBUFFERTEXTUREPROC                          = unsafe extern "C" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);
pub type PFNGLFRAMEBUFFERTEXTURE1DPROC                        = unsafe extern "C" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
pub type PFNGLFRAMEBUFFERTEXTURE1DEXTPROC                     = unsafe extern "C" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
pub type PFNGLFRAMEBUFFERTEXTURE2DPROC                        = unsafe extern "C" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
pub type PFNGLFRAMEBUFFERTEXTURE2DEXTPROC                     = unsafe extern "C" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
pub type PFNGLFRAMEBUFFERTEXTURE3DPROC                        = unsafe extern "C" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint);
pub type PFNGLFRAMEBUFFERTEXTURE3DEXTPROC                     = unsafe extern "C" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint);
pub type PFNGLFRAMEBUFFERTEXTUREARBPROC                       = unsafe extern "C" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);
pub type PFNGLFRAMEBUFFERTEXTUREFACEARBPROC                   = unsafe extern "C" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, face: GLenum);
pub type PFNGLFRAMEBUFFERTEXTURELAYERPROC                     = unsafe extern "C" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
pub type PFNGLFRAMEBUFFERTEXTURELAYERARBPROC                  = unsafe extern "C" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
pub type PFNGLFRONTFACEPROC                                   = unsafe extern "C" fn(mode: GLenum);
pub type PFNGLFRUSTUMXOESPROC                                 = unsafe extern "C" fn(l: GLfixed, r: GLfixed, b: GLfixed, t: GLfixed, n: GLfixed, f: GLfixed);
pub type PFNGLGENBUFFERSPROC                                  = unsafe extern "C" fn(n: GLsizei, buffers: *mut GLuint);
pub type PFNGLGENBUFFERSARBPROC                               = unsafe extern "C" fn(n: GLsizei, buffers: *mut GLuint);
pub type PFNGLGENFRAMEBUFFERSPROC                             = unsafe extern "C" fn(n: GLsizei, framebuffers: *mut GLuint);
pub type PFNGLGENFRAMEBUFFERSEXTPROC                          = unsafe extern "C" fn(n: GLsizei, framebuffers: *mut GLuint);
pub type PFNGLGENPROGRAMPIPELINESPROC                         = unsafe extern "C" fn(n: GLsizei, pipelines: *mut GLuint);
pub type PFNGLGENPROGRAMSARBPROC                              = unsafe extern "C" fn(n: GLsizei, programs: *mut GLuint);
pub type PFNGLGENQUERIESPROC                                  = unsafe extern "C" fn(n: GLsizei, ids: *mut GLuint);
pub type PFNGLGENQUERIESARBPROC                               = unsafe extern "C" fn(n: GLsizei, ids: *mut GLuint);
pub type PFNGLGENRENDERBUFFERSPROC                            = unsafe extern "C" fn(n: GLsizei, renderbuffers: *mut GLuint);
pub type PFNGLGENRENDERBUFFERSEXTPROC                         = unsafe extern "C" fn(n: GLsizei, renderbuffers: *mut GLuint);
pub type PFNGLGENSAMPLERSPROC                                 = unsafe extern "C" fn(count: GLsizei, samplers: *mut GLuint);
pub type PFNGLGENTEXTURESPROC                                 = unsafe extern "C" fn(n: GLsizei, textures: *mut GLuint);
pub type PFNGLGENTRANSFORMFEEDBACKSPROC                       = unsafe extern "C" fn(n: GLsizei, ids: *mut GLuint);
pub type PFNGLGENVERTEXARRAYSPROC                             = unsafe extern "C" fn(n: GLsizei, arrays: *mut GLuint);
pub type PFNGLGENERATEMIPMAPPROC                              = unsafe extern "C" fn(target: GLenum);
pub type PFNGLGENERATEMIPMAPEXTPROC                           = unsafe extern "C" fn(target: GLenum);
pub type PFNGLGENERATETEXTUREMIPMAPPROC                       = unsafe extern "C" fn(texture: GLuint);
pub type PFNGLGETACTIVEATOMICCOUNTERBUFFERIVPROC              = unsafe extern "C" fn(program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETACTIVEATTRIBPROC                             = unsafe extern "C" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, r#type: *mut GLenum, name: *mut GLchar);
pub type PFNGLGETACTIVEATTRIBARBPROC                          = unsafe extern "C" fn(programObj: GLhandleARB, index: GLuint, maxLength: GLsizei, length: *mut GLsizei, size: *mut GLint, r#type: *mut GLenum, name: *mut GLcharARB);
pub type PFNGLGETACTIVESUBROUTINENAMEPROC                     = unsafe extern "C" fn(program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);
pub type PFNGLGETACTIVESUBROUTINEUNIFORMNAMEPROC              = unsafe extern "C" fn(program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);
pub type PFNGLGETACTIVESUBROUTINEUNIFORMIVPROC                = unsafe extern "C" fn(program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint);
pub type PFNGLGETACTIVEUNIFORMPROC                            = unsafe extern "C" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, r#type: *mut GLenum, name: *mut GLchar);
pub type PFNGLGETACTIVEUNIFORMARBPROC                         = unsafe extern "C" fn(programObj: GLhandleARB, index: GLuint, maxLength: GLsizei, length: *mut GLsizei, size: *mut GLint, r#type: *mut GLenum, name: *mut GLcharARB);
pub type PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC                   = unsafe extern "C" fn(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar);
pub type PFNGLGETACTIVEUNIFORMBLOCKIVPROC                     = unsafe extern "C" fn(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETACTIVEUNIFORMNAMEPROC                        = unsafe extern "C" fn(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar);
pub type PFNGLGETACTIVEUNIFORMSIVPROC                         = unsafe extern "C" fn(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETATTACHEDOBJECTSARBPROC                       = unsafe extern "C" fn(containerObj: GLhandleARB, maxCount: GLsizei, count: *mut GLsizei, obj: *mut GLhandleARB);
pub type PFNGLGETATTACHEDSHADERSPROC                          = unsafe extern "C" fn(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint);
pub type PFNGLGETATTRIBLOCATIONPROC                           = unsafe extern "C" fn(program: GLuint, name: *const GLchar) -> GLint;
pub type PFNGLGETATTRIBLOCATIONARBPROC                        = unsafe extern "C" fn(programObj: GLhandleARB, name: *const GLcharARB) -> GLint;
pub type PFNGLGETBOOLEANI_VPROC                               = unsafe extern "C" fn(target: GLenum, index: GLuint, data: *mut GLboolean);
pub type PFNGLGETBOOLEANVPROC                                 = unsafe extern "C" fn(pname: GLenum, data: *mut GLboolean);
pub type PFNGLGETBUFFERPARAMETERI64VPROC                      = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint64);
pub type PFNGLGETBUFFERPARAMETERIVPROC                        = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETBUFFERPARAMETERIVARBPROC                     = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETBUFFERPOINTERVPROC                           = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut *mut c_void);
pub type PFNGLGETBUFFERPOINTERVARBPROC                        = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut *mut c_void);
pub type PFNGLGETBUFFERSUBDATAPROC                            = unsafe extern "C" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void);
pub type PFNGLGETBUFFERSUBDATAARBPROC                         = unsafe extern "C" fn(target: GLenum, offset: GLintptrARB, size: GLsizeiptrARB, data: *mut c_void);
pub type PFNGLGETCLIPPLANEXOESPROC                            = unsafe extern "C" fn(plane: GLenum, equation: *mut GLfixed);
pub type PFNGLGETCOMPRESSEDTEXIMAGEPROC                       = unsafe extern "C" fn(target: GLenum, level: GLint, img: *mut c_void);
pub type PFNGLGETCOMPRESSEDTEXIMAGEARBPROC                    = unsafe extern "C" fn(target: GLenum, level: GLint, img: *mut c_void);
pub type PFNGLGETCOMPRESSEDTEXTUREIMAGEPROC                   = unsafe extern "C" fn(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut c_void);
pub type PFNGLGETCOMPRESSEDTEXTURESUBIMAGEPROC                = unsafe extern "C" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut c_void);
pub type PFNGLGETCONVOLUTIONPARAMETERXVOESPROC                = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLfixed);
pub type PFNGLGETDEBUGMESSAGELOGPROC                          = unsafe extern "C" fn(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint;
pub type PFNGLGETDEBUGMESSAGELOGARBPROC                       = unsafe extern "C" fn(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint;
pub type PFNGLGETDOUBLEI_VPROC                                = unsafe extern "C" fn(target: GLenum, index: GLuint, data: *mut GLdouble);
pub type PFNGLGETDOUBLEVPROC                                  = unsafe extern "C" fn(pname: GLenum, data: *mut GLdouble);
pub type PFNGLGETERRORPROC                                    = unsafe extern "C" fn() -> GLenum;
pub type PFNGLGETFIXEDVOESPROC                                = unsafe extern "C" fn(pname: GLenum, params: *mut GLfixed);
pub type PFNGLGETFLOATI_VPROC                                 = unsafe extern "C" fn(target: GLenum, index: GLuint, data: *mut GLfloat);
pub type PFNGLGETFLOATVPROC                                   = unsafe extern "C" fn(pname: GLenum, data: *mut GLfloat);
pub type PFNGLGETFRAGDATAINDEXPROC                            = unsafe extern "C" fn(program: GLuint, name: *const GLchar) -> GLint;
pub type PFNGLGETFRAGDATALOCATIONPROC                         = unsafe extern "C" fn(program: GLuint, name: *const GLchar) -> GLint;
pub type PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC         = unsafe extern "C" fn(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVEXTPROC      = unsafe extern "C" fn(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETFRAMEBUFFERPARAMETERIVPROC                   = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETHANDLEARBPROC                                = unsafe extern "C" fn(pname: GLenum) -> GLhandleARB;
pub type PFNGLGETHISTOGRAMPARAMETERXVOESPROC                  = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLfixed);
pub type PFNGLGETINFOLOGARBPROC                               = unsafe extern "C" fn(obj: GLhandleARB, maxLength: GLsizei, length: *mut GLsizei, infoLog: *mut GLcharARB);
pub type PFNGLGETINTEGER64I_VPROC                             = unsafe extern "C" fn(target: GLenum, index: GLuint, data: *mut GLint64);
pub type PFNGLGETINTEGER64VPROC                               = unsafe extern "C" fn(pname: GLenum, data: *mut GLint64);
pub type PFNGLGETINTEGERI_VPROC                               = unsafe extern "C" fn(target: GLenum, index: GLuint, data: *mut GLint);
pub type PFNGLGETINTEGERVPROC                                 = unsafe extern "C" fn(pname: GLenum, data: *mut GLint);
pub type PFNGLGETINTERNALFORMATI64VPROC                       = unsafe extern "C" fn(target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint64);
pub type PFNGLGETINTERNALFORMATIVPROC                         = unsafe extern "C" fn(target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint);
pub type PFNGLGETLIGHTXOESPROC                                = unsafe extern "C" fn(light: GLenum, pname: GLenum, params: *mut GLfixed);
pub type PFNGLGETMAPXVOESPROC                                 = unsafe extern "C" fn(target: GLenum, query: GLenum, v: *mut GLfixed);
pub type PFNGLGETMATERIALXOESPROC                             = unsafe extern "C" fn(face: GLenum, pname: GLenum, param: GLfixed);
pub type PFNGLGETMULTISAMPLEFVPROC                            = unsafe extern "C" fn(pname: GLenum, index: GLuint, val: *mut GLfloat);
pub type PFNGLGETNAMEDBUFFERPARAMETERI64VPROC                 = unsafe extern "C" fn(buffer: GLuint, pname: GLenum, params: *mut GLint64);
pub type PFNGLGETNAMEDBUFFERPARAMETERIVPROC                   = unsafe extern "C" fn(buffer: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETNAMEDBUFFERPOINTERVPROC                      = unsafe extern "C" fn(buffer: GLuint, pname: GLenum, params: *mut *mut c_void);
pub type PFNGLGETNAMEDBUFFERSUBDATAPROC                       = unsafe extern "C" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut c_void);
pub type PFNGLGETNAMEDFRAMEBUFFERATTACHMENTPARAMETERIVPROC    = unsafe extern "C" fn(framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETNAMEDFRAMEBUFFERPARAMETERIVPROC              = unsafe extern "C" fn(framebuffer: GLuint, pname: GLenum, param: *mut GLint);
pub type PFNGLGETNAMEDRENDERBUFFERPARAMETERIVPROC             = unsafe extern "C" fn(renderbuffer: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETNAMEDSTRINGARBPROC                           = unsafe extern "C" fn(namelen: GLint, name: *const GLchar, bufSize: GLsizei, stringlen: *mut GLint, string: *mut GLchar);
pub type PFNGLGETNAMEDSTRINGIVARBPROC                         = unsafe extern "C" fn(namelen: GLint, name: *const GLchar, pname: GLenum, params: *mut GLint);
pub type PFNGLGETOBJECTLABELPROC                              = unsafe extern "C" fn(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);
pub type PFNGLGETOBJECTPARAMETERFVARBPROC                     = unsafe extern "C" fn(obj: GLhandleARB, pname: GLenum, params: *mut GLfloat);
pub type PFNGLGETOBJECTPARAMETERIVARBPROC                     = unsafe extern "C" fn(obj: GLhandleARB, pname: GLenum, params: *mut GLint);
pub type PFNGLGETOBJECTPTRLABELPROC                           = unsafe extern "C" fn(ptr: *const c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);
pub type PFNGLGETPIXELMAPXVPROC                               = unsafe extern "C" fn(map: GLenum, size: GLint, values: *mut GLfixed);
pub type PFNGLGETPOINTERVPROC                                 = unsafe extern "C" fn(pname: GLenum, params: *mut *mut c_void);
pub type PFNGLGETPROGRAMBINARYPROC                            = unsafe extern "C" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut c_void);
pub type PFNGLGETPROGRAMENVPARAMETERDVARBPROC                 = unsafe extern "C" fn(target: GLenum, index: GLuint, params: *mut GLdouble);
pub type PFNGLGETPROGRAMENVPARAMETERFVARBPROC                 = unsafe extern "C" fn(target: GLenum, index: GLuint, params: *mut GLfloat);
pub type PFNGLGETPROGRAMINFOLOGPROC                           = unsafe extern "C" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
pub type PFNGLGETPROGRAMINTERFACEIVPROC                       = unsafe extern "C" fn(program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETPROGRAMLOCALPARAMETERDVARBPROC               = unsafe extern "C" fn(target: GLenum, index: GLuint, params: *mut GLdouble);
pub type PFNGLGETPROGRAMLOCALPARAMETERFVARBPROC               = unsafe extern "C" fn(target: GLenum, index: GLuint, params: *mut GLfloat);
pub type PFNGLGETPROGRAMPIPELINEINFOLOGPROC                   = unsafe extern "C" fn(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
pub type PFNGLGETPROGRAMPIPELINEIVPROC                        = unsafe extern "C" fn(pipeline: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETPROGRAMRESOURCEINDEXPROC                     = unsafe extern "C" fn(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLuint;
pub type PFNGLGETPROGRAMRESOURCELOCATIONPROC                  = unsafe extern "C" fn(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint;
pub type PFNGLGETPROGRAMRESOURCELOCATIONINDEXPROC             = unsafe extern "C" fn(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint;
pub type PFNGLGETPROGRAMRESOURCENAMEPROC                      = unsafe extern "C" fn(program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);
pub type PFNGLGETPROGRAMRESOURCEIVPROC                        = unsafe extern "C" fn(program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, count: GLsizei, length: *mut GLsizei, params: *mut GLint);
pub type PFNGLGETPROGRAMSTAGEIVPROC                           = unsafe extern "C" fn(program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint);
pub type PFNGLGETPROGRAMSTRINGARBPROC                         = unsafe extern "C" fn(target: GLenum, pname: GLenum, string: *mut c_void);
pub type PFNGLGETPROGRAMIVPROC                                = unsafe extern "C" fn(program: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETPROGRAMIVARBPROC                             = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETQUERYBUFFEROBJECTI64VPROC                    = unsafe extern "C" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
pub type PFNGLGETQUERYBUFFEROBJECTIVPROC                      = unsafe extern "C" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
pub type PFNGLGETQUERYBUFFEROBJECTUI64VPROC                   = unsafe extern "C" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
pub type PFNGLGETQUERYBUFFEROBJECTUIVPROC                     = unsafe extern "C" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
pub type PFNGLGETQUERYINDEXEDIVPROC                           = unsafe extern "C" fn(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETQUERYOBJECTI64VPROC                          = unsafe extern "C" fn(id: GLuint, pname: GLenum, params: *mut GLint64);
pub type PFNGLGETQUERYOBJECTIVPROC                            = unsafe extern "C" fn(id: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETQUERYOBJECTIVARBPROC                         = unsafe extern "C" fn(id: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETQUERYOBJECTUI64VPROC                         = unsafe extern "C" fn(id: GLuint, pname: GLenum, params: *mut GLuint64);
pub type PFNGLGETQUERYOBJECTUIVPROC                           = unsafe extern "C" fn(id: GLuint, pname: GLenum, params: *mut GLuint);
pub type PFNGLGETQUERYOBJECTUIVARBPROC                        = unsafe extern "C" fn(id: GLuint, pname: GLenum, params: *mut GLuint);
pub type PFNGLGETQUERYIVPROC                                  = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETQUERYIVARBPROC                               = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETRENDERBUFFERPARAMETERIVPROC                  = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETRENDERBUFFERPARAMETERIVEXTPROC               = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETSAMPLERPARAMETERIIVPROC                      = unsafe extern "C" fn(sampler: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETSAMPLERPARAMETERIUIVPROC                     = unsafe extern "C" fn(sampler: GLuint, pname: GLenum, params: *mut GLuint);
pub type PFNGLGETSAMPLERPARAMETERFVPROC                       = unsafe extern "C" fn(sampler: GLuint, pname: GLenum, params: *mut GLfloat);
pub type PFNGLGETSAMPLERPARAMETERIVPROC                       = unsafe extern "C" fn(sampler: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETSHADERINFOLOGPROC                            = unsafe extern "C" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
pub type PFNGLGETSHADERPRECISIONFORMATPROC                    = unsafe extern "C" fn(shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint);
pub type PFNGLGETSHADERSOURCEPROC                             = unsafe extern "C" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar);
pub type PFNGLGETSHADERSOURCEARBPROC                          = unsafe extern "C" fn(obj: GLhandleARB, maxLength: GLsizei, length: *mut GLsizei, source: *mut GLcharARB);
pub type PFNGLGETSHADERIVPROC                                 = unsafe extern "C" fn(shader: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETSTRINGPROC                                   = unsafe extern "C" fn(name: GLenum) -> *const GLubyte;
pub type PFNGLGETSTRINGIPROC                                  = unsafe extern "C" fn(name: GLenum, index: GLuint) -> *const GLubyte;
pub type PFNGLGETSUBROUTINEINDEXPROC                          = unsafe extern "C" fn(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLuint;
pub type PFNGLGETSUBROUTINEUNIFORMLOCATIONPROC                = unsafe extern "C" fn(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLint;
pub type PFNGLGETSYNCIVPROC                                   = unsafe extern "C" fn(sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint);
pub type PFNGLGETTEXENVXVOESPROC                              = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLfixed);
pub type PFNGLGETTEXGENXVOESPROC                              = unsafe extern "C" fn(coord: GLenum, pname: GLenum, params: *mut GLfixed);
pub type PFNGLGETTEXIMAGEPROC                                 = unsafe extern "C" fn(target: GLenum, level: GLint, format: GLenum, r#type: GLenum, pixels: *mut c_void);
pub type PFNGLGETTEXLEVELPARAMETERFVPROC                      = unsafe extern "C" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat);
pub type PFNGLGETTEXLEVELPARAMETERIVPROC                      = unsafe extern "C" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETTEXLEVELPARAMETERXVOESPROC                   = unsafe extern "C" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfixed);
pub type PFNGLGETTEXPARAMETERIIVPROC                          = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETTEXPARAMETERIUIVPROC                         = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLuint);
pub type PFNGLGETTEXPARAMETERFVPROC                           = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLfloat);
pub type PFNGLGETTEXPARAMETERIVPROC                           = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint);
pub type PFNGLGETTEXPARAMETERXVOESPROC                        = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLfixed);
pub type PFNGLGETTEXTUREIMAGEPROC                             = unsafe extern "C" fn(texture: GLuint, level: GLint, format: GLenum, r#type: GLenum, bufSize: GLsizei, pixels: *mut c_void);
pub type PFNGLGETTEXTURELEVELPARAMETERFVPROC                  = unsafe extern "C" fn(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat);
pub type PFNGLGETTEXTURELEVELPARAMETERIVPROC                  = unsafe extern "C" fn(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETTEXTUREPARAMETERIIVPROC                      = unsafe extern "C" fn(texture: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETTEXTUREPARAMETERIUIVPROC                     = unsafe extern "C" fn(texture: GLuint, pname: GLenum, params: *mut GLuint);
pub type PFNGLGETTEXTUREPARAMETERFVPROC                       = unsafe extern "C" fn(texture: GLuint, pname: GLenum, params: *mut GLfloat);
pub type PFNGLGETTEXTUREPARAMETERIVPROC                       = unsafe extern "C" fn(texture: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETTEXTURESUBIMAGEPROC                          = unsafe extern "C" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, r#type: GLenum, bufSize: GLsizei, pixels: *mut c_void);
pub type PFNGLGETTRANSFORMFEEDBACKVARYINGPROC                 = unsafe extern "C" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, r#type: *mut GLenum, name: *mut GLchar);
pub type PFNGLGETTRANSFORMFEEDBACKI64_VPROC                   = unsafe extern "C" fn(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64);
pub type PFNGLGETTRANSFORMFEEDBACKI_VPROC                     = unsafe extern "C" fn(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint);
pub type PFNGLGETTRANSFORMFEEDBACKIVPROC                      = unsafe extern "C" fn(xfb: GLuint, pname: GLenum, param: *mut GLint);
pub type PFNGLGETUNIFORMBLOCKINDEXPROC                        = unsafe extern "C" fn(program: GLuint, uniformBlockName: *const GLchar) -> GLuint;
pub type PFNGLGETUNIFORMINDICESPROC                           = unsafe extern "C" fn(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint);
pub type PFNGLGETUNIFORMLOCATIONPROC                          = unsafe extern "C" fn(program: GLuint, name: *const GLchar) -> GLint;
pub type PFNGLGETUNIFORMLOCATIONARBPROC                       = unsafe extern "C" fn(programObj: GLhandleARB, name: *const GLcharARB) -> GLint;
pub type PFNGLGETUNIFORMSUBROUTINEUIVPROC                     = unsafe extern "C" fn(shadertype: GLenum, location: GLint, params: *mut GLuint);
pub type PFNGLGETUNIFORMDVPROC                                = unsafe extern "C" fn(program: GLuint, location: GLint, params: *mut GLdouble);
pub type PFNGLGETUNIFORMFVPROC                                = unsafe extern "C" fn(program: GLuint, location: GLint, params: *mut GLfloat);
pub type PFNGLGETUNIFORMFVARBPROC                             = unsafe extern "C" fn(programObj: GLhandleARB, location: GLint, params: *mut GLfloat);
pub type PFNGLGETUNIFORMI64VARBPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, params: *mut GLint64);
pub type PFNGLGETUNIFORMIVPROC                                = unsafe extern "C" fn(program: GLuint, location: GLint, params: *mut GLint);
pub type PFNGLGETUNIFORMIVARBPROC                             = unsafe extern "C" fn(programObj: GLhandleARB, location: GLint, params: *mut GLint);
pub type PFNGLGETUNIFORMUI64VARBPROC                          = unsafe extern "C" fn(program: GLuint, location: GLint, params: *mut GLuint64);
pub type PFNGLGETUNIFORMUIVPROC                               = unsafe extern "C" fn(program: GLuint, location: GLint, params: *mut GLuint);
pub type PFNGLGETVERTEXARRAYINDEXED64IVPROC                   = unsafe extern "C" fn(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64);
pub type PFNGLGETVERTEXARRAYINDEXEDIVPROC                     = unsafe extern "C" fn(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint);
pub type PFNGLGETVERTEXARRAYIVPROC                            = unsafe extern "C" fn(vaobj: GLuint, pname: GLenum, param: *mut GLint);
pub type PFNGLGETVERTEXATTRIBIIVPROC                          = unsafe extern "C" fn(index: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETVERTEXATTRIBIUIVPROC                         = unsafe extern "C" fn(index: GLuint, pname: GLenum, params: *mut GLuint);
pub type PFNGLGETVERTEXATTRIBLDVPROC                          = unsafe extern "C" fn(index: GLuint, pname: GLenum, params: *mut GLdouble);
pub type PFNGLGETVERTEXATTRIBPOINTERVPROC                     = unsafe extern "C" fn(index: GLuint, pname: GLenum, pointer: *mut *mut c_void);
pub type PFNGLGETVERTEXATTRIBPOINTERVARBPROC                  = unsafe extern "C" fn(index: GLuint, pname: GLenum, pointer: *mut *mut c_void);
pub type PFNGLGETVERTEXATTRIBDVPROC                           = unsafe extern "C" fn(index: GLuint, pname: GLenum, params: *mut GLdouble);
pub type PFNGLGETVERTEXATTRIBDVARBPROC                        = unsafe extern "C" fn(index: GLuint, pname: GLenum, params: *mut GLdouble);
pub type PFNGLGETVERTEXATTRIBFVPROC                           = unsafe extern "C" fn(index: GLuint, pname: GLenum, params: *mut GLfloat);
pub type PFNGLGETVERTEXATTRIBFVARBPROC                        = unsafe extern "C" fn(index: GLuint, pname: GLenum, params: *mut GLfloat);
pub type PFNGLGETVERTEXATTRIBIVPROC                           = unsafe extern "C" fn(index: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETVERTEXATTRIBIVARBPROC                        = unsafe extern "C" fn(index: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETNUNIFORMI64VARBPROC                          = unsafe extern "C" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint64);
pub type PFNGLGETNUNIFORMUI64VARBPROC                         = unsafe extern "C" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint64);
pub type PFNGLHINTPROC                                        = unsafe extern "C" fn(target: GLenum, mode: GLenum);
pub type PFNGLINDEXXOESPROC                                   = unsafe extern "C" fn(component: GLfixed);
pub type PFNGLINDEXXVOESPROC                                  = unsafe extern "C" fn(component: *const GLfixed);
pub type PFNGLINVALIDATEBUFFERDATAPROC                        = unsafe extern "C" fn(buffer: GLuint);
pub type PFNGLINVALIDATEBUFFERSUBDATAPROC                     = unsafe extern "C" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
pub type PFNGLINVALIDATEFRAMEBUFFERPROC                       = unsafe extern "C" fn(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum);
pub type PFNGLINVALIDATENAMEDFRAMEBUFFERDATAPROC              = unsafe extern "C" fn(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum);
pub type PFNGLINVALIDATENAMEDFRAMEBUFFERSUBDATAPROC           = unsafe extern "C" fn(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
pub type PFNGLINVALIDATESUBFRAMEBUFFERPROC                    = unsafe extern "C" fn(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
pub type PFNGLINVALIDATETEXIMAGEPROC                          = unsafe extern "C" fn(texture: GLuint, level: GLint);
pub type PFNGLINVALIDATETEXSUBIMAGEPROC                       = unsafe extern "C" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);
pub type PFNGLISBUFFERPROC                                    = unsafe extern "C" fn(buffer: GLuint) -> GLboolean;
pub type PFNGLISBUFFERARBPROC                                 = unsafe extern "C" fn(buffer: GLuint) -> GLboolean;
pub type PFNGLISENABLEDPROC                                   = unsafe extern "C" fn(cap: GLenum) -> GLboolean;
pub type PFNGLISENABLEDIPROC                                  = unsafe extern "C" fn(target: GLenum, index: GLuint) -> GLboolean;
pub type PFNGLISFRAMEBUFFERPROC                               = unsafe extern "C" fn(framebuffer: GLuint) -> GLboolean;
pub type PFNGLISFRAMEBUFFEREXTPROC                            = unsafe extern "C" fn(framebuffer: GLuint) -> GLboolean;
pub type PFNGLISNAMEDSTRINGARBPROC                            = unsafe extern "C" fn(namelen: GLint, name: *const GLchar) -> GLboolean;
pub type PFNGLISPROGRAMPROC                                   = unsafe extern "C" fn(program: GLuint) -> GLboolean;
pub type PFNGLISPROGRAMARBPROC                                = unsafe extern "C" fn(program: GLuint) -> GLboolean;
pub type PFNGLISPROGRAMPIPELINEPROC                           = unsafe extern "C" fn(pipeline: GLuint) -> GLboolean;
pub type PFNGLISQUERYPROC                                     = unsafe extern "C" fn(id: GLuint) -> GLboolean;
pub type PFNGLISQUERYARBPROC                                  = unsafe extern "C" fn(id: GLuint) -> GLboolean;
pub type PFNGLISRENDERBUFFERPROC                              = unsafe extern "C" fn(renderbuffer: GLuint) -> GLboolean;
pub type PFNGLISRENDERBUFFEREXTPROC                           = unsafe extern "C" fn(renderbuffer: GLuint) -> GLboolean;
pub type PFNGLISSAMPLERPROC                                   = unsafe extern "C" fn(sampler: GLuint) -> GLboolean;
pub type PFNGLISSHADERPROC                                    = unsafe extern "C" fn(shader: GLuint) -> GLboolean;
pub type PFNGLISSYNCPROC                                      = unsafe extern "C" fn(sync: GLsync) -> GLboolean;
pub type PFNGLISTEXTUREPROC                                   = unsafe extern "C" fn(texture: GLuint) -> GLboolean;
pub type PFNGLISTRANSFORMFEEDBACKPROC                         = unsafe extern "C" fn(id: GLuint) -> GLboolean;
pub type PFNGLISVERTEXARRAYPROC                               = unsafe extern "C" fn(array: GLuint) -> GLboolean;
pub type PFNGLLIGHTMODELXOESPROC                              = unsafe extern "C" fn(pname: GLenum, param: GLfixed);
pub type PFNGLLIGHTMODELXVOESPROC                             = unsafe extern "C" fn(pname: GLenum, param: *const GLfixed);
pub type PFNGLLIGHTXOESPROC                                   = unsafe extern "C" fn(light: GLenum, pname: GLenum, param: GLfixed);
pub type PFNGLLIGHTXVOESPROC                                  = unsafe extern "C" fn(light: GLenum, pname: GLenum, params: *const GLfixed);
pub type PFNGLLINEWIDTHPROC                                   = unsafe extern "C" fn(width: GLfloat);
pub type PFNGLLINEWIDTHXOESPROC                               = unsafe extern "C" fn(width: GLfixed);
pub type PFNGLLINKPROGRAMPROC                                 = unsafe extern "C" fn(program: GLuint);
pub type PFNGLLINKPROGRAMARBPROC                              = unsafe extern "C" fn(programObj: GLhandleARB);
pub type PFNGLLOADMATRIXXOESPROC                              = unsafe extern "C" fn(m: *const GLfixed);
pub type PFNGLLOADTRANSPOSEMATRIXDARBPROC                     = unsafe extern "C" fn(m: *const GLdouble);
pub type PFNGLLOADTRANSPOSEMATRIXFARBPROC                     = unsafe extern "C" fn(m: *const GLfloat);
pub type PFNGLLOADTRANSPOSEMATRIXXOESPROC                     = unsafe extern "C" fn(m: *const GLfixed);
pub type PFNGLLOGICOPPROC                                     = unsafe extern "C" fn(opcode: GLenum);
pub type PFNGLMAP1XOESPROC                                    = unsafe extern "C" fn(target: GLenum, u1: GLfixed, u2: GLfixed, stride: GLint, order: GLint, points: GLfixed);
pub type PFNGLMAP2XOESPROC                                    = unsafe extern "C" fn(target: GLenum, u1: GLfixed, u2: GLfixed, ustride: GLint, uorder: GLint, v1: GLfixed, v2: GLfixed, vstride: GLint, vorder: GLint, points: GLfixed);
pub type PFNGLMAPBUFFERPROC                                   = unsafe extern "C" fn(target: GLenum, access: GLenum) -> *mut c_void;
pub type PFNGLMAPBUFFERARBPROC                                = unsafe extern "C" fn(target: GLenum, access: GLenum) -> *mut c_void;
pub type PFNGLMAPBUFFERRANGEPROC                              = unsafe extern "C" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void;
pub type PFNGLMAPGRID1XOESPROC                                = unsafe extern "C" fn(n: GLint, u1: GLfixed, u2: GLfixed);
pub type PFNGLMAPGRID2XOESPROC                                = unsafe extern "C" fn(n: GLint, u1: GLfixed, u2: GLfixed, v1: GLfixed, v2: GLfixed);
pub type PFNGLMAPNAMEDBUFFERPROC                              = unsafe extern "C" fn(buffer: GLuint, access: GLenum) -> *mut c_void;
pub type PFNGLMAPNAMEDBUFFERRANGEPROC                         = unsafe extern "C" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void;
pub type PFNGLMATERIALXOESPROC                                = unsafe extern "C" fn(face: GLenum, pname: GLenum, param: GLfixed);
pub type PFNGLMATERIALXVOESPROC                               = unsafe extern "C" fn(face: GLenum, pname: GLenum, param: *const GLfixed);
pub type PFNGLMEMORYBARRIERPROC                               = unsafe extern "C" fn(barriers: GLbitfield);
pub type PFNGLMEMORYBARRIERBYREGIONPROC                       = unsafe extern "C" fn(barriers: GLbitfield);
pub type PFNGLMINSAMPLESHADINGPROC                            = unsafe extern "C" fn(value: GLfloat);
pub type PFNGLMINSAMPLESHADINGARBPROC                         = unsafe extern "C" fn(value: GLfloat);
pub type PFNGLMULTMATRIXXOESPROC                              = unsafe extern "C" fn(m: *const GLfixed);
pub type PFNGLMULTTRANSPOSEMATRIXDARBPROC                     = unsafe extern "C" fn(m: *const GLdouble);
pub type PFNGLMULTTRANSPOSEMATRIXFARBPROC                     = unsafe extern "C" fn(m: *const GLfloat);
pub type PFNGLMULTTRANSPOSEMATRIXXOESPROC                     = unsafe extern "C" fn(m: *const GLfixed);
pub type PFNGLMULTIDRAWARRAYSPROC                             = unsafe extern "C" fn(mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei);
pub type PFNGLMULTIDRAWARRAYSINDIRECTPROC                     = unsafe extern "C" fn(mode: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei);
pub type PFNGLMULTIDRAWELEMENTSPROC                           = unsafe extern "C" fn(mode: GLenum, count: *const GLsizei, r#type: GLenum, indices: *const *const c_void, drawcount: GLsizei);
pub type PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC                 = unsafe extern "C" fn(mode: GLenum, count: *const GLsizei, r#type: GLenum, indices: *const *const c_void, drawcount: GLsizei, basevertex: *const GLint);
pub type PFNGLMULTIDRAWELEMENTSINDIRECTPROC                   = unsafe extern "C" fn(mode: GLenum, r#type: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei);
pub type PFNGLMULTITEXCOORD1DARBPROC                          = unsafe extern "C" fn(target: GLenum, s: GLdouble);
pub type PFNGLMULTITEXCOORD1DVARBPROC                         = unsafe extern "C" fn(target: GLenum, v: *const GLdouble);
pub type PFNGLMULTITEXCOORD1FARBPROC                          = unsafe extern "C" fn(target: GLenum, s: GLfloat);
pub type PFNGLMULTITEXCOORD1FVARBPROC                         = unsafe extern "C" fn(target: GLenum, v: *const GLfloat);
pub type PFNGLMULTITEXCOORD1IARBPROC                          = unsafe extern "C" fn(target: GLenum, s: GLint);
pub type PFNGLMULTITEXCOORD1IVARBPROC                         = unsafe extern "C" fn(target: GLenum, v: *const GLint);
pub type PFNGLMULTITEXCOORD1SARBPROC                          = unsafe extern "C" fn(target: GLenum, s: GLshort);
pub type PFNGLMULTITEXCOORD1SVARBPROC                         = unsafe extern "C" fn(target: GLenum, v: *const GLshort);
pub type PFNGLMULTITEXCOORD1XOESPROC                          = unsafe extern "C" fn(texture: GLenum, s: GLfixed);
pub type PFNGLMULTITEXCOORD1XVOESPROC                         = unsafe extern "C" fn(texture: GLenum, coords: *const GLfixed);
pub type PFNGLMULTITEXCOORD2DARBPROC                          = unsafe extern "C" fn(target: GLenum, s: GLdouble, t: GLdouble);
pub type PFNGLMULTITEXCOORD2DVARBPROC                         = unsafe extern "C" fn(target: GLenum, v: *const GLdouble);
pub type PFNGLMULTITEXCOORD2FARBPROC                          = unsafe extern "C" fn(target: GLenum, s: GLfloat, t: GLfloat);
pub type PFNGLMULTITEXCOORD2FVARBPROC                         = unsafe extern "C" fn(target: GLenum, v: *const GLfloat);
pub type PFNGLMULTITEXCOORD2IARBPROC                          = unsafe extern "C" fn(target: GLenum, s: GLint, t: GLint);
pub type PFNGLMULTITEXCOORD2IVARBPROC                         = unsafe extern "C" fn(target: GLenum, v: *const GLint);
pub type PFNGLMULTITEXCOORD2SARBPROC                          = unsafe extern "C" fn(target: GLenum, s: GLshort, t: GLshort);
pub type PFNGLMULTITEXCOORD2SVARBPROC                         = unsafe extern "C" fn(target: GLenum, v: *const GLshort);
pub type PFNGLMULTITEXCOORD2XOESPROC                          = unsafe extern "C" fn(texture: GLenum, s: GLfixed, t: GLfixed);
pub type PFNGLMULTITEXCOORD2XVOESPROC                         = unsafe extern "C" fn(texture: GLenum, coords: *const GLfixed);
pub type PFNGLMULTITEXCOORD3DARBPROC                          = unsafe extern "C" fn(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble);
pub type PFNGLMULTITEXCOORD3DVARBPROC                         = unsafe extern "C" fn(target: GLenum, v: *const GLdouble);
pub type PFNGLMULTITEXCOORD3FARBPROC                          = unsafe extern "C" fn(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat);
pub type PFNGLMULTITEXCOORD3FVARBPROC                         = unsafe extern "C" fn(target: GLenum, v: *const GLfloat);
pub type PFNGLMULTITEXCOORD3IARBPROC                          = unsafe extern "C" fn(target: GLenum, s: GLint, t: GLint, r: GLint);
pub type PFNGLMULTITEXCOORD3IVARBPROC                         = unsafe extern "C" fn(target: GLenum, v: *const GLint);
pub type PFNGLMULTITEXCOORD3SARBPROC                          = unsafe extern "C" fn(target: GLenum, s: GLshort, t: GLshort, r: GLshort);
pub type PFNGLMULTITEXCOORD3SVARBPROC                         = unsafe extern "C" fn(target: GLenum, v: *const GLshort);
pub type PFNGLMULTITEXCOORD3XOESPROC                          = unsafe extern "C" fn(texture: GLenum, s: GLfixed, t: GLfixed, r: GLfixed);
pub type PFNGLMULTITEXCOORD3XVOESPROC                         = unsafe extern "C" fn(texture: GLenum, coords: *const GLfixed);
pub type PFNGLMULTITEXCOORD4DARBPROC                          = unsafe extern "C" fn(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble);
pub type PFNGLMULTITEXCOORD4DVARBPROC                         = unsafe extern "C" fn(target: GLenum, v: *const GLdouble);
pub type PFNGLMULTITEXCOORD4FARBPROC                          = unsafe extern "C" fn(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);
pub type PFNGLMULTITEXCOORD4FVARBPROC                         = unsafe extern "C" fn(target: GLenum, v: *const GLfloat);
pub type PFNGLMULTITEXCOORD4IARBPROC                          = unsafe extern "C" fn(target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint);
pub type PFNGLMULTITEXCOORD4IVARBPROC                         = unsafe extern "C" fn(target: GLenum, v: *const GLint);
pub type PFNGLMULTITEXCOORD4SARBPROC                          = unsafe extern "C" fn(target: GLenum, s: GLshort, t: GLshort, r: GLshort, q: GLshort);
pub type PFNGLMULTITEXCOORD4SVARBPROC                         = unsafe extern "C" fn(target: GLenum, v: *const GLshort);
pub type PFNGLMULTITEXCOORD4XOESPROC                          = unsafe extern "C" fn(texture: GLenum, s: GLfixed, t: GLfixed, r: GLfixed, q: GLfixed);
pub type PFNGLMULTITEXCOORD4XVOESPROC                         = unsafe extern "C" fn(texture: GLenum, coords: *const GLfixed);
pub type PFNGLNAMEDBUFFERDATAPROC                             = unsafe extern "C" fn(buffer: GLuint, size: GLsizeiptr, data: *const c_void, usage: GLenum);
pub type PFNGLNAMEDBUFFERSTORAGEPROC                          = unsafe extern "C" fn(buffer: GLuint, size: GLsizeiptr, data: *const c_void, flags: GLbitfield);
pub type PFNGLNAMEDBUFFERSUBDATAPROC                          = unsafe extern "C" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const c_void);
pub type PFNGLNAMEDFRAMEBUFFERDRAWBUFFERPROC                  = unsafe extern "C" fn(framebuffer: GLuint, buf: GLenum);
pub type PFNGLNAMEDFRAMEBUFFERDRAWBUFFERSPROC                 = unsafe extern "C" fn(framebuffer: GLuint, n: GLsizei, bufs: *const GLenum);
pub type PFNGLNAMEDFRAMEBUFFERPARAMETERIPROC                  = unsafe extern "C" fn(framebuffer: GLuint, pname: GLenum, param: GLint);
pub type PFNGLNAMEDFRAMEBUFFERREADBUFFERPROC                  = unsafe extern "C" fn(framebuffer: GLuint, src: GLenum);
pub type PFNGLNAMEDFRAMEBUFFERRENDERBUFFERPROC                = unsafe extern "C" fn(framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);
pub type PFNGLNAMEDFRAMEBUFFERSAMPLELOCATIONSFVARBPROC        = unsafe extern "C" fn(framebuffer: GLuint, start: GLuint, count: GLsizei, v: *const GLfloat);
pub type PFNGLNAMEDFRAMEBUFFERTEXTUREPROC                     = unsafe extern "C" fn(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint);
pub type PFNGLNAMEDFRAMEBUFFERTEXTURELAYERPROC                = unsafe extern "C" fn(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
pub type PFNGLNAMEDRENDERBUFFERSTORAGEPROC                    = unsafe extern "C" fn(renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei);
pub type PFNGLNAMEDRENDERBUFFERSTORAGEMULTISAMPLEPROC         = unsafe extern "C" fn(renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
pub type PFNGLNAMEDSTRINGARBPROC                              = unsafe extern "C" fn(r#type: GLenum, namelen: GLint, name: *const GLchar, stringlen: GLint, string: *const GLchar);
pub type PFNGLNORMAL3XOESPROC                                 = unsafe extern "C" fn(nx: GLfixed, ny: GLfixed, nz: GLfixed);
pub type PFNGLNORMAL3XVOESPROC                                = unsafe extern "C" fn(coords: *const GLfixed);
pub type PFNGLOBJECTLABELPROC                                 = unsafe extern "C" fn(identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar);
pub type PFNGLOBJECTPTRLABELPROC                              = unsafe extern "C" fn(ptr: *const c_void, length: GLsizei, label: *const GLchar);
pub type PFNGLORTHOXOESPROC                                   = unsafe extern "C" fn(l: GLfixed, r: GLfixed, b: GLfixed, t: GLfixed, n: GLfixed, f: GLfixed);
pub type PFNGLPASSTHROUGHXOESPROC                             = unsafe extern "C" fn(token: GLfixed);
pub type PFNGLPATCHPARAMETERFVPROC                            = unsafe extern "C" fn(pname: GLenum, values: *const GLfloat);
pub type PFNGLPATCHPARAMETERIPROC                             = unsafe extern "C" fn(pname: GLenum, value: GLint);
pub type PFNGLPAUSETRANSFORMFEEDBACKPROC                      = unsafe extern "C" fn();
pub type PFNGLPIXELMAPXPROC                                   = unsafe extern "C" fn(map: GLenum, size: GLint, values: *const GLfixed);
pub type PFNGLPIXELSTOREFPROC                                 = unsafe extern "C" fn(pname: GLenum, param: GLfloat);
pub type PFNGLPIXELSTOREIPROC                                 = unsafe extern "C" fn(pname: GLenum, param: GLint);
pub type PFNGLPIXELSTOREXPROC                                 = unsafe extern "C" fn(pname: GLenum, param: GLfixed);
pub type PFNGLPIXELTRANSFERXOESPROC                           = unsafe extern "C" fn(pname: GLenum, param: GLfixed);
pub type PFNGLPIXELZOOMXOESPROC                               = unsafe extern "C" fn(xfactor: GLfixed, yfactor: GLfixed);
pub type PFNGLPOINTPARAMETERFPROC                             = unsafe extern "C" fn(pname: GLenum, param: GLfloat);
pub type PFNGLPOINTPARAMETERFVPROC                            = unsafe extern "C" fn(pname: GLenum, params: *const GLfloat);
pub type PFNGLPOINTPARAMETERIPROC                             = unsafe extern "C" fn(pname: GLenum, param: GLint);
pub type PFNGLPOINTPARAMETERIVPROC                            = unsafe extern "C" fn(pname: GLenum, params: *const GLint);
pub type PFNGLPOINTPARAMETERXVOESPROC                         = unsafe extern "C" fn(pname: GLenum, params: *const GLfixed);
pub type PFNGLPOINTSIZEPROC                                   = unsafe extern "C" fn(size: GLfloat);
pub type PFNGLPOINTSIZEXOESPROC                               = unsafe extern "C" fn(size: GLfixed);
pub type PFNGLPOLYGONMODEPROC                                 = unsafe extern "C" fn(face: GLenum, mode: GLenum);
pub type PFNGLPOLYGONOFFSETPROC                               = unsafe extern "C" fn(factor: GLfloat, units: GLfloat);
pub type PFNGLPOLYGONOFFSETXOESPROC                           = unsafe extern "C" fn(factor: GLfixed, units: GLfixed);
pub type PFNGLPOPDEBUGGROUPPROC                               = unsafe extern "C" fn();
pub type PFNGLPRIMITIVEBOUNDINGBOXARBPROC                     = unsafe extern "C" fn(minX: GLfloat, minY: GLfloat, minZ: GLfloat, minW: GLfloat, maxX: GLfloat, maxY: GLfloat, maxZ: GLfloat, maxW: GLfloat);
pub type PFNGLPRIMITIVERESTARTINDEXPROC                       = unsafe extern "C" fn(index: GLuint);
pub type PFNGLPRIORITIZETEXTURESXOESPROC                      = unsafe extern "C" fn(n: GLsizei, textures: *const GLuint, priorities: *const GLfixed);
pub type PFNGLPROGRAMBINARYPROC                               = unsafe extern "C" fn(program: GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei);
pub type PFNGLPROGRAMENVPARAMETER4DARBPROC                    = unsafe extern "C" fn(target: GLenum, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
pub type PFNGLPROGRAMENVPARAMETER4DVARBPROC                   = unsafe extern "C" fn(target: GLenum, index: GLuint, params: *const GLdouble);
pub type PFNGLPROGRAMENVPARAMETER4FARBPROC                    = unsafe extern "C" fn(target: GLenum, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
pub type PFNGLPROGRAMENVPARAMETER4FVARBPROC                   = unsafe extern "C" fn(target: GLenum, index: GLuint, params: *const GLfloat);
pub type PFNGLPROGRAMLOCALPARAMETER4DARBPROC                  = unsafe extern "C" fn(target: GLenum, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
pub type PFNGLPROGRAMLOCALPARAMETER4DVARBPROC                 = unsafe extern "C" fn(target: GLenum, index: GLuint, params: *const GLdouble);
pub type PFNGLPROGRAMLOCALPARAMETER4FARBPROC                  = unsafe extern "C" fn(target: GLenum, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
pub type PFNGLPROGRAMLOCALPARAMETER4FVARBPROC                 = unsafe extern "C" fn(target: GLenum, index: GLuint, params: *const GLfloat);
pub type PFNGLPROGRAMPARAMETERIPROC                           = unsafe extern "C" fn(program: GLuint, pname: GLenum, value: GLint);
pub type PFNGLPROGRAMPARAMETERIARBPROC                        = unsafe extern "C" fn(program: GLuint, pname: GLenum, value: GLint);
pub type PFNGLPROGRAMSTRINGARBPROC                            = unsafe extern "C" fn(target: GLenum, format: GLenum, len: GLsizei, string: *const c_void);
pub type PFNGLPROGRAMUNIFORM1DPROC                            = unsafe extern "C" fn(program: GLuint, location: GLint, v0: GLdouble);
pub type PFNGLPROGRAMUNIFORM1DVPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
pub type PFNGLPROGRAMUNIFORM1FPROC                            = unsafe extern "C" fn(program: GLuint, location: GLint, v0: GLfloat);
pub type PFNGLPROGRAMUNIFORM1FVPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
pub type PFNGLPROGRAMUNIFORM1IPROC                            = unsafe extern "C" fn(program: GLuint, location: GLint, v0: GLint);
pub type PFNGLPROGRAMUNIFORM1I64ARBPROC                       = unsafe extern "C" fn(program: GLuint, location: GLint, x: GLint64);
pub type PFNGLPROGRAMUNIFORM1I64VARBPROC                      = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);
pub type PFNGLPROGRAMUNIFORM1IVPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
pub type PFNGLPROGRAMUNIFORM1UIPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, v0: GLuint);
pub type PFNGLPROGRAMUNIFORM1UI64ARBPROC                      = unsafe extern "C" fn(program: GLuint, location: GLint, x: GLuint64);
pub type PFNGLPROGRAMUNIFORM1UI64VARBPROC                     = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);
pub type PFNGLPROGRAMUNIFORM1UIVPROC                          = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
pub type PFNGLPROGRAMUNIFORM2DPROC                            = unsafe extern "C" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble);
pub type PFNGLPROGRAMUNIFORM2DVPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
pub type PFNGLPROGRAMUNIFORM2FPROC                            = unsafe extern "C" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);
pub type PFNGLPROGRAMUNIFORM2FVPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
pub type PFNGLPROGRAMUNIFORM2IPROC                            = unsafe extern "C" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint);
pub type PFNGLPROGRAMUNIFORM2I64ARBPROC                       = unsafe extern "C" fn(program: GLuint, location: GLint, x: GLint64, y: GLint64);
pub type PFNGLPROGRAMUNIFORM2I64VARBPROC                      = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);
pub type PFNGLPROGRAMUNIFORM2IVPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
pub type PFNGLPROGRAMUNIFORM2UIPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint);
pub type PFNGLPROGRAMUNIFORM2UI64ARBPROC                      = unsafe extern "C" fn(program: GLuint, location: GLint, x: GLuint64, y: GLuint64);
pub type PFNGLPROGRAMUNIFORM2UI64VARBPROC                     = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);
pub type PFNGLPROGRAMUNIFORM2UIVPROC                          = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
pub type PFNGLPROGRAMUNIFORM3DPROC                            = unsafe extern "C" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble);
pub type PFNGLPROGRAMUNIFORM3DVPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
pub type PFNGLPROGRAMUNIFORM3FPROC                            = unsafe extern "C" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
pub type PFNGLPROGRAMUNIFORM3FVPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
pub type PFNGLPROGRAMUNIFORM3IPROC                            = unsafe extern "C" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);
pub type PFNGLPROGRAMUNIFORM3I64ARBPROC                       = unsafe extern "C" fn(program: GLuint, location: GLint, x: GLint64, y: GLint64, z: GLint64);
pub type PFNGLPROGRAMUNIFORM3I64VARBPROC                      = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);
pub type PFNGLPROGRAMUNIFORM3IVPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
pub type PFNGLPROGRAMUNIFORM3UIPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
pub type PFNGLPROGRAMUNIFORM3UI64ARBPROC                      = unsafe extern "C" fn(program: GLuint, location: GLint, x: GLuint64, y: GLuint64, z: GLuint64);
pub type PFNGLPROGRAMUNIFORM3UI64VARBPROC                     = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);
pub type PFNGLPROGRAMUNIFORM3UIVPROC                          = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
pub type PFNGLPROGRAMUNIFORM4DPROC                            = unsafe extern "C" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble);
pub type PFNGLPROGRAMUNIFORM4DVPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
pub type PFNGLPROGRAMUNIFORM4FPROC                            = unsafe extern "C" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
pub type PFNGLPROGRAMUNIFORM4FVPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
pub type PFNGLPROGRAMUNIFORM4IPROC                            = unsafe extern "C" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
pub type PFNGLPROGRAMUNIFORM4I64ARBPROC                       = unsafe extern "C" fn(program: GLuint, location: GLint, x: GLint64, y: GLint64, z: GLint64, w: GLint64);
pub type PFNGLPROGRAMUNIFORM4I64VARBPROC                      = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint64);
pub type PFNGLPROGRAMUNIFORM4IVPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
pub type PFNGLPROGRAMUNIFORM4UIPROC                           = unsafe extern "C" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
pub type PFNGLPROGRAMUNIFORM4UI64ARBPROC                      = unsafe extern "C" fn(program: GLuint, location: GLint, x: GLuint64, y: GLuint64, z: GLuint64, w: GLuint64);
pub type PFNGLPROGRAMUNIFORM4UI64VARBPROC                     = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint64);
pub type PFNGLPROGRAMUNIFORM4UIVPROC                          = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
pub type PFNGLPROGRAMUNIFORMMATRIX2DVPROC                     = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLPROGRAMUNIFORMMATRIX2FVPROC                     = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLPROGRAMUNIFORMMATRIX2X3DVPROC                   = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLPROGRAMUNIFORMMATRIX2X3FVPROC                   = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLPROGRAMUNIFORMMATRIX2X4DVPROC                   = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLPROGRAMUNIFORMMATRIX2X4FVPROC                   = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLPROGRAMUNIFORMMATRIX3DVPROC                     = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLPROGRAMUNIFORMMATRIX3FVPROC                     = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLPROGRAMUNIFORMMATRIX3X2DVPROC                   = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLPROGRAMUNIFORMMATRIX3X2FVPROC                   = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLPROGRAMUNIFORMMATRIX3X4DVPROC                   = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLPROGRAMUNIFORMMATRIX3X4FVPROC                   = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLPROGRAMUNIFORMMATRIX4DVPROC                     = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLPROGRAMUNIFORMMATRIX4FVPROC                     = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLPROGRAMUNIFORMMATRIX4X2DVPROC                   = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLPROGRAMUNIFORMMATRIX4X2FVPROC                   = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLPROGRAMUNIFORMMATRIX4X3DVPROC                   = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLPROGRAMUNIFORMMATRIX4X3FVPROC                   = unsafe extern "C" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLPROVOKINGVERTEXPROC                             = unsafe extern "C" fn(mode: GLenum);
pub type PFNGLPUSHDEBUGGROUPPROC                              = unsafe extern "C" fn(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar);
pub type PFNGLQUERYCOUNTERPROC                                = unsafe extern "C" fn(id: GLuint, target: GLenum);
pub type PFNGLRASTERPOS2XOESPROC                              = unsafe extern "C" fn(x: GLfixed, y: GLfixed);
pub type PFNGLRASTERPOS2XVOESPROC                             = unsafe extern "C" fn(coords: *const GLfixed);
pub type PFNGLRASTERPOS3XOESPROC                              = unsafe extern "C" fn(x: GLfixed, y: GLfixed, z: GLfixed);
pub type PFNGLRASTERPOS3XVOESPROC                             = unsafe extern "C" fn(coords: *const GLfixed);
pub type PFNGLRASTERPOS4XOESPROC                              = unsafe extern "C" fn(x: GLfixed, y: GLfixed, z: GLfixed, w: GLfixed);
pub type PFNGLRASTERPOS4XVOESPROC                             = unsafe extern "C" fn(coords: *const GLfixed);
pub type PFNGLREADBUFFERPROC                                  = unsafe extern "C" fn(src: GLenum);
pub type PFNGLREADPIXELSPROC                                  = unsafe extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, r#type: GLenum, pixels: *mut c_void);
pub type PFNGLRECTXOESPROC                                    = unsafe extern "C" fn(x1: GLfixed, y1: GLfixed, x2: GLfixed, y2: GLfixed);
pub type PFNGLRECTXVOESPROC                                   = unsafe extern "C" fn(v1: *const GLfixed, v2: *const GLfixed);
pub type PFNGLRELEASESHADERCOMPILERPROC                       = unsafe extern "C" fn();
pub type PFNGLRENDERBUFFERSTORAGEPROC                         = unsafe extern "C" fn(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei);
pub type PFNGLRENDERBUFFERSTORAGEEXTPROC                      = unsafe extern "C" fn(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei);
pub type PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC              = unsafe extern "C" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
pub type PFNGLRENDERBUFFERSTORAGEMULTISAMPLEEXTPROC           = unsafe extern "C" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
pub type PFNGLRESUMETRANSFORMFEEDBACKPROC                     = unsafe extern "C" fn();
pub type PFNGLROTATEXOESPROC                                  = unsafe extern "C" fn(angle: GLfixed, x: GLfixed, y: GLfixed, z: GLfixed);
pub type PFNGLSAMPLECOVERAGEPROC                              = unsafe extern "C" fn(value: GLfloat, invert: GLboolean);
pub type PFNGLSAMPLECOVERAGEARBPROC                           = unsafe extern "C" fn(value: GLfloat, invert: GLboolean);
pub type PFNGLSAMPLEMASKIPROC                                 = unsafe extern "C" fn(maskNumber: GLuint, mask: GLbitfield);
pub type PFNGLSAMPLERPARAMETERIIVPROC                         = unsafe extern "C" fn(sampler: GLuint, pname: GLenum, param: *const GLint);
pub type PFNGLSAMPLERPARAMETERIUIVPROC                        = unsafe extern "C" fn(sampler: GLuint, pname: GLenum, param: *const GLuint);
pub type PFNGLSAMPLERPARAMETERFPROC                           = unsafe extern "C" fn(sampler: GLuint, pname: GLenum, param: GLfloat);
pub type PFNGLSAMPLERPARAMETERFVPROC                          = unsafe extern "C" fn(sampler: GLuint, pname: GLenum, param: *const GLfloat);
pub type PFNGLSAMPLERPARAMETERIPROC                           = unsafe extern "C" fn(sampler: GLuint, pname: GLenum, param: GLint);
pub type PFNGLSAMPLERPARAMETERIVPROC                          = unsafe extern "C" fn(sampler: GLuint, pname: GLenum, param: *const GLint);
pub type PFNGLSCALEXOESPROC                                   = unsafe extern "C" fn(x: GLfixed, y: GLfixed, z: GLfixed);
pub type PFNGLSCISSORPROC                                     = unsafe extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
pub type PFNGLSCISSORARRAYVPROC                               = unsafe extern "C" fn(first: GLuint, count: GLsizei, v: *const GLint);
pub type PFNGLSCISSORINDEXEDPROC                              = unsafe extern "C" fn(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei);
pub type PFNGLSCISSORINDEXEDVPROC                             = unsafe extern "C" fn(index: GLuint, v: *const GLint);
pub type PFNGLSHADERBINARYPROC                                = unsafe extern "C" fn(count: GLsizei, shaders: *const GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei);
pub type PFNGLSHADERSOURCEPROC                                = unsafe extern "C" fn(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint);
pub type PFNGLSHADERSOURCEARBPROC                             = unsafe extern "C" fn(shaderObj: GLhandleARB, count: GLsizei, string: *mut *const GLcharARB, length: *const GLint);
pub type PFNGLSHADERSTORAGEBLOCKBINDINGPROC                   = unsafe extern "C" fn(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint);
pub type PFNGLSPECIALIZESHADERARBPROC                         = unsafe extern "C" fn(shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint);
pub type PFNGLSTENCILFUNCPROC                                 = unsafe extern "C" fn(func: GLenum, r#ref: GLint, mask: GLuint);
pub type PFNGLSTENCILFUNCSEPARATEPROC                         = unsafe extern "C" fn(face: GLenum, func: GLenum, r#ref: GLint, mask: GLuint);
pub type PFNGLSTENCILMASKPROC                                 = unsafe extern "C" fn(mask: GLuint);
pub type PFNGLSTENCILMASKSEPARATEPROC                         = unsafe extern "C" fn(face: GLenum, mask: GLuint);
pub type PFNGLSTENCILOPPROC                                   = unsafe extern "C" fn(fail: GLenum, zfail: GLenum, zpass: GLenum);
pub type PFNGLSTENCILOPSEPARATEPROC                           = unsafe extern "C" fn(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
pub type PFNGLTEXBUFFERPROC                                   = unsafe extern "C" fn(target: GLenum, internalformat: GLenum, buffer: GLuint);
pub type PFNGLTEXBUFFERRANGEPROC                              = unsafe extern "C" fn(target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
pub type PFNGLTEXCOORD1XOESPROC                               = unsafe extern "C" fn(s: GLfixed);
pub type PFNGLTEXCOORD1XVOESPROC                              = unsafe extern "C" fn(coords: *const GLfixed);
pub type PFNGLTEXCOORD2XOESPROC                               = unsafe extern "C" fn(s: GLfixed, t: GLfixed);
pub type PFNGLTEXCOORD2XVOESPROC                              = unsafe extern "C" fn(coords: *const GLfixed);
pub type PFNGLTEXCOORD3XOESPROC                               = unsafe extern "C" fn(s: GLfixed, t: GLfixed, r: GLfixed);
pub type PFNGLTEXCOORD3XVOESPROC                              = unsafe extern "C" fn(coords: *const GLfixed);
pub type PFNGLTEXCOORD4XOESPROC                               = unsafe extern "C" fn(s: GLfixed, t: GLfixed, r: GLfixed, q: GLfixed);
pub type PFNGLTEXCOORD4XVOESPROC                              = unsafe extern "C" fn(coords: *const GLfixed);
pub type PFNGLTEXENVXOESPROC                                  = unsafe extern "C" fn(target: GLenum, pname: GLenum, param: GLfixed);
pub type PFNGLTEXENVXVOESPROC                                 = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *const GLfixed);
pub type PFNGLTEXGENXOESPROC                                  = unsafe extern "C" fn(coord: GLenum, pname: GLenum, param: GLfixed);
pub type PFNGLTEXGENXVOESPROC                                 = unsafe extern "C" fn(coord: GLenum, pname: GLenum, params: *const GLfixed);
pub type PFNGLTEXIMAGE1DPROC                                  = unsafe extern "C" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, r#type: GLenum, pixels: *const c_void);
pub type PFNGLTEXIMAGE2DPROC                                  = unsafe extern "C" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, r#type: GLenum, pixels: *const c_void);
pub type PFNGLTEXIMAGE2DMULTISAMPLEPROC                       = unsafe extern "C" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
pub type PFNGLTEXIMAGE3DPROC                                  = unsafe extern "C" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, r#type: GLenum, pixels: *const c_void);
pub type PFNGLTEXIMAGE3DMULTISAMPLEPROC                       = unsafe extern "C" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
pub type PFNGLTEXPARAMETERIIVPROC                             = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *const GLint);
pub type PFNGLTEXPARAMETERIUIVPROC                            = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *const GLuint);
pub type PFNGLTEXPARAMETERFPROC                               = unsafe extern "C" fn(target: GLenum, pname: GLenum, param: GLfloat);
pub type PFNGLTEXPARAMETERFVPROC                              = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *const GLfloat);
pub type PFNGLTEXPARAMETERIPROC                               = unsafe extern "C" fn(target: GLenum, pname: GLenum, param: GLint);
pub type PFNGLTEXPARAMETERIVPROC                              = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *const GLint);
pub type PFNGLTEXPARAMETERXOESPROC                            = unsafe extern "C" fn(target: GLenum, pname: GLenum, param: GLfixed);
pub type PFNGLTEXPARAMETERXVOESPROC                           = unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *const GLfixed);
pub type PFNGLTEXSTORAGE1DPROC                                = unsafe extern "C" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei);
pub type PFNGLTEXSTORAGE2DPROC                                = unsafe extern "C" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
pub type PFNGLTEXSTORAGE2DMULTISAMPLEPROC                     = unsafe extern "C" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
pub type PFNGLTEXSTORAGE3DPROC                                = unsafe extern "C" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);
pub type PFNGLTEXSTORAGE3DMULTISAMPLEPROC                     = unsafe extern "C" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
pub type PFNGLTEXSUBIMAGE1DPROC                               = unsafe extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, r#type: GLenum, pixels: *const c_void);
pub type PFNGLTEXSUBIMAGE2DPROC                               = unsafe extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, r#type: GLenum, pixels: *const c_void);
pub type PFNGLTEXSUBIMAGE3DPROC                               = unsafe extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, r#type: GLenum, pixels: *const c_void);
pub type PFNGLTEXTUREBUFFERPROC                               = unsafe extern "C" fn(texture: GLuint, internalformat: GLenum, buffer: GLuint);
pub type PFNGLTEXTUREBUFFERRANGEPROC                          = unsafe extern "C" fn(texture: GLuint, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
pub type PFNGLTEXTUREPARAMETERIIVPROC                         = unsafe extern "C" fn(texture: GLuint, pname: GLenum, params: *const GLint);
pub type PFNGLTEXTUREPARAMETERIUIVPROC                        = unsafe extern "C" fn(texture: GLuint, pname: GLenum, params: *const GLuint);
pub type PFNGLTEXTUREPARAMETERFPROC                           = unsafe extern "C" fn(texture: GLuint, pname: GLenum, param: GLfloat);
pub type PFNGLTEXTUREPARAMETERFVPROC                          = unsafe extern "C" fn(texture: GLuint, pname: GLenum, param: *const GLfloat);
pub type PFNGLTEXTUREPARAMETERIPROC                           = unsafe extern "C" fn(texture: GLuint, pname: GLenum, param: GLint);
pub type PFNGLTEXTUREPARAMETERIVPROC                          = unsafe extern "C" fn(texture: GLuint, pname: GLenum, param: *const GLint);
pub type PFNGLTEXTURESTORAGE1DPROC                            = unsafe extern "C" fn(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei);
pub type PFNGLTEXTURESTORAGE2DPROC                            = unsafe extern "C" fn(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
pub type PFNGLTEXTURESTORAGE2DMULTISAMPLEPROC                 = unsafe extern "C" fn(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
pub type PFNGLTEXTURESTORAGE3DPROC                            = unsafe extern "C" fn(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);
pub type PFNGLTEXTURESTORAGE3DMULTISAMPLEPROC                 = unsafe extern "C" fn(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
pub type PFNGLTEXTURESUBIMAGE1DPROC                           = unsafe extern "C" fn(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, r#type: GLenum, pixels: *const c_void);
pub type PFNGLTEXTURESUBIMAGE2DPROC                           = unsafe extern "C" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, r#type: GLenum, pixels: *const c_void);
pub type PFNGLTEXTURESUBIMAGE3DPROC                           = unsafe extern "C" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, r#type: GLenum, pixels: *const c_void);
pub type PFNGLTEXTUREVIEWPROC                                 = unsafe extern "C" fn(texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint);
pub type PFNGLTRANSFORMFEEDBACKBUFFERBASEPROC                 = unsafe extern "C" fn(xfb: GLuint, index: GLuint, buffer: GLuint);
pub type PFNGLTRANSFORMFEEDBACKBUFFERRANGEPROC                = unsafe extern "C" fn(xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
pub type PFNGLTRANSFORMFEEDBACKVARYINGSPROC                   = unsafe extern "C" fn(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum);
pub type PFNGLTRANSLATEXOESPROC                               = unsafe extern "C" fn(x: GLfixed, y: GLfixed, z: GLfixed);
pub type PFNGLUNIFORM1DPROC                                   = unsafe extern "C" fn(location: GLint, x: GLdouble);
pub type PFNGLUNIFORM1DVPROC                                  = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLdouble);
pub type PFNGLUNIFORM1FPROC                                   = unsafe extern "C" fn(location: GLint, v0: GLfloat);
pub type PFNGLUNIFORM1FARBPROC                                = unsafe extern "C" fn(location: GLint, v0: GLfloat);
pub type PFNGLUNIFORM1FVPROC                                  = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type PFNGLUNIFORM1FVARBPROC                               = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type PFNGLUNIFORM1IPROC                                   = unsafe extern "C" fn(location: GLint, v0: GLint);
pub type PFNGLUNIFORM1I64ARBPROC                              = unsafe extern "C" fn(location: GLint, x: GLint64);
pub type PFNGLUNIFORM1I64VARBPROC                             = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLint64);
pub type PFNGLUNIFORM1IARBPROC                                = unsafe extern "C" fn(location: GLint, v0: GLint);
pub type PFNGLUNIFORM1IVPROC                                  = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLint);
pub type PFNGLUNIFORM1IVARBPROC                               = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLint);
pub type PFNGLUNIFORM1UIPROC                                  = unsafe extern "C" fn(location: GLint, v0: GLuint);
pub type PFNGLUNIFORM1UI64ARBPROC                             = unsafe extern "C" fn(location: GLint, x: GLuint64);
pub type PFNGLUNIFORM1UI64VARBPROC                            = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLuint64);
pub type PFNGLUNIFORM1UIVPROC                                 = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLuint);
pub type PFNGLUNIFORM2DPROC                                   = unsafe extern "C" fn(location: GLint, x: GLdouble, y: GLdouble);
pub type PFNGLUNIFORM2DVPROC                                  = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLdouble);
pub type PFNGLUNIFORM2FPROC                                   = unsafe extern "C" fn(location: GLint, v0: GLfloat, v1: GLfloat);
pub type PFNGLUNIFORM2FARBPROC                                = unsafe extern "C" fn(location: GLint, v0: GLfloat, v1: GLfloat);
pub type PFNGLUNIFORM2FVPROC                                  = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type PFNGLUNIFORM2FVARBPROC                               = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type PFNGLUNIFORM2IPROC                                   = unsafe extern "C" fn(location: GLint, v0: GLint, v1: GLint);
pub type PFNGLUNIFORM2I64ARBPROC                              = unsafe extern "C" fn(location: GLint, x: GLint64, y: GLint64);
pub type PFNGLUNIFORM2I64VARBPROC                             = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLint64);
pub type PFNGLUNIFORM2IARBPROC                                = unsafe extern "C" fn(location: GLint, v0: GLint, v1: GLint);
pub type PFNGLUNIFORM2IVPROC                                  = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLint);
pub type PFNGLUNIFORM2IVARBPROC                               = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLint);
pub type PFNGLUNIFORM2UIPROC                                  = unsafe extern "C" fn(location: GLint, v0: GLuint, v1: GLuint);
pub type PFNGLUNIFORM2UI64ARBPROC                             = unsafe extern "C" fn(location: GLint, x: GLuint64, y: GLuint64);
pub type PFNGLUNIFORM2UI64VARBPROC                            = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLuint64);
pub type PFNGLUNIFORM2UIVPROC                                 = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLuint);
pub type PFNGLUNIFORM3DPROC                                   = unsafe extern "C" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);
pub type PFNGLUNIFORM3DVPROC                                  = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLdouble);
pub type PFNGLUNIFORM3FPROC                                   = unsafe extern "C" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
pub type PFNGLUNIFORM3FARBPROC                                = unsafe extern "C" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
pub type PFNGLUNIFORM3FVPROC                                  = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type PFNGLUNIFORM3FVARBPROC                               = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type PFNGLUNIFORM3IPROC                                   = unsafe extern "C" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint);
pub type PFNGLUNIFORM3I64ARBPROC                              = unsafe extern "C" fn(location: GLint, x: GLint64, y: GLint64, z: GLint64);
pub type PFNGLUNIFORM3I64VARBPROC                             = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLint64);
pub type PFNGLUNIFORM3IARBPROC                                = unsafe extern "C" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint);
pub type PFNGLUNIFORM3IVPROC                                  = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLint);
pub type PFNGLUNIFORM3IVARBPROC                               = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLint);
pub type PFNGLUNIFORM3UIPROC                                  = unsafe extern "C" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
pub type PFNGLUNIFORM3UI64ARBPROC                             = unsafe extern "C" fn(location: GLint, x: GLuint64, y: GLuint64, z: GLuint64);
pub type PFNGLUNIFORM3UI64VARBPROC                            = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLuint64);
pub type PFNGLUNIFORM3UIVPROC                                 = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLuint);
pub type PFNGLUNIFORM4DPROC                                   = unsafe extern "C" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
pub type PFNGLUNIFORM4DVPROC                                  = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLdouble);
pub type PFNGLUNIFORM4FPROC                                   = unsafe extern "C" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
pub type PFNGLUNIFORM4FARBPROC                                = unsafe extern "C" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
pub type PFNGLUNIFORM4FVPROC                                  = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type PFNGLUNIFORM4FVARBPROC                               = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLfloat);
pub type PFNGLUNIFORM4IPROC                                   = unsafe extern "C" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
pub type PFNGLUNIFORM4I64ARBPROC                              = unsafe extern "C" fn(location: GLint, x: GLint64, y: GLint64, z: GLint64, w: GLint64);
pub type PFNGLUNIFORM4I64VARBPROC                             = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLint64);
pub type PFNGLUNIFORM4IARBPROC                                = unsafe extern "C" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
pub type PFNGLUNIFORM4IVPROC                                  = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLint);
pub type PFNGLUNIFORM4IVARBPROC                               = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLint);
pub type PFNGLUNIFORM4UIPROC                                  = unsafe extern "C" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
pub type PFNGLUNIFORM4UI64ARBPROC                             = unsafe extern "C" fn(location: GLint, x: GLuint64, y: GLuint64, z: GLuint64, w: GLuint64);
pub type PFNGLUNIFORM4UI64VARBPROC                            = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLuint64);
pub type PFNGLUNIFORM4UIVPROC                                 = unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLuint);
pub type PFNGLUNIFORMBLOCKBINDINGPROC                         = unsafe extern "C" fn(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint);
pub type PFNGLUNIFORMMATRIX2DVPROC                            = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLUNIFORMMATRIX2FVPROC                            = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLUNIFORMMATRIX2FVARBPROC                         = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLUNIFORMMATRIX2X3DVPROC                          = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLUNIFORMMATRIX2X3FVPROC                          = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLUNIFORMMATRIX2X4DVPROC                          = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLUNIFORMMATRIX2X4FVPROC                          = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLUNIFORMMATRIX3DVPROC                            = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLUNIFORMMATRIX3FVPROC                            = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLUNIFORMMATRIX3FVARBPROC                         = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLUNIFORMMATRIX3X2DVPROC                          = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLUNIFORMMATRIX3X2FVPROC                          = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLUNIFORMMATRIX3X4DVPROC                          = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLUNIFORMMATRIX3X4FVPROC                          = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLUNIFORMMATRIX4DVPROC                            = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLUNIFORMMATRIX4FVPROC                            = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLUNIFORMMATRIX4FVARBPROC                         = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLUNIFORMMATRIX4X2DVPROC                          = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLUNIFORMMATRIX4X2FVPROC                          = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLUNIFORMMATRIX4X3DVPROC                          = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
pub type PFNGLUNIFORMMATRIX4X3FVPROC                          = unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
pub type PFNGLUNIFORMSUBROUTINESUIVPROC                       = unsafe extern "C" fn(shadertype: GLenum, count: GLsizei, indices: *const GLuint);
pub type PFNGLUNMAPBUFFERPROC                                 = unsafe extern "C" fn(target: GLenum) -> GLboolean;
pub type PFNGLUNMAPBUFFERARBPROC                              = unsafe extern "C" fn(target: GLenum) -> GLboolean;
pub type PFNGLUNMAPNAMEDBUFFERPROC                            = unsafe extern "C" fn(buffer: GLuint) -> GLboolean;
pub type PFNGLUSEPROGRAMPROC                                  = unsafe extern "C" fn(program: GLuint);
pub type PFNGLUSEPROGRAMOBJECTARBPROC                         = unsafe extern "C" fn(programObj: GLhandleARB);
pub type PFNGLUSEPROGRAMSTAGESPROC                            = unsafe extern "C" fn(pipeline: GLuint, stages: GLbitfield, program: GLuint);
pub type PFNGLVALIDATEPROGRAMPROC                             = unsafe extern "C" fn(program: GLuint);
pub type PFNGLVALIDATEPROGRAMARBPROC                          = unsafe extern "C" fn(programObj: GLhandleARB);
pub type PFNGLVALIDATEPROGRAMPIPELINEPROC                     = unsafe extern "C" fn(pipeline: GLuint);
pub type PFNGLVERTEX2XOESPROC                                 = unsafe extern "C" fn(x: GLfixed);
pub type PFNGLVERTEX2XVOESPROC                                = unsafe extern "C" fn(coords: *const GLfixed);
pub type PFNGLVERTEX3XOESPROC                                 = unsafe extern "C" fn(x: GLfixed, y: GLfixed);
pub type PFNGLVERTEX3XVOESPROC                                = unsafe extern "C" fn(coords: *const GLfixed);
pub type PFNGLVERTEX4XOESPROC                                 = unsafe extern "C" fn(x: GLfixed, y: GLfixed, z: GLfixed);
pub type PFNGLVERTEX4XVOESPROC                                = unsafe extern "C" fn(coords: *const GLfixed);
pub type PFNGLVERTEXARRAYATTRIBBINDINGPROC                    = unsafe extern "C" fn(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);
pub type PFNGLVERTEXARRAYATTRIBFORMATPROC                     = unsafe extern "C" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, r#type: GLenum, normalized: GLboolean, relativeoffset: GLuint);
pub type PFNGLVERTEXARRAYATTRIBIFORMATPROC                    = unsafe extern "C" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, r#type: GLenum, relativeoffset: GLuint);
pub type PFNGLVERTEXARRAYATTRIBLFORMATPROC                    = unsafe extern "C" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, r#type: GLenum, relativeoffset: GLuint);
pub type PFNGLVERTEXARRAYBINDINGDIVISORPROC                   = unsafe extern "C" fn(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);
pub type PFNGLVERTEXARRAYELEMENTBUFFERPROC                    = unsafe extern "C" fn(vaobj: GLuint, buffer: GLuint);
pub type PFNGLVERTEXARRAYVERTEXBUFFERPROC                     = unsafe extern "C" fn(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);
pub type PFNGLVERTEXARRAYVERTEXBUFFERSPROC                    = unsafe extern "C" fn(vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);
pub type PFNGLVERTEXATTRIB1DPROC                              = unsafe extern "C" fn(index: GLuint, x: GLdouble);
pub type PFNGLVERTEXATTRIB1DARBPROC                           = unsafe extern "C" fn(index: GLuint, x: GLdouble);
pub type PFNGLVERTEXATTRIB1DVPROC                             = unsafe extern "C" fn(index: GLuint, v: *const GLdouble);
pub type PFNGLVERTEXATTRIB1DVARBPROC                          = unsafe extern "C" fn(index: GLuint, v: *const GLdouble);
pub type PFNGLVERTEXATTRIB1FPROC                              = unsafe extern "C" fn(index: GLuint, x: GLfloat);
pub type PFNGLVERTEXATTRIB1FARBPROC                           = unsafe extern "C" fn(index: GLuint, x: GLfloat);
pub type PFNGLVERTEXATTRIB1FVPROC                             = unsafe extern "C" fn(index: GLuint, v: *const GLfloat);
pub type PFNGLVERTEXATTRIB1FVARBPROC                          = unsafe extern "C" fn(index: GLuint, v: *const GLfloat);
pub type PFNGLVERTEXATTRIB1SPROC                              = unsafe extern "C" fn(index: GLuint, x: GLshort);
pub type PFNGLVERTEXATTRIB1SARBPROC                           = unsafe extern "C" fn(index: GLuint, x: GLshort);
pub type PFNGLVERTEXATTRIB1SVPROC                             = unsafe extern "C" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIB1SVARBPROC                          = unsafe extern "C" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIB2DPROC                              = unsafe extern "C" fn(index: GLuint, x: GLdouble, y: GLdouble);
pub type PFNGLVERTEXATTRIB2DARBPROC                           = unsafe extern "C" fn(index: GLuint, x: GLdouble, y: GLdouble);
pub type PFNGLVERTEXATTRIB2DVPROC                             = unsafe extern "C" fn(index: GLuint, v: *const GLdouble);
pub type PFNGLVERTEXATTRIB2DVARBPROC                          = unsafe extern "C" fn(index: GLuint, v: *const GLdouble);
pub type PFNGLVERTEXATTRIB2FPROC                              = unsafe extern "C" fn(index: GLuint, x: GLfloat, y: GLfloat);
pub type PFNGLVERTEXATTRIB2FARBPROC                           = unsafe extern "C" fn(index: GLuint, x: GLfloat, y: GLfloat);
pub type PFNGLVERTEXATTRIB2FVPROC                             = unsafe extern "C" fn(index: GLuint, v: *const GLfloat);
pub type PFNGLVERTEXATTRIB2FVARBPROC                          = unsafe extern "C" fn(index: GLuint, v: *const GLfloat);
pub type PFNGLVERTEXATTRIB2SPROC                              = unsafe extern "C" fn(index: GLuint, x: GLshort, y: GLshort);
pub type PFNGLVERTEXATTRIB2SARBPROC                           = unsafe extern "C" fn(index: GLuint, x: GLshort, y: GLshort);
pub type PFNGLVERTEXATTRIB2SVPROC                             = unsafe extern "C" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIB2SVARBPROC                          = unsafe extern "C" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIB3DPROC                              = unsafe extern "C" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
pub type PFNGLVERTEXATTRIB3DARBPROC                           = unsafe extern "C" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
pub type PFNGLVERTEXATTRIB3DVPROC                             = unsafe extern "C" fn(index: GLuint, v: *const GLdouble);
pub type PFNGLVERTEXATTRIB3DVARBPROC                          = unsafe extern "C" fn(index: GLuint, v: *const GLdouble);
pub type PFNGLVERTEXATTRIB3FPROC                              = unsafe extern "C" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
pub type PFNGLVERTEXATTRIB3FARBPROC                           = unsafe extern "C" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
pub type PFNGLVERTEXATTRIB3FVPROC                             = unsafe extern "C" fn(index: GLuint, v: *const GLfloat);
pub type PFNGLVERTEXATTRIB3FVARBPROC                          = unsafe extern "C" fn(index: GLuint, v: *const GLfloat);
pub type PFNGLVERTEXATTRIB3SPROC                              = unsafe extern "C" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);
pub type PFNGLVERTEXATTRIB3SARBPROC                           = unsafe extern "C" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);
pub type PFNGLVERTEXATTRIB3SVPROC                             = unsafe extern "C" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIB3SVARBPROC                          = unsafe extern "C" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIB4NBVPROC                            = unsafe extern "C" fn(index: GLuint, v: *const GLbyte);
pub type PFNGLVERTEXATTRIB4NBVARBPROC                         = unsafe extern "C" fn(index: GLuint, v: *const GLbyte);
pub type PFNGLVERTEXATTRIB4NIVPROC                            = unsafe extern "C" fn(index: GLuint, v: *const GLint);
pub type PFNGLVERTEXATTRIB4NIVARBPROC                         = unsafe extern "C" fn(index: GLuint, v: *const GLint);
pub type PFNGLVERTEXATTRIB4NSVPROC                            = unsafe extern "C" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIB4NSVARBPROC                         = unsafe extern "C" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIB4NUBPROC                            = unsafe extern "C" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);
pub type PFNGLVERTEXATTRIB4NUBARBPROC                         = unsafe extern "C" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);
pub type PFNGLVERTEXATTRIB4NUBVPROC                           = unsafe extern "C" fn(index: GLuint, v: *const GLubyte);
pub type PFNGLVERTEXATTRIB4NUBVARBPROC                        = unsafe extern "C" fn(index: GLuint, v: *const GLubyte);
pub type PFNGLVERTEXATTRIB4NUIVPROC                           = unsafe extern "C" fn(index: GLuint, v: *const GLuint);
pub type PFNGLVERTEXATTRIB4NUIVARBPROC                        = unsafe extern "C" fn(index: GLuint, v: *const GLuint);
pub type PFNGLVERTEXATTRIB4NUSVPROC                           = unsafe extern "C" fn(index: GLuint, v: *const GLushort);
pub type PFNGLVERTEXATTRIB4NUSVARBPROC                        = unsafe extern "C" fn(index: GLuint, v: *const GLushort);
pub type PFNGLVERTEXATTRIB4BVPROC                             = unsafe extern "C" fn(index: GLuint, v: *const GLbyte);
pub type PFNGLVERTEXATTRIB4BVARBPROC                          = unsafe extern "C" fn(index: GLuint, v: *const GLbyte);
pub type PFNGLVERTEXATTRIB4DPROC                              = unsafe extern "C" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
pub type PFNGLVERTEXATTRIB4DARBPROC                           = unsafe extern "C" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
pub type PFNGLVERTEXATTRIB4DVPROC                             = unsafe extern "C" fn(index: GLuint, v: *const GLdouble);
pub type PFNGLVERTEXATTRIB4DVARBPROC                          = unsafe extern "C" fn(index: GLuint, v: *const GLdouble);
pub type PFNGLVERTEXATTRIB4FPROC                              = unsafe extern "C" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
pub type PFNGLVERTEXATTRIB4FARBPROC                           = unsafe extern "C" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
pub type PFNGLVERTEXATTRIB4FVPROC                             = unsafe extern "C" fn(index: GLuint, v: *const GLfloat);
pub type PFNGLVERTEXATTRIB4FVARBPROC                          = unsafe extern "C" fn(index: GLuint, v: *const GLfloat);
pub type PFNGLVERTEXATTRIB4IVPROC                             = unsafe extern "C" fn(index: GLuint, v: *const GLint);
pub type PFNGLVERTEXATTRIB4IVARBPROC                          = unsafe extern "C" fn(index: GLuint, v: *const GLint);
pub type PFNGLVERTEXATTRIB4SPROC                              = unsafe extern "C" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
pub type PFNGLVERTEXATTRIB4SARBPROC                           = unsafe extern "C" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
pub type PFNGLVERTEXATTRIB4SVPROC                             = unsafe extern "C" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIB4SVARBPROC                          = unsafe extern "C" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIB4UBVPROC                            = unsafe extern "C" fn(index: GLuint, v: *const GLubyte);
pub type PFNGLVERTEXATTRIB4UBVARBPROC                         = unsafe extern "C" fn(index: GLuint, v: *const GLubyte);
pub type PFNGLVERTEXATTRIB4UIVPROC                            = unsafe extern "C" fn(index: GLuint, v: *const GLuint);
pub type PFNGLVERTEXATTRIB4UIVARBPROC                         = unsafe extern "C" fn(index: GLuint, v: *const GLuint);
pub type PFNGLVERTEXATTRIB4USVPROC                            = unsafe extern "C" fn(index: GLuint, v: *const GLushort);
pub type PFNGLVERTEXATTRIB4USVARBPROC                         = unsafe extern "C" fn(index: GLuint, v: *const GLushort);
pub type PFNGLVERTEXATTRIBBINDINGPROC                         = unsafe extern "C" fn(attribindex: GLuint, bindingindex: GLuint);
pub type PFNGLVERTEXATTRIBDIVISORPROC                         = unsafe extern "C" fn(index: GLuint, divisor: GLuint);
pub type PFNGLVERTEXATTRIBDIVISORARBPROC                      = unsafe extern "C" fn(index: GLuint, divisor: GLuint);
pub type PFNGLVERTEXATTRIBFORMATPROC                          = unsafe extern "C" fn(attribindex: GLuint, size: GLint, r#type: GLenum, normalized: GLboolean, relativeoffset: GLuint);
pub type PFNGLVERTEXATTRIBI1IPROC                             = unsafe extern "C" fn(index: GLuint, x: GLint);
pub type PFNGLVERTEXATTRIBI1IVPROC                            = unsafe extern "C" fn(index: GLuint, v: *const GLint);
pub type PFNGLVERTEXATTRIBI1UIPROC                            = unsafe extern "C" fn(index: GLuint, x: GLuint);
pub type PFNGLVERTEXATTRIBI1UIVPROC                           = unsafe extern "C" fn(index: GLuint, v: *const GLuint);
pub type PFNGLVERTEXATTRIBI2IPROC                             = unsafe extern "C" fn(index: GLuint, x: GLint, y: GLint);
pub type PFNGLVERTEXATTRIBI2IVPROC                            = unsafe extern "C" fn(index: GLuint, v: *const GLint);
pub type PFNGLVERTEXATTRIBI2UIPROC                            = unsafe extern "C" fn(index: GLuint, x: GLuint, y: GLuint);
pub type PFNGLVERTEXATTRIBI2UIVPROC                           = unsafe extern "C" fn(index: GLuint, v: *const GLuint);
pub type PFNGLVERTEXATTRIBI3IPROC                             = unsafe extern "C" fn(index: GLuint, x: GLint, y: GLint, z: GLint);
pub type PFNGLVERTEXATTRIBI3IVPROC                            = unsafe extern "C" fn(index: GLuint, v: *const GLint);
pub type PFNGLVERTEXATTRIBI3UIPROC                            = unsafe extern "C" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint);
pub type PFNGLVERTEXATTRIBI3UIVPROC                           = unsafe extern "C" fn(index: GLuint, v: *const GLuint);
pub type PFNGLVERTEXATTRIBI4BVPROC                            = unsafe extern "C" fn(index: GLuint, v: *const GLbyte);
pub type PFNGLVERTEXATTRIBI4IPROC                             = unsafe extern "C" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
pub type PFNGLVERTEXATTRIBI4IVPROC                            = unsafe extern "C" fn(index: GLuint, v: *const GLint);
pub type PFNGLVERTEXATTRIBI4SVPROC                            = unsafe extern "C" fn(index: GLuint, v: *const GLshort);
pub type PFNGLVERTEXATTRIBI4UBVPROC                           = unsafe extern "C" fn(index: GLuint, v: *const GLubyte);
pub type PFNGLVERTEXATTRIBI4UIPROC                            = unsafe extern "C" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
pub type PFNGLVERTEXATTRIBI4UIVPROC                           = unsafe extern "C" fn(index: GLuint, v: *const GLuint);
pub type PFNGLVERTEXATTRIBI4USVPROC                           = unsafe extern "C" fn(index: GLuint, v: *const GLushort);
pub type PFNGLVERTEXATTRIBIFORMATPROC                         = unsafe extern "C" fn(attribindex: GLuint, size: GLint, r#type: GLenum, relativeoffset: GLuint);
pub type PFNGLVERTEXATTRIBIPOINTERPROC                        = unsafe extern "C" fn(index: GLuint, size: GLint, r#type: GLenum, stride: GLsizei, pointer: *const c_void);
pub type PFNGLVERTEXATTRIBL1DPROC                             = unsafe extern "C" fn(index: GLuint, x: GLdouble);
pub type PFNGLVERTEXATTRIBL1DVPROC                            = unsafe extern "C" fn(index: GLuint, v: *const GLdouble);
pub type PFNGLVERTEXATTRIBL2DPROC                             = unsafe extern "C" fn(index: GLuint, x: GLdouble, y: GLdouble);
pub type PFNGLVERTEXATTRIBL2DVPROC                            = unsafe extern "C" fn(index: GLuint, v: *const GLdouble);
pub type PFNGLVERTEXATTRIBL3DPROC                             = unsafe extern "C" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
pub type PFNGLVERTEXATTRIBL3DVPROC                            = unsafe extern "C" fn(index: GLuint, v: *const GLdouble);
pub type PFNGLVERTEXATTRIBL4DPROC                             = unsafe extern "C" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
pub type PFNGLVERTEXATTRIBL4DVPROC                            = unsafe extern "C" fn(index: GLuint, v: *const GLdouble);
pub type PFNGLVERTEXATTRIBLFORMATPROC                         = unsafe extern "C" fn(attribindex: GLuint, size: GLint, r#type: GLenum, relativeoffset: GLuint);
pub type PFNGLVERTEXATTRIBLPOINTERPROC                        = unsafe extern "C" fn(index: GLuint, size: GLint, r#type: GLenum, stride: GLsizei, pointer: *const c_void);
pub type PFNGLVERTEXATTRIBP1UIPROC                            = unsafe extern "C" fn(index: GLuint, r#type: GLenum, normalized: GLboolean, value: GLuint);
pub type PFNGLVERTEXATTRIBP1UIVPROC                           = unsafe extern "C" fn(index: GLuint, r#type: GLenum, normalized: GLboolean, value: *const GLuint);
pub type PFNGLVERTEXATTRIBP2UIPROC                            = unsafe extern "C" fn(index: GLuint, r#type: GLenum, normalized: GLboolean, value: GLuint);
pub type PFNGLVERTEXATTRIBP2UIVPROC                           = unsafe extern "C" fn(index: GLuint, r#type: GLenum, normalized: GLboolean, value: *const GLuint);
pub type PFNGLVERTEXATTRIBP3UIPROC                            = unsafe extern "C" fn(index: GLuint, r#type: GLenum, normalized: GLboolean, value: GLuint);
pub type PFNGLVERTEXATTRIBP3UIVPROC                           = unsafe extern "C" fn(index: GLuint, r#type: GLenum, normalized: GLboolean, value: *const GLuint);
pub type PFNGLVERTEXATTRIBP4UIPROC                            = unsafe extern "C" fn(index: GLuint, r#type: GLenum, normalized: GLboolean, value: GLuint);
pub type PFNGLVERTEXATTRIBP4UIVPROC                           = unsafe extern "C" fn(index: GLuint, r#type: GLenum, normalized: GLboolean, value: *const GLuint);
pub type PFNGLVERTEXATTRIBPOINTERPROC                         = unsafe extern "C" fn(index: GLuint, size: GLint, r#type: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void);
pub type PFNGLVERTEXATTRIBPOINTERARBPROC                      = unsafe extern "C" fn(index: GLuint, size: GLint, r#type: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void);
pub type PFNGLVERTEXBINDINGDIVISORPROC                        = unsafe extern "C" fn(bindingindex: GLuint, divisor: GLuint);
pub type PFNGLVIEWPORTPROC                                    = unsafe extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
pub type PFNGLVIEWPORTARRAYVPROC                              = unsafe extern "C" fn(first: GLuint, count: GLsizei, v: *const GLfloat);
pub type PFNGLVIEWPORTINDEXEDFPROC                            = unsafe extern "C" fn(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);
pub type PFNGLVIEWPORTINDEXEDFVPROC                           = unsafe extern "C" fn(index: GLuint, v: *const GLfloat);
pub type PFNGLWAITSYNCPROC                                    = unsafe extern "C" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64);

unsafe extern "C" {
    static glad_glAccumxOES: Option<PFNGLACCUMXOESPROC>;
    static glad_glActiveShaderProgram: Option<PFNGLACTIVESHADERPROGRAMPROC>;
    static glad_glActiveTexture: Option<PFNGLACTIVETEXTUREPROC>;
    static glad_glActiveTextureARB: Option<PFNGLACTIVETEXTUREARBPROC>;
    static glad_glAlphaFuncxOES: Option<PFNGLALPHAFUNCXOESPROC>;
    static glad_glAttachObjectARB: Option<PFNGLATTACHOBJECTARBPROC>;
    static glad_glAttachShader: Option<PFNGLATTACHSHADERPROC>;
    static glad_glBeginConditionalRender: Option<PFNGLBEGINCONDITIONALRENDERPROC>;
    static glad_glBeginQuery: Option<PFNGLBEGINQUERYPROC>;
    static glad_glBeginQueryARB: Option<PFNGLBEGINQUERYARBPROC>;
    static glad_glBeginQueryIndexed: Option<PFNGLBEGINQUERYINDEXEDPROC>;
    static glad_glBeginTransformFeedback: Option<PFNGLBEGINTRANSFORMFEEDBACKPROC>;
    static glad_glBindAttribLocation: Option<PFNGLBINDATTRIBLOCATIONPROC>;
    static glad_glBindAttribLocationARB: Option<PFNGLBINDATTRIBLOCATIONARBPROC>;
    static glad_glBindBuffer: Option<PFNGLBINDBUFFERPROC>;
    static glad_glBindBufferARB: Option<PFNGLBINDBUFFERARBPROC>;
    static glad_glBindBufferBase: Option<PFNGLBINDBUFFERBASEPROC>;
    static glad_glBindBufferRange: Option<PFNGLBINDBUFFERRANGEPROC>;
    static glad_glBindBuffersBase: Option<PFNGLBINDBUFFERSBASEPROC>;
    static glad_glBindBuffersRange: Option<PFNGLBINDBUFFERSRANGEPROC>;
    static glad_glBindFragDataLocation: Option<PFNGLBINDFRAGDATALOCATIONPROC>;
    static glad_glBindFragDataLocationIndexed: Option<PFNGLBINDFRAGDATALOCATIONINDEXEDPROC>;
    static glad_glBindFramebuffer: Option<PFNGLBINDFRAMEBUFFERPROC>;
    static glad_glBindFramebufferEXT: Option<PFNGLBINDFRAMEBUFFEREXTPROC>;
    static glad_glBindImageTexture: Option<PFNGLBINDIMAGETEXTUREPROC>;
    static glad_glBindImageTextures: Option<PFNGLBINDIMAGETEXTURESPROC>;
    static glad_glBindProgramARB: Option<PFNGLBINDPROGRAMARBPROC>;
    static glad_glBindProgramPipeline: Option<PFNGLBINDPROGRAMPIPELINEPROC>;
    static glad_glBindRenderbuffer: Option<PFNGLBINDRENDERBUFFERPROC>;
    static glad_glBindRenderbufferEXT: Option<PFNGLBINDRENDERBUFFEREXTPROC>;
    static glad_glBindSampler: Option<PFNGLBINDSAMPLERPROC>;
    static glad_glBindSamplers: Option<PFNGLBINDSAMPLERSPROC>;
    static glad_glBindTexture: Option<PFNGLBINDTEXTUREPROC>;
    static glad_glBindTextureUnit: Option<PFNGLBINDTEXTUREUNITPROC>;
    static glad_glBindTextures: Option<PFNGLBINDTEXTURESPROC>;
    static glad_glBindTransformFeedback: Option<PFNGLBINDTRANSFORMFEEDBACKPROC>;
    static glad_glBindVertexArray: Option<PFNGLBINDVERTEXARRAYPROC>;
    static glad_glBindVertexBuffer: Option<PFNGLBINDVERTEXBUFFERPROC>;
    static glad_glBindVertexBuffers: Option<PFNGLBINDVERTEXBUFFERSPROC>;
    static glad_glBitmapxOES: Option<PFNGLBITMAPXOESPROC>;
    static glad_glBlendColor: Option<PFNGLBLENDCOLORPROC>;
    static glad_glBlendColorxOES: Option<PFNGLBLENDCOLORXOESPROC>;
    static glad_glBlendEquation: Option<PFNGLBLENDEQUATIONPROC>;
    static glad_glBlendEquationSeparate: Option<PFNGLBLENDEQUATIONSEPARATEPROC>;
    static glad_glBlendEquationSeparatei: Option<PFNGLBLENDEQUATIONSEPARATEIPROC>;
    static glad_glBlendEquationSeparateiARB: Option<PFNGLBLENDEQUATIONSEPARATEIARBPROC>;
    static glad_glBlendEquationi: Option<PFNGLBLENDEQUATIONIPROC>;
    static glad_glBlendEquationiARB: Option<PFNGLBLENDEQUATIONIARBPROC>;
    static glad_glBlendFunc: Option<PFNGLBLENDFUNCPROC>;
    static glad_glBlendFuncSeparate: Option<PFNGLBLENDFUNCSEPARATEPROC>;
    static glad_glBlendFuncSeparatei: Option<PFNGLBLENDFUNCSEPARATEIPROC>;
    static glad_glBlendFuncSeparateiARB: Option<PFNGLBLENDFUNCSEPARATEIARBPROC>;
    static glad_glBlendFunci: Option<PFNGLBLENDFUNCIPROC>;
    static glad_glBlendFunciARB: Option<PFNGLBLENDFUNCIARBPROC>;
    static glad_glBlitFramebuffer: Option<PFNGLBLITFRAMEBUFFERPROC>;
    static glad_glBlitFramebufferEXT: Option<PFNGLBLITFRAMEBUFFEREXTPROC>;
    static glad_glBlitNamedFramebuffer: Option<PFNGLBLITNAMEDFRAMEBUFFERPROC>;
    static glad_glBufferData: Option<PFNGLBUFFERDATAPROC>;
    static glad_glBufferDataARB: Option<PFNGLBUFFERDATAARBPROC>;
    static glad_glBufferStorage: Option<PFNGLBUFFERSTORAGEPROC>;
    static glad_glBufferSubData: Option<PFNGLBUFFERSUBDATAPROC>;
    static glad_glBufferSubDataARB: Option<PFNGLBUFFERSUBDATAARBPROC>;
    static glad_glCheckFramebufferStatus: Option<PFNGLCHECKFRAMEBUFFERSTATUSPROC>;
    static glad_glCheckFramebufferStatusEXT: Option<PFNGLCHECKFRAMEBUFFERSTATUSEXTPROC>;
    static glad_glCheckNamedFramebufferStatus: Option<PFNGLCHECKNAMEDFRAMEBUFFERSTATUSPROC>;
    static glad_glClampColor: Option<PFNGLCLAMPCOLORPROC>;
    static glad_glClampColorARB: Option<PFNGLCLAMPCOLORARBPROC>;
    static glad_glClear: Option<PFNGLCLEARPROC>;
    static glad_glClearAccumxOES: Option<PFNGLCLEARACCUMXOESPROC>;
    static glad_glClearBufferData: Option<PFNGLCLEARBUFFERDATAPROC>;
    static glad_glClearBufferSubData: Option<PFNGLCLEARBUFFERSUBDATAPROC>;
    static glad_glClearBufferfi: Option<PFNGLCLEARBUFFERFIPROC>;
    static glad_glClearBufferfv: Option<PFNGLCLEARBUFFERFVPROC>;
    static glad_glClearBufferiv: Option<PFNGLCLEARBUFFERIVPROC>;
    static glad_glClearBufferuiv: Option<PFNGLCLEARBUFFERUIVPROC>;
    static glad_glClearColor: Option<PFNGLCLEARCOLORPROC>;
    static glad_glClearColorxOES: Option<PFNGLCLEARCOLORXOESPROC>;
    static glad_glClearDepth: Option<PFNGLCLEARDEPTHPROC>;
    static glad_glClearDepthf: Option<PFNGLCLEARDEPTHFPROC>;
    static glad_glClearDepthxOES: Option<PFNGLCLEARDEPTHXOESPROC>;
    static glad_glClearNamedBufferData: Option<PFNGLCLEARNAMEDBUFFERDATAPROC>;
    static glad_glClearNamedBufferSubData: Option<PFNGLCLEARNAMEDBUFFERSUBDATAPROC>;
    static glad_glClearNamedFramebufferfi: Option<PFNGLCLEARNAMEDFRAMEBUFFERFIPROC>;
    static glad_glClearNamedFramebufferfv: Option<PFNGLCLEARNAMEDFRAMEBUFFERFVPROC>;
    static glad_glClearNamedFramebufferiv: Option<PFNGLCLEARNAMEDFRAMEBUFFERIVPROC>;
    static glad_glClearNamedFramebufferuiv: Option<PFNGLCLEARNAMEDFRAMEBUFFERUIVPROC>;
    static glad_glClearStencil: Option<PFNGLCLEARSTENCILPROC>;
    static glad_glClearTexImage: Option<PFNGLCLEARTEXIMAGEPROC>;
    static glad_glClearTexSubImage: Option<PFNGLCLEARTEXSUBIMAGEPROC>;
    static glad_glClientActiveTextureARB: Option<PFNGLCLIENTACTIVETEXTUREARBPROC>;
    static glad_glClientWaitSync: Option<PFNGLCLIENTWAITSYNCPROC>;
    static glad_glClipPlanexOES: Option<PFNGLCLIPPLANEXOESPROC>;
    static glad_glColor3xOES: Option<PFNGLCOLOR3XOESPROC>;
    static glad_glColor3xvOES: Option<PFNGLCOLOR3XVOESPROC>;
    static glad_glColor4xOES: Option<PFNGLCOLOR4XOESPROC>;
    static glad_glColor4xvOES: Option<PFNGLCOLOR4XVOESPROC>;
    static glad_glColorMask: Option<PFNGLCOLORMASKPROC>;
    static glad_glColorMaski: Option<PFNGLCOLORMASKIPROC>;
    static glad_glCompileShader: Option<PFNGLCOMPILESHADERPROC>;
    static glad_glCompileShaderARB: Option<PFNGLCOMPILESHADERARBPROC>;
    static glad_glCompileShaderIncludeARB: Option<PFNGLCOMPILESHADERINCLUDEARBPROC>;
    static glad_glCompressedTexImage1D: Option<PFNGLCOMPRESSEDTEXIMAGE1DPROC>;
    static glad_glCompressedTexImage1DARB: Option<PFNGLCOMPRESSEDTEXIMAGE1DARBPROC>;
    static glad_glCompressedTexImage2D: Option<PFNGLCOMPRESSEDTEXIMAGE2DPROC>;
    static glad_glCompressedTexImage2DARB: Option<PFNGLCOMPRESSEDTEXIMAGE2DARBPROC>;
    static glad_glCompressedTexImage3D: Option<PFNGLCOMPRESSEDTEXIMAGE3DPROC>;
    static glad_glCompressedTexImage3DARB: Option<PFNGLCOMPRESSEDTEXIMAGE3DARBPROC>;
    static glad_glCompressedTexSubImage1D: Option<PFNGLCOMPRESSEDTEXSUBIMAGE1DPROC>;
    static glad_glCompressedTexSubImage1DARB: Option<PFNGLCOMPRESSEDTEXSUBIMAGE1DARBPROC>;
    static glad_glCompressedTexSubImage2D: Option<PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC>;
    static glad_glCompressedTexSubImage2DARB: Option<PFNGLCOMPRESSEDTEXSUBIMAGE2DARBPROC>;
    static glad_glCompressedTexSubImage3D: Option<PFNGLCOMPRESSEDTEXSUBIMAGE3DPROC>;
    static glad_glCompressedTexSubImage3DARB: Option<PFNGLCOMPRESSEDTEXSUBIMAGE3DARBPROC>;
    static glad_glCompressedTextureSubImage1D: Option<PFNGLCOMPRESSEDTEXTURESUBIMAGE1DPROC>;
    static glad_glCompressedTextureSubImage2D: Option<PFNGLCOMPRESSEDTEXTURESUBIMAGE2DPROC>;
    static glad_glCompressedTextureSubImage3D: Option<PFNGLCOMPRESSEDTEXTURESUBIMAGE3DPROC>;
    static glad_glConvolutionParameterxOES: Option<PFNGLCONVOLUTIONPARAMETERXOESPROC>;
    static glad_glConvolutionParameterxvOES: Option<PFNGLCONVOLUTIONPARAMETERXVOESPROC>;
    static glad_glCopyBufferSubData: Option<PFNGLCOPYBUFFERSUBDATAPROC>;
    static glad_glCopyImageSubData: Option<PFNGLCOPYIMAGESUBDATAPROC>;
    static glad_glCopyNamedBufferSubData: Option<PFNGLCOPYNAMEDBUFFERSUBDATAPROC>;
    static glad_glCopyTexImage1D: Option<PFNGLCOPYTEXIMAGE1DPROC>;
    static glad_glCopyTexImage2D: Option<PFNGLCOPYTEXIMAGE2DPROC>;
    static glad_glCopyTexSubImage1D: Option<PFNGLCOPYTEXSUBIMAGE1DPROC>;
    static glad_glCopyTexSubImage2D: Option<PFNGLCOPYTEXSUBIMAGE2DPROC>;
    static glad_glCopyTexSubImage3D: Option<PFNGLCOPYTEXSUBIMAGE3DPROC>;
    static glad_glCopyTextureSubImage1D: Option<PFNGLCOPYTEXTURESUBIMAGE1DPROC>;
    static glad_glCopyTextureSubImage2D: Option<PFNGLCOPYTEXTURESUBIMAGE2DPROC>;
    static glad_glCopyTextureSubImage3D: Option<PFNGLCOPYTEXTURESUBIMAGE3DPROC>;
    static glad_glCreateBuffers: Option<PFNGLCREATEBUFFERSPROC>;
    static glad_glCreateFramebuffers: Option<PFNGLCREATEFRAMEBUFFERSPROC>;
    static glad_glCreateProgram: Option<PFNGLCREATEPROGRAMPROC>;
    static glad_glCreateProgramObjectARB: Option<PFNGLCREATEPROGRAMOBJECTARBPROC>;
    static glad_glCreateProgramPipelines: Option<PFNGLCREATEPROGRAMPIPELINESPROC>;
    static glad_glCreateQueries: Option<PFNGLCREATEQUERIESPROC>;
    static glad_glCreateRenderbuffers: Option<PFNGLCREATERENDERBUFFERSPROC>;
    static glad_glCreateSamplers: Option<PFNGLCREATESAMPLERSPROC>;
    static glad_glCreateShader: Option<PFNGLCREATESHADERPROC>;
    static glad_glCreateShaderObjectARB: Option<PFNGLCREATESHADEROBJECTARBPROC>;
    static glad_glCreateShaderProgramv: Option<PFNGLCREATESHADERPROGRAMVPROC>;
    static glad_glCreateTextures: Option<PFNGLCREATETEXTURESPROC>;
    static glad_glCreateTransformFeedbacks: Option<PFNGLCREATETRANSFORMFEEDBACKSPROC>;
    static glad_glCreateVertexArrays: Option<PFNGLCREATEVERTEXARRAYSPROC>;
    static glad_glCullFace: Option<PFNGLCULLFACEPROC>;
    static glad_glDebugMessageCallback: Option<PFNGLDEBUGMESSAGECALLBACKPROC>;
    static glad_glDebugMessageCallbackARB: Option<PFNGLDEBUGMESSAGECALLBACKARBPROC>;
    static glad_glDebugMessageControl: Option<PFNGLDEBUGMESSAGECONTROLPROC>;
    static glad_glDebugMessageControlARB: Option<PFNGLDEBUGMESSAGECONTROLARBPROC>;
    static glad_glDebugMessageInsert: Option<PFNGLDEBUGMESSAGEINSERTPROC>;
    static glad_glDebugMessageInsertARB: Option<PFNGLDEBUGMESSAGEINSERTARBPROC>;
    static glad_glDeleteBuffers: Option<PFNGLDELETEBUFFERSPROC>;
    static glad_glDeleteBuffersARB: Option<PFNGLDELETEBUFFERSARBPROC>;
    static glad_glDeleteFramebuffers: Option<PFNGLDELETEFRAMEBUFFERSPROC>;
    static glad_glDeleteFramebuffersEXT: Option<PFNGLDELETEFRAMEBUFFERSEXTPROC>;
    static glad_glDeleteNamedStringARB: Option<PFNGLDELETENAMEDSTRINGARBPROC>;
    static glad_glDeleteObjectARB: Option<PFNGLDELETEOBJECTARBPROC>;
    static glad_glDeleteProgram: Option<PFNGLDELETEPROGRAMPROC>;
    static glad_glDeleteProgramPipelines: Option<PFNGLDELETEPROGRAMPIPELINESPROC>;
    static glad_glDeleteProgramsARB: Option<PFNGLDELETEPROGRAMSARBPROC>;
    static glad_glDeleteQueries: Option<PFNGLDELETEQUERIESPROC>;
    static glad_glDeleteQueriesARB: Option<PFNGLDELETEQUERIESARBPROC>;
    static glad_glDeleteRenderbuffers: Option<PFNGLDELETERENDERBUFFERSPROC>;
    static glad_glDeleteRenderbuffersEXT: Option<PFNGLDELETERENDERBUFFERSEXTPROC>;
    static glad_glDeleteSamplers: Option<PFNGLDELETESAMPLERSPROC>;
    static glad_glDeleteShader: Option<PFNGLDELETESHADERPROC>;
    static glad_glDeleteSync: Option<PFNGLDELETESYNCPROC>;
    static glad_glDeleteTextures: Option<PFNGLDELETETEXTURESPROC>;
    static glad_glDeleteTransformFeedbacks: Option<PFNGLDELETETRANSFORMFEEDBACKSPROC>;
    static glad_glDeleteVertexArrays: Option<PFNGLDELETEVERTEXARRAYSPROC>;
    static glad_glDepthFunc: Option<PFNGLDEPTHFUNCPROC>;
    static glad_glDepthMask: Option<PFNGLDEPTHMASKPROC>;
    static glad_glDepthRange: Option<PFNGLDEPTHRANGEPROC>;
    static glad_glDepthRangeArrayv: Option<PFNGLDEPTHRANGEARRAYVPROC>;
    static glad_glDepthRangeIndexed: Option<PFNGLDEPTHRANGEINDEXEDPROC>;
    static glad_glDepthRangef: Option<PFNGLDEPTHRANGEFPROC>;
    static glad_glDepthRangexOES: Option<PFNGLDEPTHRANGEXOESPROC>;
    static glad_glDetachObjectARB: Option<PFNGLDETACHOBJECTARBPROC>;
    static glad_glDetachShader: Option<PFNGLDETACHSHADERPROC>;
    static glad_glDisable: Option<PFNGLDISABLEPROC>;
    static glad_glDisableVertexArrayAttrib: Option<PFNGLDISABLEVERTEXARRAYATTRIBPROC>;
    static glad_glDisableVertexAttribArray: Option<PFNGLDISABLEVERTEXATTRIBARRAYPROC>;
    static glad_glDisableVertexAttribArrayARB: Option<PFNGLDISABLEVERTEXATTRIBARRAYARBPROC>;
    static glad_glDisablei: Option<PFNGLDISABLEIPROC>;
    static glad_glDispatchCompute: Option<PFNGLDISPATCHCOMPUTEPROC>;
    static glad_glDispatchComputeGroupSizeARB: Option<PFNGLDISPATCHCOMPUTEGROUPSIZEARBPROC>;
    static glad_glDispatchComputeIndirect: Option<PFNGLDISPATCHCOMPUTEINDIRECTPROC>;
    static glad_glDrawArrays: Option<PFNGLDRAWARRAYSPROC>;
    static glad_glDrawArraysIndirect: Option<PFNGLDRAWARRAYSINDIRECTPROC>;
    static glad_glDrawArraysInstanced: Option<PFNGLDRAWARRAYSINSTANCEDPROC>;
    static glad_glDrawArraysInstancedARB: Option<PFNGLDRAWARRAYSINSTANCEDARBPROC>;
    static glad_glDrawArraysInstancedBaseInstance: Option<PFNGLDRAWARRAYSINSTANCEDBASEINSTANCEPROC>;
    static glad_glDrawArraysInstancedEXT: Option<PFNGLDRAWARRAYSINSTANCEDEXTPROC>;
    static glad_glDrawBuffer: Option<PFNGLDRAWBUFFERPROC>;
    static glad_glDrawBuffers: Option<PFNGLDRAWBUFFERSPROC>;
    static glad_glDrawBuffersARB: Option<PFNGLDRAWBUFFERSARBPROC>;
    static glad_glDrawElements: Option<PFNGLDRAWELEMENTSPROC>;
    static glad_glDrawElementsBaseVertex: Option<PFNGLDRAWELEMENTSBASEVERTEXPROC>;
    static glad_glDrawElementsIndirect: Option<PFNGLDRAWELEMENTSINDIRECTPROC>;
    static glad_glDrawElementsInstanced: Option<PFNGLDRAWELEMENTSINSTANCEDPROC>;
    static glad_glDrawElementsInstancedARB: Option<PFNGLDRAWELEMENTSINSTANCEDARBPROC>;
    static glad_glDrawElementsInstancedBaseInstance: Option<PFNGLDRAWELEMENTSINSTANCEDBASEINSTANCEPROC>;
    static glad_glDrawElementsInstancedBaseVertex: Option<PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC>;
    static glad_glDrawElementsInstancedBaseVertexBaseInstance: Option<PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXBASEINSTANCEPROC>;
    static glad_glDrawElementsInstancedEXT: Option<PFNGLDRAWELEMENTSINSTANCEDEXTPROC>;
    static glad_glDrawRangeElements: Option<PFNGLDRAWRANGEELEMENTSPROC>;
    static glad_glDrawRangeElementsBaseVertex: Option<PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC>;
    static glad_glDrawTransformFeedback: Option<PFNGLDRAWTRANSFORMFEEDBACKPROC>;
    static glad_glDrawTransformFeedbackInstanced: Option<PFNGLDRAWTRANSFORMFEEDBACKINSTANCEDPROC>;
    static glad_glDrawTransformFeedbackStream: Option<PFNGLDRAWTRANSFORMFEEDBACKSTREAMPROC>;
    static glad_glDrawTransformFeedbackStreamInstanced: Option<PFNGLDRAWTRANSFORMFEEDBACKSTREAMINSTANCEDPROC>;
    static glad_glEnable: Option<PFNGLENABLEPROC>;
    static glad_glEnableVertexArrayAttrib: Option<PFNGLENABLEVERTEXARRAYATTRIBPROC>;
    static glad_glEnableVertexAttribArray: Option<PFNGLENABLEVERTEXATTRIBARRAYPROC>;
    static glad_glEnableVertexAttribArrayARB: Option<PFNGLENABLEVERTEXATTRIBARRAYARBPROC>;
    static glad_glEnablei: Option<PFNGLENABLEIPROC>;
    static glad_glEndConditionalRender: Option<PFNGLENDCONDITIONALRENDERPROC>;
    static glad_glEndQuery: Option<PFNGLENDQUERYPROC>;
    static glad_glEndQueryARB: Option<PFNGLENDQUERYARBPROC>;
    static glad_glEndQueryIndexed: Option<PFNGLENDQUERYINDEXEDPROC>;
    static glad_glEndTransformFeedback: Option<PFNGLENDTRANSFORMFEEDBACKPROC>;
    static glad_glEvalCoord1xOES: Option<PFNGLEVALCOORD1XOESPROC>;
    static glad_glEvalCoord1xvOES: Option<PFNGLEVALCOORD1XVOESPROC>;
    static glad_glEvalCoord2xOES: Option<PFNGLEVALCOORD2XOESPROC>;
    static glad_glEvalCoord2xvOES: Option<PFNGLEVALCOORD2XVOESPROC>;
    static glad_glEvaluateDepthValuesARB: Option<PFNGLEVALUATEDEPTHVALUESARBPROC>;
    static glad_glFeedbackBufferxOES: Option<PFNGLFEEDBACKBUFFERXOESPROC>;
    static glad_glFenceSync: Option<PFNGLFENCESYNCPROC>;
    static glad_glFinish: Option<PFNGLFINISHPROC>;
    static glad_glFlush: Option<PFNGLFLUSHPROC>;
    static glad_glFlushMappedBufferRange: Option<PFNGLFLUSHMAPPEDBUFFERRANGEPROC>;
    static glad_glFlushMappedNamedBufferRange: Option<PFNGLFLUSHMAPPEDNAMEDBUFFERRANGEPROC>;
    static glad_glFogCoordPointerEXT: Option<PFNGLFOGCOORDPOINTEREXTPROC>;
    static glad_glFogCoorddEXT: Option<PFNGLFOGCOORDDEXTPROC>;
    static glad_glFogCoorddvEXT: Option<PFNGLFOGCOORDDVEXTPROC>;
    static glad_glFogCoordfEXT: Option<PFNGLFOGCOORDFEXTPROC>;
    static glad_glFogCoordfvEXT: Option<PFNGLFOGCOORDFVEXTPROC>;
    static glad_glFogxOES: Option<PFNGLFOGXOESPROC>;
    static glad_glFogxvOES: Option<PFNGLFOGXVOESPROC>;
    static glad_glFramebufferParameteri: Option<PFNGLFRAMEBUFFERPARAMETERIPROC>;
    static glad_glFramebufferRenderbuffer: Option<PFNGLFRAMEBUFFERRENDERBUFFERPROC>;
    static glad_glFramebufferRenderbufferEXT: Option<PFNGLFRAMEBUFFERRENDERBUFFEREXTPROC>;
    static glad_glFramebufferSampleLocationsfvARB: Option<PFNGLFRAMEBUFFERSAMPLELOCATIONSFVARBPROC>;
    static glad_glFramebufferTexture: Option<PFNGLFRAMEBUFFERTEXTUREPROC>;
    static glad_glFramebufferTexture1D: Option<PFNGLFRAMEBUFFERTEXTURE1DPROC>;
    static glad_glFramebufferTexture1DEXT: Option<PFNGLFRAMEBUFFERTEXTURE1DEXTPROC>;
    static glad_glFramebufferTexture2D: Option<PFNGLFRAMEBUFFERTEXTURE2DPROC>;
    static glad_glFramebufferTexture2DEXT: Option<PFNGLFRAMEBUFFERTEXTURE2DEXTPROC>;
    static glad_glFramebufferTexture3D: Option<PFNGLFRAMEBUFFERTEXTURE3DPROC>;
    static glad_glFramebufferTexture3DEXT: Option<PFNGLFRAMEBUFFERTEXTURE3DEXTPROC>;
    static glad_glFramebufferTextureARB: Option<PFNGLFRAMEBUFFERTEXTUREARBPROC>;
    static glad_glFramebufferTextureFaceARB: Option<PFNGLFRAMEBUFFERTEXTUREFACEARBPROC>;
    static glad_glFramebufferTextureLayer: Option<PFNGLFRAMEBUFFERTEXTURELAYERPROC>;
    static glad_glFramebufferTextureLayerARB: Option<PFNGLFRAMEBUFFERTEXTURELAYERARBPROC>;
    static glad_glFrontFace: Option<PFNGLFRONTFACEPROC>;
    static glad_glFrustumxOES: Option<PFNGLFRUSTUMXOESPROC>;
    static glad_glGenBuffers: Option<PFNGLGENBUFFERSPROC>;
    static glad_glGenBuffersARB: Option<PFNGLGENBUFFERSARBPROC>;
    static glad_glGenFramebuffers: Option<PFNGLGENFRAMEBUFFERSPROC>;
    static glad_glGenFramebuffersEXT: Option<PFNGLGENFRAMEBUFFERSEXTPROC>;
    static glad_glGenProgramPipelines: Option<PFNGLGENPROGRAMPIPELINESPROC>;
    static glad_glGenProgramsARB: Option<PFNGLGENPROGRAMSARBPROC>;
    static glad_glGenQueries: Option<PFNGLGENQUERIESPROC>;
    static glad_glGenQueriesARB: Option<PFNGLGENQUERIESARBPROC>;
    static glad_glGenRenderbuffers: Option<PFNGLGENRENDERBUFFERSPROC>;
    static glad_glGenRenderbuffersEXT: Option<PFNGLGENRENDERBUFFERSEXTPROC>;
    static glad_glGenSamplers: Option<PFNGLGENSAMPLERSPROC>;
    static glad_glGenTextures: Option<PFNGLGENTEXTURESPROC>;
    static glad_glGenTransformFeedbacks: Option<PFNGLGENTRANSFORMFEEDBACKSPROC>;
    static glad_glGenVertexArrays: Option<PFNGLGENVERTEXARRAYSPROC>;
    static glad_glGenerateMipmap: Option<PFNGLGENERATEMIPMAPPROC>;
    static glad_glGenerateMipmapEXT: Option<PFNGLGENERATEMIPMAPEXTPROC>;
    static glad_glGenerateTextureMipmap: Option<PFNGLGENERATETEXTUREMIPMAPPROC>;
    static glad_glGetActiveAtomicCounterBufferiv: Option<PFNGLGETACTIVEATOMICCOUNTERBUFFERIVPROC>;
    static glad_glGetActiveAttrib: Option<PFNGLGETACTIVEATTRIBPROC>;
    static glad_glGetActiveAttribARB: Option<PFNGLGETACTIVEATTRIBARBPROC>;
    static glad_glGetActiveSubroutineName: Option<PFNGLGETACTIVESUBROUTINENAMEPROC>;
    static glad_glGetActiveSubroutineUniformName: Option<PFNGLGETACTIVESUBROUTINEUNIFORMNAMEPROC>;
    static glad_glGetActiveSubroutineUniformiv: Option<PFNGLGETACTIVESUBROUTINEUNIFORMIVPROC>;
    static glad_glGetActiveUniform: Option<PFNGLGETACTIVEUNIFORMPROC>;
    static glad_glGetActiveUniformARB: Option<PFNGLGETACTIVEUNIFORMARBPROC>;
    static glad_glGetActiveUniformBlockName: Option<PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC>;
    static glad_glGetActiveUniformBlockiv: Option<PFNGLGETACTIVEUNIFORMBLOCKIVPROC>;
    static glad_glGetActiveUniformName: Option<PFNGLGETACTIVEUNIFORMNAMEPROC>;
    static glad_glGetActiveUniformsiv: Option<PFNGLGETACTIVEUNIFORMSIVPROC>;
    static glad_glGetAttachedObjectsARB: Option<PFNGLGETATTACHEDOBJECTSARBPROC>;
    static glad_glGetAttachedShaders: Option<PFNGLGETATTACHEDSHADERSPROC>;
    static glad_glGetAttribLocation: Option<PFNGLGETATTRIBLOCATIONPROC>;
    static glad_glGetAttribLocationARB: Option<PFNGLGETATTRIBLOCATIONARBPROC>;
    static glad_glGetBooleani_v: Option<PFNGLGETBOOLEANI_VPROC>;
    static glad_glGetBooleanv: Option<PFNGLGETBOOLEANVPROC>;
    static glad_glGetBufferParameteri64v: Option<PFNGLGETBUFFERPARAMETERI64VPROC>;
    static glad_glGetBufferParameteriv: Option<PFNGLGETBUFFERPARAMETERIVPROC>;
    static glad_glGetBufferParameterivARB: Option<PFNGLGETBUFFERPARAMETERIVARBPROC>;
    static glad_glGetBufferPointerv: Option<PFNGLGETBUFFERPOINTERVPROC>;
    static glad_glGetBufferPointervARB: Option<PFNGLGETBUFFERPOINTERVARBPROC>;
    static glad_glGetBufferSubData: Option<PFNGLGETBUFFERSUBDATAPROC>;
    static glad_glGetBufferSubDataARB: Option<PFNGLGETBUFFERSUBDATAARBPROC>;
    static glad_glGetClipPlanexOES: Option<PFNGLGETCLIPPLANEXOESPROC>;
    static glad_glGetCompressedTexImage: Option<PFNGLGETCOMPRESSEDTEXIMAGEPROC>;
    static glad_glGetCompressedTexImageARB: Option<PFNGLGETCOMPRESSEDTEXIMAGEARBPROC>;
    static glad_glGetCompressedTextureImage: Option<PFNGLGETCOMPRESSEDTEXTUREIMAGEPROC>;
    static glad_glGetCompressedTextureSubImage: Option<PFNGLGETCOMPRESSEDTEXTURESUBIMAGEPROC>;
    static glad_glGetConvolutionParameterxvOES: Option<PFNGLGETCONVOLUTIONPARAMETERXVOESPROC>;
    static glad_glGetDebugMessageLog: Option<PFNGLGETDEBUGMESSAGELOGPROC>;
    static glad_glGetDebugMessageLogARB: Option<PFNGLGETDEBUGMESSAGELOGARBPROC>;
    static glad_glGetDoublei_v: Option<PFNGLGETDOUBLEI_VPROC>;
    static glad_glGetDoublev: Option<PFNGLGETDOUBLEVPROC>;
    static glad_glGetError: Option<PFNGLGETERRORPROC>;
    static glad_glGetFixedvOES: Option<PFNGLGETFIXEDVOESPROC>;
    static glad_glGetFloati_v: Option<PFNGLGETFLOATI_VPROC>;
    static glad_glGetFloatv: Option<PFNGLGETFLOATVPROC>;
    static glad_glGetFragDataIndex: Option<PFNGLGETFRAGDATAINDEXPROC>;
    static glad_glGetFragDataLocation: Option<PFNGLGETFRAGDATALOCATIONPROC>;
    static glad_glGetFramebufferAttachmentParameteriv: Option<PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC>;
    static glad_glGetFramebufferAttachmentParameterivEXT: Option<PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVEXTPROC>;
    static glad_glGetFramebufferParameteriv: Option<PFNGLGETFRAMEBUFFERPARAMETERIVPROC>;
    static glad_glGetHandleARB: Option<PFNGLGETHANDLEARBPROC>;
    static glad_glGetHistogramParameterxvOES: Option<PFNGLGETHISTOGRAMPARAMETERXVOESPROC>;
    static glad_glGetInfoLogARB: Option<PFNGLGETINFOLOGARBPROC>;
    static glad_glGetInteger64i_v: Option<PFNGLGETINTEGER64I_VPROC>;
    static glad_glGetInteger64v: Option<PFNGLGETINTEGER64VPROC>;
    static glad_glGetIntegeri_v: Option<PFNGLGETINTEGERI_VPROC>;
    static glad_glGetIntegerv: Option<PFNGLGETINTEGERVPROC>;
    static glad_glGetInternalformati64v: Option<PFNGLGETINTERNALFORMATI64VPROC>;
    static glad_glGetInternalformativ: Option<PFNGLGETINTERNALFORMATIVPROC>;
    static glad_glGetLightxOES: Option<PFNGLGETLIGHTXOESPROC>;
    static glad_glGetMapxvOES: Option<PFNGLGETMAPXVOESPROC>;
    static glad_glGetMaterialxOES: Option<PFNGLGETMATERIALXOESPROC>;
    static glad_glGetMultisamplefv: Option<PFNGLGETMULTISAMPLEFVPROC>;
    static glad_glGetNamedBufferParameteri64v: Option<PFNGLGETNAMEDBUFFERPARAMETERI64VPROC>;
    static glad_glGetNamedBufferParameteriv: Option<PFNGLGETNAMEDBUFFERPARAMETERIVPROC>;
    static glad_glGetNamedBufferPointerv: Option<PFNGLGETNAMEDBUFFERPOINTERVPROC>;
    static glad_glGetNamedBufferSubData: Option<PFNGLGETNAMEDBUFFERSUBDATAPROC>;
    static glad_glGetNamedFramebufferAttachmentParameteriv: Option<PFNGLGETNAMEDFRAMEBUFFERATTACHMENTPARAMETERIVPROC>;
    static glad_glGetNamedFramebufferParameteriv: Option<PFNGLGETNAMEDFRAMEBUFFERPARAMETERIVPROC>;
    static glad_glGetNamedRenderbufferParameteriv: Option<PFNGLGETNAMEDRENDERBUFFERPARAMETERIVPROC>;
    static glad_glGetNamedStringARB: Option<PFNGLGETNAMEDSTRINGARBPROC>;
    static glad_glGetNamedStringivARB: Option<PFNGLGETNAMEDSTRINGIVARBPROC>;
    static glad_glGetObjectLabel: Option<PFNGLGETOBJECTLABELPROC>;
    static glad_glGetObjectParameterfvARB: Option<PFNGLGETOBJECTPARAMETERFVARBPROC>;
    static glad_glGetObjectParameterivARB: Option<PFNGLGETOBJECTPARAMETERIVARBPROC>;
    static glad_glGetObjectPtrLabel: Option<PFNGLGETOBJECTPTRLABELPROC>;
    static glad_glGetPixelMapxv: Option<PFNGLGETPIXELMAPXVPROC>;
    static glad_glGetPointerv: Option<PFNGLGETPOINTERVPROC>;
    static glad_glGetProgramBinary: Option<PFNGLGETPROGRAMBINARYPROC>;
    static glad_glGetProgramEnvParameterdvARB: Option<PFNGLGETPROGRAMENVPARAMETERDVARBPROC>;
    static glad_glGetProgramEnvParameterfvARB: Option<PFNGLGETPROGRAMENVPARAMETERFVARBPROC>;
    static glad_glGetProgramInfoLog: Option<PFNGLGETPROGRAMINFOLOGPROC>;
    static glad_glGetProgramInterfaceiv: Option<PFNGLGETPROGRAMINTERFACEIVPROC>;
    static glad_glGetProgramLocalParameterdvARB: Option<PFNGLGETPROGRAMLOCALPARAMETERDVARBPROC>;
    static glad_glGetProgramLocalParameterfvARB: Option<PFNGLGETPROGRAMLOCALPARAMETERFVARBPROC>;
    static glad_glGetProgramPipelineInfoLog: Option<PFNGLGETPROGRAMPIPELINEINFOLOGPROC>;
    static glad_glGetProgramPipelineiv: Option<PFNGLGETPROGRAMPIPELINEIVPROC>;
    static glad_glGetProgramResourceIndex: Option<PFNGLGETPROGRAMRESOURCEINDEXPROC>;
    static glad_glGetProgramResourceLocation: Option<PFNGLGETPROGRAMRESOURCELOCATIONPROC>;
    static glad_glGetProgramResourceLocationIndex: Option<PFNGLGETPROGRAMRESOURCELOCATIONINDEXPROC>;
    static glad_glGetProgramResourceName: Option<PFNGLGETPROGRAMRESOURCENAMEPROC>;
    static glad_glGetProgramResourceiv: Option<PFNGLGETPROGRAMRESOURCEIVPROC>;
    static glad_glGetProgramStageiv: Option<PFNGLGETPROGRAMSTAGEIVPROC>;
    static glad_glGetProgramStringARB: Option<PFNGLGETPROGRAMSTRINGARBPROC>;
    static glad_glGetProgramiv: Option<PFNGLGETPROGRAMIVPROC>;
    static glad_glGetProgramivARB: Option<PFNGLGETPROGRAMIVARBPROC>;
    static glad_glGetQueryBufferObjecti64v: Option<PFNGLGETQUERYBUFFEROBJECTI64VPROC>;
    static glad_glGetQueryBufferObjectiv: Option<PFNGLGETQUERYBUFFEROBJECTIVPROC>;
    static glad_glGetQueryBufferObjectui64v: Option<PFNGLGETQUERYBUFFEROBJECTUI64VPROC>;
    static glad_glGetQueryBufferObjectuiv: Option<PFNGLGETQUERYBUFFEROBJECTUIVPROC>;
    static glad_glGetQueryIndexediv: Option<PFNGLGETQUERYINDEXEDIVPROC>;
    static glad_glGetQueryObjecti64v: Option<PFNGLGETQUERYOBJECTI64VPROC>;
    static glad_glGetQueryObjectiv: Option<PFNGLGETQUERYOBJECTIVPROC>;
    static glad_glGetQueryObjectivARB: Option<PFNGLGETQUERYOBJECTIVARBPROC>;
    static glad_glGetQueryObjectui64v: Option<PFNGLGETQUERYOBJECTUI64VPROC>;
    static glad_glGetQueryObjectuiv: Option<PFNGLGETQUERYOBJECTUIVPROC>;
    static glad_glGetQueryObjectuivARB: Option<PFNGLGETQUERYOBJECTUIVARBPROC>;
    static glad_glGetQueryiv: Option<PFNGLGETQUERYIVPROC>;
    static glad_glGetQueryivARB: Option<PFNGLGETQUERYIVARBPROC>;
    static glad_glGetRenderbufferParameteriv: Option<PFNGLGETRENDERBUFFERPARAMETERIVPROC>;
    static glad_glGetRenderbufferParameterivEXT: Option<PFNGLGETRENDERBUFFERPARAMETERIVEXTPROC>;
    static glad_glGetSamplerParameterIiv: Option<PFNGLGETSAMPLERPARAMETERIIVPROC>;
    static glad_glGetSamplerParameterIuiv: Option<PFNGLGETSAMPLERPARAMETERIUIVPROC>;
    static glad_glGetSamplerParameterfv: Option<PFNGLGETSAMPLERPARAMETERFVPROC>;
    static glad_glGetSamplerParameteriv: Option<PFNGLGETSAMPLERPARAMETERIVPROC>;
    static glad_glGetShaderInfoLog: Option<PFNGLGETSHADERINFOLOGPROC>;
    static glad_glGetShaderPrecisionFormat: Option<PFNGLGETSHADERPRECISIONFORMATPROC>;
    static glad_glGetShaderSource: Option<PFNGLGETSHADERSOURCEPROC>;
    static glad_glGetShaderSourceARB: Option<PFNGLGETSHADERSOURCEARBPROC>;
    static glad_glGetShaderiv: Option<PFNGLGETSHADERIVPROC>;
    static glad_glGetString: Option<PFNGLGETSTRINGPROC>;
    static glad_glGetStringi: Option<PFNGLGETSTRINGIPROC>;
    static glad_glGetSubroutineIndex: Option<PFNGLGETSUBROUTINEINDEXPROC>;
    static glad_glGetSubroutineUniformLocation: Option<PFNGLGETSUBROUTINEUNIFORMLOCATIONPROC>;
    static glad_glGetSynciv: Option<PFNGLGETSYNCIVPROC>;
    static glad_glGetTexEnvxvOES: Option<PFNGLGETTEXENVXVOESPROC>;
    static glad_glGetTexGenxvOES: Option<PFNGLGETTEXGENXVOESPROC>;
    static glad_glGetTexImage: Option<PFNGLGETTEXIMAGEPROC>;
    static glad_glGetTexLevelParameterfv: Option<PFNGLGETTEXLEVELPARAMETERFVPROC>;
    static glad_glGetTexLevelParameteriv: Option<PFNGLGETTEXLEVELPARAMETERIVPROC>;
    static glad_glGetTexLevelParameterxvOES: Option<PFNGLGETTEXLEVELPARAMETERXVOESPROC>;
    static glad_glGetTexParameterIiv: Option<PFNGLGETTEXPARAMETERIIVPROC>;
    static glad_glGetTexParameterIuiv: Option<PFNGLGETTEXPARAMETERIUIVPROC>;
    static glad_glGetTexParameterfv: Option<PFNGLGETTEXPARAMETERFVPROC>;
    static glad_glGetTexParameteriv: Option<PFNGLGETTEXPARAMETERIVPROC>;
    static glad_glGetTexParameterxvOES: Option<PFNGLGETTEXPARAMETERXVOESPROC>;
    static glad_glGetTextureImage: Option<PFNGLGETTEXTUREIMAGEPROC>;
    static glad_glGetTextureLevelParameterfv: Option<PFNGLGETTEXTURELEVELPARAMETERFVPROC>;
    static glad_glGetTextureLevelParameteriv: Option<PFNGLGETTEXTURELEVELPARAMETERIVPROC>;
    static glad_glGetTextureParameterIiv: Option<PFNGLGETTEXTUREPARAMETERIIVPROC>;
    static glad_glGetTextureParameterIuiv: Option<PFNGLGETTEXTUREPARAMETERIUIVPROC>;
    static glad_glGetTextureParameterfv: Option<PFNGLGETTEXTUREPARAMETERFVPROC>;
    static glad_glGetTextureParameteriv: Option<PFNGLGETTEXTUREPARAMETERIVPROC>;
    static glad_glGetTextureSubImage: Option<PFNGLGETTEXTURESUBIMAGEPROC>;
    static glad_glGetTransformFeedbackVarying: Option<PFNGLGETTRANSFORMFEEDBACKVARYINGPROC>;
    static glad_glGetTransformFeedbacki64_v: Option<PFNGLGETTRANSFORMFEEDBACKI64_VPROC>;
    static glad_glGetTransformFeedbacki_v: Option<PFNGLGETTRANSFORMFEEDBACKI_VPROC>;
    static glad_glGetTransformFeedbackiv: Option<PFNGLGETTRANSFORMFEEDBACKIVPROC>;
    static glad_glGetUniformBlockIndex: Option<PFNGLGETUNIFORMBLOCKINDEXPROC>;
    static glad_glGetUniformIndices: Option<PFNGLGETUNIFORMINDICESPROC>;
    static glad_glGetUniformLocation: Option<PFNGLGETUNIFORMLOCATIONPROC>;
    static glad_glGetUniformLocationARB: Option<PFNGLGETUNIFORMLOCATIONARBPROC>;
    static glad_glGetUniformSubroutineuiv: Option<PFNGLGETUNIFORMSUBROUTINEUIVPROC>;
    static glad_glGetUniformdv: Option<PFNGLGETUNIFORMDVPROC>;
    static glad_glGetUniformfv: Option<PFNGLGETUNIFORMFVPROC>;
    static glad_glGetUniformfvARB: Option<PFNGLGETUNIFORMFVARBPROC>;
    static glad_glGetUniformi64vARB: Option<PFNGLGETUNIFORMI64VARBPROC>;
    static glad_glGetUniformiv: Option<PFNGLGETUNIFORMIVPROC>;
    static glad_glGetUniformivARB: Option<PFNGLGETUNIFORMIVARBPROC>;
    static glad_glGetUniformui64vARB: Option<PFNGLGETUNIFORMUI64VARBPROC>;
    static glad_glGetUniformuiv: Option<PFNGLGETUNIFORMUIVPROC>;
    static glad_glGetVertexArrayIndexed64iv: Option<PFNGLGETVERTEXARRAYINDEXED64IVPROC>;
    static glad_glGetVertexArrayIndexediv: Option<PFNGLGETVERTEXARRAYINDEXEDIVPROC>;
    static glad_glGetVertexArrayiv: Option<PFNGLGETVERTEXARRAYIVPROC>;
    static glad_glGetVertexAttribIiv: Option<PFNGLGETVERTEXATTRIBIIVPROC>;
    static glad_glGetVertexAttribIuiv: Option<PFNGLGETVERTEXATTRIBIUIVPROC>;
    static glad_glGetVertexAttribLdv: Option<PFNGLGETVERTEXATTRIBLDVPROC>;
    static glad_glGetVertexAttribPointerv: Option<PFNGLGETVERTEXATTRIBPOINTERVPROC>;
    static glad_glGetVertexAttribPointervARB: Option<PFNGLGETVERTEXATTRIBPOINTERVARBPROC>;
    static glad_glGetVertexAttribdv: Option<PFNGLGETVERTEXATTRIBDVPROC>;
    static glad_glGetVertexAttribdvARB: Option<PFNGLGETVERTEXATTRIBDVARBPROC>;
    static glad_glGetVertexAttribfv: Option<PFNGLGETVERTEXATTRIBFVPROC>;
    static glad_glGetVertexAttribfvARB: Option<PFNGLGETVERTEXATTRIBFVARBPROC>;
    static glad_glGetVertexAttribiv: Option<PFNGLGETVERTEXATTRIBIVPROC>;
    static glad_glGetVertexAttribivARB: Option<PFNGLGETVERTEXATTRIBIVARBPROC>;
    static glad_glGetnUniformi64vARB: Option<PFNGLGETNUNIFORMI64VARBPROC>;
    static glad_glGetnUniformui64vARB: Option<PFNGLGETNUNIFORMUI64VARBPROC>;
    static glad_glHint: Option<PFNGLHINTPROC>;
    static glad_glIndexxOES: Option<PFNGLINDEXXOESPROC>;
    static glad_glIndexxvOES: Option<PFNGLINDEXXVOESPROC>;
    static glad_glInvalidateBufferData: Option<PFNGLINVALIDATEBUFFERDATAPROC>;
    static glad_glInvalidateBufferSubData: Option<PFNGLINVALIDATEBUFFERSUBDATAPROC>;
    static glad_glInvalidateFramebuffer: Option<PFNGLINVALIDATEFRAMEBUFFERPROC>;
    static glad_glInvalidateNamedFramebufferData: Option<PFNGLINVALIDATENAMEDFRAMEBUFFERDATAPROC>;
    static glad_glInvalidateNamedFramebufferSubData: Option<PFNGLINVALIDATENAMEDFRAMEBUFFERSUBDATAPROC>;
    static glad_glInvalidateSubFramebuffer: Option<PFNGLINVALIDATESUBFRAMEBUFFERPROC>;
    static glad_glInvalidateTexImage: Option<PFNGLINVALIDATETEXIMAGEPROC>;
    static glad_glInvalidateTexSubImage: Option<PFNGLINVALIDATETEXSUBIMAGEPROC>;
    static glad_glIsBuffer: Option<PFNGLISBUFFERPROC>;
    static glad_glIsBufferARB: Option<PFNGLISBUFFERARBPROC>;
    static glad_glIsEnabled: Option<PFNGLISENABLEDPROC>;
    static glad_glIsEnabledi: Option<PFNGLISENABLEDIPROC>;
    static glad_glIsFramebuffer: Option<PFNGLISFRAMEBUFFERPROC>;
    static glad_glIsFramebufferEXT: Option<PFNGLISFRAMEBUFFEREXTPROC>;
    static glad_glIsNamedStringARB: Option<PFNGLISNAMEDSTRINGARBPROC>;
    static glad_glIsProgram: Option<PFNGLISPROGRAMPROC>;
    static glad_glIsProgramARB: Option<PFNGLISPROGRAMARBPROC>;
    static glad_glIsProgramPipeline: Option<PFNGLISPROGRAMPIPELINEPROC>;
    static glad_glIsQuery: Option<PFNGLISQUERYPROC>;
    static glad_glIsQueryARB: Option<PFNGLISQUERYARBPROC>;
    static glad_glIsRenderbuffer: Option<PFNGLISRENDERBUFFERPROC>;
    static glad_glIsRenderbufferEXT: Option<PFNGLISRENDERBUFFEREXTPROC>;
    static glad_glIsSampler: Option<PFNGLISSAMPLERPROC>;
    static glad_glIsShader: Option<PFNGLISSHADERPROC>;
    static glad_glIsSync: Option<PFNGLISSYNCPROC>;
    static glad_glIsTexture: Option<PFNGLISTEXTUREPROC>;
    static glad_glIsTransformFeedback: Option<PFNGLISTRANSFORMFEEDBACKPROC>;
    static glad_glIsVertexArray: Option<PFNGLISVERTEXARRAYPROC>;
    static glad_glLightModelxOES: Option<PFNGLLIGHTMODELXOESPROC>;
    static glad_glLightModelxvOES: Option<PFNGLLIGHTMODELXVOESPROC>;
    static glad_glLightxOES: Option<PFNGLLIGHTXOESPROC>;
    static glad_glLightxvOES: Option<PFNGLLIGHTXVOESPROC>;
    static glad_glLineWidth: Option<PFNGLLINEWIDTHPROC>;
    static glad_glLineWidthxOES: Option<PFNGLLINEWIDTHXOESPROC>;
    static glad_glLinkProgram: Option<PFNGLLINKPROGRAMPROC>;
    static glad_glLinkProgramARB: Option<PFNGLLINKPROGRAMARBPROC>;
    static glad_glLoadMatrixxOES: Option<PFNGLLOADMATRIXXOESPROC>;
    static glad_glLoadTransposeMatrixdARB: Option<PFNGLLOADTRANSPOSEMATRIXDARBPROC>;
    static glad_glLoadTransposeMatrixfARB: Option<PFNGLLOADTRANSPOSEMATRIXFARBPROC>;
    static glad_glLoadTransposeMatrixxOES: Option<PFNGLLOADTRANSPOSEMATRIXXOESPROC>;
    static glad_glLogicOp: Option<PFNGLLOGICOPPROC>;
    static glad_glMap1xOES: Option<PFNGLMAP1XOESPROC>;
    static glad_glMap2xOES: Option<PFNGLMAP2XOESPROC>;
    static glad_glMapBuffer: Option<PFNGLMAPBUFFERPROC>;
    static glad_glMapBufferARB: Option<PFNGLMAPBUFFERARBPROC>;
    static glad_glMapBufferRange: Option<PFNGLMAPBUFFERRANGEPROC>;
    static glad_glMapGrid1xOES: Option<PFNGLMAPGRID1XOESPROC>;
    static glad_glMapGrid2xOES: Option<PFNGLMAPGRID2XOESPROC>;
    static glad_glMapNamedBuffer: Option<PFNGLMAPNAMEDBUFFERPROC>;
    static glad_glMapNamedBufferRange: Option<PFNGLMAPNAMEDBUFFERRANGEPROC>;
    static glad_glMaterialxOES: Option<PFNGLMATERIALXOESPROC>;
    static glad_glMaterialxvOES: Option<PFNGLMATERIALXVOESPROC>;
    static glad_glMemoryBarrier: Option<PFNGLMEMORYBARRIERPROC>;
    static glad_glMemoryBarrierByRegion: Option<PFNGLMEMORYBARRIERBYREGIONPROC>;
    static glad_glMinSampleShading: Option<PFNGLMINSAMPLESHADINGPROC>;
    static glad_glMinSampleShadingARB: Option<PFNGLMINSAMPLESHADINGARBPROC>;
    static glad_glMultMatrixxOES: Option<PFNGLMULTMATRIXXOESPROC>;
    static glad_glMultTransposeMatrixdARB: Option<PFNGLMULTTRANSPOSEMATRIXDARBPROC>;
    static glad_glMultTransposeMatrixfARB: Option<PFNGLMULTTRANSPOSEMATRIXFARBPROC>;
    static glad_glMultTransposeMatrixxOES: Option<PFNGLMULTTRANSPOSEMATRIXXOESPROC>;
    static glad_glMultiDrawArrays: Option<PFNGLMULTIDRAWARRAYSPROC>;
    static glad_glMultiDrawArraysIndirect: Option<PFNGLMULTIDRAWARRAYSINDIRECTPROC>;
    static glad_glMultiDrawElements: Option<PFNGLMULTIDRAWELEMENTSPROC>;
    static glad_glMultiDrawElementsBaseVertex: Option<PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC>;
    static glad_glMultiDrawElementsIndirect: Option<PFNGLMULTIDRAWELEMENTSINDIRECTPROC>;
    static glad_glMultiTexCoord1dARB: Option<PFNGLMULTITEXCOORD1DARBPROC>;
    static glad_glMultiTexCoord1dvARB: Option<PFNGLMULTITEXCOORD1DVARBPROC>;
    static glad_glMultiTexCoord1fARB: Option<PFNGLMULTITEXCOORD1FARBPROC>;
    static glad_glMultiTexCoord1fvARB: Option<PFNGLMULTITEXCOORD1FVARBPROC>;
    static glad_glMultiTexCoord1iARB: Option<PFNGLMULTITEXCOORD1IARBPROC>;
    static glad_glMultiTexCoord1ivARB: Option<PFNGLMULTITEXCOORD1IVARBPROC>;
    static glad_glMultiTexCoord1sARB: Option<PFNGLMULTITEXCOORD1SARBPROC>;
    static glad_glMultiTexCoord1svARB: Option<PFNGLMULTITEXCOORD1SVARBPROC>;
    static glad_glMultiTexCoord1xOES: Option<PFNGLMULTITEXCOORD1XOESPROC>;
    static glad_glMultiTexCoord1xvOES: Option<PFNGLMULTITEXCOORD1XVOESPROC>;
    static glad_glMultiTexCoord2dARB: Option<PFNGLMULTITEXCOORD2DARBPROC>;
    static glad_glMultiTexCoord2dvARB: Option<PFNGLMULTITEXCOORD2DVARBPROC>;
    static glad_glMultiTexCoord2fARB: Option<PFNGLMULTITEXCOORD2FARBPROC>;
    static glad_glMultiTexCoord2fvARB: Option<PFNGLMULTITEXCOORD2FVARBPROC>;
    static glad_glMultiTexCoord2iARB: Option<PFNGLMULTITEXCOORD2IARBPROC>;
    static glad_glMultiTexCoord2ivARB: Option<PFNGLMULTITEXCOORD2IVARBPROC>;
    static glad_glMultiTexCoord2sARB: Option<PFNGLMULTITEXCOORD2SARBPROC>;
    static glad_glMultiTexCoord2svARB: Option<PFNGLMULTITEXCOORD2SVARBPROC>;
    static glad_glMultiTexCoord2xOES: Option<PFNGLMULTITEXCOORD2XOESPROC>;
    static glad_glMultiTexCoord2xvOES: Option<PFNGLMULTITEXCOORD2XVOESPROC>;
    static glad_glMultiTexCoord3dARB: Option<PFNGLMULTITEXCOORD3DARBPROC>;
    static glad_glMultiTexCoord3dvARB: Option<PFNGLMULTITEXCOORD3DVARBPROC>;
    static glad_glMultiTexCoord3fARB: Option<PFNGLMULTITEXCOORD3FARBPROC>;
    static glad_glMultiTexCoord3fvARB: Option<PFNGLMULTITEXCOORD3FVARBPROC>;
    static glad_glMultiTexCoord3iARB: Option<PFNGLMULTITEXCOORD3IARBPROC>;
    static glad_glMultiTexCoord3ivARB: Option<PFNGLMULTITEXCOORD3IVARBPROC>;
    static glad_glMultiTexCoord3sARB: Option<PFNGLMULTITEXCOORD3SARBPROC>;
    static glad_glMultiTexCoord3svARB: Option<PFNGLMULTITEXCOORD3SVARBPROC>;
    static glad_glMultiTexCoord3xOES: Option<PFNGLMULTITEXCOORD3XOESPROC>;
    static glad_glMultiTexCoord3xvOES: Option<PFNGLMULTITEXCOORD3XVOESPROC>;
    static glad_glMultiTexCoord4dARB: Option<PFNGLMULTITEXCOORD4DARBPROC>;
    static glad_glMultiTexCoord4dvARB: Option<PFNGLMULTITEXCOORD4DVARBPROC>;
    static glad_glMultiTexCoord4fARB: Option<PFNGLMULTITEXCOORD4FARBPROC>;
    static glad_glMultiTexCoord4fvARB: Option<PFNGLMULTITEXCOORD4FVARBPROC>;
    static glad_glMultiTexCoord4iARB: Option<PFNGLMULTITEXCOORD4IARBPROC>;
    static glad_glMultiTexCoord4ivARB: Option<PFNGLMULTITEXCOORD4IVARBPROC>;
    static glad_glMultiTexCoord4sARB: Option<PFNGLMULTITEXCOORD4SARBPROC>;
    static glad_glMultiTexCoord4svARB: Option<PFNGLMULTITEXCOORD4SVARBPROC>;
    static glad_glMultiTexCoord4xOES: Option<PFNGLMULTITEXCOORD4XOESPROC>;
    static glad_glMultiTexCoord4xvOES: Option<PFNGLMULTITEXCOORD4XVOESPROC>;
    static glad_glNamedBufferData: Option<PFNGLNAMEDBUFFERDATAPROC>;
    static glad_glNamedBufferStorage: Option<PFNGLNAMEDBUFFERSTORAGEPROC>;
    static glad_glNamedBufferSubData: Option<PFNGLNAMEDBUFFERSUBDATAPROC>;
    static glad_glNamedFramebufferDrawBuffer: Option<PFNGLNAMEDFRAMEBUFFERDRAWBUFFERPROC>;
    static glad_glNamedFramebufferDrawBuffers: Option<PFNGLNAMEDFRAMEBUFFERDRAWBUFFERSPROC>;
    static glad_glNamedFramebufferParameteri: Option<PFNGLNAMEDFRAMEBUFFERPARAMETERIPROC>;
    static glad_glNamedFramebufferReadBuffer: Option<PFNGLNAMEDFRAMEBUFFERREADBUFFERPROC>;
    static glad_glNamedFramebufferRenderbuffer: Option<PFNGLNAMEDFRAMEBUFFERRENDERBUFFERPROC>;
    static glad_glNamedFramebufferSampleLocationsfvARB: Option<PFNGLNAMEDFRAMEBUFFERSAMPLELOCATIONSFVARBPROC>;
    static glad_glNamedFramebufferTexture: Option<PFNGLNAMEDFRAMEBUFFERTEXTUREPROC>;
    static glad_glNamedFramebufferTextureLayer: Option<PFNGLNAMEDFRAMEBUFFERTEXTURELAYERPROC>;
    static glad_glNamedRenderbufferStorage: Option<PFNGLNAMEDRENDERBUFFERSTORAGEPROC>;
    static glad_glNamedRenderbufferStorageMultisample: Option<PFNGLNAMEDRENDERBUFFERSTORAGEMULTISAMPLEPROC>;
    static glad_glNamedStringARB: Option<PFNGLNAMEDSTRINGARBPROC>;
    static glad_glNormal3xOES: Option<PFNGLNORMAL3XOESPROC>;
    static glad_glNormal3xvOES: Option<PFNGLNORMAL3XVOESPROC>;
    static glad_glObjectLabel: Option<PFNGLOBJECTLABELPROC>;
    static glad_glObjectPtrLabel: Option<PFNGLOBJECTPTRLABELPROC>;
    static glad_glOrthoxOES: Option<PFNGLORTHOXOESPROC>;
    static glad_glPassThroughxOES: Option<PFNGLPASSTHROUGHXOESPROC>;
    static glad_glPatchParameterfv: Option<PFNGLPATCHPARAMETERFVPROC>;
    static glad_glPatchParameteri: Option<PFNGLPATCHPARAMETERIPROC>;
    static glad_glPauseTransformFeedback: Option<PFNGLPAUSETRANSFORMFEEDBACKPROC>;
    static glad_glPixelMapx: Option<PFNGLPIXELMAPXPROC>;
    static glad_glPixelStoref: Option<PFNGLPIXELSTOREFPROC>;
    static glad_glPixelStorei: Option<PFNGLPIXELSTOREIPROC>;
    static glad_glPixelStorex: Option<PFNGLPIXELSTOREXPROC>;
    static glad_glPixelTransferxOES: Option<PFNGLPIXELTRANSFERXOESPROC>;
    static glad_glPixelZoomxOES: Option<PFNGLPIXELZOOMXOESPROC>;
    static glad_glPointParameterf: Option<PFNGLPOINTPARAMETERFPROC>;
    static glad_glPointParameterfv: Option<PFNGLPOINTPARAMETERFVPROC>;
    static glad_glPointParameteri: Option<PFNGLPOINTPARAMETERIPROC>;
    static glad_glPointParameteriv: Option<PFNGLPOINTPARAMETERIVPROC>;
    static glad_glPointParameterxvOES: Option<PFNGLPOINTPARAMETERXVOESPROC>;
    static glad_glPointSize: Option<PFNGLPOINTSIZEPROC>;
    static glad_glPointSizexOES: Option<PFNGLPOINTSIZEXOESPROC>;
    static glad_glPolygonMode: Option<PFNGLPOLYGONMODEPROC>;
    static glad_glPolygonOffset: Option<PFNGLPOLYGONOFFSETPROC>;
    static glad_glPolygonOffsetxOES: Option<PFNGLPOLYGONOFFSETXOESPROC>;
    static glad_glPopDebugGroup: Option<PFNGLPOPDEBUGGROUPPROC>;
    static glad_glPrimitiveBoundingBoxARB: Option<PFNGLPRIMITIVEBOUNDINGBOXARBPROC>;
    static glad_glPrimitiveRestartIndex: Option<PFNGLPRIMITIVERESTARTINDEXPROC>;
    static glad_glPrioritizeTexturesxOES: Option<PFNGLPRIORITIZETEXTURESXOESPROC>;
    static glad_glProgramBinary: Option<PFNGLPROGRAMBINARYPROC>;
    static glad_glProgramEnvParameter4dARB: Option<PFNGLPROGRAMENVPARAMETER4DARBPROC>;
    static glad_glProgramEnvParameter4dvARB: Option<PFNGLPROGRAMENVPARAMETER4DVARBPROC>;
    static glad_glProgramEnvParameter4fARB: Option<PFNGLPROGRAMENVPARAMETER4FARBPROC>;
    static glad_glProgramEnvParameter4fvARB: Option<PFNGLPROGRAMENVPARAMETER4FVARBPROC>;
    static glad_glProgramLocalParameter4dARB: Option<PFNGLPROGRAMLOCALPARAMETER4DARBPROC>;
    static glad_glProgramLocalParameter4dvARB: Option<PFNGLPROGRAMLOCALPARAMETER4DVARBPROC>;
    static glad_glProgramLocalParameter4fARB: Option<PFNGLPROGRAMLOCALPARAMETER4FARBPROC>;
    static glad_glProgramLocalParameter4fvARB: Option<PFNGLPROGRAMLOCALPARAMETER4FVARBPROC>;
    static glad_glProgramParameteri: Option<PFNGLPROGRAMPARAMETERIPROC>;
    static glad_glProgramParameteriARB: Option<PFNGLPROGRAMPARAMETERIARBPROC>;
    static glad_glProgramStringARB: Option<PFNGLPROGRAMSTRINGARBPROC>;
    static glad_glProgramUniform1d: Option<PFNGLPROGRAMUNIFORM1DPROC>;
    static glad_glProgramUniform1dv: Option<PFNGLPROGRAMUNIFORM1DVPROC>;
    static glad_glProgramUniform1f: Option<PFNGLPROGRAMUNIFORM1FPROC>;
    static glad_glProgramUniform1fv: Option<PFNGLPROGRAMUNIFORM1FVPROC>;
    static glad_glProgramUniform1i: Option<PFNGLPROGRAMUNIFORM1IPROC>;
    static glad_glProgramUniform1i64ARB: Option<PFNGLPROGRAMUNIFORM1I64ARBPROC>;
    static glad_glProgramUniform1i64vARB: Option<PFNGLPROGRAMUNIFORM1I64VARBPROC>;
    static glad_glProgramUniform1iv: Option<PFNGLPROGRAMUNIFORM1IVPROC>;
    static glad_glProgramUniform1ui: Option<PFNGLPROGRAMUNIFORM1UIPROC>;
    static glad_glProgramUniform1ui64ARB: Option<PFNGLPROGRAMUNIFORM1UI64ARBPROC>;
    static glad_glProgramUniform1ui64vARB: Option<PFNGLPROGRAMUNIFORM1UI64VARBPROC>;
    static glad_glProgramUniform1uiv: Option<PFNGLPROGRAMUNIFORM1UIVPROC>;
    static glad_glProgramUniform2d: Option<PFNGLPROGRAMUNIFORM2DPROC>;
    static glad_glProgramUniform2dv: Option<PFNGLPROGRAMUNIFORM2DVPROC>;
    static glad_glProgramUniform2f: Option<PFNGLPROGRAMUNIFORM2FPROC>;
    static glad_glProgramUniform2fv: Option<PFNGLPROGRAMUNIFORM2FVPROC>;
    static glad_glProgramUniform2i: Option<PFNGLPROGRAMUNIFORM2IPROC>;
    static glad_glProgramUniform2i64ARB: Option<PFNGLPROGRAMUNIFORM2I64ARBPROC>;
    static glad_glProgramUniform2i64vARB: Option<PFNGLPROGRAMUNIFORM2I64VARBPROC>;
    static glad_glProgramUniform2iv: Option<PFNGLPROGRAMUNIFORM2IVPROC>;
    static glad_glProgramUniform2ui: Option<PFNGLPROGRAMUNIFORM2UIPROC>;
    static glad_glProgramUniform2ui64ARB: Option<PFNGLPROGRAMUNIFORM2UI64ARBPROC>;
    static glad_glProgramUniform2ui64vARB: Option<PFNGLPROGRAMUNIFORM2UI64VARBPROC>;
    static glad_glProgramUniform2uiv: Option<PFNGLPROGRAMUNIFORM2UIVPROC>;
    static glad_glProgramUniform3d: Option<PFNGLPROGRAMUNIFORM3DPROC>;
    static glad_glProgramUniform3dv: Option<PFNGLPROGRAMUNIFORM3DVPROC>;
    static glad_glProgramUniform3f: Option<PFNGLPROGRAMUNIFORM3FPROC>;
    static glad_glProgramUniform3fv: Option<PFNGLPROGRAMUNIFORM3FVPROC>;
    static glad_glProgramUniform3i: Option<PFNGLPROGRAMUNIFORM3IPROC>;
    static glad_glProgramUniform3i64ARB: Option<PFNGLPROGRAMUNIFORM3I64ARBPROC>;
    static glad_glProgramUniform3i64vARB: Option<PFNGLPROGRAMUNIFORM3I64VARBPROC>;
    static glad_glProgramUniform3iv: Option<PFNGLPROGRAMUNIFORM3IVPROC>;
    static glad_glProgramUniform3ui: Option<PFNGLPROGRAMUNIFORM3UIPROC>;
    static glad_glProgramUniform3ui64ARB: Option<PFNGLPROGRAMUNIFORM3UI64ARBPROC>;
    static glad_glProgramUniform3ui64vARB: Option<PFNGLPROGRAMUNIFORM3UI64VARBPROC>;
    static glad_glProgramUniform3uiv: Option<PFNGLPROGRAMUNIFORM3UIVPROC>;
    static glad_glProgramUniform4d: Option<PFNGLPROGRAMUNIFORM4DPROC>;
    static glad_glProgramUniform4dv: Option<PFNGLPROGRAMUNIFORM4DVPROC>;
    static glad_glProgramUniform4f: Option<PFNGLPROGRAMUNIFORM4FPROC>;
    static glad_glProgramUniform4fv: Option<PFNGLPROGRAMUNIFORM4FVPROC>;
    static glad_glProgramUniform4i: Option<PFNGLPROGRAMUNIFORM4IPROC>;
    static glad_glProgramUniform4i64ARB: Option<PFNGLPROGRAMUNIFORM4I64ARBPROC>;
    static glad_glProgramUniform4i64vARB: Option<PFNGLPROGRAMUNIFORM4I64VARBPROC>;
    static glad_glProgramUniform4iv: Option<PFNGLPROGRAMUNIFORM4IVPROC>;
    static glad_glProgramUniform4ui: Option<PFNGLPROGRAMUNIFORM4UIPROC>;
    static glad_glProgramUniform4ui64ARB: Option<PFNGLPROGRAMUNIFORM4UI64ARBPROC>;
    static glad_glProgramUniform4ui64vARB: Option<PFNGLPROGRAMUNIFORM4UI64VARBPROC>;
    static glad_glProgramUniform4uiv: Option<PFNGLPROGRAMUNIFORM4UIVPROC>;
    static glad_glProgramUniformMatrix2dv: Option<PFNGLPROGRAMUNIFORMMATRIX2DVPROC>;
    static glad_glProgramUniformMatrix2fv: Option<PFNGLPROGRAMUNIFORMMATRIX2FVPROC>;
    static glad_glProgramUniformMatrix2x3dv: Option<PFNGLPROGRAMUNIFORMMATRIX2X3DVPROC>;
    static glad_glProgramUniformMatrix2x3fv: Option<PFNGLPROGRAMUNIFORMMATRIX2X3FVPROC>;
    static glad_glProgramUniformMatrix2x4dv: Option<PFNGLPROGRAMUNIFORMMATRIX2X4DVPROC>;
    static glad_glProgramUniformMatrix2x4fv: Option<PFNGLPROGRAMUNIFORMMATRIX2X4FVPROC>;
    static glad_glProgramUniformMatrix3dv: Option<PFNGLPROGRAMUNIFORMMATRIX3DVPROC>;
    static glad_glProgramUniformMatrix3fv: Option<PFNGLPROGRAMUNIFORMMATRIX3FVPROC>;
    static glad_glProgramUniformMatrix3x2dv: Option<PFNGLPROGRAMUNIFORMMATRIX3X2DVPROC>;
    static glad_glProgramUniformMatrix3x2fv: Option<PFNGLPROGRAMUNIFORMMATRIX3X2FVPROC>;
    static glad_glProgramUniformMatrix3x4dv: Option<PFNGLPROGRAMUNIFORMMATRIX3X4DVPROC>;
    static glad_glProgramUniformMatrix3x4fv: Option<PFNGLPROGRAMUNIFORMMATRIX3X4FVPROC>;
    static glad_glProgramUniformMatrix4dv: Option<PFNGLPROGRAMUNIFORMMATRIX4DVPROC>;
    static glad_glProgramUniformMatrix4fv: Option<PFNGLPROGRAMUNIFORMMATRIX4FVPROC>;
    static glad_glProgramUniformMatrix4x2dv: Option<PFNGLPROGRAMUNIFORMMATRIX4X2DVPROC>;
    static glad_glProgramUniformMatrix4x2fv: Option<PFNGLPROGRAMUNIFORMMATRIX4X2FVPROC>;
    static glad_glProgramUniformMatrix4x3dv: Option<PFNGLPROGRAMUNIFORMMATRIX4X3DVPROC>;
    static glad_glProgramUniformMatrix4x3fv: Option<PFNGLPROGRAMUNIFORMMATRIX4X3FVPROC>;
    static glad_glProvokingVertex: Option<PFNGLPROVOKINGVERTEXPROC>;
    static glad_glPushDebugGroup: Option<PFNGLPUSHDEBUGGROUPPROC>;
    static glad_glQueryCounter: Option<PFNGLQUERYCOUNTERPROC>;
    static glad_glRasterPos2xOES: Option<PFNGLRASTERPOS2XOESPROC>;
    static glad_glRasterPos2xvOES: Option<PFNGLRASTERPOS2XVOESPROC>;
    static glad_glRasterPos3xOES: Option<PFNGLRASTERPOS3XOESPROC>;
    static glad_glRasterPos3xvOES: Option<PFNGLRASTERPOS3XVOESPROC>;
    static glad_glRasterPos4xOES: Option<PFNGLRASTERPOS4XOESPROC>;
    static glad_glRasterPos4xvOES: Option<PFNGLRASTERPOS4XVOESPROC>;
    static glad_glReadBuffer: Option<PFNGLREADBUFFERPROC>;
    static glad_glReadPixels: Option<PFNGLREADPIXELSPROC>;
    static glad_glRectxOES: Option<PFNGLRECTXOESPROC>;
    static glad_glRectxvOES: Option<PFNGLRECTXVOESPROC>;
    static glad_glReleaseShaderCompiler: Option<PFNGLRELEASESHADERCOMPILERPROC>;
    static glad_glRenderbufferStorage: Option<PFNGLRENDERBUFFERSTORAGEPROC>;
    static glad_glRenderbufferStorageEXT: Option<PFNGLRENDERBUFFERSTORAGEEXTPROC>;
    static glad_glRenderbufferStorageMultisample: Option<PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC>;
    static glad_glRenderbufferStorageMultisampleEXT: Option<PFNGLRENDERBUFFERSTORAGEMULTISAMPLEEXTPROC>;
    static glad_glResumeTransformFeedback: Option<PFNGLRESUMETRANSFORMFEEDBACKPROC>;
    static glad_glRotatexOES: Option<PFNGLROTATEXOESPROC>;
    static glad_glSampleCoverage: Option<PFNGLSAMPLECOVERAGEPROC>;
    static glad_glSampleCoverageARB: Option<PFNGLSAMPLECOVERAGEARBPROC>;
    static glad_glSampleMaski: Option<PFNGLSAMPLEMASKIPROC>;
    static glad_glSamplerParameterIiv: Option<PFNGLSAMPLERPARAMETERIIVPROC>;
    static glad_glSamplerParameterIuiv: Option<PFNGLSAMPLERPARAMETERIUIVPROC>;
    static glad_glSamplerParameterf: Option<PFNGLSAMPLERPARAMETERFPROC>;
    static glad_glSamplerParameterfv: Option<PFNGLSAMPLERPARAMETERFVPROC>;
    static glad_glSamplerParameteri: Option<PFNGLSAMPLERPARAMETERIPROC>;
    static glad_glSamplerParameteriv: Option<PFNGLSAMPLERPARAMETERIVPROC>;
    static glad_glScalexOES: Option<PFNGLSCALEXOESPROC>;
    static glad_glScissor: Option<PFNGLSCISSORPROC>;
    static glad_glScissorArrayv: Option<PFNGLSCISSORARRAYVPROC>;
    static glad_glScissorIndexed: Option<PFNGLSCISSORINDEXEDPROC>;
    static glad_glScissorIndexedv: Option<PFNGLSCISSORINDEXEDVPROC>;
    static glad_glShaderBinary: Option<PFNGLSHADERBINARYPROC>;
    static glad_glShaderSource: Option<PFNGLSHADERSOURCEPROC>;
    static glad_glShaderSourceARB: Option<PFNGLSHADERSOURCEARBPROC>;
    static glad_glShaderStorageBlockBinding: Option<PFNGLSHADERSTORAGEBLOCKBINDINGPROC>;
    static glad_glSpecializeShaderARB: Option<PFNGLSPECIALIZESHADERARBPROC>;
    static glad_glStencilFunc: Option<PFNGLSTENCILFUNCPROC>;
    static glad_glStencilFuncSeparate: Option<PFNGLSTENCILFUNCSEPARATEPROC>;
    static glad_glStencilMask: Option<PFNGLSTENCILMASKPROC>;
    static glad_glStencilMaskSeparate: Option<PFNGLSTENCILMASKSEPARATEPROC>;
    static glad_glStencilOp: Option<PFNGLSTENCILOPPROC>;
    static glad_glStencilOpSeparate: Option<PFNGLSTENCILOPSEPARATEPROC>;
    static glad_glTexBuffer: Option<PFNGLTEXBUFFERPROC>;
    static glad_glTexBufferRange: Option<PFNGLTEXBUFFERRANGEPROC>;
    static glad_glTexCoord1xOES: Option<PFNGLTEXCOORD1XOESPROC>;
    static glad_glTexCoord1xvOES: Option<PFNGLTEXCOORD1XVOESPROC>;
    static glad_glTexCoord2xOES: Option<PFNGLTEXCOORD2XOESPROC>;
    static glad_glTexCoord2xvOES: Option<PFNGLTEXCOORD2XVOESPROC>;
    static glad_glTexCoord3xOES: Option<PFNGLTEXCOORD3XOESPROC>;
    static glad_glTexCoord3xvOES: Option<PFNGLTEXCOORD3XVOESPROC>;
    static glad_glTexCoord4xOES: Option<PFNGLTEXCOORD4XOESPROC>;
    static glad_glTexCoord4xvOES: Option<PFNGLTEXCOORD4XVOESPROC>;
    static glad_glTexEnvxOES: Option<PFNGLTEXENVXOESPROC>;
    static glad_glTexEnvxvOES: Option<PFNGLTEXENVXVOESPROC>;
    static glad_glTexGenxOES: Option<PFNGLTEXGENXOESPROC>;
    static glad_glTexGenxvOES: Option<PFNGLTEXGENXVOESPROC>;
    static glad_glTexImage1D: Option<PFNGLTEXIMAGE1DPROC>;
    static glad_glTexImage2D: Option<PFNGLTEXIMAGE2DPROC>;
    static glad_glTexImage2DMultisample: Option<PFNGLTEXIMAGE2DMULTISAMPLEPROC>;
    static glad_glTexImage3D: Option<PFNGLTEXIMAGE3DPROC>;
    static glad_glTexImage3DMultisample: Option<PFNGLTEXIMAGE3DMULTISAMPLEPROC>;
    static glad_glTexParameterIiv: Option<PFNGLTEXPARAMETERIIVPROC>;
    static glad_glTexParameterIuiv: Option<PFNGLTEXPARAMETERIUIVPROC>;
    static glad_glTexParameterf: Option<PFNGLTEXPARAMETERFPROC>;
    static glad_glTexParameterfv: Option<PFNGLTEXPARAMETERFVPROC>;
    static glad_glTexParameteri: Option<PFNGLTEXPARAMETERIPROC>;
    static glad_glTexParameteriv: Option<PFNGLTEXPARAMETERIVPROC>;
    static glad_glTexParameterxOES: Option<PFNGLTEXPARAMETERXOESPROC>;
    static glad_glTexParameterxvOES: Option<PFNGLTEXPARAMETERXVOESPROC>;
    static glad_glTexStorage1D: Option<PFNGLTEXSTORAGE1DPROC>;
    static glad_glTexStorage2D: Option<PFNGLTEXSTORAGE2DPROC>;
    static glad_glTexStorage2DMultisample: Option<PFNGLTEXSTORAGE2DMULTISAMPLEPROC>;
    static glad_glTexStorage3D: Option<PFNGLTEXSTORAGE3DPROC>;
    static glad_glTexStorage3DMultisample: Option<PFNGLTEXSTORAGE3DMULTISAMPLEPROC>;
    static glad_glTexSubImage1D: Option<PFNGLTEXSUBIMAGE1DPROC>;
    static glad_glTexSubImage2D: Option<PFNGLTEXSUBIMAGE2DPROC>;
    static glad_glTexSubImage3D: Option<PFNGLTEXSUBIMAGE3DPROC>;
    static glad_glTextureBuffer: Option<PFNGLTEXTUREBUFFERPROC>;
    static glad_glTextureBufferRange: Option<PFNGLTEXTUREBUFFERRANGEPROC>;
    static glad_glTextureParameterIiv: Option<PFNGLTEXTUREPARAMETERIIVPROC>;
    static glad_glTextureParameterIuiv: Option<PFNGLTEXTUREPARAMETERIUIVPROC>;
    static glad_glTextureParameterf: Option<PFNGLTEXTUREPARAMETERFPROC>;
    static glad_glTextureParameterfv: Option<PFNGLTEXTUREPARAMETERFVPROC>;
    static glad_glTextureParameteri: Option<PFNGLTEXTUREPARAMETERIPROC>;
    static glad_glTextureParameteriv: Option<PFNGLTEXTUREPARAMETERIVPROC>;
    static glad_glTextureStorage1D: Option<PFNGLTEXTURESTORAGE1DPROC>;
    static glad_glTextureStorage2D: Option<PFNGLTEXTURESTORAGE2DPROC>;
    static glad_glTextureStorage2DMultisample: Option<PFNGLTEXTURESTORAGE2DMULTISAMPLEPROC>;
    static glad_glTextureStorage3D: Option<PFNGLTEXTURESTORAGE3DPROC>;
    static glad_glTextureStorage3DMultisample: Option<PFNGLTEXTURESTORAGE3DMULTISAMPLEPROC>;
    static glad_glTextureSubImage1D: Option<PFNGLTEXTURESUBIMAGE1DPROC>;
    static glad_glTextureSubImage2D: Option<PFNGLTEXTURESUBIMAGE2DPROC>;
    static glad_glTextureSubImage3D: Option<PFNGLTEXTURESUBIMAGE3DPROC>;
    static glad_glTextureView: Option<PFNGLTEXTUREVIEWPROC>;
    static glad_glTransformFeedbackBufferBase: Option<PFNGLTRANSFORMFEEDBACKBUFFERBASEPROC>;
    static glad_glTransformFeedbackBufferRange: Option<PFNGLTRANSFORMFEEDBACKBUFFERRANGEPROC>;
    static glad_glTransformFeedbackVaryings: Option<PFNGLTRANSFORMFEEDBACKVARYINGSPROC>;
    static glad_glTranslatexOES: Option<PFNGLTRANSLATEXOESPROC>;
    static glad_glUniform1d: Option<PFNGLUNIFORM1DPROC>;
    static glad_glUniform1dv: Option<PFNGLUNIFORM1DVPROC>;
    static glad_glUniform1f: Option<PFNGLUNIFORM1FPROC>;
    static glad_glUniform1fARB: Option<PFNGLUNIFORM1FARBPROC>;
    static glad_glUniform1fv: Option<PFNGLUNIFORM1FVPROC>;
    static glad_glUniform1fvARB: Option<PFNGLUNIFORM1FVARBPROC>;
    static glad_glUniform1i: Option<PFNGLUNIFORM1IPROC>;
    static glad_glUniform1i64ARB: Option<PFNGLUNIFORM1I64ARBPROC>;
    static glad_glUniform1i64vARB: Option<PFNGLUNIFORM1I64VARBPROC>;
    static glad_glUniform1iARB: Option<PFNGLUNIFORM1IARBPROC>;
    static glad_glUniform1iv: Option<PFNGLUNIFORM1IVPROC>;
    static glad_glUniform1ivARB: Option<PFNGLUNIFORM1IVARBPROC>;
    static glad_glUniform1ui: Option<PFNGLUNIFORM1UIPROC>;
    static glad_glUniform1ui64ARB: Option<PFNGLUNIFORM1UI64ARBPROC>;
    static glad_glUniform1ui64vARB: Option<PFNGLUNIFORM1UI64VARBPROC>;
    static glad_glUniform1uiv: Option<PFNGLUNIFORM1UIVPROC>;
    static glad_glUniform2d: Option<PFNGLUNIFORM2DPROC>;
    static glad_glUniform2dv: Option<PFNGLUNIFORM2DVPROC>;
    static glad_glUniform2f: Option<PFNGLUNIFORM2FPROC>;
    static glad_glUniform2fARB: Option<PFNGLUNIFORM2FARBPROC>;
    static glad_glUniform2fv: Option<PFNGLUNIFORM2FVPROC>;
    static glad_glUniform2fvARB: Option<PFNGLUNIFORM2FVARBPROC>;
    static glad_glUniform2i: Option<PFNGLUNIFORM2IPROC>;
    static glad_glUniform2i64ARB: Option<PFNGLUNIFORM2I64ARBPROC>;
    static glad_glUniform2i64vARB: Option<PFNGLUNIFORM2I64VARBPROC>;
    static glad_glUniform2iARB: Option<PFNGLUNIFORM2IARBPROC>;
    static glad_glUniform2iv: Option<PFNGLUNIFORM2IVPROC>;
    static glad_glUniform2ivARB: Option<PFNGLUNIFORM2IVARBPROC>;
    static glad_glUniform2ui: Option<PFNGLUNIFORM2UIPROC>;
    static glad_glUniform2ui64ARB: Option<PFNGLUNIFORM2UI64ARBPROC>;
    static glad_glUniform2ui64vARB: Option<PFNGLUNIFORM2UI64VARBPROC>;
    static glad_glUniform2uiv: Option<PFNGLUNIFORM2UIVPROC>;
    static glad_glUniform3d: Option<PFNGLUNIFORM3DPROC>;
    static glad_glUniform3dv: Option<PFNGLUNIFORM3DVPROC>;
    static glad_glUniform3f: Option<PFNGLUNIFORM3FPROC>;
    static glad_glUniform3fARB: Option<PFNGLUNIFORM3FARBPROC>;
    static glad_glUniform3fv: Option<PFNGLUNIFORM3FVPROC>;
    static glad_glUniform3fvARB: Option<PFNGLUNIFORM3FVARBPROC>;
    static glad_glUniform3i: Option<PFNGLUNIFORM3IPROC>;
    static glad_glUniform3i64ARB: Option<PFNGLUNIFORM3I64ARBPROC>;
    static glad_glUniform3i64vARB: Option<PFNGLUNIFORM3I64VARBPROC>;
    static glad_glUniform3iARB: Option<PFNGLUNIFORM3IARBPROC>;
    static glad_glUniform3iv: Option<PFNGLUNIFORM3IVPROC>;
    static glad_glUniform3ivARB: Option<PFNGLUNIFORM3IVARBPROC>;
    static glad_glUniform3ui: Option<PFNGLUNIFORM3UIPROC>;
    static glad_glUniform3ui64ARB: Option<PFNGLUNIFORM3UI64ARBPROC>;
    static glad_glUniform3ui64vARB: Option<PFNGLUNIFORM3UI64VARBPROC>;
    static glad_glUniform3uiv: Option<PFNGLUNIFORM3UIVPROC>;
    static glad_glUniform4d: Option<PFNGLUNIFORM4DPROC>;
    static glad_glUniform4dv: Option<PFNGLUNIFORM4DVPROC>;
    static glad_glUniform4f: Option<PFNGLUNIFORM4FPROC>;
    static glad_glUniform4fARB: Option<PFNGLUNIFORM4FARBPROC>;
    static glad_glUniform4fv: Option<PFNGLUNIFORM4FVPROC>;
    static glad_glUniform4fvARB: Option<PFNGLUNIFORM4FVARBPROC>;
    static glad_glUniform4i: Option<PFNGLUNIFORM4IPROC>;
    static glad_glUniform4i64ARB: Option<PFNGLUNIFORM4I64ARBPROC>;
    static glad_glUniform4i64vARB: Option<PFNGLUNIFORM4I64VARBPROC>;
    static glad_glUniform4iARB: Option<PFNGLUNIFORM4IARBPROC>;
    static glad_glUniform4iv: Option<PFNGLUNIFORM4IVPROC>;
    static glad_glUniform4ivARB: Option<PFNGLUNIFORM4IVARBPROC>;
    static glad_glUniform4ui: Option<PFNGLUNIFORM4UIPROC>;
    static glad_glUniform4ui64ARB: Option<PFNGLUNIFORM4UI64ARBPROC>;
    static glad_glUniform4ui64vARB: Option<PFNGLUNIFORM4UI64VARBPROC>;
    static glad_glUniform4uiv: Option<PFNGLUNIFORM4UIVPROC>;
    static glad_glUniformBlockBinding: Option<PFNGLUNIFORMBLOCKBINDINGPROC>;
    static glad_glUniformMatrix2dv: Option<PFNGLUNIFORMMATRIX2DVPROC>;
    static glad_glUniformMatrix2fv: Option<PFNGLUNIFORMMATRIX2FVPROC>;
    static glad_glUniformMatrix2fvARB: Option<PFNGLUNIFORMMATRIX2FVARBPROC>;
    static glad_glUniformMatrix2x3dv: Option<PFNGLUNIFORMMATRIX2X3DVPROC>;
    static glad_glUniformMatrix2x3fv: Option<PFNGLUNIFORMMATRIX2X3FVPROC>;
    static glad_glUniformMatrix2x4dv: Option<PFNGLUNIFORMMATRIX2X4DVPROC>;
    static glad_glUniformMatrix2x4fv: Option<PFNGLUNIFORMMATRIX2X4FVPROC>;
    static glad_glUniformMatrix3dv: Option<PFNGLUNIFORMMATRIX3DVPROC>;
    static glad_glUniformMatrix3fv: Option<PFNGLUNIFORMMATRIX3FVPROC>;
    static glad_glUniformMatrix3fvARB: Option<PFNGLUNIFORMMATRIX3FVARBPROC>;
    static glad_glUniformMatrix3x2dv: Option<PFNGLUNIFORMMATRIX3X2DVPROC>;
    static glad_glUniformMatrix3x2fv: Option<PFNGLUNIFORMMATRIX3X2FVPROC>;
    static glad_glUniformMatrix3x4dv: Option<PFNGLUNIFORMMATRIX3X4DVPROC>;
    static glad_glUniformMatrix3x4fv: Option<PFNGLUNIFORMMATRIX3X4FVPROC>;
    static glad_glUniformMatrix4dv: Option<PFNGLUNIFORMMATRIX4DVPROC>;
    static glad_glUniformMatrix4fv: Option<PFNGLUNIFORMMATRIX4FVPROC>;
    static glad_glUniformMatrix4fvARB: Option<PFNGLUNIFORMMATRIX4FVARBPROC>;
    static glad_glUniformMatrix4x2dv: Option<PFNGLUNIFORMMATRIX4X2DVPROC>;
    static glad_glUniformMatrix4x2fv: Option<PFNGLUNIFORMMATRIX4X2FVPROC>;
    static glad_glUniformMatrix4x3dv: Option<PFNGLUNIFORMMATRIX4X3DVPROC>;
    static glad_glUniformMatrix4x3fv: Option<PFNGLUNIFORMMATRIX4X3FVPROC>;
    static glad_glUniformSubroutinesuiv: Option<PFNGLUNIFORMSUBROUTINESUIVPROC>;
    static glad_glUnmapBuffer: Option<PFNGLUNMAPBUFFERPROC>;
    static glad_glUnmapBufferARB: Option<PFNGLUNMAPBUFFERARBPROC>;
    static glad_glUnmapNamedBuffer: Option<PFNGLUNMAPNAMEDBUFFERPROC>;
    static glad_glUseProgram: Option<PFNGLUSEPROGRAMPROC>;
    static glad_glUseProgramObjectARB: Option<PFNGLUSEPROGRAMOBJECTARBPROC>;
    static glad_glUseProgramStages: Option<PFNGLUSEPROGRAMSTAGESPROC>;
    static glad_glValidateProgram: Option<PFNGLVALIDATEPROGRAMPROC>;
    static glad_glValidateProgramARB: Option<PFNGLVALIDATEPROGRAMARBPROC>;
    static glad_glValidateProgramPipeline: Option<PFNGLVALIDATEPROGRAMPIPELINEPROC>;
    static glad_glVertex2xOES: Option<PFNGLVERTEX2XOESPROC>;
    static glad_glVertex2xvOES: Option<PFNGLVERTEX2XVOESPROC>;
    static glad_glVertex3xOES: Option<PFNGLVERTEX3XOESPROC>;
    static glad_glVertex3xvOES: Option<PFNGLVERTEX3XVOESPROC>;
    static glad_glVertex4xOES: Option<PFNGLVERTEX4XOESPROC>;
    static glad_glVertex4xvOES: Option<PFNGLVERTEX4XVOESPROC>;
    static glad_glVertexArrayAttribBinding: Option<PFNGLVERTEXARRAYATTRIBBINDINGPROC>;
    static glad_glVertexArrayAttribFormat: Option<PFNGLVERTEXARRAYATTRIBFORMATPROC>;
    static glad_glVertexArrayAttribIFormat: Option<PFNGLVERTEXARRAYATTRIBIFORMATPROC>;
    static glad_glVertexArrayAttribLFormat: Option<PFNGLVERTEXARRAYATTRIBLFORMATPROC>;
    static glad_glVertexArrayBindingDivisor: Option<PFNGLVERTEXARRAYBINDINGDIVISORPROC>;
    static glad_glVertexArrayElementBuffer: Option<PFNGLVERTEXARRAYELEMENTBUFFERPROC>;
    static glad_glVertexArrayVertexBuffer: Option<PFNGLVERTEXARRAYVERTEXBUFFERPROC>;
    static glad_glVertexArrayVertexBuffers: Option<PFNGLVERTEXARRAYVERTEXBUFFERSPROC>;
    static glad_glVertexAttrib1d: Option<PFNGLVERTEXATTRIB1DPROC>;
    static glad_glVertexAttrib1dARB: Option<PFNGLVERTEXATTRIB1DARBPROC>;
    static glad_glVertexAttrib1dv: Option<PFNGLVERTEXATTRIB1DVPROC>;
    static glad_glVertexAttrib1dvARB: Option<PFNGLVERTEXATTRIB1DVARBPROC>;
    static glad_glVertexAttrib1f: Option<PFNGLVERTEXATTRIB1FPROC>;
    static glad_glVertexAttrib1fARB: Option<PFNGLVERTEXATTRIB1FARBPROC>;
    static glad_glVertexAttrib1fv: Option<PFNGLVERTEXATTRIB1FVPROC>;
    static glad_glVertexAttrib1fvARB: Option<PFNGLVERTEXATTRIB1FVARBPROC>;
    static glad_glVertexAttrib1s: Option<PFNGLVERTEXATTRIB1SPROC>;
    static glad_glVertexAttrib1sARB: Option<PFNGLVERTEXATTRIB1SARBPROC>;
    static glad_glVertexAttrib1sv: Option<PFNGLVERTEXATTRIB1SVPROC>;
    static glad_glVertexAttrib1svARB: Option<PFNGLVERTEXATTRIB1SVARBPROC>;
    static glad_glVertexAttrib2d: Option<PFNGLVERTEXATTRIB2DPROC>;
    static glad_glVertexAttrib2dARB: Option<PFNGLVERTEXATTRIB2DARBPROC>;
    static glad_glVertexAttrib2dv: Option<PFNGLVERTEXATTRIB2DVPROC>;
    static glad_glVertexAttrib2dvARB: Option<PFNGLVERTEXATTRIB2DVARBPROC>;
    static glad_glVertexAttrib2f: Option<PFNGLVERTEXATTRIB2FPROC>;
    static glad_glVertexAttrib2fARB: Option<PFNGLVERTEXATTRIB2FARBPROC>;
    static glad_glVertexAttrib2fv: Option<PFNGLVERTEXATTRIB2FVPROC>;
    static glad_glVertexAttrib2fvARB: Option<PFNGLVERTEXATTRIB2FVARBPROC>;
    static glad_glVertexAttrib2s: Option<PFNGLVERTEXATTRIB2SPROC>;
    static glad_glVertexAttrib2sARB: Option<PFNGLVERTEXATTRIB2SARBPROC>;
    static glad_glVertexAttrib2sv: Option<PFNGLVERTEXATTRIB2SVPROC>;
    static glad_glVertexAttrib2svARB: Option<PFNGLVERTEXATTRIB2SVARBPROC>;
    static glad_glVertexAttrib3d: Option<PFNGLVERTEXATTRIB3DPROC>;
    static glad_glVertexAttrib3dARB: Option<PFNGLVERTEXATTRIB3DARBPROC>;
    static glad_glVertexAttrib3dv: Option<PFNGLVERTEXATTRIB3DVPROC>;
    static glad_glVertexAttrib3dvARB: Option<PFNGLVERTEXATTRIB3DVARBPROC>;
    static glad_glVertexAttrib3f: Option<PFNGLVERTEXATTRIB3FPROC>;
    static glad_glVertexAttrib3fARB: Option<PFNGLVERTEXATTRIB3FARBPROC>;
    static glad_glVertexAttrib3fv: Option<PFNGLVERTEXATTRIB3FVPROC>;
    static glad_glVertexAttrib3fvARB: Option<PFNGLVERTEXATTRIB3FVARBPROC>;
    static glad_glVertexAttrib3s: Option<PFNGLVERTEXATTRIB3SPROC>;
    static glad_glVertexAttrib3sARB: Option<PFNGLVERTEXATTRIB3SARBPROC>;
    static glad_glVertexAttrib3sv: Option<PFNGLVERTEXATTRIB3SVPROC>;
    static glad_glVertexAttrib3svARB: Option<PFNGLVERTEXATTRIB3SVARBPROC>;
    static glad_glVertexAttrib4Nbv: Option<PFNGLVERTEXATTRIB4NBVPROC>;
    static glad_glVertexAttrib4NbvARB: Option<PFNGLVERTEXATTRIB4NBVARBPROC>;
    static glad_glVertexAttrib4Niv: Option<PFNGLVERTEXATTRIB4NIVPROC>;
    static glad_glVertexAttrib4NivARB: Option<PFNGLVERTEXATTRIB4NIVARBPROC>;
    static glad_glVertexAttrib4Nsv: Option<PFNGLVERTEXATTRIB4NSVPROC>;
    static glad_glVertexAttrib4NsvARB: Option<PFNGLVERTEXATTRIB4NSVARBPROC>;
    static glad_glVertexAttrib4Nub: Option<PFNGLVERTEXATTRIB4NUBPROC>;
    static glad_glVertexAttrib4NubARB: Option<PFNGLVERTEXATTRIB4NUBARBPROC>;
    static glad_glVertexAttrib4Nubv: Option<PFNGLVERTEXATTRIB4NUBVPROC>;
    static glad_glVertexAttrib4NubvARB: Option<PFNGLVERTEXATTRIB4NUBVARBPROC>;
    static glad_glVertexAttrib4Nuiv: Option<PFNGLVERTEXATTRIB4NUIVPROC>;
    static glad_glVertexAttrib4NuivARB: Option<PFNGLVERTEXATTRIB4NUIVARBPROC>;
    static glad_glVertexAttrib4Nusv: Option<PFNGLVERTEXATTRIB4NUSVPROC>;
    static glad_glVertexAttrib4NusvARB: Option<PFNGLVERTEXATTRIB4NUSVARBPROC>;
    static glad_glVertexAttrib4bv: Option<PFNGLVERTEXATTRIB4BVPROC>;
    static glad_glVertexAttrib4bvARB: Option<PFNGLVERTEXATTRIB4BVARBPROC>;
    static glad_glVertexAttrib4d: Option<PFNGLVERTEXATTRIB4DPROC>;
    static glad_glVertexAttrib4dARB: Option<PFNGLVERTEXATTRIB4DARBPROC>;
    static glad_glVertexAttrib4dv: Option<PFNGLVERTEXATTRIB4DVPROC>;
    static glad_glVertexAttrib4dvARB: Option<PFNGLVERTEXATTRIB4DVARBPROC>;
    static glad_glVertexAttrib4f: Option<PFNGLVERTEXATTRIB4FPROC>;
    static glad_glVertexAttrib4fARB: Option<PFNGLVERTEXATTRIB4FARBPROC>;
    static glad_glVertexAttrib4fv: Option<PFNGLVERTEXATTRIB4FVPROC>;
    static glad_glVertexAttrib4fvARB: Option<PFNGLVERTEXATTRIB4FVARBPROC>;
    static glad_glVertexAttrib4iv: Option<PFNGLVERTEXATTRIB4IVPROC>;
    static glad_glVertexAttrib4ivARB: Option<PFNGLVERTEXATTRIB4IVARBPROC>;
    static glad_glVertexAttrib4s: Option<PFNGLVERTEXATTRIB4SPROC>;
    static glad_glVertexAttrib4sARB: Option<PFNGLVERTEXATTRIB4SARBPROC>;
    static glad_glVertexAttrib4sv: Option<PFNGLVERTEXATTRIB4SVPROC>;
    static glad_glVertexAttrib4svARB: Option<PFNGLVERTEXATTRIB4SVARBPROC>;
    static glad_glVertexAttrib4ubv: Option<PFNGLVERTEXATTRIB4UBVPROC>;
    static glad_glVertexAttrib4ubvARB: Option<PFNGLVERTEXATTRIB4UBVARBPROC>;
    static glad_glVertexAttrib4uiv: Option<PFNGLVERTEXATTRIB4UIVPROC>;
    static glad_glVertexAttrib4uivARB: Option<PFNGLVERTEXATTRIB4UIVARBPROC>;
    static glad_glVertexAttrib4usv: Option<PFNGLVERTEXATTRIB4USVPROC>;
    static glad_glVertexAttrib4usvARB: Option<PFNGLVERTEXATTRIB4USVARBPROC>;
    static glad_glVertexAttribBinding: Option<PFNGLVERTEXATTRIBBINDINGPROC>;
    static glad_glVertexAttribDivisor: Option<PFNGLVERTEXATTRIBDIVISORPROC>;
    static glad_glVertexAttribDivisorARB: Option<PFNGLVERTEXATTRIBDIVISORARBPROC>;
    static glad_glVertexAttribFormat: Option<PFNGLVERTEXATTRIBFORMATPROC>;
    static glad_glVertexAttribI1i: Option<PFNGLVERTEXATTRIBI1IPROC>;
    static glad_glVertexAttribI1iv: Option<PFNGLVERTEXATTRIBI1IVPROC>;
    static glad_glVertexAttribI1ui: Option<PFNGLVERTEXATTRIBI1UIPROC>;
    static glad_glVertexAttribI1uiv: Option<PFNGLVERTEXATTRIBI1UIVPROC>;
    static glad_glVertexAttribI2i: Option<PFNGLVERTEXATTRIBI2IPROC>;
    static glad_glVertexAttribI2iv: Option<PFNGLVERTEXATTRIBI2IVPROC>;
    static glad_glVertexAttribI2ui: Option<PFNGLVERTEXATTRIBI2UIPROC>;
    static glad_glVertexAttribI2uiv: Option<PFNGLVERTEXATTRIBI2UIVPROC>;
    static glad_glVertexAttribI3i: Option<PFNGLVERTEXATTRIBI3IPROC>;
    static glad_glVertexAttribI3iv: Option<PFNGLVERTEXATTRIBI3IVPROC>;
    static glad_glVertexAttribI3ui: Option<PFNGLVERTEXATTRIBI3UIPROC>;
    static glad_glVertexAttribI3uiv: Option<PFNGLVERTEXATTRIBI3UIVPROC>;
    static glad_glVertexAttribI4bv: Option<PFNGLVERTEXATTRIBI4BVPROC>;
    static glad_glVertexAttribI4i: Option<PFNGLVERTEXATTRIBI4IPROC>;
    static glad_glVertexAttribI4iv: Option<PFNGLVERTEXATTRIBI4IVPROC>;
    static glad_glVertexAttribI4sv: Option<PFNGLVERTEXATTRIBI4SVPROC>;
    static glad_glVertexAttribI4ubv: Option<PFNGLVERTEXATTRIBI4UBVPROC>;
    static glad_glVertexAttribI4ui: Option<PFNGLVERTEXATTRIBI4UIPROC>;
    static glad_glVertexAttribI4uiv: Option<PFNGLVERTEXATTRIBI4UIVPROC>;
    static glad_glVertexAttribI4usv: Option<PFNGLVERTEXATTRIBI4USVPROC>;
    static glad_glVertexAttribIFormat: Option<PFNGLVERTEXATTRIBIFORMATPROC>;
    static glad_glVertexAttribIPointer: Option<PFNGLVERTEXATTRIBIPOINTERPROC>;
    static glad_glVertexAttribL1d: Option<PFNGLVERTEXATTRIBL1DPROC>;
    static glad_glVertexAttribL1dv: Option<PFNGLVERTEXATTRIBL1DVPROC>;
    static glad_glVertexAttribL2d: Option<PFNGLVERTEXATTRIBL2DPROC>;
    static glad_glVertexAttribL2dv: Option<PFNGLVERTEXATTRIBL2DVPROC>;
    static glad_glVertexAttribL3d: Option<PFNGLVERTEXATTRIBL3DPROC>;
    static glad_glVertexAttribL3dv: Option<PFNGLVERTEXATTRIBL3DVPROC>;
    static glad_glVertexAttribL4d: Option<PFNGLVERTEXATTRIBL4DPROC>;
    static glad_glVertexAttribL4dv: Option<PFNGLVERTEXATTRIBL4DVPROC>;
    static glad_glVertexAttribLFormat: Option<PFNGLVERTEXATTRIBLFORMATPROC>;
    static glad_glVertexAttribLPointer: Option<PFNGLVERTEXATTRIBLPOINTERPROC>;
    static glad_glVertexAttribP1ui: Option<PFNGLVERTEXATTRIBP1UIPROC>;
    static glad_glVertexAttribP1uiv: Option<PFNGLVERTEXATTRIBP1UIVPROC>;
    static glad_glVertexAttribP2ui: Option<PFNGLVERTEXATTRIBP2UIPROC>;
    static glad_glVertexAttribP2uiv: Option<PFNGLVERTEXATTRIBP2UIVPROC>;
    static glad_glVertexAttribP3ui: Option<PFNGLVERTEXATTRIBP3UIPROC>;
    static glad_glVertexAttribP3uiv: Option<PFNGLVERTEXATTRIBP3UIVPROC>;
    static glad_glVertexAttribP4ui: Option<PFNGLVERTEXATTRIBP4UIPROC>;
    static glad_glVertexAttribP4uiv: Option<PFNGLVERTEXATTRIBP4UIVPROC>;
    static glad_glVertexAttribPointer: Option<PFNGLVERTEXATTRIBPOINTERPROC>;
    static glad_glVertexAttribPointerARB: Option<PFNGLVERTEXATTRIBPOINTERARBPROC>;
    static glad_glVertexBindingDivisor: Option<PFNGLVERTEXBINDINGDIVISORPROC>;
    static glad_glViewport: Option<PFNGLVIEWPORTPROC>;
    static glad_glViewportArrayv: Option<PFNGLVIEWPORTARRAYVPROC>;
    static glad_glViewportIndexedf: Option<PFNGLVIEWPORTINDEXEDFPROC>;
    static glad_glViewportIndexedfv: Option<PFNGLVIEWPORTINDEXEDFVPROC>;
    static glad_glWaitSync: Option<PFNGLWAITSYNCPROC>;
}

pub const GLAD_GL_IS_SOME_NEW_VERSION: bool = true;

unsafe fn glad_gl_has_extension(version: c_int, exts: &Option<&'static CStr>, exts_i: &Option<Box<[Option<CString>]>>, ext: &CStr) -> bool {
    unsafe {
        if GLAD_VERSION_MAJOR!(version) < 3 || !GLAD_GL_IS_SOME_NEW_VERSION {
            if let Some(mut extensions) = *exts {
                loop {
                    let ptr = libc::strstr(extensions.as_ptr(), ext.as_ptr());
                    if !ptr.is_null() {
                        let loc = CStr::from_ptr(ptr);
                        let terminator = CStr::from_ptr(loc.as_ptr().add(ext.count_bytes()));
                        if (loc.as_ptr() == extensions.as_ptr() || *loc.as_ptr().sub(1) == const{b' ' as c_char}) &&
                            (*terminator.as_ptr() == const{b' ' as c_char} || *terminator.as_ptr() == const{b'\0' as c_char}) {
                            break true;
                        }
                        extensions = terminator;
                    } else {
                        break false;
                    }
                }
            } else {
                false
            }
        } else {
            exts_i.as_ref().is_some_and(|exts_i| exts_i.iter().any(|e| e.as_ref().is_some_and(|e| e.as_c_str() == ext)))
        }
    }
}

unsafe fn glad_gl_get_extensions(version: c_int) -> Result<(Option<&'static CStr>, c_uint, Option<Box<[Option<CString>]>>), ()> {
    unsafe {
        if GLAD_VERSION_MAJOR!(version) < 3 {
            if let Some(glGetString) = glad_glGetString {
                let ptr: *const c_char = glGetString(GL_EXTENSIONS).cast();
                Ok((
                    (!ptr.is_null()).then(|| CStr::from_ptr(ptr)),
                    0,
                    None,
                ))
            } else {
                Err(())
            }
        } else {
            if let (Some(glGetStringi), Some(glGetIntegerv)) = (glad_glGetStringi, glad_glGetIntegerv) {
                let mut num_exts_i: c_uint = 0;
                glGetIntegerv(GL_NUM_EXTENSIONS, &raw mut num_exts_i as *mut c_int);
                if num_exts_i > 0 {
                    let mut exts_i_arr: Box<[Option<CString>]> = std::iter::from_fn(|| None).take(num_exts_i as usize).collect();

                    for index in 0..num_exts_i {
                        let gl_str_tmp: *const c_char = glGetStringi(GL_EXTENSIONS, index).cast();
                        let local_str = (!gl_str_tmp.is_null())
                            .then(|| CStr::from_ptr::<'static>(gl_str_tmp))
                            .and_then(|cstr| CString::from_vec_with_nul(cstr.to_bytes_with_nul().iter().copied().collect()).ok());
                        exts_i_arr[index as usize] = local_str;
                    }

                    Ok((
                        None,
                        num_exts_i,
                        Some(exts_i_arr),
                    ))
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        }
    }
}

unsafe fn glad_gl_find_extensions_gl(version: c_int) -> Result<GladCompat, ()> {
    unsafe {
        if let Ok((exts, _num_exts_i, exts_i)) = glad_gl_get_extensions(version) {
            Ok(GladCompat {
                GL_VERSION_1_0:                          false,
                GL_VERSION_1_1:                          false,
                GL_VERSION_1_2:                          false,
                GL_VERSION_1_3:                          false,
                GL_VERSION_1_4:                          false,
                GL_VERSION_1_5:                          false,
                GL_VERSION_2_0:                          false,
                GL_VERSION_2_1:                          false,
                GL_VERSION_3_0:                          false,
                GL_VERSION_3_1:                          false,
                GL_VERSION_3_2:                          false,
                GL_VERSION_3_3:                          false,
                GL_VERSION_4_0:                          false,
                GL_VERSION_4_1:                          false,
                GL_VERSION_4_2:                          false,
                GL_VERSION_4_3:                          false,
                GL_ARB_ES2_compatibility:                glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_ES2_compatibility"               ),
                GL_ARB_ES3_1_compatibility:              glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_ES3_1_compatibility"             ),
                GL_ARB_ES3_2_compatibility:              glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_ES3_2_compatibility"             ),
                GL_ARB_ES3_compatibility:                glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_ES3_compatibility"               ),
                GL_ARB_blend_func_extended:              glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_blend_func_extended"             ),
                GL_ARB_buffer_storage:                   glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_buffer_storage"                  ),
                GL_ARB_clear_buffer_object:              glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_clear_buffer_object"             ),
                GL_ARB_clear_texture:                    glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_clear_texture"                   ),
                GL_ARB_color_buffer_float:               glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_color_buffer_float"              ),
                GL_ARB_compatibility:                    glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_compatibility"                   ),
                GL_ARB_compressed_texture_pixel_storage: glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_compressed_texture_pixel_storage"),
                GL_ARB_compute_shader:                   glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_compute_shader"                  ),
                GL_ARB_compute_variable_group_size:      glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_compute_variable_group_size"     ),
                GL_ARB_copy_buffer:                      glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_copy_buffer"                     ),
                GL_ARB_copy_image:                       glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_copy_image"                      ),
                GL_ARB_debug_output:                     glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_debug_output"                    ),
                GL_ARB_depth_buffer_float:               glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_depth_buffer_float"              ),
                GL_ARB_depth_clamp:                      glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_depth_clamp"                     ),
                GL_ARB_depth_texture:                    glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_depth_texture"                   ),
                GL_ARB_direct_state_access:              glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_direct_state_access"             ),
                GL_ARB_draw_buffers:                     glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_draw_buffers"                    ),
                GL_ARB_draw_buffers_blend:               glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_draw_buffers_blend"              ),
                GL_ARB_draw_elements_base_vertex:        glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_draw_elements_base_vertex"       ),
                GL_ARB_draw_indirect:                    glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_draw_indirect"                   ),
                GL_ARB_draw_instanced:                   glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_draw_instanced"                  ),
                GL_ARB_enhanced_layouts:                 glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_enhanced_layouts"                ),
                GL_ARB_explicit_attrib_location:         glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_explicit_attrib_location"        ),
                GL_ARB_explicit_uniform_location:        glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_explicit_uniform_location"       ),
                GL_ARB_fragment_coord_conventions:       glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_fragment_coord_conventions"      ),
                GL_ARB_fragment_layer_viewport:          glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_fragment_layer_viewport"         ),
                GL_ARB_fragment_program:                 glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_fragment_program"                ),
                GL_ARB_fragment_program_shadow:          glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_fragment_program_shadow"         ),
                GL_ARB_fragment_shader:                  glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_fragment_shader"                 ),
                GL_ARB_fragment_shader_interlock:        glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_fragment_shader_interlock"       ),
                GL_ARB_framebuffer_no_attachments:       glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_framebuffer_no_attachments"      ),
                GL_ARB_framebuffer_object:               glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_framebuffer_object"              ),
                GL_ARB_framebuffer_sRGB:                 glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_framebuffer_sRGB"                ),
                GL_ARB_geometry_shader4:                 glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_geometry_shader4"                ),
                GL_ARB_get_program_binary:               glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_get_program_binary"              ),
                GL_ARB_get_texture_sub_image:            glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_get_texture_sub_image"           ),
                GL_ARB_gl_spirv:                         glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_gl_spirv"                        ),
                GL_ARB_gpu_shader5:                      glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_gpu_shader5"                     ),
                GL_ARB_gpu_shader_fp64:                  glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_gpu_shader_fp64"                 ),
                GL_ARB_gpu_shader_int64:                 glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_gpu_shader_int64"                ),
                GL_ARB_half_float_pixel:                 glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_half_float_pixel"                ),
                GL_ARB_half_float_vertex:                glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_half_float_vertex"               ),
                GL_ARB_instanced_arrays:                 glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_instanced_arrays"                ),
                GL_ARB_internalformat_query:             glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_internalformat_query"            ),
                GL_ARB_internalformat_query2:            glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_internalformat_query2"           ),
                GL_ARB_map_buffer_range:                 glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_map_buffer_range"                ),
                GL_ARB_multi_bind:                       glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_multi_bind"                      ),
                GL_ARB_multi_draw_indirect:              glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_multi_draw_indirect"             ),
                GL_ARB_multisample:                      glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_multisample"                     ),
                GL_ARB_multitexture:                     glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_multitexture"                    ),
                GL_ARB_occlusion_query:                  glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_occlusion_query"                 ),
                GL_ARB_occlusion_query2:                 glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_occlusion_query2"                ),
                GL_ARB_pipeline_statistics_query:        glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_pipeline_statistics_query"       ),
                GL_ARB_query_buffer_object:              glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_query_buffer_object"             ),
                GL_ARB_sample_locations:                 glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_sample_locations"                ),
                GL_ARB_sample_shading:                   glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_sample_shading"                  ),
                GL_ARB_seamless_cube_map:                glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_seamless_cube_map"               ),
                GL_ARB_seamless_cubemap_per_texture:     glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_seamless_cubemap_per_texture"    ),
                GL_ARB_shader_atomic_counter_ops:        glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_shader_atomic_counter_ops"       ),
                GL_ARB_shader_atomic_counters:           glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_shader_atomic_counters"          ),
                GL_ARB_shader_bit_encoding:              glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_shader_bit_encoding"             ),
                GL_ARB_shader_clock:                     glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_shader_clock"                    ),
                GL_ARB_shader_image_load_store:          glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_shader_image_load_store"         ),
                GL_ARB_shader_image_size:                glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_shader_image_size"               ),
                GL_ARB_shader_objects:                   glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_shader_objects"                  ),
                GL_ARB_shader_storage_buffer_object:     glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_shader_storage_buffer_object"    ),
                GL_ARB_shader_texture_lod:               glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_shader_texture_lod"              ),
                GL_ARB_shading_language_100:             glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_shading_language_100"            ),
                GL_ARB_shading_language_420pack:         glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_shading_language_420pack"        ),
                GL_ARB_shading_language_include:         glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_shading_language_include"        ),
                GL_ARB_shading_language_packing:         glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_shading_language_packing"        ),
                GL_ARB_spirv_extensions:                 glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_spirv_extensions"                ),
                GL_ARB_tessellation_shader:              glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_tessellation_shader"             ),
                GL_ARB_texture_border_clamp:             glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_border_clamp"            ),
                GL_ARB_texture_buffer_object_rgb32:      glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_buffer_object_rgb32"     ),
                GL_ARB_texture_compression:              glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_compression"             ),
                GL_ARB_texture_cube_map:                 glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_cube_map"                ),
                GL_ARB_texture_cube_map_array:           glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_cube_map_array"          ),
                GL_ARB_texture_env_add:                  glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_env_add"                 ),
                GL_ARB_texture_filter_anisotropic:       glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_filter_anisotropic"      ),
                GL_ARB_texture_filter_minmax:            glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_filter_minmax"           ),
                GL_ARB_texture_float:                    glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_float"                   ),
                GL_ARB_texture_mirror_clamp_to_edge:     glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_mirror_clamp_to_edge"    ),
                GL_ARB_texture_mirrored_repeat:          glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_mirrored_repeat"         ),
                GL_ARB_texture_multisample:              glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_multisample"             ),
                GL_ARB_texture_non_power_of_two:         glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_non_power_of_two"        ),
                GL_ARB_texture_rg:                       glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_rg"                      ),
                GL_ARB_texture_storage:                  glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_storage"                 ),
                GL_ARB_texture_swizzle:                  glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_swizzle"                 ),
                GL_ARB_texture_view:                     glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_texture_view"                    ),
                GL_ARB_timer_query:                      glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_timer_query"                     ),
                GL_ARB_transpose_matrix:                 glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_transpose_matrix"                ),
                GL_ARB_uniform_buffer_object:            glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_uniform_buffer_object"           ),
                GL_ARB_vertex_array_bgra:                glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_vertex_array_bgra"               ),
                GL_ARB_vertex_array_object:              glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_vertex_array_object"             ),
                GL_ARB_vertex_attrib_binding:            glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_vertex_attrib_binding"           ),
                GL_ARB_vertex_buffer_object:             glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_vertex_buffer_object"            ),
                GL_ARB_vertex_program:                   glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_vertex_program"                  ),
                GL_ARB_vertex_shader:                    glad_gl_has_extension(version, &exts, &exts_i, c"GL_ARB_vertex_shader"                   ),
                GL_EXT_draw_instanced:                   glad_gl_has_extension(version, &exts, &exts_i, c"GL_EXT_draw_instanced"                  ),
                GL_EXT_fog_coord:                        glad_gl_has_extension(version, &exts, &exts_i, c"GL_EXT_fog_coord"                       ),
                GL_EXT_framebuffer_blit:                 glad_gl_has_extension(version, &exts, &exts_i, c"GL_EXT_framebuffer_blit"                ),
                GL_EXT_framebuffer_multisample:          glad_gl_has_extension(version, &exts, &exts_i, c"GL_EXT_framebuffer_multisample"         ),
                GL_EXT_framebuffer_object:               glad_gl_has_extension(version, &exts, &exts_i, c"GL_EXT_framebuffer_object"              ),
                GL_EXT_framebuffer_sRGB:                 glad_gl_has_extension(version, &exts, &exts_i, c"GL_EXT_framebuffer_sRGB"                ),
                GL_EXT_texture_compression_s3tc:         glad_gl_has_extension(version, &exts, &exts_i, c"GL_EXT_texture_compression_s3tc"        ),
                GL_EXT_texture_filter_anisotropic:       glad_gl_has_extension(version, &exts, &exts_i, c"GL_EXT_texture_filter_anisotropic"      ),
                GL_EXT_texture_mirror_clamp:             glad_gl_has_extension(version, &exts, &exts_i, c"GL_EXT_texture_mirror_clamp"            ),
                GL_KHR_texture_compression_astc_hdr:     glad_gl_has_extension(version, &exts, &exts_i, c"GL_KHR_texture_compression_astc_hdr"    ),
                GL_KHR_texture_compression_astc_ldr:     glad_gl_has_extension(version, &exts, &exts_i, c"GL_KHR_texture_compression_astc_ldr"    ),
                GL_OES_compressed_paletted_texture:      glad_gl_has_extension(version, &exts, &exts_i, c"GL_OES_compressed_paletted_texture"     ),
                GL_OES_fixed_point:                      glad_gl_has_extension(version, &exts, &exts_i, c"GL_OES_fixed_point"                     ),
            })
        } else {
            Err(())
        }
    }
}

#[derive(Default)]
pub struct Glad {
    glAccumxOES:                                   Option<PFNGLACCUMXOESPROC>,
    glActiveShaderProgram:                         Option<PFNGLACTIVESHADERPROGRAMPROC>,
    glActiveTexture:                               Option<PFNGLACTIVETEXTUREPROC>,
    glActiveTextureARB:                            Option<PFNGLACTIVETEXTUREARBPROC>,
    glAlphaFuncxOES:                               Option<PFNGLALPHAFUNCXOESPROC>,
    glAttachObjectARB:                             Option<PFNGLATTACHOBJECTARBPROC>,
    glAttachShader:                                Option<PFNGLATTACHSHADERPROC>,
    glBeginConditionalRender:                      Option<PFNGLBEGINCONDITIONALRENDERPROC>,
    glBeginQuery:                                  Option<PFNGLBEGINQUERYPROC>,
    glBeginQueryARB:                               Option<PFNGLBEGINQUERYARBPROC>,
    glBeginQueryIndexed:                           Option<PFNGLBEGINQUERYINDEXEDPROC>,
    glBeginTransformFeedback:                      Option<PFNGLBEGINTRANSFORMFEEDBACKPROC>,
    glBindAttribLocation:                          Option<PFNGLBINDATTRIBLOCATIONPROC>,
    glBindAttribLocationARB:                       Option<PFNGLBINDATTRIBLOCATIONARBPROC>,
    glBindBuffer:                                  Option<PFNGLBINDBUFFERPROC>,
    glBindBufferARB:                               Option<PFNGLBINDBUFFERARBPROC>,
    glBindBufferBase:                              Option<PFNGLBINDBUFFERBASEPROC>,
    glBindBufferRange:                             Option<PFNGLBINDBUFFERRANGEPROC>,
    glBindBuffersBase:                             Option<PFNGLBINDBUFFERSBASEPROC>,
    glBindBuffersRange:                            Option<PFNGLBINDBUFFERSRANGEPROC>,
    glBindFragDataLocation:                        Option<PFNGLBINDFRAGDATALOCATIONPROC>,
    glBindFragDataLocationIndexed:                 Option<PFNGLBINDFRAGDATALOCATIONINDEXEDPROC>,
    glBindFramebuffer:                             Option<PFNGLBINDFRAMEBUFFERPROC>,
    glBindFramebufferEXT:                          Option<PFNGLBINDFRAMEBUFFEREXTPROC>,
    glBindImageTexture:                            Option<PFNGLBINDIMAGETEXTUREPROC>,
    glBindImageTextures:                           Option<PFNGLBINDIMAGETEXTURESPROC>,
    glBindProgramARB:                              Option<PFNGLBINDPROGRAMARBPROC>,
    glBindProgramPipeline:                         Option<PFNGLBINDPROGRAMPIPELINEPROC>,
    glBindRenderbuffer:                            Option<PFNGLBINDRENDERBUFFERPROC>,
    glBindRenderbufferEXT:                         Option<PFNGLBINDRENDERBUFFEREXTPROC>,
    glBindSampler:                                 Option<PFNGLBINDSAMPLERPROC>,
    glBindSamplers:                                Option<PFNGLBINDSAMPLERSPROC>,
    glBindTexture:                                 Option<PFNGLBINDTEXTUREPROC>,
    glBindTextureUnit:                             Option<PFNGLBINDTEXTUREUNITPROC>,
    glBindTextures:                                Option<PFNGLBINDTEXTURESPROC>,
    glBindTransformFeedback:                       Option<PFNGLBINDTRANSFORMFEEDBACKPROC>,
    glBindVertexArray:                             Option<PFNGLBINDVERTEXARRAYPROC>,
    glBindVertexBuffer:                            Option<PFNGLBINDVERTEXBUFFERPROC>,
    glBindVertexBuffers:                           Option<PFNGLBINDVERTEXBUFFERSPROC>,
    glBitmapxOES:                                  Option<PFNGLBITMAPXOESPROC>,
    glBlendColor:                                  Option<PFNGLBLENDCOLORPROC>,
    glBlendColorxOES:                              Option<PFNGLBLENDCOLORXOESPROC>,
    glBlendEquation:                               Option<PFNGLBLENDEQUATIONPROC>,
    glBlendEquationSeparate:                       Option<PFNGLBLENDEQUATIONSEPARATEPROC>,
    glBlendEquationSeparatei:                      Option<PFNGLBLENDEQUATIONSEPARATEIPROC>,
    glBlendEquationSeparateiARB:                   Option<PFNGLBLENDEQUATIONSEPARATEIARBPROC>,
    glBlendEquationi:                              Option<PFNGLBLENDEQUATIONIPROC>,
    glBlendEquationiARB:                           Option<PFNGLBLENDEQUATIONIARBPROC>,
    glBlendFunc:                                   Option<PFNGLBLENDFUNCPROC>,
    glBlendFuncSeparate:                           Option<PFNGLBLENDFUNCSEPARATEPROC>,
    glBlendFuncSeparatei:                          Option<PFNGLBLENDFUNCSEPARATEIPROC>,
    glBlendFuncSeparateiARB:                       Option<PFNGLBLENDFUNCSEPARATEIARBPROC>,
    glBlendFunci:                                  Option<PFNGLBLENDFUNCIPROC>,
    glBlendFunciARB:                               Option<PFNGLBLENDFUNCIARBPROC>,
    glBlitFramebuffer:                             Option<PFNGLBLITFRAMEBUFFERPROC>,
    glBlitFramebufferEXT:                          Option<PFNGLBLITFRAMEBUFFEREXTPROC>,
    glBlitNamedFramebuffer:                        Option<PFNGLBLITNAMEDFRAMEBUFFERPROC>,
    glBufferData:                                  Option<PFNGLBUFFERDATAPROC>,
    glBufferDataARB:                               Option<PFNGLBUFFERDATAARBPROC>,
    glBufferStorage:                               Option<PFNGLBUFFERSTORAGEPROC>,
    glBufferSubData:                               Option<PFNGLBUFFERSUBDATAPROC>,
    glBufferSubDataARB:                            Option<PFNGLBUFFERSUBDATAARBPROC>,
    glCheckFramebufferStatus:                      Option<PFNGLCHECKFRAMEBUFFERSTATUSPROC>,
    glCheckFramebufferStatusEXT:                   Option<PFNGLCHECKFRAMEBUFFERSTATUSEXTPROC>,
    glCheckNamedFramebufferStatus:                 Option<PFNGLCHECKNAMEDFRAMEBUFFERSTATUSPROC>,
    glClampColor:                                  Option<PFNGLCLAMPCOLORPROC>,
    glClampColorARB:                               Option<PFNGLCLAMPCOLORARBPROC>,
    glClear:                                       Option<PFNGLCLEARPROC>,
    glClearAccumxOES:                              Option<PFNGLCLEARACCUMXOESPROC>,
    glClearBufferData:                             Option<PFNGLCLEARBUFFERDATAPROC>,
    glClearBufferSubData:                          Option<PFNGLCLEARBUFFERSUBDATAPROC>,
    glClearBufferfi:                               Option<PFNGLCLEARBUFFERFIPROC>,
    glClearBufferfv:                               Option<PFNGLCLEARBUFFERFVPROC>,
    glClearBufferiv:                               Option<PFNGLCLEARBUFFERIVPROC>,
    glClearBufferuiv:                              Option<PFNGLCLEARBUFFERUIVPROC>,
    glClearColor:                                  Option<PFNGLCLEARCOLORPROC>,
    glClearColorxOES:                              Option<PFNGLCLEARCOLORXOESPROC>,
    glClearDepth:                                  Option<PFNGLCLEARDEPTHPROC>,
    glClearDepthf:                                 Option<PFNGLCLEARDEPTHFPROC>,
    glClearDepthxOES:                              Option<PFNGLCLEARDEPTHXOESPROC>,
    glClearNamedBufferData:                        Option<PFNGLCLEARNAMEDBUFFERDATAPROC>,
    glClearNamedBufferSubData:                     Option<PFNGLCLEARNAMEDBUFFERSUBDATAPROC>,
    glClearNamedFramebufferfi:                     Option<PFNGLCLEARNAMEDFRAMEBUFFERFIPROC>,
    glClearNamedFramebufferfv:                     Option<PFNGLCLEARNAMEDFRAMEBUFFERFVPROC>,
    glClearNamedFramebufferiv:                     Option<PFNGLCLEARNAMEDFRAMEBUFFERIVPROC>,
    glClearNamedFramebufferuiv:                    Option<PFNGLCLEARNAMEDFRAMEBUFFERUIVPROC>,
    glClearStencil:                                Option<PFNGLCLEARSTENCILPROC>,
    glClearTexImage:                               Option<PFNGLCLEARTEXIMAGEPROC>,
    glClearTexSubImage:                            Option<PFNGLCLEARTEXSUBIMAGEPROC>,
    glClientActiveTextureARB:                      Option<PFNGLCLIENTACTIVETEXTUREARBPROC>,
    glClientWaitSync:                              Option<PFNGLCLIENTWAITSYNCPROC>,
    glClipPlanexOES:                               Option<PFNGLCLIPPLANEXOESPROC>,
    glColor3xOES:                                  Option<PFNGLCOLOR3XOESPROC>,
    glColor3xvOES:                                 Option<PFNGLCOLOR3XVOESPROC>,
    glColor4xOES:                                  Option<PFNGLCOLOR4XOESPROC>,
    glColor4xvOES:                                 Option<PFNGLCOLOR4XVOESPROC>,
    glColorMask:                                   Option<PFNGLCOLORMASKPROC>,
    glColorMaski:                                  Option<PFNGLCOLORMASKIPROC>,
    glCompileShader:                               Option<PFNGLCOMPILESHADERPROC>,
    glCompileShaderARB:                            Option<PFNGLCOMPILESHADERARBPROC>,
    glCompileShaderIncludeARB:                     Option<PFNGLCOMPILESHADERINCLUDEARBPROC>,
    glCompressedTexImage1D:                        Option<PFNGLCOMPRESSEDTEXIMAGE1DPROC>,
    glCompressedTexImage1DARB:                     Option<PFNGLCOMPRESSEDTEXIMAGE1DARBPROC>,
    glCompressedTexImage2D:                        Option<PFNGLCOMPRESSEDTEXIMAGE2DPROC>,
    glCompressedTexImage2DARB:                     Option<PFNGLCOMPRESSEDTEXIMAGE2DARBPROC>,
    glCompressedTexImage3D:                        Option<PFNGLCOMPRESSEDTEXIMAGE3DPROC>,
    glCompressedTexImage3DARB:                     Option<PFNGLCOMPRESSEDTEXIMAGE3DARBPROC>,
    glCompressedTexSubImage1D:                     Option<PFNGLCOMPRESSEDTEXSUBIMAGE1DPROC>,
    glCompressedTexSubImage1DARB:                  Option<PFNGLCOMPRESSEDTEXSUBIMAGE1DARBPROC>,
    glCompressedTexSubImage2D:                     Option<PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC>,
    glCompressedTexSubImage2DARB:                  Option<PFNGLCOMPRESSEDTEXSUBIMAGE2DARBPROC>,
    glCompressedTexSubImage3D:                     Option<PFNGLCOMPRESSEDTEXSUBIMAGE3DPROC>,
    glCompressedTexSubImage3DARB:                  Option<PFNGLCOMPRESSEDTEXSUBIMAGE3DARBPROC>,
    glCompressedTextureSubImage1D:                 Option<PFNGLCOMPRESSEDTEXTURESUBIMAGE1DPROC>,
    glCompressedTextureSubImage2D:                 Option<PFNGLCOMPRESSEDTEXTURESUBIMAGE2DPROC>,
    glCompressedTextureSubImage3D:                 Option<PFNGLCOMPRESSEDTEXTURESUBIMAGE3DPROC>,
    glConvolutionParameterxOES:                    Option<PFNGLCONVOLUTIONPARAMETERXOESPROC>,
    glConvolutionParameterxvOES:                   Option<PFNGLCONVOLUTIONPARAMETERXVOESPROC>,
    glCopyBufferSubData:                           Option<PFNGLCOPYBUFFERSUBDATAPROC>,
    glCopyImageSubData:                            Option<PFNGLCOPYIMAGESUBDATAPROC>,
    glCopyNamedBufferSubData:                      Option<PFNGLCOPYNAMEDBUFFERSUBDATAPROC>,
    glCopyTexImage1D:                              Option<PFNGLCOPYTEXIMAGE1DPROC>,
    glCopyTexImage2D:                              Option<PFNGLCOPYTEXIMAGE2DPROC>,
    glCopyTexSubImage1D:                           Option<PFNGLCOPYTEXSUBIMAGE1DPROC>,
    glCopyTexSubImage2D:                           Option<PFNGLCOPYTEXSUBIMAGE2DPROC>,
    glCopyTexSubImage3D:                           Option<PFNGLCOPYTEXSUBIMAGE3DPROC>,
    glCopyTextureSubImage1D:                       Option<PFNGLCOPYTEXTURESUBIMAGE1DPROC>,
    glCopyTextureSubImage2D:                       Option<PFNGLCOPYTEXTURESUBIMAGE2DPROC>,
    glCopyTextureSubImage3D:                       Option<PFNGLCOPYTEXTURESUBIMAGE3DPROC>,
    glCreateBuffers:                               Option<PFNGLCREATEBUFFERSPROC>,
    glCreateFramebuffers:                          Option<PFNGLCREATEFRAMEBUFFERSPROC>,
    glCreateProgram:                               Option<PFNGLCREATEPROGRAMPROC>,
    glCreateProgramObjectARB:                      Option<PFNGLCREATEPROGRAMOBJECTARBPROC>,
    glCreateProgramPipelines:                      Option<PFNGLCREATEPROGRAMPIPELINESPROC>,
    glCreateQueries:                               Option<PFNGLCREATEQUERIESPROC>,
    glCreateRenderbuffers:                         Option<PFNGLCREATERENDERBUFFERSPROC>,
    glCreateSamplers:                              Option<PFNGLCREATESAMPLERSPROC>,
    glCreateShader:                                Option<PFNGLCREATESHADERPROC>,
    glCreateShaderObjectARB:                       Option<PFNGLCREATESHADEROBJECTARBPROC>,
    glCreateShaderProgramv:                        Option<PFNGLCREATESHADERPROGRAMVPROC>,
    glCreateTextures:                              Option<PFNGLCREATETEXTURESPROC>,
    glCreateTransformFeedbacks:                    Option<PFNGLCREATETRANSFORMFEEDBACKSPROC>,
    glCreateVertexArrays:                          Option<PFNGLCREATEVERTEXARRAYSPROC>,
    glCullFace:                                    Option<PFNGLCULLFACEPROC>,
    glDebugMessageCallback:                        Option<PFNGLDEBUGMESSAGECALLBACKPROC>,
    glDebugMessageCallbackARB:                     Option<PFNGLDEBUGMESSAGECALLBACKARBPROC>,
    glDebugMessageControl:                         Option<PFNGLDEBUGMESSAGECONTROLPROC>,
    glDebugMessageControlARB:                      Option<PFNGLDEBUGMESSAGECONTROLARBPROC>,
    glDebugMessageInsert:                          Option<PFNGLDEBUGMESSAGEINSERTPROC>,
    glDebugMessageInsertARB:                       Option<PFNGLDEBUGMESSAGEINSERTARBPROC>,
    glDeleteBuffers:                               Option<PFNGLDELETEBUFFERSPROC>,
    glDeleteBuffersARB:                            Option<PFNGLDELETEBUFFERSARBPROC>,
    glDeleteFramebuffers:                          Option<PFNGLDELETEFRAMEBUFFERSPROC>,
    glDeleteFramebuffersEXT:                       Option<PFNGLDELETEFRAMEBUFFERSEXTPROC>,
    glDeleteNamedStringARB:                        Option<PFNGLDELETENAMEDSTRINGARBPROC>,
    glDeleteObjectARB:                             Option<PFNGLDELETEOBJECTARBPROC>,
    glDeleteProgram:                               Option<PFNGLDELETEPROGRAMPROC>,
    glDeleteProgramPipelines:                      Option<PFNGLDELETEPROGRAMPIPELINESPROC>,
    glDeleteProgramsARB:                           Option<PFNGLDELETEPROGRAMSARBPROC>,
    glDeleteQueries:                               Option<PFNGLDELETEQUERIESPROC>,
    glDeleteQueriesARB:                            Option<PFNGLDELETEQUERIESARBPROC>,
    glDeleteRenderbuffers:                         Option<PFNGLDELETERENDERBUFFERSPROC>,
    glDeleteRenderbuffersEXT:                      Option<PFNGLDELETERENDERBUFFERSEXTPROC>,
    glDeleteSamplers:                              Option<PFNGLDELETESAMPLERSPROC>,
    glDeleteShader:                                Option<PFNGLDELETESHADERPROC>,
    glDeleteSync:                                  Option<PFNGLDELETESYNCPROC>,
    glDeleteTextures:                              Option<PFNGLDELETETEXTURESPROC>,
    glDeleteTransformFeedbacks:                    Option<PFNGLDELETETRANSFORMFEEDBACKSPROC>,
    glDeleteVertexArrays:                          Option<PFNGLDELETEVERTEXARRAYSPROC>,
    glDepthFunc:                                   Option<PFNGLDEPTHFUNCPROC>,
    glDepthMask:                                   Option<PFNGLDEPTHMASKPROC>,
    glDepthRange:                                  Option<PFNGLDEPTHRANGEPROC>,
    glDepthRangeArrayv:                            Option<PFNGLDEPTHRANGEARRAYVPROC>,
    glDepthRangeIndexed:                           Option<PFNGLDEPTHRANGEINDEXEDPROC>,
    glDepthRangef:                                 Option<PFNGLDEPTHRANGEFPROC>,
    glDepthRangexOES:                              Option<PFNGLDEPTHRANGEXOESPROC>,
    glDetachObjectARB:                             Option<PFNGLDETACHOBJECTARBPROC>,
    glDetachShader:                                Option<PFNGLDETACHSHADERPROC>,
    glDisable:                                     Option<PFNGLDISABLEPROC>,
    glDisableVertexArrayAttrib:                    Option<PFNGLDISABLEVERTEXARRAYATTRIBPROC>,
    glDisableVertexAttribArray:                    Option<PFNGLDISABLEVERTEXATTRIBARRAYPROC>,
    glDisableVertexAttribArrayARB:                 Option<PFNGLDISABLEVERTEXATTRIBARRAYARBPROC>,
    glDisablei:                                    Option<PFNGLDISABLEIPROC>,
    glDispatchCompute:                             Option<PFNGLDISPATCHCOMPUTEPROC>,
    glDispatchComputeGroupSizeARB:                 Option<PFNGLDISPATCHCOMPUTEGROUPSIZEARBPROC>,
    glDispatchComputeIndirect:                     Option<PFNGLDISPATCHCOMPUTEINDIRECTPROC>,
    glDrawArrays:                                  Option<PFNGLDRAWARRAYSPROC>,
    glDrawArraysIndirect:                          Option<PFNGLDRAWARRAYSINDIRECTPROC>,
    glDrawArraysInstanced:                         Option<PFNGLDRAWARRAYSINSTANCEDPROC>,
    glDrawArraysInstancedARB:                      Option<PFNGLDRAWARRAYSINSTANCEDARBPROC>,
    glDrawArraysInstancedBaseInstance:             Option<PFNGLDRAWARRAYSINSTANCEDBASEINSTANCEPROC>,
    glDrawArraysInstancedEXT:                      Option<PFNGLDRAWARRAYSINSTANCEDEXTPROC>,
    glDrawBuffer:                                  Option<PFNGLDRAWBUFFERPROC>,
    glDrawBuffers:                                 Option<PFNGLDRAWBUFFERSPROC>,
    glDrawBuffersARB:                              Option<PFNGLDRAWBUFFERSARBPROC>,
    glDrawElements:                                Option<PFNGLDRAWELEMENTSPROC>,
    glDrawElementsBaseVertex:                      Option<PFNGLDRAWELEMENTSBASEVERTEXPROC>,
    glDrawElementsIndirect:                        Option<PFNGLDRAWELEMENTSINDIRECTPROC>,
    glDrawElementsInstanced:                       Option<PFNGLDRAWELEMENTSINSTANCEDPROC>,
    glDrawElementsInstancedARB:                    Option<PFNGLDRAWELEMENTSINSTANCEDARBPROC>,
    glDrawElementsInstancedBaseInstance:           Option<PFNGLDRAWELEMENTSINSTANCEDBASEINSTANCEPROC>,
    glDrawElementsInstancedBaseVertex:             Option<PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC>,
    glDrawElementsInstancedBaseVertexBaseInstance: Option<PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXBASEINSTANCEPROC>,
    glDrawElementsInstancedEXT:                    Option<PFNGLDRAWELEMENTSINSTANCEDEXTPROC>,
    glDrawRangeElements:                           Option<PFNGLDRAWRANGEELEMENTSPROC>,
    glDrawRangeElementsBaseVertex:                 Option<PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC>,
    glDrawTransformFeedback:                       Option<PFNGLDRAWTRANSFORMFEEDBACKPROC>,
    glDrawTransformFeedbackInstanced:              Option<PFNGLDRAWTRANSFORMFEEDBACKINSTANCEDPROC>,
    glDrawTransformFeedbackStream:                 Option<PFNGLDRAWTRANSFORMFEEDBACKSTREAMPROC>,
    glDrawTransformFeedbackStreamInstanced:        Option<PFNGLDRAWTRANSFORMFEEDBACKSTREAMINSTANCEDPROC>,
    glEnable:                                      Option<PFNGLENABLEPROC>,
    glEnableVertexArrayAttrib:                     Option<PFNGLENABLEVERTEXARRAYATTRIBPROC>,
    glEnableVertexAttribArray:                     Option<PFNGLENABLEVERTEXATTRIBARRAYPROC>,
    glEnableVertexAttribArrayARB:                  Option<PFNGLENABLEVERTEXATTRIBARRAYARBPROC>,
    glEnablei:                                     Option<PFNGLENABLEIPROC>,
    glEndConditionalRender:                        Option<PFNGLENDCONDITIONALRENDERPROC>,
    glEndQuery:                                    Option<PFNGLENDQUERYPROC>,
    glEndQueryARB:                                 Option<PFNGLENDQUERYARBPROC>,
    glEndQueryIndexed:                             Option<PFNGLENDQUERYINDEXEDPROC>,
    glEndTransformFeedback:                        Option<PFNGLENDTRANSFORMFEEDBACKPROC>,
    glEvalCoord1xOES:                              Option<PFNGLEVALCOORD1XOESPROC>,
    glEvalCoord1xvOES:                             Option<PFNGLEVALCOORD1XVOESPROC>,
    glEvalCoord2xOES:                              Option<PFNGLEVALCOORD2XOESPROC>,
    glEvalCoord2xvOES:                             Option<PFNGLEVALCOORD2XVOESPROC>,
    glEvaluateDepthValuesARB:                      Option<PFNGLEVALUATEDEPTHVALUESARBPROC>,
    glFeedbackBufferxOES:                          Option<PFNGLFEEDBACKBUFFERXOESPROC>,
    glFenceSync:                                   Option<PFNGLFENCESYNCPROC>,
    glFinish:                                      Option<PFNGLFINISHPROC>,
    glFlush:                                       Option<PFNGLFLUSHPROC>,
    glFlushMappedBufferRange:                      Option<PFNGLFLUSHMAPPEDBUFFERRANGEPROC>,
    glFlushMappedNamedBufferRange:                 Option<PFNGLFLUSHMAPPEDNAMEDBUFFERRANGEPROC>,
    glFogCoordPointerEXT:                          Option<PFNGLFOGCOORDPOINTEREXTPROC>,
    glFogCoorddEXT:                                Option<PFNGLFOGCOORDDEXTPROC>,
    glFogCoorddvEXT:                               Option<PFNGLFOGCOORDDVEXTPROC>,
    glFogCoordfEXT:                                Option<PFNGLFOGCOORDFEXTPROC>,
    glFogCoordfvEXT:                               Option<PFNGLFOGCOORDFVEXTPROC>,
    glFogxOES:                                     Option<PFNGLFOGXOESPROC>,
    glFogxvOES:                                    Option<PFNGLFOGXVOESPROC>,
    glFramebufferParameteri:                       Option<PFNGLFRAMEBUFFERPARAMETERIPROC>,
    glFramebufferRenderbuffer:                     Option<PFNGLFRAMEBUFFERRENDERBUFFERPROC>,
    glFramebufferRenderbufferEXT:                  Option<PFNGLFRAMEBUFFERRENDERBUFFEREXTPROC>,
    glFramebufferSampleLocationsfvARB:             Option<PFNGLFRAMEBUFFERSAMPLELOCATIONSFVARBPROC>,
    glFramebufferTexture:                          Option<PFNGLFRAMEBUFFERTEXTUREPROC>,
    glFramebufferTexture1D:                        Option<PFNGLFRAMEBUFFERTEXTURE1DPROC>,
    glFramebufferTexture1DEXT:                     Option<PFNGLFRAMEBUFFERTEXTURE1DEXTPROC>,
    glFramebufferTexture2D:                        Option<PFNGLFRAMEBUFFERTEXTURE2DPROC>,
    glFramebufferTexture2DEXT:                     Option<PFNGLFRAMEBUFFERTEXTURE2DEXTPROC>,
    glFramebufferTexture3D:                        Option<PFNGLFRAMEBUFFERTEXTURE3DPROC>,
    glFramebufferTexture3DEXT:                     Option<PFNGLFRAMEBUFFERTEXTURE3DEXTPROC>,
    glFramebufferTextureARB:                       Option<PFNGLFRAMEBUFFERTEXTUREARBPROC>,
    glFramebufferTextureFaceARB:                   Option<PFNGLFRAMEBUFFERTEXTUREFACEARBPROC>,
    glFramebufferTextureLayer:                     Option<PFNGLFRAMEBUFFERTEXTURELAYERPROC>,
    glFramebufferTextureLayerARB:                  Option<PFNGLFRAMEBUFFERTEXTURELAYERARBPROC>,
    glFrontFace:                                   Option<PFNGLFRONTFACEPROC>,
    glFrustumxOES:                                 Option<PFNGLFRUSTUMXOESPROC>,
    glGenBuffers:                                  Option<PFNGLGENBUFFERSPROC>,
    glGenBuffersARB:                               Option<PFNGLGENBUFFERSARBPROC>,
    glGenFramebuffers:                             Option<PFNGLGENFRAMEBUFFERSPROC>,
    glGenFramebuffersEXT:                          Option<PFNGLGENFRAMEBUFFERSEXTPROC>,
    glGenProgramPipelines:                         Option<PFNGLGENPROGRAMPIPELINESPROC>,
    glGenProgramsARB:                              Option<PFNGLGENPROGRAMSARBPROC>,
    glGenQueries:                                  Option<PFNGLGENQUERIESPROC>,
    glGenQueriesARB:                               Option<PFNGLGENQUERIESARBPROC>,
    glGenRenderbuffers:                            Option<PFNGLGENRENDERBUFFERSPROC>,
    glGenRenderbuffersEXT:                         Option<PFNGLGENRENDERBUFFERSEXTPROC>,
    glGenSamplers:                                 Option<PFNGLGENSAMPLERSPROC>,
    glGenTextures:                                 Option<PFNGLGENTEXTURESPROC>,
    glGenTransformFeedbacks:                       Option<PFNGLGENTRANSFORMFEEDBACKSPROC>,
    glGenVertexArrays:                             Option<PFNGLGENVERTEXARRAYSPROC>,
    glGenerateMipmap:                              Option<PFNGLGENERATEMIPMAPPROC>,
    glGenerateMipmapEXT:                           Option<PFNGLGENERATEMIPMAPEXTPROC>,
    glGenerateTextureMipmap:                       Option<PFNGLGENERATETEXTUREMIPMAPPROC>,
    glGetActiveAtomicCounterBufferiv:              Option<PFNGLGETACTIVEATOMICCOUNTERBUFFERIVPROC>,
    glGetActiveAttrib:                             Option<PFNGLGETACTIVEATTRIBPROC>,
    glGetActiveAttribARB:                          Option<PFNGLGETACTIVEATTRIBARBPROC>,
    glGetActiveSubroutineName:                     Option<PFNGLGETACTIVESUBROUTINENAMEPROC>,
    glGetActiveSubroutineUniformName:              Option<PFNGLGETACTIVESUBROUTINEUNIFORMNAMEPROC>,
    glGetActiveSubroutineUniformiv:                Option<PFNGLGETACTIVESUBROUTINEUNIFORMIVPROC>,
    glGetActiveUniform:                            Option<PFNGLGETACTIVEUNIFORMPROC>,
    glGetActiveUniformARB:                         Option<PFNGLGETACTIVEUNIFORMARBPROC>,
    glGetActiveUniformBlockName:                   Option<PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC>,
    glGetActiveUniformBlockiv:                     Option<PFNGLGETACTIVEUNIFORMBLOCKIVPROC>,
    glGetActiveUniformName:                        Option<PFNGLGETACTIVEUNIFORMNAMEPROC>,
    glGetActiveUniformsiv:                         Option<PFNGLGETACTIVEUNIFORMSIVPROC>,
    glGetAttachedObjectsARB:                       Option<PFNGLGETATTACHEDOBJECTSARBPROC>,
    glGetAttachedShaders:                          Option<PFNGLGETATTACHEDSHADERSPROC>,
    glGetAttribLocation:                           Option<PFNGLGETATTRIBLOCATIONPROC>,
    glGetAttribLocationARB:                        Option<PFNGLGETATTRIBLOCATIONARBPROC>,
    glGetBooleani_v:                               Option<PFNGLGETBOOLEANI_VPROC>,
    glGetBooleanv:                                 Option<PFNGLGETBOOLEANVPROC>,
    glGetBufferParameteri64v:                      Option<PFNGLGETBUFFERPARAMETERI64VPROC>,
    glGetBufferParameteriv:                        Option<PFNGLGETBUFFERPARAMETERIVPROC>,
    glGetBufferParameterivARB:                     Option<PFNGLGETBUFFERPARAMETERIVARBPROC>,
    glGetBufferPointerv:                           Option<PFNGLGETBUFFERPOINTERVPROC>,
    glGetBufferPointervARB:                        Option<PFNGLGETBUFFERPOINTERVARBPROC>,
    glGetBufferSubData:                            Option<PFNGLGETBUFFERSUBDATAPROC>,
    glGetBufferSubDataARB:                         Option<PFNGLGETBUFFERSUBDATAARBPROC>,
    glGetClipPlanexOES:                            Option<PFNGLGETCLIPPLANEXOESPROC>,
    glGetCompressedTexImage:                       Option<PFNGLGETCOMPRESSEDTEXIMAGEPROC>,
    glGetCompressedTexImageARB:                    Option<PFNGLGETCOMPRESSEDTEXIMAGEARBPROC>,
    glGetCompressedTextureImage:                   Option<PFNGLGETCOMPRESSEDTEXTUREIMAGEPROC>,
    glGetCompressedTextureSubImage:                Option<PFNGLGETCOMPRESSEDTEXTURESUBIMAGEPROC>,
    glGetConvolutionParameterxvOES:                Option<PFNGLGETCONVOLUTIONPARAMETERXVOESPROC>,
    glGetDebugMessageLog:                          Option<PFNGLGETDEBUGMESSAGELOGPROC>,
    glGetDebugMessageLogARB:                       Option<PFNGLGETDEBUGMESSAGELOGARBPROC>,
    glGetDoublei_v:                                Option<PFNGLGETDOUBLEI_VPROC>,
    glGetDoublev:                                  Option<PFNGLGETDOUBLEVPROC>,
    glGetError:                                    Option<PFNGLGETERRORPROC>,
    glGetFixedvOES:                                Option<PFNGLGETFIXEDVOESPROC>,
    glGetFloati_v:                                 Option<PFNGLGETFLOATI_VPROC>,
    glGetFloatv:                                   Option<PFNGLGETFLOATVPROC>,
    glGetFragDataIndex:                            Option<PFNGLGETFRAGDATAINDEXPROC>,
    glGetFragDataLocation:                         Option<PFNGLGETFRAGDATALOCATIONPROC>,
    glGetFramebufferAttachmentParameteriv:         Option<PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC>,
    glGetFramebufferAttachmentParameterivEXT:      Option<PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVEXTPROC>,
    glGetFramebufferParameteriv:                   Option<PFNGLGETFRAMEBUFFERPARAMETERIVPROC>,
    glGetHandleARB:                                Option<PFNGLGETHANDLEARBPROC>,
    glGetHistogramParameterxvOES:                  Option<PFNGLGETHISTOGRAMPARAMETERXVOESPROC>,
    glGetInfoLogARB:                               Option<PFNGLGETINFOLOGARBPROC>,
    glGetInteger64i_v:                             Option<PFNGLGETINTEGER64I_VPROC>,
    glGetInteger64v:                               Option<PFNGLGETINTEGER64VPROC>,
    glGetIntegeri_v:                               Option<PFNGLGETINTEGERI_VPROC>,
    glGetIntegerv:                                 Option<PFNGLGETINTEGERVPROC>,
    glGetInternalformati64v:                       Option<PFNGLGETINTERNALFORMATI64VPROC>,
    glGetInternalformativ:                         Option<PFNGLGETINTERNALFORMATIVPROC>,
    glGetLightxOES:                                Option<PFNGLGETLIGHTXOESPROC>,
    glGetMapxvOES:                                 Option<PFNGLGETMAPXVOESPROC>,
    glGetMaterialxOES:                             Option<PFNGLGETMATERIALXOESPROC>,
    glGetMultisamplefv:                            Option<PFNGLGETMULTISAMPLEFVPROC>,
    glGetNamedBufferParameteri64v:                 Option<PFNGLGETNAMEDBUFFERPARAMETERI64VPROC>,
    glGetNamedBufferParameteriv:                   Option<PFNGLGETNAMEDBUFFERPARAMETERIVPROC>,
    glGetNamedBufferPointerv:                      Option<PFNGLGETNAMEDBUFFERPOINTERVPROC>,
    glGetNamedBufferSubData:                       Option<PFNGLGETNAMEDBUFFERSUBDATAPROC>,
    glGetNamedFramebufferAttachmentParameteriv:    Option<PFNGLGETNAMEDFRAMEBUFFERATTACHMENTPARAMETERIVPROC>,
    glGetNamedFramebufferParameteriv:              Option<PFNGLGETNAMEDFRAMEBUFFERPARAMETERIVPROC>,
    glGetNamedRenderbufferParameteriv:             Option<PFNGLGETNAMEDRENDERBUFFERPARAMETERIVPROC>,
    glGetNamedStringARB:                           Option<PFNGLGETNAMEDSTRINGARBPROC>,
    glGetNamedStringivARB:                         Option<PFNGLGETNAMEDSTRINGIVARBPROC>,
    glGetObjectLabel:                              Option<PFNGLGETOBJECTLABELPROC>,
    glGetObjectParameterfvARB:                     Option<PFNGLGETOBJECTPARAMETERFVARBPROC>,
    glGetObjectParameterivARB:                     Option<PFNGLGETOBJECTPARAMETERIVARBPROC>,
    glGetObjectPtrLabel:                           Option<PFNGLGETOBJECTPTRLABELPROC>,
    glGetPixelMapxv:                               Option<PFNGLGETPIXELMAPXVPROC>,
    glGetPointerv:                                 Option<PFNGLGETPOINTERVPROC>,
    glGetProgramBinary:                            Option<PFNGLGETPROGRAMBINARYPROC>,
    glGetProgramEnvParameterdvARB:                 Option<PFNGLGETPROGRAMENVPARAMETERDVARBPROC>,
    glGetProgramEnvParameterfvARB:                 Option<PFNGLGETPROGRAMENVPARAMETERFVARBPROC>,
    glGetProgramInfoLog:                           Option<PFNGLGETPROGRAMINFOLOGPROC>,
    glGetProgramInterfaceiv:                       Option<PFNGLGETPROGRAMINTERFACEIVPROC>,
    glGetProgramLocalParameterdvARB:               Option<PFNGLGETPROGRAMLOCALPARAMETERDVARBPROC>,
    glGetProgramLocalParameterfvARB:               Option<PFNGLGETPROGRAMLOCALPARAMETERFVARBPROC>,
    glGetProgramPipelineInfoLog:                   Option<PFNGLGETPROGRAMPIPELINEINFOLOGPROC>,
    glGetProgramPipelineiv:                        Option<PFNGLGETPROGRAMPIPELINEIVPROC>,
    glGetProgramResourceIndex:                     Option<PFNGLGETPROGRAMRESOURCEINDEXPROC>,
    glGetProgramResourceLocation:                  Option<PFNGLGETPROGRAMRESOURCELOCATIONPROC>,
    glGetProgramResourceLocationIndex:             Option<PFNGLGETPROGRAMRESOURCELOCATIONINDEXPROC>,
    glGetProgramResourceName:                      Option<PFNGLGETPROGRAMRESOURCENAMEPROC>,
    glGetProgramResourceiv:                        Option<PFNGLGETPROGRAMRESOURCEIVPROC>,
    glGetProgramStageiv:                           Option<PFNGLGETPROGRAMSTAGEIVPROC>,
    glGetProgramStringARB:                         Option<PFNGLGETPROGRAMSTRINGARBPROC>,
    glGetProgramiv:                                Option<PFNGLGETPROGRAMIVPROC>,
    glGetProgramivARB:                             Option<PFNGLGETPROGRAMIVARBPROC>,
    glGetQueryBufferObjecti64v:                    Option<PFNGLGETQUERYBUFFEROBJECTI64VPROC>,
    glGetQueryBufferObjectiv:                      Option<PFNGLGETQUERYBUFFEROBJECTIVPROC>,
    glGetQueryBufferObjectui64v:                   Option<PFNGLGETQUERYBUFFEROBJECTUI64VPROC>,
    glGetQueryBufferObjectuiv:                     Option<PFNGLGETQUERYBUFFEROBJECTUIVPROC>,
    glGetQueryIndexediv:                           Option<PFNGLGETQUERYINDEXEDIVPROC>,
    glGetQueryObjecti64v:                          Option<PFNGLGETQUERYOBJECTI64VPROC>,
    glGetQueryObjectiv:                            Option<PFNGLGETQUERYOBJECTIVPROC>,
    glGetQueryObjectivARB:                         Option<PFNGLGETQUERYOBJECTIVARBPROC>,
    glGetQueryObjectui64v:                         Option<PFNGLGETQUERYOBJECTUI64VPROC>,
    glGetQueryObjectuiv:                           Option<PFNGLGETQUERYOBJECTUIVPROC>,
    glGetQueryObjectuivARB:                        Option<PFNGLGETQUERYOBJECTUIVARBPROC>,
    glGetQueryiv:                                  Option<PFNGLGETQUERYIVPROC>,
    glGetQueryivARB:                               Option<PFNGLGETQUERYIVARBPROC>,
    glGetRenderbufferParameteriv:                  Option<PFNGLGETRENDERBUFFERPARAMETERIVPROC>,
    glGetRenderbufferParameterivEXT:               Option<PFNGLGETRENDERBUFFERPARAMETERIVEXTPROC>,
    glGetSamplerParameterIiv:                      Option<PFNGLGETSAMPLERPARAMETERIIVPROC>,
    glGetSamplerParameterIuiv:                     Option<PFNGLGETSAMPLERPARAMETERIUIVPROC>,
    glGetSamplerParameterfv:                       Option<PFNGLGETSAMPLERPARAMETERFVPROC>,
    glGetSamplerParameteriv:                       Option<PFNGLGETSAMPLERPARAMETERIVPROC>,
    glGetShaderInfoLog:                            Option<PFNGLGETSHADERINFOLOGPROC>,
    glGetShaderPrecisionFormat:                    Option<PFNGLGETSHADERPRECISIONFORMATPROC>,
    glGetShaderSource:                             Option<PFNGLGETSHADERSOURCEPROC>,
    glGetShaderSourceARB:                          Option<PFNGLGETSHADERSOURCEARBPROC>,
    glGetShaderiv:                                 Option<PFNGLGETSHADERIVPROC>,
    glGetString:                                   Option<PFNGLGETSTRINGPROC>,
    glGetStringi:                                  Option<PFNGLGETSTRINGIPROC>,
    glGetSubroutineIndex:                          Option<PFNGLGETSUBROUTINEINDEXPROC>,
    glGetSubroutineUniformLocation:                Option<PFNGLGETSUBROUTINEUNIFORMLOCATIONPROC>,
    glGetSynciv:                                   Option<PFNGLGETSYNCIVPROC>,
    glGetTexEnvxvOES:                              Option<PFNGLGETTEXENVXVOESPROC>,
    glGetTexGenxvOES:                              Option<PFNGLGETTEXGENXVOESPROC>,
    glGetTexImage:                                 Option<PFNGLGETTEXIMAGEPROC>,
    glGetTexLevelParameterfv:                      Option<PFNGLGETTEXLEVELPARAMETERFVPROC>,
    glGetTexLevelParameteriv:                      Option<PFNGLGETTEXLEVELPARAMETERIVPROC>,
    glGetTexLevelParameterxvOES:                   Option<PFNGLGETTEXLEVELPARAMETERXVOESPROC>,
    glGetTexParameterIiv:                          Option<PFNGLGETTEXPARAMETERIIVPROC>,
    glGetTexParameterIuiv:                         Option<PFNGLGETTEXPARAMETERIUIVPROC>,
    glGetTexParameterfv:                           Option<PFNGLGETTEXPARAMETERFVPROC>,
    glGetTexParameteriv:                           Option<PFNGLGETTEXPARAMETERIVPROC>,
    glGetTexParameterxvOES:                        Option<PFNGLGETTEXPARAMETERXVOESPROC>,
    glGetTextureImage:                             Option<PFNGLGETTEXTUREIMAGEPROC>,
    glGetTextureLevelParameterfv:                  Option<PFNGLGETTEXTURELEVELPARAMETERFVPROC>,
    glGetTextureLevelParameteriv:                  Option<PFNGLGETTEXTURELEVELPARAMETERIVPROC>,
    glGetTextureParameterIiv:                      Option<PFNGLGETTEXTUREPARAMETERIIVPROC>,
    glGetTextureParameterIuiv:                     Option<PFNGLGETTEXTUREPARAMETERIUIVPROC>,
    glGetTextureParameterfv:                       Option<PFNGLGETTEXTUREPARAMETERFVPROC>,
    glGetTextureParameteriv:                       Option<PFNGLGETTEXTUREPARAMETERIVPROC>,
    glGetTextureSubImage:                          Option<PFNGLGETTEXTURESUBIMAGEPROC>,
    glGetTransformFeedbackVarying:                 Option<PFNGLGETTRANSFORMFEEDBACKVARYINGPROC>,
    glGetTransformFeedbacki64_v:                   Option<PFNGLGETTRANSFORMFEEDBACKI64_VPROC>,
    glGetTransformFeedbacki_v:                     Option<PFNGLGETTRANSFORMFEEDBACKI_VPROC>,
    glGetTransformFeedbackiv:                      Option<PFNGLGETTRANSFORMFEEDBACKIVPROC>,
    glGetUniformBlockIndex:                        Option<PFNGLGETUNIFORMBLOCKINDEXPROC>,
    glGetUniformIndices:                           Option<PFNGLGETUNIFORMINDICESPROC>,
    glGetUniformLocation:                          Option<PFNGLGETUNIFORMLOCATIONPROC>,
    glGetUniformLocationARB:                       Option<PFNGLGETUNIFORMLOCATIONARBPROC>,
    glGetUniformSubroutineuiv:                     Option<PFNGLGETUNIFORMSUBROUTINEUIVPROC>,
    glGetUniformdv:                                Option<PFNGLGETUNIFORMDVPROC>,
    glGetUniformfv:                                Option<PFNGLGETUNIFORMFVPROC>,
    glGetUniformfvARB:                             Option<PFNGLGETUNIFORMFVARBPROC>,
    glGetUniformi64vARB:                           Option<PFNGLGETUNIFORMI64VARBPROC>,
    glGetUniformiv:                                Option<PFNGLGETUNIFORMIVPROC>,
    glGetUniformivARB:                             Option<PFNGLGETUNIFORMIVARBPROC>,
    glGetUniformui64vARB:                          Option<PFNGLGETUNIFORMUI64VARBPROC>,
    glGetUniformuiv:                               Option<PFNGLGETUNIFORMUIVPROC>,
    glGetVertexArrayIndexed64iv:                   Option<PFNGLGETVERTEXARRAYINDEXED64IVPROC>,
    glGetVertexArrayIndexediv:                     Option<PFNGLGETVERTEXARRAYINDEXEDIVPROC>,
    glGetVertexArrayiv:                            Option<PFNGLGETVERTEXARRAYIVPROC>,
    glGetVertexAttribIiv:                          Option<PFNGLGETVERTEXATTRIBIIVPROC>,
    glGetVertexAttribIuiv:                         Option<PFNGLGETVERTEXATTRIBIUIVPROC>,
    glGetVertexAttribLdv:                          Option<PFNGLGETVERTEXATTRIBLDVPROC>,
    glGetVertexAttribPointerv:                     Option<PFNGLGETVERTEXATTRIBPOINTERVPROC>,
    glGetVertexAttribPointervARB:                  Option<PFNGLGETVERTEXATTRIBPOINTERVARBPROC>,
    glGetVertexAttribdv:                           Option<PFNGLGETVERTEXATTRIBDVPROC>,
    glGetVertexAttribdvARB:                        Option<PFNGLGETVERTEXATTRIBDVARBPROC>,
    glGetVertexAttribfv:                           Option<PFNGLGETVERTEXATTRIBFVPROC>,
    glGetVertexAttribfvARB:                        Option<PFNGLGETVERTEXATTRIBFVARBPROC>,
    glGetVertexAttribiv:                           Option<PFNGLGETVERTEXATTRIBIVPROC>,
    glGetVertexAttribivARB:                        Option<PFNGLGETVERTEXATTRIBIVARBPROC>,
    glGetnUniformi64vARB:                          Option<PFNGLGETNUNIFORMI64VARBPROC>,
    glGetnUniformui64vARB:                         Option<PFNGLGETNUNIFORMUI64VARBPROC>,
    glHint:                                        Option<PFNGLHINTPROC>,
    glIndexxOES:                                   Option<PFNGLINDEXXOESPROC>,
    glIndexxvOES:                                  Option<PFNGLINDEXXVOESPROC>,
    glInvalidateBufferData:                        Option<PFNGLINVALIDATEBUFFERDATAPROC>,
    glInvalidateBufferSubData:                     Option<PFNGLINVALIDATEBUFFERSUBDATAPROC>,
    glInvalidateFramebuffer:                       Option<PFNGLINVALIDATEFRAMEBUFFERPROC>,
    glInvalidateNamedFramebufferData:              Option<PFNGLINVALIDATENAMEDFRAMEBUFFERDATAPROC>,
    glInvalidateNamedFramebufferSubData:           Option<PFNGLINVALIDATENAMEDFRAMEBUFFERSUBDATAPROC>,
    glInvalidateSubFramebuffer:                    Option<PFNGLINVALIDATESUBFRAMEBUFFERPROC>,
    glInvalidateTexImage:                          Option<PFNGLINVALIDATETEXIMAGEPROC>,
    glInvalidateTexSubImage:                       Option<PFNGLINVALIDATETEXSUBIMAGEPROC>,
    glIsBuffer:                                    Option<PFNGLISBUFFERPROC>,
    glIsBufferARB:                                 Option<PFNGLISBUFFERARBPROC>,
    glIsEnabled:                                   Option<PFNGLISENABLEDPROC>,
    glIsEnabledi:                                  Option<PFNGLISENABLEDIPROC>,
    glIsFramebuffer:                               Option<PFNGLISFRAMEBUFFERPROC>,
    glIsFramebufferEXT:                            Option<PFNGLISFRAMEBUFFEREXTPROC>,
    glIsNamedStringARB:                            Option<PFNGLISNAMEDSTRINGARBPROC>,
    glIsProgram:                                   Option<PFNGLISPROGRAMPROC>,
    glIsProgramARB:                                Option<PFNGLISPROGRAMARBPROC>,
    glIsProgramPipeline:                           Option<PFNGLISPROGRAMPIPELINEPROC>,
    glIsQuery:                                     Option<PFNGLISQUERYPROC>,
    glIsQueryARB:                                  Option<PFNGLISQUERYARBPROC>,
    glIsRenderbuffer:                              Option<PFNGLISRENDERBUFFERPROC>,
    glIsRenderbufferEXT:                           Option<PFNGLISRENDERBUFFEREXTPROC>,
    glIsSampler:                                   Option<PFNGLISSAMPLERPROC>,
    glIsShader:                                    Option<PFNGLISSHADERPROC>,
    glIsSync:                                      Option<PFNGLISSYNCPROC>,
    glIsTexture:                                   Option<PFNGLISTEXTUREPROC>,
    glIsTransformFeedback:                         Option<PFNGLISTRANSFORMFEEDBACKPROC>,
    glIsVertexArray:                               Option<PFNGLISVERTEXARRAYPROC>,
    glLightModelxOES:                              Option<PFNGLLIGHTMODELXOESPROC>,
    glLightModelxvOES:                             Option<PFNGLLIGHTMODELXVOESPROC>,
    glLightxOES:                                   Option<PFNGLLIGHTXOESPROC>,
    glLightxvOES:                                  Option<PFNGLLIGHTXVOESPROC>,
    glLineWidth:                                   Option<PFNGLLINEWIDTHPROC>,
    glLineWidthxOES:                               Option<PFNGLLINEWIDTHXOESPROC>,
    glLinkProgram:                                 Option<PFNGLLINKPROGRAMPROC>,
    glLinkProgramARB:                              Option<PFNGLLINKPROGRAMARBPROC>,
    glLoadMatrixxOES:                              Option<PFNGLLOADMATRIXXOESPROC>,
    glLoadTransposeMatrixdARB:                     Option<PFNGLLOADTRANSPOSEMATRIXDARBPROC>,
    glLoadTransposeMatrixfARB:                     Option<PFNGLLOADTRANSPOSEMATRIXFARBPROC>,
    glLoadTransposeMatrixxOES:                     Option<PFNGLLOADTRANSPOSEMATRIXXOESPROC>,
    glLogicOp:                                     Option<PFNGLLOGICOPPROC>,
    glMap1xOES:                                    Option<PFNGLMAP1XOESPROC>,
    glMap2xOES:                                    Option<PFNGLMAP2XOESPROC>,
    glMapBuffer:                                   Option<PFNGLMAPBUFFERPROC>,
    glMapBufferARB:                                Option<PFNGLMAPBUFFERARBPROC>,
    glMapBufferRange:                              Option<PFNGLMAPBUFFERRANGEPROC>,
    glMapGrid1xOES:                                Option<PFNGLMAPGRID1XOESPROC>,
    glMapGrid2xOES:                                Option<PFNGLMAPGRID2XOESPROC>,
    glMapNamedBuffer:                              Option<PFNGLMAPNAMEDBUFFERPROC>,
    glMapNamedBufferRange:                         Option<PFNGLMAPNAMEDBUFFERRANGEPROC>,
    glMaterialxOES:                                Option<PFNGLMATERIALXOESPROC>,
    glMaterialxvOES:                               Option<PFNGLMATERIALXVOESPROC>,
    glMemoryBarrier:                               Option<PFNGLMEMORYBARRIERPROC>,
    glMemoryBarrierByRegion:                       Option<PFNGLMEMORYBARRIERBYREGIONPROC>,
    glMinSampleShading:                            Option<PFNGLMINSAMPLESHADINGPROC>,
    glMinSampleShadingARB:                         Option<PFNGLMINSAMPLESHADINGARBPROC>,
    glMultMatrixxOES:                              Option<PFNGLMULTMATRIXXOESPROC>,
    glMultTransposeMatrixdARB:                     Option<PFNGLMULTTRANSPOSEMATRIXDARBPROC>,
    glMultTransposeMatrixfARB:                     Option<PFNGLMULTTRANSPOSEMATRIXFARBPROC>,
    glMultTransposeMatrixxOES:                     Option<PFNGLMULTTRANSPOSEMATRIXXOESPROC>,
    glMultiDrawArrays:                             Option<PFNGLMULTIDRAWARRAYSPROC>,
    glMultiDrawArraysIndirect:                     Option<PFNGLMULTIDRAWARRAYSINDIRECTPROC>,
    glMultiDrawElements:                           Option<PFNGLMULTIDRAWELEMENTSPROC>,
    glMultiDrawElementsBaseVertex:                 Option<PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC>,
    glMultiDrawElementsIndirect:                   Option<PFNGLMULTIDRAWELEMENTSINDIRECTPROC>,
    glMultiTexCoord1dARB:                          Option<PFNGLMULTITEXCOORD1DARBPROC>,
    glMultiTexCoord1dvARB:                         Option<PFNGLMULTITEXCOORD1DVARBPROC>,
    glMultiTexCoord1fARB:                          Option<PFNGLMULTITEXCOORD1FARBPROC>,
    glMultiTexCoord1fvARB:                         Option<PFNGLMULTITEXCOORD1FVARBPROC>,
    glMultiTexCoord1iARB:                          Option<PFNGLMULTITEXCOORD1IARBPROC>,
    glMultiTexCoord1ivARB:                         Option<PFNGLMULTITEXCOORD1IVARBPROC>,
    glMultiTexCoord1sARB:                          Option<PFNGLMULTITEXCOORD1SARBPROC>,
    glMultiTexCoord1svARB:                         Option<PFNGLMULTITEXCOORD1SVARBPROC>,
    glMultiTexCoord1xOES:                          Option<PFNGLMULTITEXCOORD1XOESPROC>,
    glMultiTexCoord1xvOES:                         Option<PFNGLMULTITEXCOORD1XVOESPROC>,
    glMultiTexCoord2dARB:                          Option<PFNGLMULTITEXCOORD2DARBPROC>,
    glMultiTexCoord2dvARB:                         Option<PFNGLMULTITEXCOORD2DVARBPROC>,
    glMultiTexCoord2fARB:                          Option<PFNGLMULTITEXCOORD2FARBPROC>,
    glMultiTexCoord2fvARB:                         Option<PFNGLMULTITEXCOORD2FVARBPROC>,
    glMultiTexCoord2iARB:                          Option<PFNGLMULTITEXCOORD2IARBPROC>,
    glMultiTexCoord2ivARB:                         Option<PFNGLMULTITEXCOORD2IVARBPROC>,
    glMultiTexCoord2sARB:                          Option<PFNGLMULTITEXCOORD2SARBPROC>,
    glMultiTexCoord2svARB:                         Option<PFNGLMULTITEXCOORD2SVARBPROC>,
    glMultiTexCoord2xOES:                          Option<PFNGLMULTITEXCOORD2XOESPROC>,
    glMultiTexCoord2xvOES:                         Option<PFNGLMULTITEXCOORD2XVOESPROC>,
    glMultiTexCoord3dARB:                          Option<PFNGLMULTITEXCOORD3DARBPROC>,
    glMultiTexCoord3dvARB:                         Option<PFNGLMULTITEXCOORD3DVARBPROC>,
    glMultiTexCoord3fARB:                          Option<PFNGLMULTITEXCOORD3FARBPROC>,
    glMultiTexCoord3fvARB:                         Option<PFNGLMULTITEXCOORD3FVARBPROC>,
    glMultiTexCoord3iARB:                          Option<PFNGLMULTITEXCOORD3IARBPROC>,
    glMultiTexCoord3ivARB:                         Option<PFNGLMULTITEXCOORD3IVARBPROC>,
    glMultiTexCoord3sARB:                          Option<PFNGLMULTITEXCOORD3SARBPROC>,
    glMultiTexCoord3svARB:                         Option<PFNGLMULTITEXCOORD3SVARBPROC>,
    glMultiTexCoord3xOES:                          Option<PFNGLMULTITEXCOORD3XOESPROC>,
    glMultiTexCoord3xvOES:                         Option<PFNGLMULTITEXCOORD3XVOESPROC>,
    glMultiTexCoord4dARB:                          Option<PFNGLMULTITEXCOORD4DARBPROC>,
    glMultiTexCoord4dvARB:                         Option<PFNGLMULTITEXCOORD4DVARBPROC>,
    glMultiTexCoord4fARB:                          Option<PFNGLMULTITEXCOORD4FARBPROC>,
    glMultiTexCoord4fvARB:                         Option<PFNGLMULTITEXCOORD4FVARBPROC>,
    glMultiTexCoord4iARB:                          Option<PFNGLMULTITEXCOORD4IARBPROC>,
    glMultiTexCoord4ivARB:                         Option<PFNGLMULTITEXCOORD4IVARBPROC>,
    glMultiTexCoord4sARB:                          Option<PFNGLMULTITEXCOORD4SARBPROC>,
    glMultiTexCoord4svARB:                         Option<PFNGLMULTITEXCOORD4SVARBPROC>,
    glMultiTexCoord4xOES:                          Option<PFNGLMULTITEXCOORD4XOESPROC>,
    glMultiTexCoord4xvOES:                         Option<PFNGLMULTITEXCOORD4XVOESPROC>,
    glNamedBufferData:                             Option<PFNGLNAMEDBUFFERDATAPROC>,
    glNamedBufferStorage:                          Option<PFNGLNAMEDBUFFERSTORAGEPROC>,
    glNamedBufferSubData:                          Option<PFNGLNAMEDBUFFERSUBDATAPROC>,
    glNamedFramebufferDrawBuffer:                  Option<PFNGLNAMEDFRAMEBUFFERDRAWBUFFERPROC>,
    glNamedFramebufferDrawBuffers:                 Option<PFNGLNAMEDFRAMEBUFFERDRAWBUFFERSPROC>,
    glNamedFramebufferParameteri:                  Option<PFNGLNAMEDFRAMEBUFFERPARAMETERIPROC>,
    glNamedFramebufferReadBuffer:                  Option<PFNGLNAMEDFRAMEBUFFERREADBUFFERPROC>,
    glNamedFramebufferRenderbuffer:                Option<PFNGLNAMEDFRAMEBUFFERRENDERBUFFERPROC>,
    glNamedFramebufferSampleLocationsfvARB:        Option<PFNGLNAMEDFRAMEBUFFERSAMPLELOCATIONSFVARBPROC>,
    glNamedFramebufferTexture:                     Option<PFNGLNAMEDFRAMEBUFFERTEXTUREPROC>,
    glNamedFramebufferTextureLayer:                Option<PFNGLNAMEDFRAMEBUFFERTEXTURELAYERPROC>,
    glNamedRenderbufferStorage:                    Option<PFNGLNAMEDRENDERBUFFERSTORAGEPROC>,
    glNamedRenderbufferStorageMultisample:         Option<PFNGLNAMEDRENDERBUFFERSTORAGEMULTISAMPLEPROC>,
    glNamedStringARB:                              Option<PFNGLNAMEDSTRINGARBPROC>,
    glNormal3xOES:                                 Option<PFNGLNORMAL3XOESPROC>,
    glNormal3xvOES:                                Option<PFNGLNORMAL3XVOESPROC>,
    glObjectLabel:                                 Option<PFNGLOBJECTLABELPROC>,
    glObjectPtrLabel:                              Option<PFNGLOBJECTPTRLABELPROC>,
    glOrthoxOES:                                   Option<PFNGLORTHOXOESPROC>,
    glPassThroughxOES:                             Option<PFNGLPASSTHROUGHXOESPROC>,
    glPatchParameterfv:                            Option<PFNGLPATCHPARAMETERFVPROC>,
    glPatchParameteri:                             Option<PFNGLPATCHPARAMETERIPROC>,
    glPauseTransformFeedback:                      Option<PFNGLPAUSETRANSFORMFEEDBACKPROC>,
    glPixelMapx:                                   Option<PFNGLPIXELMAPXPROC>,
    glPixelStoref:                                 Option<PFNGLPIXELSTOREFPROC>,
    glPixelStorei:                                 Option<PFNGLPIXELSTOREIPROC>,
    glPixelStorex:                                 Option<PFNGLPIXELSTOREXPROC>,
    glPixelTransferxOES:                           Option<PFNGLPIXELTRANSFERXOESPROC>,
    glPixelZoomxOES:                               Option<PFNGLPIXELZOOMXOESPROC>,
    glPointParameterf:                             Option<PFNGLPOINTPARAMETERFPROC>,
    glPointParameterfv:                            Option<PFNGLPOINTPARAMETERFVPROC>,
    glPointParameteri:                             Option<PFNGLPOINTPARAMETERIPROC>,
    glPointParameteriv:                            Option<PFNGLPOINTPARAMETERIVPROC>,
    glPointParameterxvOES:                         Option<PFNGLPOINTPARAMETERXVOESPROC>,
    glPointSize:                                   Option<PFNGLPOINTSIZEPROC>,
    glPointSizexOES:                               Option<PFNGLPOINTSIZEXOESPROC>,
    glPolygonMode:                                 Option<PFNGLPOLYGONMODEPROC>,
    glPolygonOffset:                               Option<PFNGLPOLYGONOFFSETPROC>,
    glPolygonOffsetxOES:                           Option<PFNGLPOLYGONOFFSETXOESPROC>,
    glPopDebugGroup:                               Option<PFNGLPOPDEBUGGROUPPROC>,
    glPrimitiveBoundingBoxARB:                     Option<PFNGLPRIMITIVEBOUNDINGBOXARBPROC>,
    glPrimitiveRestartIndex:                       Option<PFNGLPRIMITIVERESTARTINDEXPROC>,
    glPrioritizeTexturesxOES:                      Option<PFNGLPRIORITIZETEXTURESXOESPROC>,
    glProgramBinary:                               Option<PFNGLPROGRAMBINARYPROC>,
    glProgramEnvParameter4dARB:                    Option<PFNGLPROGRAMENVPARAMETER4DARBPROC>,
    glProgramEnvParameter4dvARB:                   Option<PFNGLPROGRAMENVPARAMETER4DVARBPROC>,
    glProgramEnvParameter4fARB:                    Option<PFNGLPROGRAMENVPARAMETER4FARBPROC>,
    glProgramEnvParameter4fvARB:                   Option<PFNGLPROGRAMENVPARAMETER4FVARBPROC>,
    glProgramLocalParameter4dARB:                  Option<PFNGLPROGRAMLOCALPARAMETER4DARBPROC>,
    glProgramLocalParameter4dvARB:                 Option<PFNGLPROGRAMLOCALPARAMETER4DVARBPROC>,
    glProgramLocalParameter4fARB:                  Option<PFNGLPROGRAMLOCALPARAMETER4FARBPROC>,
    glProgramLocalParameter4fvARB:                 Option<PFNGLPROGRAMLOCALPARAMETER4FVARBPROC>,
    glProgramParameteri:                           Option<PFNGLPROGRAMPARAMETERIPROC>,
    glProgramParameteriARB:                        Option<PFNGLPROGRAMPARAMETERIARBPROC>,
    glProgramStringARB:                            Option<PFNGLPROGRAMSTRINGARBPROC>,
    glProgramUniform1d:                            Option<PFNGLPROGRAMUNIFORM1DPROC>,
    glProgramUniform1dv:                           Option<PFNGLPROGRAMUNIFORM1DVPROC>,
    glProgramUniform1f:                            Option<PFNGLPROGRAMUNIFORM1FPROC>,
    glProgramUniform1fv:                           Option<PFNGLPROGRAMUNIFORM1FVPROC>,
    glProgramUniform1i:                            Option<PFNGLPROGRAMUNIFORM1IPROC>,
    glProgramUniform1i64ARB:                       Option<PFNGLPROGRAMUNIFORM1I64ARBPROC>,
    glProgramUniform1i64vARB:                      Option<PFNGLPROGRAMUNIFORM1I64VARBPROC>,
    glProgramUniform1iv:                           Option<PFNGLPROGRAMUNIFORM1IVPROC>,
    glProgramUniform1ui:                           Option<PFNGLPROGRAMUNIFORM1UIPROC>,
    glProgramUniform1ui64ARB:                      Option<PFNGLPROGRAMUNIFORM1UI64ARBPROC>,
    glProgramUniform1ui64vARB:                     Option<PFNGLPROGRAMUNIFORM1UI64VARBPROC>,
    glProgramUniform1uiv:                          Option<PFNGLPROGRAMUNIFORM1UIVPROC>,
    glProgramUniform2d:                            Option<PFNGLPROGRAMUNIFORM2DPROC>,
    glProgramUniform2dv:                           Option<PFNGLPROGRAMUNIFORM2DVPROC>,
    glProgramUniform2f:                            Option<PFNGLPROGRAMUNIFORM2FPROC>,
    glProgramUniform2fv:                           Option<PFNGLPROGRAMUNIFORM2FVPROC>,
    glProgramUniform2i:                            Option<PFNGLPROGRAMUNIFORM2IPROC>,
    glProgramUniform2i64ARB:                       Option<PFNGLPROGRAMUNIFORM2I64ARBPROC>,
    glProgramUniform2i64vARB:                      Option<PFNGLPROGRAMUNIFORM2I64VARBPROC>,
    glProgramUniform2iv:                           Option<PFNGLPROGRAMUNIFORM2IVPROC>,
    glProgramUniform2ui:                           Option<PFNGLPROGRAMUNIFORM2UIPROC>,
    glProgramUniform2ui64ARB:                      Option<PFNGLPROGRAMUNIFORM2UI64ARBPROC>,
    glProgramUniform2ui64vARB:                     Option<PFNGLPROGRAMUNIFORM2UI64VARBPROC>,
    glProgramUniform2uiv:                          Option<PFNGLPROGRAMUNIFORM2UIVPROC>,
    glProgramUniform3d:                            Option<PFNGLPROGRAMUNIFORM3DPROC>,
    glProgramUniform3dv:                           Option<PFNGLPROGRAMUNIFORM3DVPROC>,
    glProgramUniform3f:                            Option<PFNGLPROGRAMUNIFORM3FPROC>,
    glProgramUniform3fv:                           Option<PFNGLPROGRAMUNIFORM3FVPROC>,
    glProgramUniform3i:                            Option<PFNGLPROGRAMUNIFORM3IPROC>,
    glProgramUniform3i64ARB:                       Option<PFNGLPROGRAMUNIFORM3I64ARBPROC>,
    glProgramUniform3i64vARB:                      Option<PFNGLPROGRAMUNIFORM3I64VARBPROC>,
    glProgramUniform3iv:                           Option<PFNGLPROGRAMUNIFORM3IVPROC>,
    glProgramUniform3ui:                           Option<PFNGLPROGRAMUNIFORM3UIPROC>,
    glProgramUniform3ui64ARB:                      Option<PFNGLPROGRAMUNIFORM3UI64ARBPROC>,
    glProgramUniform3ui64vARB:                     Option<PFNGLPROGRAMUNIFORM3UI64VARBPROC>,
    glProgramUniform3uiv:                          Option<PFNGLPROGRAMUNIFORM3UIVPROC>,
    glProgramUniform4d:                            Option<PFNGLPROGRAMUNIFORM4DPROC>,
    glProgramUniform4dv:                           Option<PFNGLPROGRAMUNIFORM4DVPROC>,
    glProgramUniform4f:                            Option<PFNGLPROGRAMUNIFORM4FPROC>,
    glProgramUniform4fv:                           Option<PFNGLPROGRAMUNIFORM4FVPROC>,
    glProgramUniform4i:                            Option<PFNGLPROGRAMUNIFORM4IPROC>,
    glProgramUniform4i64ARB:                       Option<PFNGLPROGRAMUNIFORM4I64ARBPROC>,
    glProgramUniform4i64vARB:                      Option<PFNGLPROGRAMUNIFORM4I64VARBPROC>,
    glProgramUniform4iv:                           Option<PFNGLPROGRAMUNIFORM4IVPROC>,
    glProgramUniform4ui:                           Option<PFNGLPROGRAMUNIFORM4UIPROC>,
    glProgramUniform4ui64ARB:                      Option<PFNGLPROGRAMUNIFORM4UI64ARBPROC>,
    glProgramUniform4ui64vARB:                     Option<PFNGLPROGRAMUNIFORM4UI64VARBPROC>,
    glProgramUniform4uiv:                          Option<PFNGLPROGRAMUNIFORM4UIVPROC>,
    glProgramUniformMatrix2dv:                     Option<PFNGLPROGRAMUNIFORMMATRIX2DVPROC>,
    glProgramUniformMatrix2fv:                     Option<PFNGLPROGRAMUNIFORMMATRIX2FVPROC>,
    glProgramUniformMatrix2x3dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX2X3DVPROC>,
    glProgramUniformMatrix2x3fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX2X3FVPROC>,
    glProgramUniformMatrix2x4dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX2X4DVPROC>,
    glProgramUniformMatrix2x4fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX2X4FVPROC>,
    glProgramUniformMatrix3dv:                     Option<PFNGLPROGRAMUNIFORMMATRIX3DVPROC>,
    glProgramUniformMatrix3fv:                     Option<PFNGLPROGRAMUNIFORMMATRIX3FVPROC>,
    glProgramUniformMatrix3x2dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX3X2DVPROC>,
    glProgramUniformMatrix3x2fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX3X2FVPROC>,
    glProgramUniformMatrix3x4dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX3X4DVPROC>,
    glProgramUniformMatrix3x4fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX3X4FVPROC>,
    glProgramUniformMatrix4dv:                     Option<PFNGLPROGRAMUNIFORMMATRIX4DVPROC>,
    glProgramUniformMatrix4fv:                     Option<PFNGLPROGRAMUNIFORMMATRIX4FVPROC>,
    glProgramUniformMatrix4x2dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX4X2DVPROC>,
    glProgramUniformMatrix4x2fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX4X2FVPROC>,
    glProgramUniformMatrix4x3dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX4X3DVPROC>,
    glProgramUniformMatrix4x3fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX4X3FVPROC>,
    glProvokingVertex:                             Option<PFNGLPROVOKINGVERTEXPROC>,
    glPushDebugGroup:                              Option<PFNGLPUSHDEBUGGROUPPROC>,
    glQueryCounter:                                Option<PFNGLQUERYCOUNTERPROC>,
    glRasterPos2xOES:                              Option<PFNGLRASTERPOS2XOESPROC>,
    glRasterPos2xvOES:                             Option<PFNGLRASTERPOS2XVOESPROC>,
    glRasterPos3xOES:                              Option<PFNGLRASTERPOS3XOESPROC>,
    glRasterPos3xvOES:                             Option<PFNGLRASTERPOS3XVOESPROC>,
    glRasterPos4xOES:                              Option<PFNGLRASTERPOS4XOESPROC>,
    glRasterPos4xvOES:                             Option<PFNGLRASTERPOS4XVOESPROC>,
    glReadBuffer:                                  Option<PFNGLREADBUFFERPROC>,
    glReadPixels:                                  Option<PFNGLREADPIXELSPROC>,
    glRectxOES:                                    Option<PFNGLRECTXOESPROC>,
    glRectxvOES:                                   Option<PFNGLRECTXVOESPROC>,
    glReleaseShaderCompiler:                       Option<PFNGLRELEASESHADERCOMPILERPROC>,
    glRenderbufferStorage:                         Option<PFNGLRENDERBUFFERSTORAGEPROC>,
    glRenderbufferStorageEXT:                      Option<PFNGLRENDERBUFFERSTORAGEEXTPROC>,
    glRenderbufferStorageMultisample:              Option<PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC>,
    glRenderbufferStorageMultisampleEXT:           Option<PFNGLRENDERBUFFERSTORAGEMULTISAMPLEEXTPROC>,
    glResumeTransformFeedback:                     Option<PFNGLRESUMETRANSFORMFEEDBACKPROC>,
    glRotatexOES:                                  Option<PFNGLROTATEXOESPROC>,
    glSampleCoverage:                              Option<PFNGLSAMPLECOVERAGEPROC>,
    glSampleCoverageARB:                           Option<PFNGLSAMPLECOVERAGEARBPROC>,
    glSampleMaski:                                 Option<PFNGLSAMPLEMASKIPROC>,
    glSamplerParameterIiv:                         Option<PFNGLSAMPLERPARAMETERIIVPROC>,
    glSamplerParameterIuiv:                        Option<PFNGLSAMPLERPARAMETERIUIVPROC>,
    glSamplerParameterf:                           Option<PFNGLSAMPLERPARAMETERFPROC>,
    glSamplerParameterfv:                          Option<PFNGLSAMPLERPARAMETERFVPROC>,
    glSamplerParameteri:                           Option<PFNGLSAMPLERPARAMETERIPROC>,
    glSamplerParameteriv:                          Option<PFNGLSAMPLERPARAMETERIVPROC>,
    glScalexOES:                                   Option<PFNGLSCALEXOESPROC>,
    glScissor:                                     Option<PFNGLSCISSORPROC>,
    glScissorArrayv:                               Option<PFNGLSCISSORARRAYVPROC>,
    glScissorIndexed:                              Option<PFNGLSCISSORINDEXEDPROC>,
    glScissorIndexedv:                             Option<PFNGLSCISSORINDEXEDVPROC>,
    glShaderBinary:                                Option<PFNGLSHADERBINARYPROC>,
    glShaderSource:                                Option<PFNGLSHADERSOURCEPROC>,
    glShaderSourceARB:                             Option<PFNGLSHADERSOURCEARBPROC>,
    glShaderStorageBlockBinding:                   Option<PFNGLSHADERSTORAGEBLOCKBINDINGPROC>,
    glSpecializeShaderARB:                         Option<PFNGLSPECIALIZESHADERARBPROC>,
    glStencilFunc:                                 Option<PFNGLSTENCILFUNCPROC>,
    glStencilFuncSeparate:                         Option<PFNGLSTENCILFUNCSEPARATEPROC>,
    glStencilMask:                                 Option<PFNGLSTENCILMASKPROC>,
    glStencilMaskSeparate:                         Option<PFNGLSTENCILMASKSEPARATEPROC>,
    glStencilOp:                                   Option<PFNGLSTENCILOPPROC>,
    glStencilOpSeparate:                           Option<PFNGLSTENCILOPSEPARATEPROC>,
    glTexBuffer:                                   Option<PFNGLTEXBUFFERPROC>,
    glTexBufferRange:                              Option<PFNGLTEXBUFFERRANGEPROC>,
    glTexCoord1xOES:                               Option<PFNGLTEXCOORD1XOESPROC>,
    glTexCoord1xvOES:                              Option<PFNGLTEXCOORD1XVOESPROC>,
    glTexCoord2xOES:                               Option<PFNGLTEXCOORD2XOESPROC>,
    glTexCoord2xvOES:                              Option<PFNGLTEXCOORD2XVOESPROC>,
    glTexCoord3xOES:                               Option<PFNGLTEXCOORD3XOESPROC>,
    glTexCoord3xvOES:                              Option<PFNGLTEXCOORD3XVOESPROC>,
    glTexCoord4xOES:                               Option<PFNGLTEXCOORD4XOESPROC>,
    glTexCoord4xvOES:                              Option<PFNGLTEXCOORD4XVOESPROC>,
    glTexEnvxOES:                                  Option<PFNGLTEXENVXOESPROC>,
    glTexEnvxvOES:                                 Option<PFNGLTEXENVXVOESPROC>,
    glTexGenxOES:                                  Option<PFNGLTEXGENXOESPROC>,
    glTexGenxvOES:                                 Option<PFNGLTEXGENXVOESPROC>,
    glTexImage1D:                                  Option<PFNGLTEXIMAGE1DPROC>,
    glTexImage2D:                                  Option<PFNGLTEXIMAGE2DPROC>,
    glTexImage2DMultisample:                       Option<PFNGLTEXIMAGE2DMULTISAMPLEPROC>,
    glTexImage3D:                                  Option<PFNGLTEXIMAGE3DPROC>,
    glTexImage3DMultisample:                       Option<PFNGLTEXIMAGE3DMULTISAMPLEPROC>,
    glTexParameterIiv:                             Option<PFNGLTEXPARAMETERIIVPROC>,
    glTexParameterIuiv:                            Option<PFNGLTEXPARAMETERIUIVPROC>,
    glTexParameterf:                               Option<PFNGLTEXPARAMETERFPROC>,
    glTexParameterfv:                              Option<PFNGLTEXPARAMETERFVPROC>,
    glTexParameteri:                               Option<PFNGLTEXPARAMETERIPROC>,
    glTexParameteriv:                              Option<PFNGLTEXPARAMETERIVPROC>,
    glTexParameterxOES:                            Option<PFNGLTEXPARAMETERXOESPROC>,
    glTexParameterxvOES:                           Option<PFNGLTEXPARAMETERXVOESPROC>,
    glTexStorage1D:                                Option<PFNGLTEXSTORAGE1DPROC>,
    glTexStorage2D:                                Option<PFNGLTEXSTORAGE2DPROC>,
    glTexStorage2DMultisample:                     Option<PFNGLTEXSTORAGE2DMULTISAMPLEPROC>,
    glTexStorage3D:                                Option<PFNGLTEXSTORAGE3DPROC>,
    glTexStorage3DMultisample:                     Option<PFNGLTEXSTORAGE3DMULTISAMPLEPROC>,
    glTexSubImage1D:                               Option<PFNGLTEXSUBIMAGE1DPROC>,
    glTexSubImage2D:                               Option<PFNGLTEXSUBIMAGE2DPROC>,
    glTexSubImage3D:                               Option<PFNGLTEXSUBIMAGE3DPROC>,
    glTextureBuffer:                               Option<PFNGLTEXTUREBUFFERPROC>,
    glTextureBufferRange:                          Option<PFNGLTEXTUREBUFFERRANGEPROC>,
    glTextureParameterIiv:                         Option<PFNGLTEXTUREPARAMETERIIVPROC>,
    glTextureParameterIuiv:                        Option<PFNGLTEXTUREPARAMETERIUIVPROC>,
    glTextureParameterf:                           Option<PFNGLTEXTUREPARAMETERFPROC>,
    glTextureParameterfv:                          Option<PFNGLTEXTUREPARAMETERFVPROC>,
    glTextureParameteri:                           Option<PFNGLTEXTUREPARAMETERIPROC>,
    glTextureParameteriv:                          Option<PFNGLTEXTUREPARAMETERIVPROC>,
    glTextureStorage1D:                            Option<PFNGLTEXTURESTORAGE1DPROC>,
    glTextureStorage2D:                            Option<PFNGLTEXTURESTORAGE2DPROC>,
    glTextureStorage2DMultisample:                 Option<PFNGLTEXTURESTORAGE2DMULTISAMPLEPROC>,
    glTextureStorage3D:                            Option<PFNGLTEXTURESTORAGE3DPROC>,
    glTextureStorage3DMultisample:                 Option<PFNGLTEXTURESTORAGE3DMULTISAMPLEPROC>,
    glTextureSubImage1D:                           Option<PFNGLTEXTURESUBIMAGE1DPROC>,
    glTextureSubImage2D:                           Option<PFNGLTEXTURESUBIMAGE2DPROC>,
    glTextureSubImage3D:                           Option<PFNGLTEXTURESUBIMAGE3DPROC>,
    glTextureView:                                 Option<PFNGLTEXTUREVIEWPROC>,
    glTransformFeedbackBufferBase:                 Option<PFNGLTRANSFORMFEEDBACKBUFFERBASEPROC>,
    glTransformFeedbackBufferRange:                Option<PFNGLTRANSFORMFEEDBACKBUFFERRANGEPROC>,
    glTransformFeedbackVaryings:                   Option<PFNGLTRANSFORMFEEDBACKVARYINGSPROC>,
    glTranslatexOES:                               Option<PFNGLTRANSLATEXOESPROC>,
    glUniform1d:                                   Option<PFNGLUNIFORM1DPROC>,
    glUniform1dv:                                  Option<PFNGLUNIFORM1DVPROC>,
    glUniform1f:                                   Option<PFNGLUNIFORM1FPROC>,
    glUniform1fARB:                                Option<PFNGLUNIFORM1FARBPROC>,
    glUniform1fv:                                  Option<PFNGLUNIFORM1FVPROC>,
    glUniform1fvARB:                               Option<PFNGLUNIFORM1FVARBPROC>,
    glUniform1i:                                   Option<PFNGLUNIFORM1IPROC>,
    glUniform1i64ARB:                              Option<PFNGLUNIFORM1I64ARBPROC>,
    glUniform1i64vARB:                             Option<PFNGLUNIFORM1I64VARBPROC>,
    glUniform1iARB:                                Option<PFNGLUNIFORM1IARBPROC>,
    glUniform1iv:                                  Option<PFNGLUNIFORM1IVPROC>,
    glUniform1ivARB:                               Option<PFNGLUNIFORM1IVARBPROC>,
    glUniform1ui:                                  Option<PFNGLUNIFORM1UIPROC>,
    glUniform1ui64ARB:                             Option<PFNGLUNIFORM1UI64ARBPROC>,
    glUniform1ui64vARB:                            Option<PFNGLUNIFORM1UI64VARBPROC>,
    glUniform1uiv:                                 Option<PFNGLUNIFORM1UIVPROC>,
    glUniform2d:                                   Option<PFNGLUNIFORM2DPROC>,
    glUniform2dv:                                  Option<PFNGLUNIFORM2DVPROC>,
    glUniform2f:                                   Option<PFNGLUNIFORM2FPROC>,
    glUniform2fARB:                                Option<PFNGLUNIFORM2FARBPROC>,
    glUniform2fv:                                  Option<PFNGLUNIFORM2FVPROC>,
    glUniform2fvARB:                               Option<PFNGLUNIFORM2FVARBPROC>,
    glUniform2i:                                   Option<PFNGLUNIFORM2IPROC>,
    glUniform2i64ARB:                              Option<PFNGLUNIFORM2I64ARBPROC>,
    glUniform2i64vARB:                             Option<PFNGLUNIFORM2I64VARBPROC>,
    glUniform2iARB:                                Option<PFNGLUNIFORM2IARBPROC>,
    glUniform2iv:                                  Option<PFNGLUNIFORM2IVPROC>,
    glUniform2ivARB:                               Option<PFNGLUNIFORM2IVARBPROC>,
    glUniform2ui:                                  Option<PFNGLUNIFORM2UIPROC>,
    glUniform2ui64ARB:                             Option<PFNGLUNIFORM2UI64ARBPROC>,
    glUniform2ui64vARB:                            Option<PFNGLUNIFORM2UI64VARBPROC>,
    glUniform2uiv:                                 Option<PFNGLUNIFORM2UIVPROC>,
    glUniform3d:                                   Option<PFNGLUNIFORM3DPROC>,
    glUniform3dv:                                  Option<PFNGLUNIFORM3DVPROC>,
    glUniform3f:                                   Option<PFNGLUNIFORM3FPROC>,
    glUniform3fARB:                                Option<PFNGLUNIFORM3FARBPROC>,
    glUniform3fv:                                  Option<PFNGLUNIFORM3FVPROC>,
    glUniform3fvARB:                               Option<PFNGLUNIFORM3FVARBPROC>,
    glUniform3i:                                   Option<PFNGLUNIFORM3IPROC>,
    glUniform3i64ARB:                              Option<PFNGLUNIFORM3I64ARBPROC>,
    glUniform3i64vARB:                             Option<PFNGLUNIFORM3I64VARBPROC>,
    glUniform3iARB:                                Option<PFNGLUNIFORM3IARBPROC>,
    glUniform3iv:                                  Option<PFNGLUNIFORM3IVPROC>,
    glUniform3ivARB:                               Option<PFNGLUNIFORM3IVARBPROC>,
    glUniform3ui:                                  Option<PFNGLUNIFORM3UIPROC>,
    glUniform3ui64ARB:                             Option<PFNGLUNIFORM3UI64ARBPROC>,
    glUniform3ui64vARB:                            Option<PFNGLUNIFORM3UI64VARBPROC>,
    glUniform3uiv:                                 Option<PFNGLUNIFORM3UIVPROC>,
    glUniform4d:                                   Option<PFNGLUNIFORM4DPROC>,
    glUniform4dv:                                  Option<PFNGLUNIFORM4DVPROC>,
    glUniform4f:                                   Option<PFNGLUNIFORM4FPROC>,
    glUniform4fARB:                                Option<PFNGLUNIFORM4FARBPROC>,
    glUniform4fv:                                  Option<PFNGLUNIFORM4FVPROC>,
    glUniform4fvARB:                               Option<PFNGLUNIFORM4FVARBPROC>,
    glUniform4i:                                   Option<PFNGLUNIFORM4IPROC>,
    glUniform4i64ARB:                              Option<PFNGLUNIFORM4I64ARBPROC>,
    glUniform4i64vARB:                             Option<PFNGLUNIFORM4I64VARBPROC>,
    glUniform4iARB:                                Option<PFNGLUNIFORM4IARBPROC>,
    glUniform4iv:                                  Option<PFNGLUNIFORM4IVPROC>,
    glUniform4ivARB:                               Option<PFNGLUNIFORM4IVARBPROC>,
    glUniform4ui:                                  Option<PFNGLUNIFORM4UIPROC>,
    glUniform4ui64ARB:                             Option<PFNGLUNIFORM4UI64ARBPROC>,
    glUniform4ui64vARB:                            Option<PFNGLUNIFORM4UI64VARBPROC>,
    glUniform4uiv:                                 Option<PFNGLUNIFORM4UIVPROC>,
    glUniformBlockBinding:                         Option<PFNGLUNIFORMBLOCKBINDINGPROC>,
    glUniformMatrix2dv:                            Option<PFNGLUNIFORMMATRIX2DVPROC>,
    glUniformMatrix2fv:                            Option<PFNGLUNIFORMMATRIX2FVPROC>,
    glUniformMatrix2fvARB:                         Option<PFNGLUNIFORMMATRIX2FVARBPROC>,
    glUniformMatrix2x3dv:                          Option<PFNGLUNIFORMMATRIX2X3DVPROC>,
    glUniformMatrix2x3fv:                          Option<PFNGLUNIFORMMATRIX2X3FVPROC>,
    glUniformMatrix2x4dv:                          Option<PFNGLUNIFORMMATRIX2X4DVPROC>,
    glUniformMatrix2x4fv:                          Option<PFNGLUNIFORMMATRIX2X4FVPROC>,
    glUniformMatrix3dv:                            Option<PFNGLUNIFORMMATRIX3DVPROC>,
    glUniformMatrix3fv:                            Option<PFNGLUNIFORMMATRIX3FVPROC>,
    glUniformMatrix3fvARB:                         Option<PFNGLUNIFORMMATRIX3FVARBPROC>,
    glUniformMatrix3x2dv:                          Option<PFNGLUNIFORMMATRIX3X2DVPROC>,
    glUniformMatrix3x2fv:                          Option<PFNGLUNIFORMMATRIX3X2FVPROC>,
    glUniformMatrix3x4dv:                          Option<PFNGLUNIFORMMATRIX3X4DVPROC>,
    glUniformMatrix3x4fv:                          Option<PFNGLUNIFORMMATRIX3X4FVPROC>,
    glUniformMatrix4dv:                            Option<PFNGLUNIFORMMATRIX4DVPROC>,
    glUniformMatrix4fv:                            Option<PFNGLUNIFORMMATRIX4FVPROC>,
    glUniformMatrix4fvARB:                         Option<PFNGLUNIFORMMATRIX4FVARBPROC>,
    glUniformMatrix4x2dv:                          Option<PFNGLUNIFORMMATRIX4X2DVPROC>,
    glUniformMatrix4x2fv:                          Option<PFNGLUNIFORMMATRIX4X2FVPROC>,
    glUniformMatrix4x3dv:                          Option<PFNGLUNIFORMMATRIX4X3DVPROC>,
    glUniformMatrix4x3fv:                          Option<PFNGLUNIFORMMATRIX4X3FVPROC>,
    glUniformSubroutinesuiv:                       Option<PFNGLUNIFORMSUBROUTINESUIVPROC>,
    glUnmapBuffer:                                 Option<PFNGLUNMAPBUFFERPROC>,
    glUnmapBufferARB:                              Option<PFNGLUNMAPBUFFERARBPROC>,
    glUnmapNamedBuffer:                            Option<PFNGLUNMAPNAMEDBUFFERPROC>,
    glUseProgram:                                  Option<PFNGLUSEPROGRAMPROC>,
    glUseProgramObjectARB:                         Option<PFNGLUSEPROGRAMOBJECTARBPROC>,
    glUseProgramStages:                            Option<PFNGLUSEPROGRAMSTAGESPROC>,
    glValidateProgram:                             Option<PFNGLVALIDATEPROGRAMPROC>,
    glValidateProgramARB:                          Option<PFNGLVALIDATEPROGRAMARBPROC>,
    glValidateProgramPipeline:                     Option<PFNGLVALIDATEPROGRAMPIPELINEPROC>,
    glVertex2xOES:                                 Option<PFNGLVERTEX2XOESPROC>,
    glVertex2xvOES:                                Option<PFNGLVERTEX2XVOESPROC>,
    glVertex3xOES:                                 Option<PFNGLVERTEX3XOESPROC>,
    glVertex3xvOES:                                Option<PFNGLVERTEX3XVOESPROC>,
    glVertex4xOES:                                 Option<PFNGLVERTEX4XOESPROC>,
    glVertex4xvOES:                                Option<PFNGLVERTEX4XVOESPROC>,
    glVertexArrayAttribBinding:                    Option<PFNGLVERTEXARRAYATTRIBBINDINGPROC>,
    glVertexArrayAttribFormat:                     Option<PFNGLVERTEXARRAYATTRIBFORMATPROC>,
    glVertexArrayAttribIFormat:                    Option<PFNGLVERTEXARRAYATTRIBIFORMATPROC>,
    glVertexArrayAttribLFormat:                    Option<PFNGLVERTEXARRAYATTRIBLFORMATPROC>,
    glVertexArrayBindingDivisor:                   Option<PFNGLVERTEXARRAYBINDINGDIVISORPROC>,
    glVertexArrayElementBuffer:                    Option<PFNGLVERTEXARRAYELEMENTBUFFERPROC>,
    glVertexArrayVertexBuffer:                     Option<PFNGLVERTEXARRAYVERTEXBUFFERPROC>,
    glVertexArrayVertexBuffers:                    Option<PFNGLVERTEXARRAYVERTEXBUFFERSPROC>,
    glVertexAttrib1d:                              Option<PFNGLVERTEXATTRIB1DPROC>,
    glVertexAttrib1dARB:                           Option<PFNGLVERTEXATTRIB1DARBPROC>,
    glVertexAttrib1dv:                             Option<PFNGLVERTEXATTRIB1DVPROC>,
    glVertexAttrib1dvARB:                          Option<PFNGLVERTEXATTRIB1DVARBPROC>,
    glVertexAttrib1f:                              Option<PFNGLVERTEXATTRIB1FPROC>,
    glVertexAttrib1fARB:                           Option<PFNGLVERTEXATTRIB1FARBPROC>,
    glVertexAttrib1fv:                             Option<PFNGLVERTEXATTRIB1FVPROC>,
    glVertexAttrib1fvARB:                          Option<PFNGLVERTEXATTRIB1FVARBPROC>,
    glVertexAttrib1s:                              Option<PFNGLVERTEXATTRIB1SPROC>,
    glVertexAttrib1sARB:                           Option<PFNGLVERTEXATTRIB1SARBPROC>,
    glVertexAttrib1sv:                             Option<PFNGLVERTEXATTRIB1SVPROC>,
    glVertexAttrib1svARB:                          Option<PFNGLVERTEXATTRIB1SVARBPROC>,
    glVertexAttrib2d:                              Option<PFNGLVERTEXATTRIB2DPROC>,
    glVertexAttrib2dARB:                           Option<PFNGLVERTEXATTRIB2DARBPROC>,
    glVertexAttrib2dv:                             Option<PFNGLVERTEXATTRIB2DVPROC>,
    glVertexAttrib2dvARB:                          Option<PFNGLVERTEXATTRIB2DVARBPROC>,
    glVertexAttrib2f:                              Option<PFNGLVERTEXATTRIB2FPROC>,
    glVertexAttrib2fARB:                           Option<PFNGLVERTEXATTRIB2FARBPROC>,
    glVertexAttrib2fv:                             Option<PFNGLVERTEXATTRIB2FVPROC>,
    glVertexAttrib2fvARB:                          Option<PFNGLVERTEXATTRIB2FVARBPROC>,
    glVertexAttrib2s:                              Option<PFNGLVERTEXATTRIB2SPROC>,
    glVertexAttrib2sARB:                           Option<PFNGLVERTEXATTRIB2SARBPROC>,
    glVertexAttrib2sv:                             Option<PFNGLVERTEXATTRIB2SVPROC>,
    glVertexAttrib2svARB:                          Option<PFNGLVERTEXATTRIB2SVARBPROC>,
    glVertexAttrib3d:                              Option<PFNGLVERTEXATTRIB3DPROC>,
    glVertexAttrib3dARB:                           Option<PFNGLVERTEXATTRIB3DARBPROC>,
    glVertexAttrib3dv:                             Option<PFNGLVERTEXATTRIB3DVPROC>,
    glVertexAttrib3dvARB:                          Option<PFNGLVERTEXATTRIB3DVARBPROC>,
    glVertexAttrib3f:                              Option<PFNGLVERTEXATTRIB3FPROC>,
    glVertexAttrib3fARB:                           Option<PFNGLVERTEXATTRIB3FARBPROC>,
    glVertexAttrib3fv:                             Option<PFNGLVERTEXATTRIB3FVPROC>,
    glVertexAttrib3fvARB:                          Option<PFNGLVERTEXATTRIB3FVARBPROC>,
    glVertexAttrib3s:                              Option<PFNGLVERTEXATTRIB3SPROC>,
    glVertexAttrib3sARB:                           Option<PFNGLVERTEXATTRIB3SARBPROC>,
    glVertexAttrib3sv:                             Option<PFNGLVERTEXATTRIB3SVPROC>,
    glVertexAttrib3svARB:                          Option<PFNGLVERTEXATTRIB3SVARBPROC>,
    glVertexAttrib4Nbv:                            Option<PFNGLVERTEXATTRIB4NBVPROC>,
    glVertexAttrib4NbvARB:                         Option<PFNGLVERTEXATTRIB4NBVARBPROC>,
    glVertexAttrib4Niv:                            Option<PFNGLVERTEXATTRIB4NIVPROC>,
    glVertexAttrib4NivARB:                         Option<PFNGLVERTEXATTRIB4NIVARBPROC>,
    glVertexAttrib4Nsv:                            Option<PFNGLVERTEXATTRIB4NSVPROC>,
    glVertexAttrib4NsvARB:                         Option<PFNGLVERTEXATTRIB4NSVARBPROC>,
    glVertexAttrib4Nub:                            Option<PFNGLVERTEXATTRIB4NUBPROC>,
    glVertexAttrib4NubARB:                         Option<PFNGLVERTEXATTRIB4NUBARBPROC>,
    glVertexAttrib4Nubv:                           Option<PFNGLVERTEXATTRIB4NUBVPROC>,
    glVertexAttrib4NubvARB:                        Option<PFNGLVERTEXATTRIB4NUBVARBPROC>,
    glVertexAttrib4Nuiv:                           Option<PFNGLVERTEXATTRIB4NUIVPROC>,
    glVertexAttrib4NuivARB:                        Option<PFNGLVERTEXATTRIB4NUIVARBPROC>,
    glVertexAttrib4Nusv:                           Option<PFNGLVERTEXATTRIB4NUSVPROC>,
    glVertexAttrib4NusvARB:                        Option<PFNGLVERTEXATTRIB4NUSVARBPROC>,
    glVertexAttrib4bv:                             Option<PFNGLVERTEXATTRIB4BVPROC>,
    glVertexAttrib4bvARB:                          Option<PFNGLVERTEXATTRIB4BVARBPROC>,
    glVertexAttrib4d:                              Option<PFNGLVERTEXATTRIB4DPROC>,
    glVertexAttrib4dARB:                           Option<PFNGLVERTEXATTRIB4DARBPROC>,
    glVertexAttrib4dv:                             Option<PFNGLVERTEXATTRIB4DVPROC>,
    glVertexAttrib4dvARB:                          Option<PFNGLVERTEXATTRIB4DVARBPROC>,
    glVertexAttrib4f:                              Option<PFNGLVERTEXATTRIB4FPROC>,
    glVertexAttrib4fARB:                           Option<PFNGLVERTEXATTRIB4FARBPROC>,
    glVertexAttrib4fv:                             Option<PFNGLVERTEXATTRIB4FVPROC>,
    glVertexAttrib4fvARB:                          Option<PFNGLVERTEXATTRIB4FVARBPROC>,
    glVertexAttrib4iv:                             Option<PFNGLVERTEXATTRIB4IVPROC>,
    glVertexAttrib4ivARB:                          Option<PFNGLVERTEXATTRIB4IVARBPROC>,
    glVertexAttrib4s:                              Option<PFNGLVERTEXATTRIB4SPROC>,
    glVertexAttrib4sARB:                           Option<PFNGLVERTEXATTRIB4SARBPROC>,
    glVertexAttrib4sv:                             Option<PFNGLVERTEXATTRIB4SVPROC>,
    glVertexAttrib4svARB:                          Option<PFNGLVERTEXATTRIB4SVARBPROC>,
    glVertexAttrib4ubv:                            Option<PFNGLVERTEXATTRIB4UBVPROC>,
    glVertexAttrib4ubvARB:                         Option<PFNGLVERTEXATTRIB4UBVARBPROC>,
    glVertexAttrib4uiv:                            Option<PFNGLVERTEXATTRIB4UIVPROC>,
    glVertexAttrib4uivARB:                         Option<PFNGLVERTEXATTRIB4UIVARBPROC>,
    glVertexAttrib4usv:                            Option<PFNGLVERTEXATTRIB4USVPROC>,
    glVertexAttrib4usvARB:                         Option<PFNGLVERTEXATTRIB4USVARBPROC>,
    glVertexAttribBinding:                         Option<PFNGLVERTEXATTRIBBINDINGPROC>,
    glVertexAttribDivisor:                         Option<PFNGLVERTEXATTRIBDIVISORPROC>,
    glVertexAttribDivisorARB:                      Option<PFNGLVERTEXATTRIBDIVISORARBPROC>,
    glVertexAttribFormat:                          Option<PFNGLVERTEXATTRIBFORMATPROC>,
    glVertexAttribI1i:                             Option<PFNGLVERTEXATTRIBI1IPROC>,
    glVertexAttribI1iv:                            Option<PFNGLVERTEXATTRIBI1IVPROC>,
    glVertexAttribI1ui:                            Option<PFNGLVERTEXATTRIBI1UIPROC>,
    glVertexAttribI1uiv:                           Option<PFNGLVERTEXATTRIBI1UIVPROC>,
    glVertexAttribI2i:                             Option<PFNGLVERTEXATTRIBI2IPROC>,
    glVertexAttribI2iv:                            Option<PFNGLVERTEXATTRIBI2IVPROC>,
    glVertexAttribI2ui:                            Option<PFNGLVERTEXATTRIBI2UIPROC>,
    glVertexAttribI2uiv:                           Option<PFNGLVERTEXATTRIBI2UIVPROC>,
    glVertexAttribI3i:                             Option<PFNGLVERTEXATTRIBI3IPROC>,
    glVertexAttribI3iv:                            Option<PFNGLVERTEXATTRIBI3IVPROC>,
    glVertexAttribI3ui:                            Option<PFNGLVERTEXATTRIBI3UIPROC>,
    glVertexAttribI3uiv:                           Option<PFNGLVERTEXATTRIBI3UIVPROC>,
    glVertexAttribI4bv:                            Option<PFNGLVERTEXATTRIBI4BVPROC>,
    glVertexAttribI4i:                             Option<PFNGLVERTEXATTRIBI4IPROC>,
    glVertexAttribI4iv:                            Option<PFNGLVERTEXATTRIBI4IVPROC>,
    glVertexAttribI4sv:                            Option<PFNGLVERTEXATTRIBI4SVPROC>,
    glVertexAttribI4ubv:                           Option<PFNGLVERTEXATTRIBI4UBVPROC>,
    glVertexAttribI4ui:                            Option<PFNGLVERTEXATTRIBI4UIPROC>,
    glVertexAttribI4uiv:                           Option<PFNGLVERTEXATTRIBI4UIVPROC>,
    glVertexAttribI4usv:                           Option<PFNGLVERTEXATTRIBI4USVPROC>,
    glVertexAttribIFormat:                         Option<PFNGLVERTEXATTRIBIFORMATPROC>,
    glVertexAttribIPointer:                        Option<PFNGLVERTEXATTRIBIPOINTERPROC>,
    glVertexAttribL1d:                             Option<PFNGLVERTEXATTRIBL1DPROC>,
    glVertexAttribL1dv:                            Option<PFNGLVERTEXATTRIBL1DVPROC>,
    glVertexAttribL2d:                             Option<PFNGLVERTEXATTRIBL2DPROC>,
    glVertexAttribL2dv:                            Option<PFNGLVERTEXATTRIBL2DVPROC>,
    glVertexAttribL3d:                             Option<PFNGLVERTEXATTRIBL3DPROC>,
    glVertexAttribL3dv:                            Option<PFNGLVERTEXATTRIBL3DVPROC>,
    glVertexAttribL4d:                             Option<PFNGLVERTEXATTRIBL4DPROC>,
    glVertexAttribL4dv:                            Option<PFNGLVERTEXATTRIBL4DVPROC>,
    glVertexAttribLFormat:                         Option<PFNGLVERTEXATTRIBLFORMATPROC>,
    glVertexAttribLPointer:                        Option<PFNGLVERTEXATTRIBLPOINTERPROC>,
    glVertexAttribP1ui:                            Option<PFNGLVERTEXATTRIBP1UIPROC>,
    glVertexAttribP1uiv:                           Option<PFNGLVERTEXATTRIBP1UIVPROC>,
    glVertexAttribP2ui:                            Option<PFNGLVERTEXATTRIBP2UIPROC>,
    glVertexAttribP2uiv:                           Option<PFNGLVERTEXATTRIBP2UIVPROC>,
    glVertexAttribP3ui:                            Option<PFNGLVERTEXATTRIBP3UIPROC>,
    glVertexAttribP3uiv:                           Option<PFNGLVERTEXATTRIBP3UIVPROC>,
    glVertexAttribP4ui:                            Option<PFNGLVERTEXATTRIBP4UIPROC>,
    glVertexAttribP4uiv:                           Option<PFNGLVERTEXATTRIBP4UIVPROC>,
    glVertexAttribPointer:                         Option<PFNGLVERTEXATTRIBPOINTERPROC>,
    glVertexAttribPointerARB:                      Option<PFNGLVERTEXATTRIBPOINTERARBPROC>,
    glVertexBindingDivisor:                        Option<PFNGLVERTEXBINDINGDIVISORPROC>,
    glViewport:                                    Option<PFNGLVIEWPORTPROC>,
    glViewportArrayv:                              Option<PFNGLVIEWPORTARRAYVPROC>,
    glViewportIndexedf:                            Option<PFNGLVIEWPORTINDEXEDFPROC>,
    glViewportIndexedfv:                           Option<PFNGLVIEWPORTINDEXEDFVPROC>,
    glWaitSync:                                    Option<PFNGLWAITSYNCPROC>,
}

macro_rules! load {
    (($loader:expr)($userptr:expr, $name:expr) as $FNTYPE:ty) => {
        $loader($userptr, $name).map(|f| std::mem::transmute::<unsafe extern "C" fn(), $FNTYPE>(f))
    };
}

impl Glad {
    unsafe fn glad_gl_load_GL_VERSION_1_0(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_1_0 { return; }
            self.glBlendFunc = load!((load)(userptr, c"glBlendFunc".as_ptr()) as PFNGLBLENDFUNCPROC);
            self.glClear = load!((load)(userptr, c"glClear".as_ptr()) as PFNGLCLEARPROC);
            self.glClearColor = load!((load)(userptr, c"glClearColor".as_ptr()) as PFNGLCLEARCOLORPROC);
            self.glClearDepth = load!((load)(userptr, c"glClearDepth".as_ptr()) as PFNGLCLEARDEPTHPROC);
            self.glClearStencil = load!((load)(userptr, c"glClearStencil".as_ptr()) as PFNGLCLEARSTENCILPROC);
            self.glColorMask = load!((load)(userptr, c"glColorMask".as_ptr()) as PFNGLCOLORMASKPROC);
            self.glCullFace = load!((load)(userptr, c"glCullFace".as_ptr()) as PFNGLCULLFACEPROC);
            self.glDepthFunc = load!((load)(userptr, c"glDepthFunc".as_ptr()) as PFNGLDEPTHFUNCPROC);
            self.glDepthMask = load!((load)(userptr, c"glDepthMask".as_ptr()) as PFNGLDEPTHMASKPROC);
            self.glDepthRange = load!((load)(userptr, c"glDepthRange".as_ptr()) as PFNGLDEPTHRANGEPROC);
            self.glDisable = load!((load)(userptr, c"glDisable".as_ptr()) as PFNGLDISABLEPROC);
            self.glDrawBuffer = load!((load)(userptr, c"glDrawBuffer".as_ptr()) as PFNGLDRAWBUFFERPROC);
            self.glEnable = load!((load)(userptr, c"glEnable".as_ptr()) as PFNGLENABLEPROC);
            self.glFinish = load!((load)(userptr, c"glFinish".as_ptr()) as PFNGLFINISHPROC);
            self.glFlush = load!((load)(userptr, c"glFlush".as_ptr()) as PFNGLFLUSHPROC);
            self.glFrontFace = load!((load)(userptr, c"glFrontFace".as_ptr()) as PFNGLFRONTFACEPROC);
            self.glGetBooleanv = load!((load)(userptr, c"glGetBooleanv".as_ptr()) as PFNGLGETBOOLEANVPROC);
            self.glGetDoublev = load!((load)(userptr, c"glGetDoublev".as_ptr()) as PFNGLGETDOUBLEVPROC);
            self.glGetError = load!((load)(userptr, c"glGetError".as_ptr()) as PFNGLGETERRORPROC);
            self.glGetFloatv = load!((load)(userptr, c"glGetFloatv".as_ptr()) as PFNGLGETFLOATVPROC);
            self.glGetIntegerv = load!((load)(userptr, c"glGetIntegerv".as_ptr()) as PFNGLGETINTEGERVPROC);
            self.glGetString = load!((load)(userptr, c"glGetString".as_ptr()) as PFNGLGETSTRINGPROC);
            self.glGetTexImage = load!((load)(userptr, c"glGetTexImage".as_ptr()) as PFNGLGETTEXIMAGEPROC);
            self.glGetTexLevelParameterfv = load!((load)(userptr, c"glGetTexLevelParameterfv".as_ptr()) as PFNGLGETTEXLEVELPARAMETERFVPROC);
            self.glGetTexLevelParameteriv = load!((load)(userptr, c"glGetTexLevelParameteriv".as_ptr()) as PFNGLGETTEXLEVELPARAMETERIVPROC);
            self.glGetTexParameterfv = load!((load)(userptr, c"glGetTexParameterfv".as_ptr()) as PFNGLGETTEXPARAMETERFVPROC);
            self.glGetTexParameteriv = load!((load)(userptr, c"glGetTexParameteriv".as_ptr()) as PFNGLGETTEXPARAMETERIVPROC);
            self.glHint = load!((load)(userptr, c"glHint".as_ptr()) as PFNGLHINTPROC);
            self.glIsEnabled = load!((load)(userptr, c"glIsEnabled".as_ptr()) as PFNGLISENABLEDPROC);
            self.glLineWidth = load!((load)(userptr, c"glLineWidth".as_ptr()) as PFNGLLINEWIDTHPROC);
            self.glLogicOp = load!((load)(userptr, c"glLogicOp".as_ptr()) as PFNGLLOGICOPPROC);
            self.glPixelStoref = load!((load)(userptr, c"glPixelStoref".as_ptr()) as PFNGLPIXELSTOREFPROC);
            self.glPixelStorei = load!((load)(userptr, c"glPixelStorei".as_ptr()) as PFNGLPIXELSTOREIPROC);
            self.glPointSize = load!((load)(userptr, c"glPointSize".as_ptr()) as PFNGLPOINTSIZEPROC);
            self.glPolygonMode = load!((load)(userptr, c"glPolygonMode".as_ptr()) as PFNGLPOLYGONMODEPROC);
            self.glReadBuffer = load!((load)(userptr, c"glReadBuffer".as_ptr()) as PFNGLREADBUFFERPROC);
            self.glReadPixels = load!((load)(userptr, c"glReadPixels".as_ptr()) as PFNGLREADPIXELSPROC);
            self.glScissor = load!((load)(userptr, c"glScissor".as_ptr()) as PFNGLSCISSORPROC);
            self.glStencilFunc = load!((load)(userptr, c"glStencilFunc".as_ptr()) as PFNGLSTENCILFUNCPROC);
            self.glStencilMask = load!((load)(userptr, c"glStencilMask".as_ptr()) as PFNGLSTENCILMASKPROC);
            self.glStencilOp = load!((load)(userptr, c"glStencilOp".as_ptr()) as PFNGLSTENCILOPPROC);
            self.glTexImage1D = load!((load)(userptr, c"glTexImage1D".as_ptr()) as PFNGLTEXIMAGE1DPROC);
            self.glTexImage2D = load!((load)(userptr, c"glTexImage2D".as_ptr()) as PFNGLTEXIMAGE2DPROC);
            self.glTexParameterf = load!((load)(userptr, c"glTexParameterf".as_ptr()) as PFNGLTEXPARAMETERFPROC);
            self.glTexParameterfv = load!((load)(userptr, c"glTexParameterfv".as_ptr()) as PFNGLTEXPARAMETERFVPROC);
            self.glTexParameteri = load!((load)(userptr, c"glTexParameteri".as_ptr()) as PFNGLTEXPARAMETERIPROC);
            self.glTexParameteriv = load!((load)(userptr, c"glTexParameteriv".as_ptr()) as PFNGLTEXPARAMETERIVPROC);
            self.glViewport = load!((load)(userptr, c"glViewport".as_ptr()) as PFNGLVIEWPORTPROC);
        }
    }
    unsafe fn glad_gl_load_GL_VERSION_1_1(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_1_1 { return; }
            self.glBindTexture = load!((load)(userptr, c"glBindTexture".as_ptr()) as PFNGLBINDTEXTUREPROC);
            self.glCopyTexImage1D = load!((load)(userptr, c"glCopyTexImage1D".as_ptr()) as PFNGLCOPYTEXIMAGE1DPROC);
            self.glCopyTexImage2D = load!((load)(userptr, c"glCopyTexImage2D".as_ptr()) as PFNGLCOPYTEXIMAGE2DPROC);
            self.glCopyTexSubImage1D = load!((load)(userptr, c"glCopyTexSubImage1D".as_ptr()) as PFNGLCOPYTEXSUBIMAGE1DPROC);
            self.glCopyTexSubImage2D = load!((load)(userptr, c"glCopyTexSubImage2D".as_ptr()) as PFNGLCOPYTEXSUBIMAGE2DPROC);
            self.glDeleteTextures = load!((load)(userptr, c"glDeleteTextures".as_ptr()) as PFNGLDELETETEXTURESPROC);
            self.glDrawArrays = load!((load)(userptr, c"glDrawArrays".as_ptr()) as PFNGLDRAWARRAYSPROC);
            self.glDrawElements = load!((load)(userptr, c"glDrawElements".as_ptr()) as PFNGLDRAWELEMENTSPROC);
            self.glGenTextures = load!((load)(userptr, c"glGenTextures".as_ptr()) as PFNGLGENTEXTURESPROC);
            self.glGetPointerv = load!((load)(userptr, c"glGetPointerv".as_ptr()) as PFNGLGETPOINTERVPROC);
            self.glIsTexture = load!((load)(userptr, c"glIsTexture".as_ptr()) as PFNGLISTEXTUREPROC);
            self.glPolygonOffset = load!((load)(userptr, c"glPolygonOffset".as_ptr()) as PFNGLPOLYGONOFFSETPROC);
            self.glTexSubImage1D = load!((load)(userptr, c"glTexSubImage1D".as_ptr()) as PFNGLTEXSUBIMAGE1DPROC);
            self.glTexSubImage2D = load!((load)(userptr, c"glTexSubImage2D".as_ptr()) as PFNGLTEXSUBIMAGE2DPROC);
        }
    }
    unsafe fn glad_gl_load_GL_VERSION_1_2(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_1_2 { return; }
            self.glCopyTexSubImage3D = load!((load)(userptr, c"glCopyTexSubImage3D".as_ptr()) as PFNGLCOPYTEXSUBIMAGE3DPROC);
            self.glDrawRangeElements = load!((load)(userptr, c"glDrawRangeElements".as_ptr()) as PFNGLDRAWRANGEELEMENTSPROC);
            self.glTexImage3D = load!((load)(userptr, c"glTexImage3D".as_ptr()) as PFNGLTEXIMAGE3DPROC);
            self.glTexSubImage3D = load!((load)(userptr, c"glTexSubImage3D".as_ptr()) as PFNGLTEXSUBIMAGE3DPROC);
        }
    }
    unsafe fn glad_gl_load_GL_VERSION_1_3(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_1_3 { return; }
            self.glActiveTexture = load!((load)(userptr, c"glActiveTexture".as_ptr()) as PFNGLACTIVETEXTUREPROC);
            self.glCompressedTexImage1D = load!((load)(userptr, c"glCompressedTexImage1D".as_ptr()) as PFNGLCOMPRESSEDTEXIMAGE1DPROC);
            self.glCompressedTexImage2D = load!((load)(userptr, c"glCompressedTexImage2D".as_ptr()) as PFNGLCOMPRESSEDTEXIMAGE2DPROC);
            self.glCompressedTexImage3D = load!((load)(userptr, c"glCompressedTexImage3D".as_ptr()) as PFNGLCOMPRESSEDTEXIMAGE3DPROC);
            self.glCompressedTexSubImage1D = load!((load)(userptr, c"glCompressedTexSubImage1D".as_ptr()) as PFNGLCOMPRESSEDTEXSUBIMAGE1DPROC);
            self.glCompressedTexSubImage2D = load!((load)(userptr, c"glCompressedTexSubImage2D".as_ptr()) as PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC);
            self.glCompressedTexSubImage3D = load!((load)(userptr, c"glCompressedTexSubImage3D".as_ptr()) as PFNGLCOMPRESSEDTEXSUBIMAGE3DPROC);
            self.glGetCompressedTexImage = load!((load)(userptr, c"glGetCompressedTexImage".as_ptr()) as PFNGLGETCOMPRESSEDTEXIMAGEPROC);
            self.glSampleCoverage = load!((load)(userptr, c"glSampleCoverage".as_ptr()) as PFNGLSAMPLECOVERAGEPROC);
        }
    }
    unsafe fn glad_gl_load_GL_VERSION_1_4(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_1_4 { return; }
            self.glBlendColor = load!((load)(userptr, c"glBlendColor".as_ptr()) as PFNGLBLENDCOLORPROC);
            self.glBlendEquation = load!((load)(userptr, c"glBlendEquation".as_ptr()) as PFNGLBLENDEQUATIONPROC);
            self.glBlendFuncSeparate = load!((load)(userptr, c"glBlendFuncSeparate".as_ptr()) as PFNGLBLENDFUNCSEPARATEPROC);
            self.glMultiDrawArrays = load!((load)(userptr, c"glMultiDrawArrays".as_ptr()) as PFNGLMULTIDRAWARRAYSPROC);
            self.glMultiDrawElements = load!((load)(userptr, c"glMultiDrawElements".as_ptr()) as PFNGLMULTIDRAWELEMENTSPROC);
            self.glPointParameterf = load!((load)(userptr, c"glPointParameterf".as_ptr()) as PFNGLPOINTPARAMETERFPROC);
            self.glPointParameterfv = load!((load)(userptr, c"glPointParameterfv".as_ptr()) as PFNGLPOINTPARAMETERFVPROC);
            self.glPointParameteri = load!((load)(userptr, c"glPointParameteri".as_ptr()) as PFNGLPOINTPARAMETERIPROC);
            self.glPointParameteriv = load!((load)(userptr, c"glPointParameteriv".as_ptr()) as PFNGLPOINTPARAMETERIVPROC);
        }
    }
    unsafe fn glad_gl_load_GL_VERSION_1_5(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_1_5 { return; }
            self.glBeginQuery = load!((load)(userptr, c"glBeginQuery".as_ptr()) as PFNGLBEGINQUERYPROC);
            self.glBindBuffer = load!((load)(userptr, c"glBindBuffer".as_ptr()) as PFNGLBINDBUFFERPROC);
            self.glBufferData = load!((load)(userptr, c"glBufferData".as_ptr()) as PFNGLBUFFERDATAPROC);
            self.glBufferSubData = load!((load)(userptr, c"glBufferSubData".as_ptr()) as PFNGLBUFFERSUBDATAPROC);
            self.glDeleteBuffers = load!((load)(userptr, c"glDeleteBuffers".as_ptr()) as PFNGLDELETEBUFFERSPROC);
            self.glDeleteQueries = load!((load)(userptr, c"glDeleteQueries".as_ptr()) as PFNGLDELETEQUERIESPROC);
            self.glEndQuery = load!((load)(userptr, c"glEndQuery".as_ptr()) as PFNGLENDQUERYPROC);
            self.glGenBuffers = load!((load)(userptr, c"glGenBuffers".as_ptr()) as PFNGLGENBUFFERSPROC);
            self.glGenQueries = load!((load)(userptr, c"glGenQueries".as_ptr()) as PFNGLGENQUERIESPROC);
            self.glGetBufferParameteriv = load!((load)(userptr, c"glGetBufferParameteriv".as_ptr()) as PFNGLGETBUFFERPARAMETERIVPROC);
            self.glGetBufferPointerv = load!((load)(userptr, c"glGetBufferPointerv".as_ptr()) as PFNGLGETBUFFERPOINTERVPROC);
            self.glGetBufferSubData = load!((load)(userptr, c"glGetBufferSubData".as_ptr()) as PFNGLGETBUFFERSUBDATAPROC);
            self.glGetQueryObjectiv = load!((load)(userptr, c"glGetQueryObjectiv".as_ptr()) as PFNGLGETQUERYOBJECTIVPROC);
            self.glGetQueryObjectuiv = load!((load)(userptr, c"glGetQueryObjectuiv".as_ptr()) as PFNGLGETQUERYOBJECTUIVPROC);
            self.glGetQueryiv = load!((load)(userptr, c"glGetQueryiv".as_ptr()) as PFNGLGETQUERYIVPROC);
            self.glIsBuffer = load!((load)(userptr, c"glIsBuffer".as_ptr()) as PFNGLISBUFFERPROC);
            self.glIsQuery = load!((load)(userptr, c"glIsQuery".as_ptr()) as PFNGLISQUERYPROC);
            self.glMapBuffer = load!((load)(userptr, c"glMapBuffer".as_ptr()) as PFNGLMAPBUFFERPROC);
            self.glUnmapBuffer = load!((load)(userptr, c"glUnmapBuffer".as_ptr()) as PFNGLUNMAPBUFFERPROC);
        }
    }
    unsafe fn glad_gl_load_GL_VERSION_2_0(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_2_0 { return; }
            self.glAttachShader = load!((load)(userptr, c"glAttachShader".as_ptr()) as PFNGLATTACHSHADERPROC);
            self.glBindAttribLocation = load!((load)(userptr, c"glBindAttribLocation".as_ptr()) as PFNGLBINDATTRIBLOCATIONPROC);
            self.glBlendEquationSeparate = load!((load)(userptr, c"glBlendEquationSeparate".as_ptr()) as PFNGLBLENDEQUATIONSEPARATEPROC);
            self.glCompileShader = load!((load)(userptr, c"glCompileShader".as_ptr()) as PFNGLCOMPILESHADERPROC);
            self.glCreateProgram = load!((load)(userptr, c"glCreateProgram".as_ptr()) as PFNGLCREATEPROGRAMPROC);
            self.glCreateShader = load!((load)(userptr, c"glCreateShader".as_ptr()) as PFNGLCREATESHADERPROC);
            self.glDeleteProgram = load!((load)(userptr, c"glDeleteProgram".as_ptr()) as PFNGLDELETEPROGRAMPROC);
            self.glDeleteShader = load!((load)(userptr, c"glDeleteShader".as_ptr()) as PFNGLDELETESHADERPROC);
            self.glDetachShader = load!((load)(userptr, c"glDetachShader".as_ptr()) as PFNGLDETACHSHADERPROC);
            self.glDisableVertexAttribArray = load!((load)(userptr, c"glDisableVertexAttribArray".as_ptr()) as PFNGLDISABLEVERTEXATTRIBARRAYPROC);
            self.glDrawBuffers = load!((load)(userptr, c"glDrawBuffers".as_ptr()) as PFNGLDRAWBUFFERSPROC);
            self.glEnableVertexAttribArray = load!((load)(userptr, c"glEnableVertexAttribArray".as_ptr()) as PFNGLENABLEVERTEXATTRIBARRAYPROC);
            self.glGetActiveAttrib = load!((load)(userptr, c"glGetActiveAttrib".as_ptr()) as PFNGLGETACTIVEATTRIBPROC);
            self.glGetActiveUniform = load!((load)(userptr, c"glGetActiveUniform".as_ptr()) as PFNGLGETACTIVEUNIFORMPROC);
            self.glGetAttachedShaders = load!((load)(userptr, c"glGetAttachedShaders".as_ptr()) as PFNGLGETATTACHEDSHADERSPROC);
            self.glGetAttribLocation = load!((load)(userptr, c"glGetAttribLocation".as_ptr()) as PFNGLGETATTRIBLOCATIONPROC);
            self.glGetProgramInfoLog = load!((load)(userptr, c"glGetProgramInfoLog".as_ptr()) as PFNGLGETPROGRAMINFOLOGPROC);
            self.glGetProgramiv = load!((load)(userptr, c"glGetProgramiv".as_ptr()) as PFNGLGETPROGRAMIVPROC);
            self.glGetShaderInfoLog = load!((load)(userptr, c"glGetShaderInfoLog".as_ptr()) as PFNGLGETSHADERINFOLOGPROC);
            self.glGetShaderSource = load!((load)(userptr, c"glGetShaderSource".as_ptr()) as PFNGLGETSHADERSOURCEPROC);
            self.glGetShaderiv = load!((load)(userptr, c"glGetShaderiv".as_ptr()) as PFNGLGETSHADERIVPROC);
            self.glGetUniformLocation = load!((load)(userptr, c"glGetUniformLocation".as_ptr()) as PFNGLGETUNIFORMLOCATIONPROC);
            self.glGetUniformfv = load!((load)(userptr, c"glGetUniformfv".as_ptr()) as PFNGLGETUNIFORMFVPROC);
            self.glGetUniformiv = load!((load)(userptr, c"glGetUniformiv".as_ptr()) as PFNGLGETUNIFORMIVPROC);
            self.glGetVertexAttribPointerv = load!((load)(userptr, c"glGetVertexAttribPointerv".as_ptr()) as PFNGLGETVERTEXATTRIBPOINTERVPROC);
            self.glGetVertexAttribdv = load!((load)(userptr, c"glGetVertexAttribdv".as_ptr()) as PFNGLGETVERTEXATTRIBDVPROC);
            self.glGetVertexAttribfv = load!((load)(userptr, c"glGetVertexAttribfv".as_ptr()) as PFNGLGETVERTEXATTRIBFVPROC);
            self.glGetVertexAttribiv = load!((load)(userptr, c"glGetVertexAttribiv".as_ptr()) as PFNGLGETVERTEXATTRIBIVPROC);
            self.glIsProgram = load!((load)(userptr, c"glIsProgram".as_ptr()) as PFNGLISPROGRAMPROC);
            self.glIsShader = load!((load)(userptr, c"glIsShader".as_ptr()) as PFNGLISSHADERPROC);
            self.glLinkProgram = load!((load)(userptr, c"glLinkProgram".as_ptr()) as PFNGLLINKPROGRAMPROC);
            self.glShaderSource = load!((load)(userptr, c"glShaderSource".as_ptr()) as PFNGLSHADERSOURCEPROC);
            self.glStencilFuncSeparate = load!((load)(userptr, c"glStencilFuncSeparate".as_ptr()) as PFNGLSTENCILFUNCSEPARATEPROC);
            self.glStencilMaskSeparate = load!((load)(userptr, c"glStencilMaskSeparate".as_ptr()) as PFNGLSTENCILMASKSEPARATEPROC);
            self.glStencilOpSeparate = load!((load)(userptr, c"glStencilOpSeparate".as_ptr()) as PFNGLSTENCILOPSEPARATEPROC);
            self.glUniform1f = load!((load)(userptr, c"glUniform1f".as_ptr()) as PFNGLUNIFORM1FPROC);
            self.glUniform1fv = load!((load)(userptr, c"glUniform1fv".as_ptr()) as PFNGLUNIFORM1FVPROC);
            self.glUniform1i = load!((load)(userptr, c"glUniform1i".as_ptr()) as PFNGLUNIFORM1IPROC);
            self.glUniform1iv = load!((load)(userptr, c"glUniform1iv".as_ptr()) as PFNGLUNIFORM1IVPROC);
            self.glUniform2f = load!((load)(userptr, c"glUniform2f".as_ptr()) as PFNGLUNIFORM2FPROC);
            self.glUniform2fv = load!((load)(userptr, c"glUniform2fv".as_ptr()) as PFNGLUNIFORM2FVPROC);
            self.glUniform2i = load!((load)(userptr, c"glUniform2i".as_ptr()) as PFNGLUNIFORM2IPROC);
            self.glUniform2iv = load!((load)(userptr, c"glUniform2iv".as_ptr()) as PFNGLUNIFORM2IVPROC);
            self.glUniform3f = load!((load)(userptr, c"glUniform3f".as_ptr()) as PFNGLUNIFORM3FPROC);
            self.glUniform3fv = load!((load)(userptr, c"glUniform3fv".as_ptr()) as PFNGLUNIFORM3FVPROC);
            self.glUniform3i = load!((load)(userptr, c"glUniform3i".as_ptr()) as PFNGLUNIFORM3IPROC);
            self.glUniform3iv = load!((load)(userptr, c"glUniform3iv".as_ptr()) as PFNGLUNIFORM3IVPROC);
            self.glUniform4f = load!((load)(userptr, c"glUniform4f".as_ptr()) as PFNGLUNIFORM4FPROC);
            self.glUniform4fv = load!((load)(userptr, c"glUniform4fv".as_ptr()) as PFNGLUNIFORM4FVPROC);
            self.glUniform4i = load!((load)(userptr, c"glUniform4i".as_ptr()) as PFNGLUNIFORM4IPROC);
            self.glUniform4iv = load!((load)(userptr, c"glUniform4iv".as_ptr()) as PFNGLUNIFORM4IVPROC);
            self.glUniformMatrix2fv = load!((load)(userptr, c"glUniformMatrix2fv".as_ptr()) as PFNGLUNIFORMMATRIX2FVPROC);
            self.glUniformMatrix3fv = load!((load)(userptr, c"glUniformMatrix3fv".as_ptr()) as PFNGLUNIFORMMATRIX3FVPROC);
            self.glUniformMatrix4fv = load!((load)(userptr, c"glUniformMatrix4fv".as_ptr()) as PFNGLUNIFORMMATRIX4FVPROC);
            self.glUseProgram = load!((load)(userptr, c"glUseProgram".as_ptr()) as PFNGLUSEPROGRAMPROC);
            self.glValidateProgram = load!((load)(userptr, c"glValidateProgram".as_ptr()) as PFNGLVALIDATEPROGRAMPROC);
            self.glVertexAttrib1d = load!((load)(userptr, c"glVertexAttrib1d".as_ptr()) as PFNGLVERTEXATTRIB1DPROC);
            self.glVertexAttrib1dv = load!((load)(userptr, c"glVertexAttrib1dv".as_ptr()) as PFNGLVERTEXATTRIB1DVPROC);
            self.glVertexAttrib1f = load!((load)(userptr, c"glVertexAttrib1f".as_ptr()) as PFNGLVERTEXATTRIB1FPROC);
            self.glVertexAttrib1fv = load!((load)(userptr, c"glVertexAttrib1fv".as_ptr()) as PFNGLVERTEXATTRIB1FVPROC);
            self.glVertexAttrib1s = load!((load)(userptr, c"glVertexAttrib1s".as_ptr()) as PFNGLVERTEXATTRIB1SPROC);
            self.glVertexAttrib1sv = load!((load)(userptr, c"glVertexAttrib1sv".as_ptr()) as PFNGLVERTEXATTRIB1SVPROC);
            self.glVertexAttrib2d = load!((load)(userptr, c"glVertexAttrib2d".as_ptr()) as PFNGLVERTEXATTRIB2DPROC);
            self.glVertexAttrib2dv = load!((load)(userptr, c"glVertexAttrib2dv".as_ptr()) as PFNGLVERTEXATTRIB2DVPROC);
            self.glVertexAttrib2f = load!((load)(userptr, c"glVertexAttrib2f".as_ptr()) as PFNGLVERTEXATTRIB2FPROC);
            self.glVertexAttrib2fv = load!((load)(userptr, c"glVertexAttrib2fv".as_ptr()) as PFNGLVERTEXATTRIB2FVPROC);
            self.glVertexAttrib2s = load!((load)(userptr, c"glVertexAttrib2s".as_ptr()) as PFNGLVERTEXATTRIB2SPROC);
            self.glVertexAttrib2sv = load!((load)(userptr, c"glVertexAttrib2sv".as_ptr()) as PFNGLVERTEXATTRIB2SVPROC);
            self.glVertexAttrib3d = load!((load)(userptr, c"glVertexAttrib3d".as_ptr()) as PFNGLVERTEXATTRIB3DPROC);
            self.glVertexAttrib3dv = load!((load)(userptr, c"glVertexAttrib3dv".as_ptr()) as PFNGLVERTEXATTRIB3DVPROC);
            self.glVertexAttrib3f = load!((load)(userptr, c"glVertexAttrib3f".as_ptr()) as PFNGLVERTEXATTRIB3FPROC);
            self.glVertexAttrib3fv = load!((load)(userptr, c"glVertexAttrib3fv".as_ptr()) as PFNGLVERTEXATTRIB3FVPROC);
            self.glVertexAttrib3s = load!((load)(userptr, c"glVertexAttrib3s".as_ptr()) as PFNGLVERTEXATTRIB3SPROC);
            self.glVertexAttrib3sv = load!((load)(userptr, c"glVertexAttrib3sv".as_ptr()) as PFNGLVERTEXATTRIB3SVPROC);
            self.glVertexAttrib4Nbv = load!((load)(userptr, c"glVertexAttrib4Nbv".as_ptr()) as PFNGLVERTEXATTRIB4NBVPROC);
            self.glVertexAttrib4Niv = load!((load)(userptr, c"glVertexAttrib4Niv".as_ptr()) as PFNGLVERTEXATTRIB4NIVPROC);
            self.glVertexAttrib4Nsv = load!((load)(userptr, c"glVertexAttrib4Nsv".as_ptr()) as PFNGLVERTEXATTRIB4NSVPROC);
            self.glVertexAttrib4Nub = load!((load)(userptr, c"glVertexAttrib4Nub".as_ptr()) as PFNGLVERTEXATTRIB4NUBPROC);
            self.glVertexAttrib4Nubv = load!((load)(userptr, c"glVertexAttrib4Nubv".as_ptr()) as PFNGLVERTEXATTRIB4NUBVPROC);
            self.glVertexAttrib4Nuiv = load!((load)(userptr, c"glVertexAttrib4Nuiv".as_ptr()) as PFNGLVERTEXATTRIB4NUIVPROC);
            self.glVertexAttrib4Nusv = load!((load)(userptr, c"glVertexAttrib4Nusv".as_ptr()) as PFNGLVERTEXATTRIB4NUSVPROC);
            self.glVertexAttrib4bv = load!((load)(userptr, c"glVertexAttrib4bv".as_ptr()) as PFNGLVERTEXATTRIB4BVPROC);
            self.glVertexAttrib4d = load!((load)(userptr, c"glVertexAttrib4d".as_ptr()) as PFNGLVERTEXATTRIB4DPROC);
            self.glVertexAttrib4dv = load!((load)(userptr, c"glVertexAttrib4dv".as_ptr()) as PFNGLVERTEXATTRIB4DVPROC);
            self.glVertexAttrib4f = load!((load)(userptr, c"glVertexAttrib4f".as_ptr()) as PFNGLVERTEXATTRIB4FPROC);
            self.glVertexAttrib4fv = load!((load)(userptr, c"glVertexAttrib4fv".as_ptr()) as PFNGLVERTEXATTRIB4FVPROC);
            self.glVertexAttrib4iv = load!((load)(userptr, c"glVertexAttrib4iv".as_ptr()) as PFNGLVERTEXATTRIB4IVPROC);
            self.glVertexAttrib4s = load!((load)(userptr, c"glVertexAttrib4s".as_ptr()) as PFNGLVERTEXATTRIB4SPROC);
            self.glVertexAttrib4sv = load!((load)(userptr, c"glVertexAttrib4sv".as_ptr()) as PFNGLVERTEXATTRIB4SVPROC);
            self.glVertexAttrib4ubv = load!((load)(userptr, c"glVertexAttrib4ubv".as_ptr()) as PFNGLVERTEXATTRIB4UBVPROC);
            self.glVertexAttrib4uiv = load!((load)(userptr, c"glVertexAttrib4uiv".as_ptr()) as PFNGLVERTEXATTRIB4UIVPROC);
            self.glVertexAttrib4usv = load!((load)(userptr, c"glVertexAttrib4usv".as_ptr()) as PFNGLVERTEXATTRIB4USVPROC);
            self.glVertexAttribPointer = load!((load)(userptr, c"glVertexAttribPointer".as_ptr()) as PFNGLVERTEXATTRIBPOINTERPROC);
        }
    }
    unsafe fn glad_gl_load_GL_VERSION_2_1(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_2_1 { return; }
            self.glUniformMatrix2x3fv = load!((load)(userptr, c"glUniformMatrix2x3fv".as_ptr()) as PFNGLUNIFORMMATRIX2X3FVPROC);
            self.glUniformMatrix2x4fv = load!((load)(userptr, c"glUniformMatrix2x4fv".as_ptr()) as PFNGLUNIFORMMATRIX2X4FVPROC);
            self.glUniformMatrix3x2fv = load!((load)(userptr, c"glUniformMatrix3x2fv".as_ptr()) as PFNGLUNIFORMMATRIX3X2FVPROC);
            self.glUniformMatrix3x4fv = load!((load)(userptr, c"glUniformMatrix3x4fv".as_ptr()) as PFNGLUNIFORMMATRIX3X4FVPROC);
            self.glUniformMatrix4x2fv = load!((load)(userptr, c"glUniformMatrix4x2fv".as_ptr()) as PFNGLUNIFORMMATRIX4X2FVPROC);
            self.glUniformMatrix4x3fv = load!((load)(userptr, c"glUniformMatrix4x3fv".as_ptr()) as PFNGLUNIFORMMATRIX4X3FVPROC);
        }
    }
    unsafe fn glad_gl_load_GL_VERSION_3_0(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_3_0 { return; }
            self.glBeginConditionalRender = load!((load)(userptr, c"glBeginConditionalRender".as_ptr()) as PFNGLBEGINCONDITIONALRENDERPROC);
            self.glBeginTransformFeedback = load!((load)(userptr, c"glBeginTransformFeedback".as_ptr()) as PFNGLBEGINTRANSFORMFEEDBACKPROC);
            self.glBindBufferBase = load!((load)(userptr, c"glBindBufferBase".as_ptr()) as PFNGLBINDBUFFERBASEPROC);
            self.glBindBufferRange = load!((load)(userptr, c"glBindBufferRange".as_ptr()) as PFNGLBINDBUFFERRANGEPROC);
            self.glBindFragDataLocation = load!((load)(userptr, c"glBindFragDataLocation".as_ptr()) as PFNGLBINDFRAGDATALOCATIONPROC);
            self.glBindFramebuffer = load!((load)(userptr, c"glBindFramebuffer".as_ptr()) as PFNGLBINDFRAMEBUFFERPROC);
            self.glBindRenderbuffer = load!((load)(userptr, c"glBindRenderbuffer".as_ptr()) as PFNGLBINDRENDERBUFFERPROC);
            self.glBindVertexArray = load!((load)(userptr, c"glBindVertexArray".as_ptr()) as PFNGLBINDVERTEXARRAYPROC);
            self.glBlitFramebuffer = load!((load)(userptr, c"glBlitFramebuffer".as_ptr()) as PFNGLBLITFRAMEBUFFERPROC);
            self.glCheckFramebufferStatus = load!((load)(userptr, c"glCheckFramebufferStatus".as_ptr()) as PFNGLCHECKFRAMEBUFFERSTATUSPROC);
            self.glClampColor = load!((load)(userptr, c"glClampColor".as_ptr()) as PFNGLCLAMPCOLORPROC);
            self.glClearBufferfi = load!((load)(userptr, c"glClearBufferfi".as_ptr()) as PFNGLCLEARBUFFERFIPROC);
            self.glClearBufferfv = load!((load)(userptr, c"glClearBufferfv".as_ptr()) as PFNGLCLEARBUFFERFVPROC);
            self.glClearBufferiv = load!((load)(userptr, c"glClearBufferiv".as_ptr()) as PFNGLCLEARBUFFERIVPROC);
            self.glClearBufferuiv = load!((load)(userptr, c"glClearBufferuiv".as_ptr()) as PFNGLCLEARBUFFERUIVPROC);
            self.glColorMaski = load!((load)(userptr, c"glColorMaski".as_ptr()) as PFNGLCOLORMASKIPROC);
            self.glDeleteFramebuffers = load!((load)(userptr, c"glDeleteFramebuffers".as_ptr()) as PFNGLDELETEFRAMEBUFFERSPROC);
            self.glDeleteRenderbuffers = load!((load)(userptr, c"glDeleteRenderbuffers".as_ptr()) as PFNGLDELETERENDERBUFFERSPROC);
            self.glDeleteVertexArrays = load!((load)(userptr, c"glDeleteVertexArrays".as_ptr()) as PFNGLDELETEVERTEXARRAYSPROC);
            self.glDisablei = load!((load)(userptr, c"glDisablei".as_ptr()) as PFNGLDISABLEIPROC);
            self.glEnablei = load!((load)(userptr, c"glEnablei".as_ptr()) as PFNGLENABLEIPROC);
            self.glEndConditionalRender = load!((load)(userptr, c"glEndConditionalRender".as_ptr()) as PFNGLENDCONDITIONALRENDERPROC);
            self.glEndTransformFeedback = load!((load)(userptr, c"glEndTransformFeedback".as_ptr()) as PFNGLENDTRANSFORMFEEDBACKPROC);
            self.glFlushMappedBufferRange = load!((load)(userptr, c"glFlushMappedBufferRange".as_ptr()) as PFNGLFLUSHMAPPEDBUFFERRANGEPROC);
            self.glFramebufferRenderbuffer = load!((load)(userptr, c"glFramebufferRenderbuffer".as_ptr()) as PFNGLFRAMEBUFFERRENDERBUFFERPROC);
            self.glFramebufferTexture1D = load!((load)(userptr, c"glFramebufferTexture1D".as_ptr()) as PFNGLFRAMEBUFFERTEXTURE1DPROC);
            self.glFramebufferTexture2D = load!((load)(userptr, c"glFramebufferTexture2D".as_ptr()) as PFNGLFRAMEBUFFERTEXTURE2DPROC);
            self.glFramebufferTexture3D = load!((load)(userptr, c"glFramebufferTexture3D".as_ptr()) as PFNGLFRAMEBUFFERTEXTURE3DPROC);
            self.glFramebufferTextureLayer = load!((load)(userptr, c"glFramebufferTextureLayer".as_ptr()) as PFNGLFRAMEBUFFERTEXTURELAYERPROC);
            self.glGenFramebuffers = load!((load)(userptr, c"glGenFramebuffers".as_ptr()) as PFNGLGENFRAMEBUFFERSPROC);
            self.glGenRenderbuffers = load!((load)(userptr, c"glGenRenderbuffers".as_ptr()) as PFNGLGENRENDERBUFFERSPROC);
            self.glGenVertexArrays = load!((load)(userptr, c"glGenVertexArrays".as_ptr()) as PFNGLGENVERTEXARRAYSPROC);
            self.glGenerateMipmap = load!((load)(userptr, c"glGenerateMipmap".as_ptr()) as PFNGLGENERATEMIPMAPPROC);
            self.glGetBooleani_v = load!((load)(userptr, c"glGetBooleani_v".as_ptr()) as PFNGLGETBOOLEANI_VPROC);
            self.glGetFragDataLocation = load!((load)(userptr, c"glGetFragDataLocation".as_ptr()) as PFNGLGETFRAGDATALOCATIONPROC);
            self.glGetFramebufferAttachmentParameteriv = load!((load)(userptr, c"glGetFramebufferAttachmentParameteriv".as_ptr()) as PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC);
            self.glGetIntegeri_v = load!((load)(userptr, c"glGetIntegeri_v".as_ptr()) as PFNGLGETINTEGERI_VPROC);
            self.glGetRenderbufferParameteriv = load!((load)(userptr, c"glGetRenderbufferParameteriv".as_ptr()) as PFNGLGETRENDERBUFFERPARAMETERIVPROC);
            self.glGetStringi = load!((load)(userptr, c"glGetStringi".as_ptr()) as PFNGLGETSTRINGIPROC);
            self.glGetTexParameterIiv = load!((load)(userptr, c"glGetTexParameterIiv".as_ptr()) as PFNGLGETTEXPARAMETERIIVPROC);
            self.glGetTexParameterIuiv = load!((load)(userptr, c"glGetTexParameterIuiv".as_ptr()) as PFNGLGETTEXPARAMETERIUIVPROC);
            self.glGetTransformFeedbackVarying = load!((load)(userptr, c"glGetTransformFeedbackVarying".as_ptr()) as PFNGLGETTRANSFORMFEEDBACKVARYINGPROC);
            self.glGetUniformuiv = load!((load)(userptr, c"glGetUniformuiv".as_ptr()) as PFNGLGETUNIFORMUIVPROC);
            self.glGetVertexAttribIiv = load!((load)(userptr, c"glGetVertexAttribIiv".as_ptr()) as PFNGLGETVERTEXATTRIBIIVPROC);
            self.glGetVertexAttribIuiv = load!((load)(userptr, c"glGetVertexAttribIuiv".as_ptr()) as PFNGLGETVERTEXATTRIBIUIVPROC);
            self.glIsEnabledi = load!((load)(userptr, c"glIsEnabledi".as_ptr()) as PFNGLISENABLEDIPROC);
            self.glIsFramebuffer = load!((load)(userptr, c"glIsFramebuffer".as_ptr()) as PFNGLISFRAMEBUFFERPROC);
            self.glIsRenderbuffer = load!((load)(userptr, c"glIsRenderbuffer".as_ptr()) as PFNGLISRENDERBUFFERPROC);
            self.glIsVertexArray = load!((load)(userptr, c"glIsVertexArray".as_ptr()) as PFNGLISVERTEXARRAYPROC);
            self.glMapBufferRange = load!((load)(userptr, c"glMapBufferRange".as_ptr()) as PFNGLMAPBUFFERRANGEPROC);
            self.glRenderbufferStorage = load!((load)(userptr, c"glRenderbufferStorage".as_ptr()) as PFNGLRENDERBUFFERSTORAGEPROC);
            self.glRenderbufferStorageMultisample = load!((load)(userptr, c"glRenderbufferStorageMultisample".as_ptr()) as PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC);
            self.glTexParameterIiv = load!((load)(userptr, c"glTexParameterIiv".as_ptr()) as PFNGLTEXPARAMETERIIVPROC);
            self.glTexParameterIuiv = load!((load)(userptr, c"glTexParameterIuiv".as_ptr()) as PFNGLTEXPARAMETERIUIVPROC);
            self.glTransformFeedbackVaryings = load!((load)(userptr, c"glTransformFeedbackVaryings".as_ptr()) as PFNGLTRANSFORMFEEDBACKVARYINGSPROC);
            self.glUniform1ui = load!((load)(userptr, c"glUniform1ui".as_ptr()) as PFNGLUNIFORM1UIPROC);
            self.glUniform1uiv = load!((load)(userptr, c"glUniform1uiv".as_ptr()) as PFNGLUNIFORM1UIVPROC);
            self.glUniform2ui = load!((load)(userptr, c"glUniform2ui".as_ptr()) as PFNGLUNIFORM2UIPROC);
            self.glUniform2uiv = load!((load)(userptr, c"glUniform2uiv".as_ptr()) as PFNGLUNIFORM2UIVPROC);
            self.glUniform3ui = load!((load)(userptr, c"glUniform3ui".as_ptr()) as PFNGLUNIFORM3UIPROC);
            self.glUniform3uiv = load!((load)(userptr, c"glUniform3uiv".as_ptr()) as PFNGLUNIFORM3UIVPROC);
            self.glUniform4ui = load!((load)(userptr, c"glUniform4ui".as_ptr()) as PFNGLUNIFORM4UIPROC);
            self.glUniform4uiv = load!((load)(userptr, c"glUniform4uiv".as_ptr()) as PFNGLUNIFORM4UIVPROC);
            self.glVertexAttribI1i = load!((load)(userptr, c"glVertexAttribI1i".as_ptr()) as PFNGLVERTEXATTRIBI1IPROC);
            self.glVertexAttribI1iv = load!((load)(userptr, c"glVertexAttribI1iv".as_ptr()) as PFNGLVERTEXATTRIBI1IVPROC);
            self.glVertexAttribI1ui = load!((load)(userptr, c"glVertexAttribI1ui".as_ptr()) as PFNGLVERTEXATTRIBI1UIPROC);
            self.glVertexAttribI1uiv = load!((load)(userptr, c"glVertexAttribI1uiv".as_ptr()) as PFNGLVERTEXATTRIBI1UIVPROC);
            self.glVertexAttribI2i = load!((load)(userptr, c"glVertexAttribI2i".as_ptr()) as PFNGLVERTEXATTRIBI2IPROC);
            self.glVertexAttribI2iv = load!((load)(userptr, c"glVertexAttribI2iv".as_ptr()) as PFNGLVERTEXATTRIBI2IVPROC);
            self.glVertexAttribI2ui = load!((load)(userptr, c"glVertexAttribI2ui".as_ptr()) as PFNGLVERTEXATTRIBI2UIPROC);
            self.glVertexAttribI2uiv = load!((load)(userptr, c"glVertexAttribI2uiv".as_ptr()) as PFNGLVERTEXATTRIBI2UIVPROC);
            self.glVertexAttribI3i = load!((load)(userptr, c"glVertexAttribI3i".as_ptr()) as PFNGLVERTEXATTRIBI3IPROC);
            self.glVertexAttribI3iv = load!((load)(userptr, c"glVertexAttribI3iv".as_ptr()) as PFNGLVERTEXATTRIBI3IVPROC);
            self.glVertexAttribI3ui = load!((load)(userptr, c"glVertexAttribI3ui".as_ptr()) as PFNGLVERTEXATTRIBI3UIPROC);
            self.glVertexAttribI3uiv = load!((load)(userptr, c"glVertexAttribI3uiv".as_ptr()) as PFNGLVERTEXATTRIBI3UIVPROC);
            self.glVertexAttribI4bv = load!((load)(userptr, c"glVertexAttribI4bv".as_ptr()) as PFNGLVERTEXATTRIBI4BVPROC);
            self.glVertexAttribI4i = load!((load)(userptr, c"glVertexAttribI4i".as_ptr()) as PFNGLVERTEXATTRIBI4IPROC);
            self.glVertexAttribI4iv = load!((load)(userptr, c"glVertexAttribI4iv".as_ptr()) as PFNGLVERTEXATTRIBI4IVPROC);
            self.glVertexAttribI4sv = load!((load)(userptr, c"glVertexAttribI4sv".as_ptr()) as PFNGLVERTEXATTRIBI4SVPROC);
            self.glVertexAttribI4ubv = load!((load)(userptr, c"glVertexAttribI4ubv".as_ptr()) as PFNGLVERTEXATTRIBI4UBVPROC);
            self.glVertexAttribI4ui = load!((load)(userptr, c"glVertexAttribI4ui".as_ptr()) as PFNGLVERTEXATTRIBI4UIPROC);
            self.glVertexAttribI4uiv = load!((load)(userptr, c"glVertexAttribI4uiv".as_ptr()) as PFNGLVERTEXATTRIBI4UIVPROC);
            self.glVertexAttribI4usv = load!((load)(userptr, c"glVertexAttribI4usv".as_ptr()) as PFNGLVERTEXATTRIBI4USVPROC);
            self.glVertexAttribIPointer = load!((load)(userptr, c"glVertexAttribIPointer".as_ptr()) as PFNGLVERTEXATTRIBIPOINTERPROC);
        }
    }
    unsafe fn glad_gl_load_GL_VERSION_3_1(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_3_1 { return; }
            self.glBindBufferBase = load!((load)(userptr, c"glBindBufferBase".as_ptr()) as PFNGLBINDBUFFERBASEPROC);
            self.glBindBufferRange = load!((load)(userptr, c"glBindBufferRange".as_ptr()) as PFNGLBINDBUFFERRANGEPROC);
            self.glCopyBufferSubData = load!((load)(userptr, c"glCopyBufferSubData".as_ptr()) as PFNGLCOPYBUFFERSUBDATAPROC);
            self.glDrawArraysInstanced = load!((load)(userptr, c"glDrawArraysInstanced".as_ptr()) as PFNGLDRAWARRAYSINSTANCEDPROC);
            self.glDrawElementsInstanced = load!((load)(userptr, c"glDrawElementsInstanced".as_ptr()) as PFNGLDRAWELEMENTSINSTANCEDPROC);
            self.glGetActiveUniformBlockName = load!((load)(userptr, c"glGetActiveUniformBlockName".as_ptr()) as PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC);
            self.glGetActiveUniformBlockiv = load!((load)(userptr, c"glGetActiveUniformBlockiv".as_ptr()) as PFNGLGETACTIVEUNIFORMBLOCKIVPROC);
            self.glGetActiveUniformName = load!((load)(userptr, c"glGetActiveUniformName".as_ptr()) as PFNGLGETACTIVEUNIFORMNAMEPROC);
            self.glGetActiveUniformsiv = load!((load)(userptr, c"glGetActiveUniformsiv".as_ptr()) as PFNGLGETACTIVEUNIFORMSIVPROC);
            self.glGetIntegeri_v = load!((load)(userptr, c"glGetIntegeri_v".as_ptr()) as PFNGLGETINTEGERI_VPROC);
            self.glGetUniformBlockIndex = load!((load)(userptr, c"glGetUniformBlockIndex".as_ptr()) as PFNGLGETUNIFORMBLOCKINDEXPROC);
            self.glGetUniformIndices = load!((load)(userptr, c"glGetUniformIndices".as_ptr()) as PFNGLGETUNIFORMINDICESPROC);
            self.glPrimitiveRestartIndex = load!((load)(userptr, c"glPrimitiveRestartIndex".as_ptr()) as PFNGLPRIMITIVERESTARTINDEXPROC);
            self.glTexBuffer = load!((load)(userptr, c"glTexBuffer".as_ptr()) as PFNGLTEXBUFFERPROC);
            self.glUniformBlockBinding = load!((load)(userptr, c"glUniformBlockBinding".as_ptr()) as PFNGLUNIFORMBLOCKBINDINGPROC);
        }
    }
    unsafe fn glad_gl_load_GL_VERSION_3_2(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_3_2 { return; }
            self.glClientWaitSync = load!((load)(userptr, c"glClientWaitSync".as_ptr()) as PFNGLCLIENTWAITSYNCPROC);
            self.glDeleteSync = load!((load)(userptr, c"glDeleteSync".as_ptr()) as PFNGLDELETESYNCPROC);
            self.glDrawElementsBaseVertex = load!((load)(userptr, c"glDrawElementsBaseVertex".as_ptr()) as PFNGLDRAWELEMENTSBASEVERTEXPROC);
            self.glDrawElementsInstancedBaseVertex = load!((load)(userptr, c"glDrawElementsInstancedBaseVertex".as_ptr()) as PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC);
            self.glDrawRangeElementsBaseVertex = load!((load)(userptr, c"glDrawRangeElementsBaseVertex".as_ptr()) as PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC);
            self.glFenceSync = load!((load)(userptr, c"glFenceSync".as_ptr()) as PFNGLFENCESYNCPROC);
            self.glFramebufferTexture = load!((load)(userptr, c"glFramebufferTexture".as_ptr()) as PFNGLFRAMEBUFFERTEXTUREPROC);
            self.glGetBufferParameteri64v = load!((load)(userptr, c"glGetBufferParameteri64v".as_ptr()) as PFNGLGETBUFFERPARAMETERI64VPROC);
            self.glGetInteger64i_v = load!((load)(userptr, c"glGetInteger64i_v".as_ptr()) as PFNGLGETINTEGER64I_VPROC);
            self.glGetInteger64v = load!((load)(userptr, c"glGetInteger64v".as_ptr()) as PFNGLGETINTEGER64VPROC);
            self.glGetMultisamplefv = load!((load)(userptr, c"glGetMultisamplefv".as_ptr()) as PFNGLGETMULTISAMPLEFVPROC);
            self.glGetSynciv = load!((load)(userptr, c"glGetSynciv".as_ptr()) as PFNGLGETSYNCIVPROC);
            self.glIsSync = load!((load)(userptr, c"glIsSync".as_ptr()) as PFNGLISSYNCPROC);
            self.glMultiDrawElementsBaseVertex = load!((load)(userptr, c"glMultiDrawElementsBaseVertex".as_ptr()) as PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC);
            self.glProvokingVertex = load!((load)(userptr, c"glProvokingVertex".as_ptr()) as PFNGLPROVOKINGVERTEXPROC);
            self.glSampleMaski = load!((load)(userptr, c"glSampleMaski".as_ptr()) as PFNGLSAMPLEMASKIPROC);
            self.glTexImage2DMultisample = load!((load)(userptr, c"glTexImage2DMultisample".as_ptr()) as PFNGLTEXIMAGE2DMULTISAMPLEPROC);
            self.glTexImage3DMultisample = load!((load)(userptr, c"glTexImage3DMultisample".as_ptr()) as PFNGLTEXIMAGE3DMULTISAMPLEPROC);
            self.glWaitSync = load!((load)(userptr, c"glWaitSync".as_ptr()) as PFNGLWAITSYNCPROC);
        }
    }
    unsafe fn glad_gl_load_GL_VERSION_3_3(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_3_3 { return; }
            self.glBindFragDataLocationIndexed = load!((load)(userptr, c"glBindFragDataLocationIndexed".as_ptr()) as PFNGLBINDFRAGDATALOCATIONINDEXEDPROC);
            self.glBindSampler = load!((load)(userptr, c"glBindSampler".as_ptr()) as PFNGLBINDSAMPLERPROC);
            self.glDeleteSamplers = load!((load)(userptr, c"glDeleteSamplers".as_ptr()) as PFNGLDELETESAMPLERSPROC);
            self.glGenSamplers = load!((load)(userptr, c"glGenSamplers".as_ptr()) as PFNGLGENSAMPLERSPROC);
            self.glGetFragDataIndex = load!((load)(userptr, c"glGetFragDataIndex".as_ptr()) as PFNGLGETFRAGDATAINDEXPROC);
            self.glGetQueryObjecti64v = load!((load)(userptr, c"glGetQueryObjecti64v".as_ptr()) as PFNGLGETQUERYOBJECTI64VPROC);
            self.glGetQueryObjectui64v = load!((load)(userptr, c"glGetQueryObjectui64v".as_ptr()) as PFNGLGETQUERYOBJECTUI64VPROC);
            self.glGetSamplerParameterIiv = load!((load)(userptr, c"glGetSamplerParameterIiv".as_ptr()) as PFNGLGETSAMPLERPARAMETERIIVPROC);
            self.glGetSamplerParameterIuiv = load!((load)(userptr, c"glGetSamplerParameterIuiv".as_ptr()) as PFNGLGETSAMPLERPARAMETERIUIVPROC);
            self.glGetSamplerParameterfv = load!((load)(userptr, c"glGetSamplerParameterfv".as_ptr()) as PFNGLGETSAMPLERPARAMETERFVPROC);
            self.glGetSamplerParameteriv = load!((load)(userptr, c"glGetSamplerParameteriv".as_ptr()) as PFNGLGETSAMPLERPARAMETERIVPROC);
            self.glIsSampler = load!((load)(userptr, c"glIsSampler".as_ptr()) as PFNGLISSAMPLERPROC);
            self.glQueryCounter = load!((load)(userptr, c"glQueryCounter".as_ptr()) as PFNGLQUERYCOUNTERPROC);
            self.glSamplerParameterIiv = load!((load)(userptr, c"glSamplerParameterIiv".as_ptr()) as PFNGLSAMPLERPARAMETERIIVPROC);
            self.glSamplerParameterIuiv = load!((load)(userptr, c"glSamplerParameterIuiv".as_ptr()) as PFNGLSAMPLERPARAMETERIUIVPROC);
            self.glSamplerParameterf = load!((load)(userptr, c"glSamplerParameterf".as_ptr()) as PFNGLSAMPLERPARAMETERFPROC);
            self.glSamplerParameterfv = load!((load)(userptr, c"glSamplerParameterfv".as_ptr()) as PFNGLSAMPLERPARAMETERFVPROC);
            self.glSamplerParameteri = load!((load)(userptr, c"glSamplerParameteri".as_ptr()) as PFNGLSAMPLERPARAMETERIPROC);
            self.glSamplerParameteriv = load!((load)(userptr, c"glSamplerParameteriv".as_ptr()) as PFNGLSAMPLERPARAMETERIVPROC);
            self.glVertexAttribDivisor = load!((load)(userptr, c"glVertexAttribDivisor".as_ptr()) as PFNGLVERTEXATTRIBDIVISORPROC);
            self.glVertexAttribP1ui = load!((load)(userptr, c"glVertexAttribP1ui".as_ptr()) as PFNGLVERTEXATTRIBP1UIPROC);
            self.glVertexAttribP1uiv = load!((load)(userptr, c"glVertexAttribP1uiv".as_ptr()) as PFNGLVERTEXATTRIBP1UIVPROC);
            self.glVertexAttribP2ui = load!((load)(userptr, c"glVertexAttribP2ui".as_ptr()) as PFNGLVERTEXATTRIBP2UIPROC);
            self.glVertexAttribP2uiv = load!((load)(userptr, c"glVertexAttribP2uiv".as_ptr()) as PFNGLVERTEXATTRIBP2UIVPROC);
            self.glVertexAttribP3ui = load!((load)(userptr, c"glVertexAttribP3ui".as_ptr()) as PFNGLVERTEXATTRIBP3UIPROC);
            self.glVertexAttribP3uiv = load!((load)(userptr, c"glVertexAttribP3uiv".as_ptr()) as PFNGLVERTEXATTRIBP3UIVPROC);
            self.glVertexAttribP4ui = load!((load)(userptr, c"glVertexAttribP4ui".as_ptr()) as PFNGLVERTEXATTRIBP4UIPROC);
            self.glVertexAttribP4uiv = load!((load)(userptr, c"glVertexAttribP4uiv".as_ptr()) as PFNGLVERTEXATTRIBP4UIVPROC);
        }
    }
    unsafe fn glad_gl_load_GL_VERSION_4_0(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_4_0 { return; }
            self.glBeginQueryIndexed = load!((load)(userptr, c"glBeginQueryIndexed".as_ptr()) as PFNGLBEGINQUERYINDEXEDPROC);
            self.glBindTransformFeedback = load!((load)(userptr, c"glBindTransformFeedback".as_ptr()) as PFNGLBINDTRANSFORMFEEDBACKPROC);
            self.glBlendEquationSeparatei = load!((load)(userptr, c"glBlendEquationSeparatei".as_ptr()) as PFNGLBLENDEQUATIONSEPARATEIPROC);
            self.glBlendEquationi = load!((load)(userptr, c"glBlendEquationi".as_ptr()) as PFNGLBLENDEQUATIONIPROC);
            self.glBlendFuncSeparatei = load!((load)(userptr, c"glBlendFuncSeparatei".as_ptr()) as PFNGLBLENDFUNCSEPARATEIPROC);
            self.glBlendFunci = load!((load)(userptr, c"glBlendFunci".as_ptr()) as PFNGLBLENDFUNCIPROC);
            self.glDeleteTransformFeedbacks = load!((load)(userptr, c"glDeleteTransformFeedbacks".as_ptr()) as PFNGLDELETETRANSFORMFEEDBACKSPROC);
            self.glDrawArraysIndirect = load!((load)(userptr, c"glDrawArraysIndirect".as_ptr()) as PFNGLDRAWARRAYSINDIRECTPROC);
            self.glDrawElementsIndirect = load!((load)(userptr, c"glDrawElementsIndirect".as_ptr()) as PFNGLDRAWELEMENTSINDIRECTPROC);
            self.glDrawTransformFeedback = load!((load)(userptr, c"glDrawTransformFeedback".as_ptr()) as PFNGLDRAWTRANSFORMFEEDBACKPROC);
            self.glDrawTransformFeedbackStream = load!((load)(userptr, c"glDrawTransformFeedbackStream".as_ptr()) as PFNGLDRAWTRANSFORMFEEDBACKSTREAMPROC);
            self.glEndQueryIndexed = load!((load)(userptr, c"glEndQueryIndexed".as_ptr()) as PFNGLENDQUERYINDEXEDPROC);
            self.glGenTransformFeedbacks = load!((load)(userptr, c"glGenTransformFeedbacks".as_ptr()) as PFNGLGENTRANSFORMFEEDBACKSPROC);
            self.glGetActiveSubroutineName = load!((load)(userptr, c"glGetActiveSubroutineName".as_ptr()) as PFNGLGETACTIVESUBROUTINENAMEPROC);
            self.glGetActiveSubroutineUniformName = load!((load)(userptr, c"glGetActiveSubroutineUniformName".as_ptr()) as PFNGLGETACTIVESUBROUTINEUNIFORMNAMEPROC);
            self.glGetActiveSubroutineUniformiv = load!((load)(userptr, c"glGetActiveSubroutineUniformiv".as_ptr()) as PFNGLGETACTIVESUBROUTINEUNIFORMIVPROC);
            self.glGetProgramStageiv = load!((load)(userptr, c"glGetProgramStageiv".as_ptr()) as PFNGLGETPROGRAMSTAGEIVPROC);
            self.glGetQueryIndexediv = load!((load)(userptr, c"glGetQueryIndexediv".as_ptr()) as PFNGLGETQUERYINDEXEDIVPROC);
            self.glGetSubroutineIndex = load!((load)(userptr, c"glGetSubroutineIndex".as_ptr()) as PFNGLGETSUBROUTINEINDEXPROC);
            self.glGetSubroutineUniformLocation = load!((load)(userptr, c"glGetSubroutineUniformLocation".as_ptr()) as PFNGLGETSUBROUTINEUNIFORMLOCATIONPROC);
            self.glGetUniformSubroutineuiv = load!((load)(userptr, c"glGetUniformSubroutineuiv".as_ptr()) as PFNGLGETUNIFORMSUBROUTINEUIVPROC);
            self.glGetUniformdv = load!((load)(userptr, c"glGetUniformdv".as_ptr()) as PFNGLGETUNIFORMDVPROC);
            self.glIsTransformFeedback = load!((load)(userptr, c"glIsTransformFeedback".as_ptr()) as PFNGLISTRANSFORMFEEDBACKPROC);
            self.glMinSampleShading = load!((load)(userptr, c"glMinSampleShading".as_ptr()) as PFNGLMINSAMPLESHADINGPROC);
            self.glPatchParameterfv = load!((load)(userptr, c"glPatchParameterfv".as_ptr()) as PFNGLPATCHPARAMETERFVPROC);
            self.glPatchParameteri = load!((load)(userptr, c"glPatchParameteri".as_ptr()) as PFNGLPATCHPARAMETERIPROC);
            self.glPauseTransformFeedback = load!((load)(userptr, c"glPauseTransformFeedback".as_ptr()) as PFNGLPAUSETRANSFORMFEEDBACKPROC);
            self.glResumeTransformFeedback = load!((load)(userptr, c"glResumeTransformFeedback".as_ptr()) as PFNGLRESUMETRANSFORMFEEDBACKPROC);
            self.glUniform1d = load!((load)(userptr, c"glUniform1d".as_ptr()) as PFNGLUNIFORM1DPROC);
            self.glUniform1dv = load!((load)(userptr, c"glUniform1dv".as_ptr()) as PFNGLUNIFORM1DVPROC);
            self.glUniform2d = load!((load)(userptr, c"glUniform2d".as_ptr()) as PFNGLUNIFORM2DPROC);
            self.glUniform2dv = load!((load)(userptr, c"glUniform2dv".as_ptr()) as PFNGLUNIFORM2DVPROC);
            self.glUniform3d = load!((load)(userptr, c"glUniform3d".as_ptr()) as PFNGLUNIFORM3DPROC);
            self.glUniform3dv = load!((load)(userptr, c"glUniform3dv".as_ptr()) as PFNGLUNIFORM3DVPROC);
            self.glUniform4d = load!((load)(userptr, c"glUniform4d".as_ptr()) as PFNGLUNIFORM4DPROC);
            self.glUniform4dv = load!((load)(userptr, c"glUniform4dv".as_ptr()) as PFNGLUNIFORM4DVPROC);
            self.glUniformMatrix2dv = load!((load)(userptr, c"glUniformMatrix2dv".as_ptr()) as PFNGLUNIFORMMATRIX2DVPROC);
            self.glUniformMatrix2x3dv = load!((load)(userptr, c"glUniformMatrix2x3dv".as_ptr()) as PFNGLUNIFORMMATRIX2X3DVPROC);
            self.glUniformMatrix2x4dv = load!((load)(userptr, c"glUniformMatrix2x4dv".as_ptr()) as PFNGLUNIFORMMATRIX2X4DVPROC);
            self.glUniformMatrix3dv = load!((load)(userptr, c"glUniformMatrix3dv".as_ptr()) as PFNGLUNIFORMMATRIX3DVPROC);
            self.glUniformMatrix3x2dv = load!((load)(userptr, c"glUniformMatrix3x2dv".as_ptr()) as PFNGLUNIFORMMATRIX3X2DVPROC);
            self.glUniformMatrix3x4dv = load!((load)(userptr, c"glUniformMatrix3x4dv".as_ptr()) as PFNGLUNIFORMMATRIX3X4DVPROC);
            self.glUniformMatrix4dv = load!((load)(userptr, c"glUniformMatrix4dv".as_ptr()) as PFNGLUNIFORMMATRIX4DVPROC);
            self.glUniformMatrix4x2dv = load!((load)(userptr, c"glUniformMatrix4x2dv".as_ptr()) as PFNGLUNIFORMMATRIX4X2DVPROC);
            self.glUniformMatrix4x3dv = load!((load)(userptr, c"glUniformMatrix4x3dv".as_ptr()) as PFNGLUNIFORMMATRIX4X3DVPROC);
            self.glUniformSubroutinesuiv = load!((load)(userptr, c"glUniformSubroutinesuiv".as_ptr()) as PFNGLUNIFORMSUBROUTINESUIVPROC);
        }
    }
    unsafe fn glad_gl_load_GL_VERSION_4_1(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_4_1 { return; }
            self.glActiveShaderProgram = load!((load)(userptr, c"glActiveShaderProgram".as_ptr()) as PFNGLACTIVESHADERPROGRAMPROC);
            self.glBindProgramPipeline = load!((load)(userptr, c"glBindProgramPipeline".as_ptr()) as PFNGLBINDPROGRAMPIPELINEPROC);
            self.glClearDepthf = load!((load)(userptr, c"glClearDepthf".as_ptr()) as PFNGLCLEARDEPTHFPROC);
            self.glCreateShaderProgramv = load!((load)(userptr, c"glCreateShaderProgramv".as_ptr()) as PFNGLCREATESHADERPROGRAMVPROC);
            self.glDeleteProgramPipelines = load!((load)(userptr, c"glDeleteProgramPipelines".as_ptr()) as PFNGLDELETEPROGRAMPIPELINESPROC);
            self.glDepthRangeArrayv = load!((load)(userptr, c"glDepthRangeArrayv".as_ptr()) as PFNGLDEPTHRANGEARRAYVPROC);
            self.glDepthRangeIndexed = load!((load)(userptr, c"glDepthRangeIndexed".as_ptr()) as PFNGLDEPTHRANGEINDEXEDPROC);
            self.glDepthRangef = load!((load)(userptr, c"glDepthRangef".as_ptr()) as PFNGLDEPTHRANGEFPROC);
            self.glGenProgramPipelines = load!((load)(userptr, c"glGenProgramPipelines".as_ptr()) as PFNGLGENPROGRAMPIPELINESPROC);
            self.glGetDoublei_v = load!((load)(userptr, c"glGetDoublei_v".as_ptr()) as PFNGLGETDOUBLEI_VPROC);
            self.glGetFloati_v = load!((load)(userptr, c"glGetFloati_v".as_ptr()) as PFNGLGETFLOATI_VPROC);
            self.glGetProgramBinary = load!((load)(userptr, c"glGetProgramBinary".as_ptr()) as PFNGLGETPROGRAMBINARYPROC);
            self.glGetProgramPipelineInfoLog = load!((load)(userptr, c"glGetProgramPipelineInfoLog".as_ptr()) as PFNGLGETPROGRAMPIPELINEINFOLOGPROC);
            self.glGetProgramPipelineiv = load!((load)(userptr, c"glGetProgramPipelineiv".as_ptr()) as PFNGLGETPROGRAMPIPELINEIVPROC);
            self.glGetShaderPrecisionFormat = load!((load)(userptr, c"glGetShaderPrecisionFormat".as_ptr()) as PFNGLGETSHADERPRECISIONFORMATPROC);
            self.glGetVertexAttribLdv = load!((load)(userptr, c"glGetVertexAttribLdv".as_ptr()) as PFNGLGETVERTEXATTRIBLDVPROC);
            self.glIsProgramPipeline = load!((load)(userptr, c"glIsProgramPipeline".as_ptr()) as PFNGLISPROGRAMPIPELINEPROC);
            self.glProgramBinary = load!((load)(userptr, c"glProgramBinary".as_ptr()) as PFNGLPROGRAMBINARYPROC);
            self.glProgramParameteri = load!((load)(userptr, c"glProgramParameteri".as_ptr()) as PFNGLPROGRAMPARAMETERIPROC);
            self.glProgramUniform1d = load!((load)(userptr, c"glProgramUniform1d".as_ptr()) as PFNGLPROGRAMUNIFORM1DPROC);
            self.glProgramUniform1dv = load!((load)(userptr, c"glProgramUniform1dv".as_ptr()) as PFNGLPROGRAMUNIFORM1DVPROC);
            self.glProgramUniform1f = load!((load)(userptr, c"glProgramUniform1f".as_ptr()) as PFNGLPROGRAMUNIFORM1FPROC);
            self.glProgramUniform1fv = load!((load)(userptr, c"glProgramUniform1fv".as_ptr()) as PFNGLPROGRAMUNIFORM1FVPROC);
            self.glProgramUniform1i = load!((load)(userptr, c"glProgramUniform1i".as_ptr()) as PFNGLPROGRAMUNIFORM1IPROC);
            self.glProgramUniform1iv = load!((load)(userptr, c"glProgramUniform1iv".as_ptr()) as PFNGLPROGRAMUNIFORM1IVPROC);
            self.glProgramUniform1ui = load!((load)(userptr, c"glProgramUniform1ui".as_ptr()) as PFNGLPROGRAMUNIFORM1UIPROC);
            self.glProgramUniform1uiv = load!((load)(userptr, c"glProgramUniform1uiv".as_ptr()) as PFNGLPROGRAMUNIFORM1UIVPROC);
            self.glProgramUniform2d = load!((load)(userptr, c"glProgramUniform2d".as_ptr()) as PFNGLPROGRAMUNIFORM2DPROC);
            self.glProgramUniform2dv = load!((load)(userptr, c"glProgramUniform2dv".as_ptr()) as PFNGLPROGRAMUNIFORM2DVPROC);
            self.glProgramUniform2f = load!((load)(userptr, c"glProgramUniform2f".as_ptr()) as PFNGLPROGRAMUNIFORM2FPROC);
            self.glProgramUniform2fv = load!((load)(userptr, c"glProgramUniform2fv".as_ptr()) as PFNGLPROGRAMUNIFORM2FVPROC);
            self.glProgramUniform2i = load!((load)(userptr, c"glProgramUniform2i".as_ptr()) as PFNGLPROGRAMUNIFORM2IPROC);
            self.glProgramUniform2iv = load!((load)(userptr, c"glProgramUniform2iv".as_ptr()) as PFNGLPROGRAMUNIFORM2IVPROC);
            self.glProgramUniform2ui = load!((load)(userptr, c"glProgramUniform2ui".as_ptr()) as PFNGLPROGRAMUNIFORM2UIPROC);
            self.glProgramUniform2uiv = load!((load)(userptr, c"glProgramUniform2uiv".as_ptr()) as PFNGLPROGRAMUNIFORM2UIVPROC);
            self.glProgramUniform3d = load!((load)(userptr, c"glProgramUniform3d".as_ptr()) as PFNGLPROGRAMUNIFORM3DPROC);
            self.glProgramUniform3dv = load!((load)(userptr, c"glProgramUniform3dv".as_ptr()) as PFNGLPROGRAMUNIFORM3DVPROC);
            self.glProgramUniform3f = load!((load)(userptr, c"glProgramUniform3f".as_ptr()) as PFNGLPROGRAMUNIFORM3FPROC);
            self.glProgramUniform3fv = load!((load)(userptr, c"glProgramUniform3fv".as_ptr()) as PFNGLPROGRAMUNIFORM3FVPROC);
            self.glProgramUniform3i = load!((load)(userptr, c"glProgramUniform3i".as_ptr()) as PFNGLPROGRAMUNIFORM3IPROC);
            self.glProgramUniform3iv = load!((load)(userptr, c"glProgramUniform3iv".as_ptr()) as PFNGLPROGRAMUNIFORM3IVPROC);
            self.glProgramUniform3ui = load!((load)(userptr, c"glProgramUniform3ui".as_ptr()) as PFNGLPROGRAMUNIFORM3UIPROC);
            self.glProgramUniform3uiv = load!((load)(userptr, c"glProgramUniform3uiv".as_ptr()) as PFNGLPROGRAMUNIFORM3UIVPROC);
            self.glProgramUniform4d = load!((load)(userptr, c"glProgramUniform4d".as_ptr()) as PFNGLPROGRAMUNIFORM4DPROC);
            self.glProgramUniform4dv = load!((load)(userptr, c"glProgramUniform4dv".as_ptr()) as PFNGLPROGRAMUNIFORM4DVPROC);
            self.glProgramUniform4f = load!((load)(userptr, c"glProgramUniform4f".as_ptr()) as PFNGLPROGRAMUNIFORM4FPROC);
            self.glProgramUniform4fv = load!((load)(userptr, c"glProgramUniform4fv".as_ptr()) as PFNGLPROGRAMUNIFORM4FVPROC);
            self.glProgramUniform4i = load!((load)(userptr, c"glProgramUniform4i".as_ptr()) as PFNGLPROGRAMUNIFORM4IPROC);
            self.glProgramUniform4iv = load!((load)(userptr, c"glProgramUniform4iv".as_ptr()) as PFNGLPROGRAMUNIFORM4IVPROC);
            self.glProgramUniform4ui = load!((load)(userptr, c"glProgramUniform4ui".as_ptr()) as PFNGLPROGRAMUNIFORM4UIPROC);
            self.glProgramUniform4uiv = load!((load)(userptr, c"glProgramUniform4uiv".as_ptr()) as PFNGLPROGRAMUNIFORM4UIVPROC);
            self.glProgramUniformMatrix2dv = load!((load)(userptr, c"glProgramUniformMatrix2dv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX2DVPROC);
            self.glProgramUniformMatrix2fv = load!((load)(userptr, c"glProgramUniformMatrix2fv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX2FVPROC);
            self.glProgramUniformMatrix2x3dv = load!((load)(userptr, c"glProgramUniformMatrix2x3dv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX2X3DVPROC);
            self.glProgramUniformMatrix2x3fv = load!((load)(userptr, c"glProgramUniformMatrix2x3fv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX2X3FVPROC);
            self.glProgramUniformMatrix2x4dv = load!((load)(userptr, c"glProgramUniformMatrix2x4dv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX2X4DVPROC);
            self.glProgramUniformMatrix2x4fv = load!((load)(userptr, c"glProgramUniformMatrix2x4fv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX2X4FVPROC);
            self.glProgramUniformMatrix3dv = load!((load)(userptr, c"glProgramUniformMatrix3dv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX3DVPROC);
            self.glProgramUniformMatrix3fv = load!((load)(userptr, c"glProgramUniformMatrix3fv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX3FVPROC);
            self.glProgramUniformMatrix3x2dv = load!((load)(userptr, c"glProgramUniformMatrix3x2dv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX3X2DVPROC);
            self.glProgramUniformMatrix3x2fv = load!((load)(userptr, c"glProgramUniformMatrix3x2fv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX3X2FVPROC);
            self.glProgramUniformMatrix3x4dv = load!((load)(userptr, c"glProgramUniformMatrix3x4dv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX3X4DVPROC);
            self.glProgramUniformMatrix3x4fv = load!((load)(userptr, c"glProgramUniformMatrix3x4fv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX3X4FVPROC);
            self.glProgramUniformMatrix4dv = load!((load)(userptr, c"glProgramUniformMatrix4dv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX4DVPROC);
            self.glProgramUniformMatrix4fv = load!((load)(userptr, c"glProgramUniformMatrix4fv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX4FVPROC);
            self.glProgramUniformMatrix4x2dv = load!((load)(userptr, c"glProgramUniformMatrix4x2dv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX4X2DVPROC);
            self.glProgramUniformMatrix4x2fv = load!((load)(userptr, c"glProgramUniformMatrix4x2fv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX4X2FVPROC);
            self.glProgramUniformMatrix4x3dv = load!((load)(userptr, c"glProgramUniformMatrix4x3dv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX4X3DVPROC);
            self.glProgramUniformMatrix4x3fv = load!((load)(userptr, c"glProgramUniformMatrix4x3fv".as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX4X3FVPROC);
            self.glReleaseShaderCompiler = load!((load)(userptr, c"glReleaseShaderCompiler".as_ptr()) as PFNGLRELEASESHADERCOMPILERPROC);
            self.glScissorArrayv = load!((load)(userptr, c"glScissorArrayv".as_ptr()) as PFNGLSCISSORARRAYVPROC);
            self.glScissorIndexed = load!((load)(userptr, c"glScissorIndexed".as_ptr()) as PFNGLSCISSORINDEXEDPROC);
            self.glScissorIndexedv = load!((load)(userptr, c"glScissorIndexedv".as_ptr()) as PFNGLSCISSORINDEXEDVPROC);
            self.glShaderBinary = load!((load)(userptr, c"glShaderBinary".as_ptr()) as PFNGLSHADERBINARYPROC);
            self.glUseProgramStages = load!((load)(userptr, c"glUseProgramStages".as_ptr()) as PFNGLUSEPROGRAMSTAGESPROC);
            self.glValidateProgramPipeline = load!((load)(userptr, c"glValidateProgramPipeline".as_ptr()) as PFNGLVALIDATEPROGRAMPIPELINEPROC);
            self.glVertexAttribL1d = load!((load)(userptr, c"glVertexAttribL1d".as_ptr()) as PFNGLVERTEXATTRIBL1DPROC);
            self.glVertexAttribL1dv = load!((load)(userptr, c"glVertexAttribL1dv".as_ptr()) as PFNGLVERTEXATTRIBL1DVPROC);
            self.glVertexAttribL2d = load!((load)(userptr, c"glVertexAttribL2d".as_ptr()) as PFNGLVERTEXATTRIBL2DPROC);
            self.glVertexAttribL2dv = load!((load)(userptr, c"glVertexAttribL2dv".as_ptr()) as PFNGLVERTEXATTRIBL2DVPROC);
            self.glVertexAttribL3d = load!((load)(userptr, c"glVertexAttribL3d".as_ptr()) as PFNGLVERTEXATTRIBL3DPROC);
            self.glVertexAttribL3dv = load!((load)(userptr, c"glVertexAttribL3dv".as_ptr()) as PFNGLVERTEXATTRIBL3DVPROC);
            self.glVertexAttribL4d = load!((load)(userptr, c"glVertexAttribL4d".as_ptr()) as PFNGLVERTEXATTRIBL4DPROC);
            self.glVertexAttribL4dv = load!((load)(userptr, c"glVertexAttribL4dv".as_ptr()) as PFNGLVERTEXATTRIBL4DVPROC);
            self.glVertexAttribLPointer = load!((load)(userptr, c"glVertexAttribLPointer".as_ptr()) as PFNGLVERTEXATTRIBLPOINTERPROC);
            self.glViewportArrayv = load!((load)(userptr, c"glViewportArrayv".as_ptr()) as PFNGLVIEWPORTARRAYVPROC);
            self.glViewportIndexedf = load!((load)(userptr, c"glViewportIndexedf".as_ptr()) as PFNGLVIEWPORTINDEXEDFPROC);
            self.glViewportIndexedfv = load!((load)(userptr, c"glViewportIndexedfv".as_ptr()) as PFNGLVIEWPORTINDEXEDFVPROC);
        }
    }
    unsafe fn glad_gl_load_GL_VERSION_4_2(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_4_2 { return; }
            self.glBindImageTexture = load!((load)(userptr, c"glBindImageTexture".as_ptr()) as PFNGLBINDIMAGETEXTUREPROC);
            self.glDrawArraysInstancedBaseInstance = load!((load)(userptr, c"glDrawArraysInstancedBaseInstance".as_ptr()) as PFNGLDRAWARRAYSINSTANCEDBASEINSTANCEPROC);
            self.glDrawElementsInstancedBaseInstance = load!((load)(userptr, c"glDrawElementsInstancedBaseInstance".as_ptr()) as PFNGLDRAWELEMENTSINSTANCEDBASEINSTANCEPROC);
            self.glDrawElementsInstancedBaseVertexBaseInstance = load!((load)(userptr, c"glDrawElementsInstancedBaseVertexBaseInstance".as_ptr()) as PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXBASEINSTANCEPROC);
            self.glDrawTransformFeedbackInstanced = load!((load)(userptr, c"glDrawTransformFeedbackInstanced".as_ptr()) as PFNGLDRAWTRANSFORMFEEDBACKINSTANCEDPROC);
            self.glDrawTransformFeedbackStreamInstanced = load!((load)(userptr, c"glDrawTransformFeedbackStreamInstanced".as_ptr()) as PFNGLDRAWTRANSFORMFEEDBACKSTREAMINSTANCEDPROC);
            self.glGetActiveAtomicCounterBufferiv = load!((load)(userptr, c"glGetActiveAtomicCounterBufferiv".as_ptr()) as PFNGLGETACTIVEATOMICCOUNTERBUFFERIVPROC);
            self.glGetInternalformativ = load!((load)(userptr, c"glGetInternalformativ".as_ptr()) as PFNGLGETINTERNALFORMATIVPROC);
            self.glMemoryBarrier = load!((load)(userptr, c"glMemoryBarrier".as_ptr()) as PFNGLMEMORYBARRIERPROC);
            self.glTexStorage1D = load!((load)(userptr, c"glTexStorage1D".as_ptr()) as PFNGLTEXSTORAGE1DPROC);
            self.glTexStorage2D = load!((load)(userptr, c"glTexStorage2D".as_ptr()) as PFNGLTEXSTORAGE2DPROC);
            self.glTexStorage3D = load!((load)(userptr, c"glTexStorage3D".as_ptr()) as PFNGLTEXSTORAGE3DPROC);
        }
    }
    unsafe fn glad_gl_load_GL_VERSION_4_3(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_4_3 { return; }
            self.glBindVertexBuffer = load!((load)(userptr, c"glBindVertexBuffer".as_ptr()) as PFNGLBINDVERTEXBUFFERPROC);
            self.glClearBufferData = load!((load)(userptr, c"glClearBufferData".as_ptr()) as PFNGLCLEARBUFFERDATAPROC);
            self.glClearBufferSubData = load!((load)(userptr, c"glClearBufferSubData".as_ptr()) as PFNGLCLEARBUFFERSUBDATAPROC);
            self.glCopyImageSubData = load!((load)(userptr, c"glCopyImageSubData".as_ptr()) as PFNGLCOPYIMAGESUBDATAPROC);
            self.glDebugMessageCallback = load!((load)(userptr, c"glDebugMessageCallback".as_ptr()) as PFNGLDEBUGMESSAGECALLBACKPROC);
            self.glDebugMessageControl = load!((load)(userptr, c"glDebugMessageControl".as_ptr()) as PFNGLDEBUGMESSAGECONTROLPROC);
            self.glDebugMessageInsert = load!((load)(userptr, c"glDebugMessageInsert".as_ptr()) as PFNGLDEBUGMESSAGEINSERTPROC);
            self.glDispatchCompute = load!((load)(userptr, c"glDispatchCompute".as_ptr()) as PFNGLDISPATCHCOMPUTEPROC);
            self.glDispatchComputeIndirect = load!((load)(userptr, c"glDispatchComputeIndirect".as_ptr()) as PFNGLDISPATCHCOMPUTEINDIRECTPROC);
            self.glFramebufferParameteri = load!((load)(userptr, c"glFramebufferParameteri".as_ptr()) as PFNGLFRAMEBUFFERPARAMETERIPROC);
            self.glGetDebugMessageLog = load!((load)(userptr, c"glGetDebugMessageLog".as_ptr()) as PFNGLGETDEBUGMESSAGELOGPROC);
            self.glGetFramebufferParameteriv = load!((load)(userptr, c"glGetFramebufferParameteriv".as_ptr()) as PFNGLGETFRAMEBUFFERPARAMETERIVPROC);
            self.glGetInternalformati64v = load!((load)(userptr, c"glGetInternalformati64v".as_ptr()) as PFNGLGETINTERNALFORMATI64VPROC);
            self.glGetObjectLabel = load!((load)(userptr, c"glGetObjectLabel".as_ptr()) as PFNGLGETOBJECTLABELPROC);
            self.glGetObjectPtrLabel = load!((load)(userptr, c"glGetObjectPtrLabel".as_ptr()) as PFNGLGETOBJECTPTRLABELPROC);
            self.glGetPointerv = load!((load)(userptr, c"glGetPointerv".as_ptr()) as PFNGLGETPOINTERVPROC);
            self.glGetProgramInterfaceiv = load!((load)(userptr, c"glGetProgramInterfaceiv".as_ptr()) as PFNGLGETPROGRAMINTERFACEIVPROC);
            self.glGetProgramResourceIndex = load!((load)(userptr, c"glGetProgramResourceIndex".as_ptr()) as PFNGLGETPROGRAMRESOURCEINDEXPROC);
            self.glGetProgramResourceLocation = load!((load)(userptr, c"glGetProgramResourceLocation".as_ptr()) as PFNGLGETPROGRAMRESOURCELOCATIONPROC);
            self.glGetProgramResourceLocationIndex = load!((load)(userptr, c"glGetProgramResourceLocationIndex".as_ptr()) as PFNGLGETPROGRAMRESOURCELOCATIONINDEXPROC);
            self.glGetProgramResourceName = load!((load)(userptr, c"glGetProgramResourceName".as_ptr()) as PFNGLGETPROGRAMRESOURCENAMEPROC);
            self.glGetProgramResourceiv = load!((load)(userptr, c"glGetProgramResourceiv".as_ptr()) as PFNGLGETPROGRAMRESOURCEIVPROC);
            self.glInvalidateBufferData = load!((load)(userptr, c"glInvalidateBufferData".as_ptr()) as PFNGLINVALIDATEBUFFERDATAPROC);
            self.glInvalidateBufferSubData = load!((load)(userptr, c"glInvalidateBufferSubData".as_ptr()) as PFNGLINVALIDATEBUFFERSUBDATAPROC);
            self.glInvalidateFramebuffer = load!((load)(userptr, c"glInvalidateFramebuffer".as_ptr()) as PFNGLINVALIDATEFRAMEBUFFERPROC);
            self.glInvalidateSubFramebuffer = load!((load)(userptr, c"glInvalidateSubFramebuffer".as_ptr()) as PFNGLINVALIDATESUBFRAMEBUFFERPROC);
            self.glInvalidateTexImage = load!((load)(userptr, c"glInvalidateTexImage".as_ptr()) as PFNGLINVALIDATETEXIMAGEPROC);
            self.glInvalidateTexSubImage = load!((load)(userptr, c"glInvalidateTexSubImage".as_ptr()) as PFNGLINVALIDATETEXSUBIMAGEPROC);
            self.glMultiDrawArraysIndirect = load!((load)(userptr, c"glMultiDrawArraysIndirect".as_ptr()) as PFNGLMULTIDRAWARRAYSINDIRECTPROC);
            self.glMultiDrawElementsIndirect = load!((load)(userptr, c"glMultiDrawElementsIndirect".as_ptr()) as PFNGLMULTIDRAWELEMENTSINDIRECTPROC);
            self.glObjectLabel = load!((load)(userptr, c"glObjectLabel".as_ptr()) as PFNGLOBJECTLABELPROC);
            self.glObjectPtrLabel = load!((load)(userptr, c"glObjectPtrLabel".as_ptr()) as PFNGLOBJECTPTRLABELPROC);
            self.glPopDebugGroup = load!((load)(userptr, c"glPopDebugGroup".as_ptr()) as PFNGLPOPDEBUGGROUPPROC);
            self.glPushDebugGroup = load!((load)(userptr, c"glPushDebugGroup".as_ptr()) as PFNGLPUSHDEBUGGROUPPROC);
            self.glShaderStorageBlockBinding = load!((load)(userptr, c"glShaderStorageBlockBinding".as_ptr()) as PFNGLSHADERSTORAGEBLOCKBINDINGPROC);
            self.glTexBufferRange = load!((load)(userptr, c"glTexBufferRange".as_ptr()) as PFNGLTEXBUFFERRANGEPROC);
            self.glTexStorage2DMultisample = load!((load)(userptr, c"glTexStorage2DMultisample".as_ptr()) as PFNGLTEXSTORAGE2DMULTISAMPLEPROC);
            self.glTexStorage3DMultisample = load!((load)(userptr, c"glTexStorage3DMultisample".as_ptr()) as PFNGLTEXSTORAGE3DMULTISAMPLEPROC);
            self.glTextureView = load!((load)(userptr, c"glTextureView".as_ptr()) as PFNGLTEXTUREVIEWPROC);
            self.glVertexAttribBinding = load!((load)(userptr, c"glVertexAttribBinding".as_ptr()) as PFNGLVERTEXATTRIBBINDINGPROC);
            self.glVertexAttribFormat = load!((load)(userptr, c"glVertexAttribFormat".as_ptr()) as PFNGLVERTEXATTRIBFORMATPROC);
            self.glVertexAttribIFormat = load!((load)(userptr, c"glVertexAttribIFormat".as_ptr()) as PFNGLVERTEXATTRIBIFORMATPROC);
            self.glVertexAttribLFormat = load!((load)(userptr, c"glVertexAttribLFormat".as_ptr()) as PFNGLVERTEXATTRIBLFORMATPROC);
            self.glVertexBindingDivisor = load!((load)(userptr, c"glVertexBindingDivisor".as_ptr()) as PFNGLVERTEXBINDINGDIVISORPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_ES2_compatibility(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_ES2_compatibility { return; }
            self.glClearDepthf = load!((load)(userptr, c"glClearDepthf".as_ptr()) as PFNGLCLEARDEPTHFPROC);
            self.glDepthRangef = load!((load)(userptr, c"glDepthRangef".as_ptr()) as PFNGLDEPTHRANGEFPROC);
            self.glGetShaderPrecisionFormat = load!((load)(userptr, c"glGetShaderPrecisionFormat".as_ptr()) as PFNGLGETSHADERPRECISIONFORMATPROC);
            self.glReleaseShaderCompiler = load!((load)(userptr, c"glReleaseShaderCompiler".as_ptr()) as PFNGLRELEASESHADERCOMPILERPROC);
            self.glShaderBinary = load!((load)(userptr, c"glShaderBinary".as_ptr()) as PFNGLSHADERBINARYPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_ES3_1_compatibility(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_ES3_1_compatibility { return; }
            self.glMemoryBarrierByRegion = load!((load)(userptr, c"glMemoryBarrierByRegion".as_ptr()) as PFNGLMEMORYBARRIERBYREGIONPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_ES3_2_compatibility(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_ES3_2_compatibility { return; }
            self.glPrimitiveBoundingBoxARB = load!((load)(userptr, c"glPrimitiveBoundingBoxARB".as_ptr()) as PFNGLPRIMITIVEBOUNDINGBOXARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_blend_func_extended(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_blend_func_extended { return; }
            self.glBindFragDataLocationIndexed = load!((load)(userptr, c"glBindFragDataLocationIndexed".as_ptr()) as PFNGLBINDFRAGDATALOCATIONINDEXEDPROC);
            self.glGetFragDataIndex = load!((load)(userptr, c"glGetFragDataIndex".as_ptr()) as PFNGLGETFRAGDATAINDEXPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_buffer_storage(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_buffer_storage { return; }
            self.glBufferStorage = load!((load)(userptr, c"glBufferStorage".as_ptr()) as PFNGLBUFFERSTORAGEPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_clear_buffer_object(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_clear_buffer_object { return; }
            self.glClearBufferData = load!((load)(userptr, c"glClearBufferData".as_ptr()) as PFNGLCLEARBUFFERDATAPROC);
            self.glClearBufferSubData = load!((load)(userptr, c"glClearBufferSubData".as_ptr()) as PFNGLCLEARBUFFERSUBDATAPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_clear_texture(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_clear_texture { return; }
            self.glClearTexImage = load!((load)(userptr, c"glClearTexImage".as_ptr()) as PFNGLCLEARTEXIMAGEPROC);
            self.glClearTexSubImage = load!((load)(userptr, c"glClearTexSubImage".as_ptr()) as PFNGLCLEARTEXSUBIMAGEPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_color_buffer_float(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_color_buffer_float { return; }
            self.glClampColorARB = load!((load)(userptr, c"glClampColorARB".as_ptr()) as PFNGLCLAMPCOLORARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_compute_shader(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_compute_shader { return; }
            self.glDispatchCompute = load!((load)(userptr, c"glDispatchCompute".as_ptr()) as PFNGLDISPATCHCOMPUTEPROC);
            self.glDispatchComputeIndirect = load!((load)(userptr, c"glDispatchComputeIndirect".as_ptr()) as PFNGLDISPATCHCOMPUTEINDIRECTPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_compute_variable_group_size(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_compute_variable_group_size { return; }
            self.glDispatchComputeGroupSizeARB = load!((load)(userptr, c"glDispatchComputeGroupSizeARB".as_ptr()) as PFNGLDISPATCHCOMPUTEGROUPSIZEARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_copy_buffer(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_copy_buffer { return; }
            self.glCopyBufferSubData = load!((load)(userptr, c"glCopyBufferSubData".as_ptr()) as PFNGLCOPYBUFFERSUBDATAPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_copy_image(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_copy_image { return; }
            self.glCopyImageSubData = load!((load)(userptr, c"glCopyImageSubData".as_ptr()) as PFNGLCOPYIMAGESUBDATAPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_debug_output(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_debug_output { return; }
            self.glDebugMessageCallbackARB = load!((load)(userptr, c"glDebugMessageCallbackARB".as_ptr()) as PFNGLDEBUGMESSAGECALLBACKARBPROC);
            self.glDebugMessageControlARB = load!((load)(userptr, c"glDebugMessageControlARB".as_ptr()) as PFNGLDEBUGMESSAGECONTROLARBPROC);
            self.glDebugMessageInsertARB = load!((load)(userptr, c"glDebugMessageInsertARB".as_ptr()) as PFNGLDEBUGMESSAGEINSERTARBPROC);
            self.glGetDebugMessageLogARB = load!((load)(userptr, c"glGetDebugMessageLogARB".as_ptr()) as PFNGLGETDEBUGMESSAGELOGARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_direct_state_access(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_direct_state_access { return; }
            self.glBindTextureUnit = load!((load)(userptr, c"glBindTextureUnit".as_ptr()) as PFNGLBINDTEXTUREUNITPROC);
            self.glBlitNamedFramebuffer = load!((load)(userptr, c"glBlitNamedFramebuffer".as_ptr()) as PFNGLBLITNAMEDFRAMEBUFFERPROC);
            self.glCheckNamedFramebufferStatus = load!((load)(userptr, c"glCheckNamedFramebufferStatus".as_ptr()) as PFNGLCHECKNAMEDFRAMEBUFFERSTATUSPROC);
            self.glClearNamedBufferData = load!((load)(userptr, c"glClearNamedBufferData".as_ptr()) as PFNGLCLEARNAMEDBUFFERDATAPROC);
            self.glClearNamedBufferSubData = load!((load)(userptr, c"glClearNamedBufferSubData".as_ptr()) as PFNGLCLEARNAMEDBUFFERSUBDATAPROC);
            self.glClearNamedFramebufferfi = load!((load)(userptr, c"glClearNamedFramebufferfi".as_ptr()) as PFNGLCLEARNAMEDFRAMEBUFFERFIPROC);
            self.glClearNamedFramebufferfv = load!((load)(userptr, c"glClearNamedFramebufferfv".as_ptr()) as PFNGLCLEARNAMEDFRAMEBUFFERFVPROC);
            self.glClearNamedFramebufferiv = load!((load)(userptr, c"glClearNamedFramebufferiv".as_ptr()) as PFNGLCLEARNAMEDFRAMEBUFFERIVPROC);
            self.glClearNamedFramebufferuiv = load!((load)(userptr, c"glClearNamedFramebufferuiv".as_ptr()) as PFNGLCLEARNAMEDFRAMEBUFFERUIVPROC);
            self.glCompressedTextureSubImage1D = load!((load)(userptr, c"glCompressedTextureSubImage1D".as_ptr()) as PFNGLCOMPRESSEDTEXTURESUBIMAGE1DPROC);
            self.glCompressedTextureSubImage2D = load!((load)(userptr, c"glCompressedTextureSubImage2D".as_ptr()) as PFNGLCOMPRESSEDTEXTURESUBIMAGE2DPROC);
            self.glCompressedTextureSubImage3D = load!((load)(userptr, c"glCompressedTextureSubImage3D".as_ptr()) as PFNGLCOMPRESSEDTEXTURESUBIMAGE3DPROC);
            self.glCopyNamedBufferSubData = load!((load)(userptr, c"glCopyNamedBufferSubData".as_ptr()) as PFNGLCOPYNAMEDBUFFERSUBDATAPROC);
            self.glCopyTextureSubImage1D = load!((load)(userptr, c"glCopyTextureSubImage1D".as_ptr()) as PFNGLCOPYTEXTURESUBIMAGE1DPROC);
            self.glCopyTextureSubImage2D = load!((load)(userptr, c"glCopyTextureSubImage2D".as_ptr()) as PFNGLCOPYTEXTURESUBIMAGE2DPROC);
            self.glCopyTextureSubImage3D = load!((load)(userptr, c"glCopyTextureSubImage3D".as_ptr()) as PFNGLCOPYTEXTURESUBIMAGE3DPROC);
            self.glCreateBuffers = load!((load)(userptr, c"glCreateBuffers".as_ptr()) as PFNGLCREATEBUFFERSPROC);
            self.glCreateFramebuffers = load!((load)(userptr, c"glCreateFramebuffers".as_ptr()) as PFNGLCREATEFRAMEBUFFERSPROC);
            self.glCreateProgramPipelines = load!((load)(userptr, c"glCreateProgramPipelines".as_ptr()) as PFNGLCREATEPROGRAMPIPELINESPROC);
            self.glCreateQueries = load!((load)(userptr, c"glCreateQueries".as_ptr()) as PFNGLCREATEQUERIESPROC);
            self.glCreateRenderbuffers = load!((load)(userptr, c"glCreateRenderbuffers".as_ptr()) as PFNGLCREATERENDERBUFFERSPROC);
            self.glCreateSamplers = load!((load)(userptr, c"glCreateSamplers".as_ptr()) as PFNGLCREATESAMPLERSPROC);
            self.glCreateTextures = load!((load)(userptr, c"glCreateTextures".as_ptr()) as PFNGLCREATETEXTURESPROC);
            self.glCreateTransformFeedbacks = load!((load)(userptr, c"glCreateTransformFeedbacks".as_ptr()) as PFNGLCREATETRANSFORMFEEDBACKSPROC);
            self.glCreateVertexArrays = load!((load)(userptr, c"glCreateVertexArrays".as_ptr()) as PFNGLCREATEVERTEXARRAYSPROC);
            self.glDisableVertexArrayAttrib = load!((load)(userptr, c"glDisableVertexArrayAttrib".as_ptr()) as PFNGLDISABLEVERTEXARRAYATTRIBPROC);
            self.glEnableVertexArrayAttrib = load!((load)(userptr, c"glEnableVertexArrayAttrib".as_ptr()) as PFNGLENABLEVERTEXARRAYATTRIBPROC);
            self.glFlushMappedNamedBufferRange = load!((load)(userptr, c"glFlushMappedNamedBufferRange".as_ptr()) as PFNGLFLUSHMAPPEDNAMEDBUFFERRANGEPROC);
            self.glGenerateTextureMipmap = load!((load)(userptr, c"glGenerateTextureMipmap".as_ptr()) as PFNGLGENERATETEXTUREMIPMAPPROC);
            self.glGetCompressedTextureImage = load!((load)(userptr, c"glGetCompressedTextureImage".as_ptr()) as PFNGLGETCOMPRESSEDTEXTUREIMAGEPROC);
            self.glGetNamedBufferParameteri64v = load!((load)(userptr, c"glGetNamedBufferParameteri64v".as_ptr()) as PFNGLGETNAMEDBUFFERPARAMETERI64VPROC);
            self.glGetNamedBufferParameteriv = load!((load)(userptr, c"glGetNamedBufferParameteriv".as_ptr()) as PFNGLGETNAMEDBUFFERPARAMETERIVPROC);
            self.glGetNamedBufferPointerv = load!((load)(userptr, c"glGetNamedBufferPointerv".as_ptr()) as PFNGLGETNAMEDBUFFERPOINTERVPROC);
            self.glGetNamedBufferSubData = load!((load)(userptr, c"glGetNamedBufferSubData".as_ptr()) as PFNGLGETNAMEDBUFFERSUBDATAPROC);
            self.glGetNamedFramebufferAttachmentParameteriv = load!((load)(userptr, c"glGetNamedFramebufferAttachmentParameteriv".as_ptr()) as PFNGLGETNAMEDFRAMEBUFFERATTACHMENTPARAMETERIVPROC);
            self.glGetNamedFramebufferParameteriv = load!((load)(userptr, c"glGetNamedFramebufferParameteriv".as_ptr()) as PFNGLGETNAMEDFRAMEBUFFERPARAMETERIVPROC);
            self.glGetNamedRenderbufferParameteriv = load!((load)(userptr, c"glGetNamedRenderbufferParameteriv".as_ptr()) as PFNGLGETNAMEDRENDERBUFFERPARAMETERIVPROC);
            self.glGetQueryBufferObjecti64v = load!((load)(userptr, c"glGetQueryBufferObjecti64v".as_ptr()) as PFNGLGETQUERYBUFFEROBJECTI64VPROC);
            self.glGetQueryBufferObjectiv = load!((load)(userptr, c"glGetQueryBufferObjectiv".as_ptr()) as PFNGLGETQUERYBUFFEROBJECTIVPROC);
            self.glGetQueryBufferObjectui64v = load!((load)(userptr, c"glGetQueryBufferObjectui64v".as_ptr()) as PFNGLGETQUERYBUFFEROBJECTUI64VPROC);
            self.glGetQueryBufferObjectuiv = load!((load)(userptr, c"glGetQueryBufferObjectuiv".as_ptr()) as PFNGLGETQUERYBUFFEROBJECTUIVPROC);
            self.glGetTextureImage = load!((load)(userptr, c"glGetTextureImage".as_ptr()) as PFNGLGETTEXTUREIMAGEPROC);
            self.glGetTextureLevelParameterfv = load!((load)(userptr, c"glGetTextureLevelParameterfv".as_ptr()) as PFNGLGETTEXTURELEVELPARAMETERFVPROC);
            self.glGetTextureLevelParameteriv = load!((load)(userptr, c"glGetTextureLevelParameteriv".as_ptr()) as PFNGLGETTEXTURELEVELPARAMETERIVPROC);
            self.glGetTextureParameterIiv = load!((load)(userptr, c"glGetTextureParameterIiv".as_ptr()) as PFNGLGETTEXTUREPARAMETERIIVPROC);
            self.glGetTextureParameterIuiv = load!((load)(userptr, c"glGetTextureParameterIuiv".as_ptr()) as PFNGLGETTEXTUREPARAMETERIUIVPROC);
            self.glGetTextureParameterfv = load!((load)(userptr, c"glGetTextureParameterfv".as_ptr()) as PFNGLGETTEXTUREPARAMETERFVPROC);
            self.glGetTextureParameteriv = load!((load)(userptr, c"glGetTextureParameteriv".as_ptr()) as PFNGLGETTEXTUREPARAMETERIVPROC);
            self.glGetTransformFeedbacki64_v = load!((load)(userptr, c"glGetTransformFeedbacki64_v".as_ptr()) as PFNGLGETTRANSFORMFEEDBACKI64_VPROC);
            self.glGetTransformFeedbacki_v = load!((load)(userptr, c"glGetTransformFeedbacki_v".as_ptr()) as PFNGLGETTRANSFORMFEEDBACKI_VPROC);
            self.glGetTransformFeedbackiv = load!((load)(userptr, c"glGetTransformFeedbackiv".as_ptr()) as PFNGLGETTRANSFORMFEEDBACKIVPROC);
            self.glGetVertexArrayIndexed64iv = load!((load)(userptr, c"glGetVertexArrayIndexed64iv".as_ptr()) as PFNGLGETVERTEXARRAYINDEXED64IVPROC);
            self.glGetVertexArrayIndexediv = load!((load)(userptr, c"glGetVertexArrayIndexediv".as_ptr()) as PFNGLGETVERTEXARRAYINDEXEDIVPROC);
            self.glGetVertexArrayiv = load!((load)(userptr, c"glGetVertexArrayiv".as_ptr()) as PFNGLGETVERTEXARRAYIVPROC);
            self.glInvalidateNamedFramebufferData = load!((load)(userptr, c"glInvalidateNamedFramebufferData".as_ptr()) as PFNGLINVALIDATENAMEDFRAMEBUFFERDATAPROC);
            self.glInvalidateNamedFramebufferSubData = load!((load)(userptr, c"glInvalidateNamedFramebufferSubData".as_ptr()) as PFNGLINVALIDATENAMEDFRAMEBUFFERSUBDATAPROC);
            self.glMapNamedBuffer = load!((load)(userptr, c"glMapNamedBuffer".as_ptr()) as PFNGLMAPNAMEDBUFFERPROC);
            self.glMapNamedBufferRange = load!((load)(userptr, c"glMapNamedBufferRange".as_ptr()) as PFNGLMAPNAMEDBUFFERRANGEPROC);
            self.glNamedBufferData = load!((load)(userptr, c"glNamedBufferData".as_ptr()) as PFNGLNAMEDBUFFERDATAPROC);
            self.glNamedBufferStorage = load!((load)(userptr, c"glNamedBufferStorage".as_ptr()) as PFNGLNAMEDBUFFERSTORAGEPROC);
            self.glNamedBufferSubData = load!((load)(userptr, c"glNamedBufferSubData".as_ptr()) as PFNGLNAMEDBUFFERSUBDATAPROC);
            self.glNamedFramebufferDrawBuffer = load!((load)(userptr, c"glNamedFramebufferDrawBuffer".as_ptr()) as PFNGLNAMEDFRAMEBUFFERDRAWBUFFERPROC);
            self.glNamedFramebufferDrawBuffers = load!((load)(userptr, c"glNamedFramebufferDrawBuffers".as_ptr()) as PFNGLNAMEDFRAMEBUFFERDRAWBUFFERSPROC);
            self.glNamedFramebufferParameteri = load!((load)(userptr, c"glNamedFramebufferParameteri".as_ptr()) as PFNGLNAMEDFRAMEBUFFERPARAMETERIPROC);
            self.glNamedFramebufferReadBuffer = load!((load)(userptr, c"glNamedFramebufferReadBuffer".as_ptr()) as PFNGLNAMEDFRAMEBUFFERREADBUFFERPROC);
            self.glNamedFramebufferRenderbuffer = load!((load)(userptr, c"glNamedFramebufferRenderbuffer".as_ptr()) as PFNGLNAMEDFRAMEBUFFERRENDERBUFFERPROC);
            self.glNamedFramebufferTexture = load!((load)(userptr, c"glNamedFramebufferTexture".as_ptr()) as PFNGLNAMEDFRAMEBUFFERTEXTUREPROC);
            self.glNamedFramebufferTextureLayer = load!((load)(userptr, c"glNamedFramebufferTextureLayer".as_ptr()) as PFNGLNAMEDFRAMEBUFFERTEXTURELAYERPROC);
            self.glNamedRenderbufferStorage = load!((load)(userptr, c"glNamedRenderbufferStorage".as_ptr()) as PFNGLNAMEDRENDERBUFFERSTORAGEPROC);
            self.glNamedRenderbufferStorageMultisample = load!((load)(userptr, c"glNamedRenderbufferStorageMultisample".as_ptr()) as PFNGLNAMEDRENDERBUFFERSTORAGEMULTISAMPLEPROC);
            self.glTextureBuffer = load!((load)(userptr, c"glTextureBuffer".as_ptr()) as PFNGLTEXTUREBUFFERPROC);
            self.glTextureBufferRange = load!((load)(userptr, c"glTextureBufferRange".as_ptr()) as PFNGLTEXTUREBUFFERRANGEPROC);
            self.glTextureParameterIiv = load!((load)(userptr, c"glTextureParameterIiv".as_ptr()) as PFNGLTEXTUREPARAMETERIIVPROC);
            self.glTextureParameterIuiv = load!((load)(userptr, c"glTextureParameterIuiv".as_ptr()) as PFNGLTEXTUREPARAMETERIUIVPROC);
            self.glTextureParameterf = load!((load)(userptr, c"glTextureParameterf".as_ptr()) as PFNGLTEXTUREPARAMETERFPROC);
            self.glTextureParameterfv = load!((load)(userptr, c"glTextureParameterfv".as_ptr()) as PFNGLTEXTUREPARAMETERFVPROC);
            self.glTextureParameteri = load!((load)(userptr, c"glTextureParameteri".as_ptr()) as PFNGLTEXTUREPARAMETERIPROC);
            self.glTextureParameteriv = load!((load)(userptr, c"glTextureParameteriv".as_ptr()) as PFNGLTEXTUREPARAMETERIVPROC);
            self.glTextureStorage1D = load!((load)(userptr, c"glTextureStorage1D".as_ptr()) as PFNGLTEXTURESTORAGE1DPROC);
            self.glTextureStorage2D = load!((load)(userptr, c"glTextureStorage2D".as_ptr()) as PFNGLTEXTURESTORAGE2DPROC);
            self.glTextureStorage2DMultisample = load!((load)(userptr, c"glTextureStorage2DMultisample".as_ptr()) as PFNGLTEXTURESTORAGE2DMULTISAMPLEPROC);
            self.glTextureStorage3D = load!((load)(userptr, c"glTextureStorage3D".as_ptr()) as PFNGLTEXTURESTORAGE3DPROC);
            self.glTextureStorage3DMultisample = load!((load)(userptr, c"glTextureStorage3DMultisample".as_ptr()) as PFNGLTEXTURESTORAGE3DMULTISAMPLEPROC);
            self.glTextureSubImage1D = load!((load)(userptr, c"glTextureSubImage1D".as_ptr()) as PFNGLTEXTURESUBIMAGE1DPROC);
            self.glTextureSubImage2D = load!((load)(userptr, c"glTextureSubImage2D".as_ptr()) as PFNGLTEXTURESUBIMAGE2DPROC);
            self.glTextureSubImage3D = load!((load)(userptr, c"glTextureSubImage3D".as_ptr()) as PFNGLTEXTURESUBIMAGE3DPROC);
            self.glTransformFeedbackBufferBase = load!((load)(userptr, c"glTransformFeedbackBufferBase".as_ptr()) as PFNGLTRANSFORMFEEDBACKBUFFERBASEPROC);
            self.glTransformFeedbackBufferRange = load!((load)(userptr, c"glTransformFeedbackBufferRange".as_ptr()) as PFNGLTRANSFORMFEEDBACKBUFFERRANGEPROC);
            self.glUnmapNamedBuffer = load!((load)(userptr, c"glUnmapNamedBuffer".as_ptr()) as PFNGLUNMAPNAMEDBUFFERPROC);
            self.glVertexArrayAttribBinding = load!((load)(userptr, c"glVertexArrayAttribBinding".as_ptr()) as PFNGLVERTEXARRAYATTRIBBINDINGPROC);
            self.glVertexArrayAttribFormat = load!((load)(userptr, c"glVertexArrayAttribFormat".as_ptr()) as PFNGLVERTEXARRAYATTRIBFORMATPROC);
            self.glVertexArrayAttribIFormat = load!((load)(userptr, c"glVertexArrayAttribIFormat".as_ptr()) as PFNGLVERTEXARRAYATTRIBIFORMATPROC);
            self.glVertexArrayAttribLFormat = load!((load)(userptr, c"glVertexArrayAttribLFormat".as_ptr()) as PFNGLVERTEXARRAYATTRIBLFORMATPROC);
            self.glVertexArrayBindingDivisor = load!((load)(userptr, c"glVertexArrayBindingDivisor".as_ptr()) as PFNGLVERTEXARRAYBINDINGDIVISORPROC);
            self.glVertexArrayElementBuffer = load!((load)(userptr, c"glVertexArrayElementBuffer".as_ptr()) as PFNGLVERTEXARRAYELEMENTBUFFERPROC);
            self.glVertexArrayVertexBuffer = load!((load)(userptr, c"glVertexArrayVertexBuffer".as_ptr()) as PFNGLVERTEXARRAYVERTEXBUFFERPROC);
            self.glVertexArrayVertexBuffers = load!((load)(userptr, c"glVertexArrayVertexBuffers".as_ptr()) as PFNGLVERTEXARRAYVERTEXBUFFERSPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_draw_buffers(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_draw_buffers { return; }
            self.glDrawBuffersARB = load!((load)(userptr, c"glDrawBuffersARB".as_ptr()) as PFNGLDRAWBUFFERSARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_draw_buffers_blend(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_draw_buffers_blend { return; }
            self.glBlendEquationSeparateiARB = load!((load)(userptr, c"glBlendEquationSeparateiARB".as_ptr()) as PFNGLBLENDEQUATIONSEPARATEIARBPROC);
            self.glBlendEquationiARB = load!((load)(userptr, c"glBlendEquationiARB".as_ptr()) as PFNGLBLENDEQUATIONIARBPROC);
            self.glBlendFuncSeparateiARB = load!((load)(userptr, c"glBlendFuncSeparateiARB".as_ptr()) as PFNGLBLENDFUNCSEPARATEIARBPROC);
            self.glBlendFunciARB = load!((load)(userptr, c"glBlendFunciARB".as_ptr()) as PFNGLBLENDFUNCIARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_draw_elements_base_vertex(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_draw_elements_base_vertex { return; }
            self.glDrawElementsBaseVertex = load!((load)(userptr, c"glDrawElementsBaseVertex".as_ptr()) as PFNGLDRAWELEMENTSBASEVERTEXPROC);
            self.glDrawElementsInstancedBaseVertex = load!((load)(userptr, c"glDrawElementsInstancedBaseVertex".as_ptr()) as PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC);
            self.glDrawRangeElementsBaseVertex = load!((load)(userptr, c"glDrawRangeElementsBaseVertex".as_ptr()) as PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC);
            self.glMultiDrawElementsBaseVertex = load!((load)(userptr, c"glMultiDrawElementsBaseVertex".as_ptr()) as PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_draw_indirect(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_draw_indirect { return; }
            self.glDrawArraysIndirect = load!((load)(userptr, c"glDrawArraysIndirect".as_ptr()) as PFNGLDRAWARRAYSINDIRECTPROC);
            self.glDrawElementsIndirect = load!((load)(userptr, c"glDrawElementsIndirect".as_ptr()) as PFNGLDRAWELEMENTSINDIRECTPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_draw_instanced(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_draw_instanced { return; }
            self.glDrawArraysInstancedARB = load!((load)(userptr, c"glDrawArraysInstancedARB".as_ptr()) as PFNGLDRAWARRAYSINSTANCEDARBPROC);
            self.glDrawElementsInstancedARB = load!((load)(userptr, c"glDrawElementsInstancedARB".as_ptr()) as PFNGLDRAWELEMENTSINSTANCEDARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_fragment_program(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_fragment_program { return; }
            self.glBindProgramARB = load!((load)(userptr, c"glBindProgramARB".as_ptr()) as PFNGLBINDPROGRAMARBPROC);
            self.glDeleteProgramsARB = load!((load)(userptr, c"glDeleteProgramsARB".as_ptr()) as PFNGLDELETEPROGRAMSARBPROC);
            self.glGenProgramsARB = load!((load)(userptr, c"glGenProgramsARB".as_ptr()) as PFNGLGENPROGRAMSARBPROC);
            self.glGetProgramEnvParameterdvARB = load!((load)(userptr, c"glGetProgramEnvParameterdvARB".as_ptr()) as PFNGLGETPROGRAMENVPARAMETERDVARBPROC);
            self.glGetProgramEnvParameterfvARB = load!((load)(userptr, c"glGetProgramEnvParameterfvARB".as_ptr()) as PFNGLGETPROGRAMENVPARAMETERFVARBPROC);
            self.glGetProgramLocalParameterdvARB = load!((load)(userptr, c"glGetProgramLocalParameterdvARB".as_ptr()) as PFNGLGETPROGRAMLOCALPARAMETERDVARBPROC);
            self.glGetProgramLocalParameterfvARB = load!((load)(userptr, c"glGetProgramLocalParameterfvARB".as_ptr()) as PFNGLGETPROGRAMLOCALPARAMETERFVARBPROC);
            self.glGetProgramStringARB = load!((load)(userptr, c"glGetProgramStringARB".as_ptr()) as PFNGLGETPROGRAMSTRINGARBPROC);
            self.glGetProgramivARB = load!((load)(userptr, c"glGetProgramivARB".as_ptr()) as PFNGLGETPROGRAMIVARBPROC);
            self.glIsProgramARB = load!((load)(userptr, c"glIsProgramARB".as_ptr()) as PFNGLISPROGRAMARBPROC);
            self.glProgramEnvParameter4dARB = load!((load)(userptr, c"glProgramEnvParameter4dARB".as_ptr()) as PFNGLPROGRAMENVPARAMETER4DARBPROC);
            self.glProgramEnvParameter4dvARB = load!((load)(userptr, c"glProgramEnvParameter4dvARB".as_ptr()) as PFNGLPROGRAMENVPARAMETER4DVARBPROC);
            self.glProgramEnvParameter4fARB = load!((load)(userptr, c"glProgramEnvParameter4fARB".as_ptr()) as PFNGLPROGRAMENVPARAMETER4FARBPROC);
            self.glProgramEnvParameter4fvARB = load!((load)(userptr, c"glProgramEnvParameter4fvARB".as_ptr()) as PFNGLPROGRAMENVPARAMETER4FVARBPROC);
            self.glProgramLocalParameter4dARB = load!((load)(userptr, c"glProgramLocalParameter4dARB".as_ptr()) as PFNGLPROGRAMLOCALPARAMETER4DARBPROC);
            self.glProgramLocalParameter4dvARB = load!((load)(userptr, c"glProgramLocalParameter4dvARB".as_ptr()) as PFNGLPROGRAMLOCALPARAMETER4DVARBPROC);
            self.glProgramLocalParameter4fARB = load!((load)(userptr, c"glProgramLocalParameter4fARB".as_ptr()) as PFNGLPROGRAMLOCALPARAMETER4FARBPROC);
            self.glProgramLocalParameter4fvARB = load!((load)(userptr, c"glProgramLocalParameter4fvARB".as_ptr()) as PFNGLPROGRAMLOCALPARAMETER4FVARBPROC);
            self.glProgramStringARB = load!((load)(userptr, c"glProgramStringARB".as_ptr()) as PFNGLPROGRAMSTRINGARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_framebuffer_no_attachments(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_framebuffer_no_attachments { return; }
            self.glFramebufferParameteri = load!((load)(userptr, c"glFramebufferParameteri".as_ptr()) as PFNGLFRAMEBUFFERPARAMETERIPROC);
            self.glGetFramebufferParameteriv = load!((load)(userptr, c"glGetFramebufferParameteriv".as_ptr()) as PFNGLGETFRAMEBUFFERPARAMETERIVPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_framebuffer_object(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_framebuffer_object { return; }
            self.glBindFramebuffer = load!((load)(userptr, c"glBindFramebuffer".as_ptr()) as PFNGLBINDFRAMEBUFFERPROC);
            self.glBindRenderbuffer = load!((load)(userptr, c"glBindRenderbuffer".as_ptr()) as PFNGLBINDRENDERBUFFERPROC);
            self.glBlitFramebuffer = load!((load)(userptr, c"glBlitFramebuffer".as_ptr()) as PFNGLBLITFRAMEBUFFERPROC);
            self.glCheckFramebufferStatus = load!((load)(userptr, c"glCheckFramebufferStatus".as_ptr()) as PFNGLCHECKFRAMEBUFFERSTATUSPROC);
            self.glDeleteFramebuffers = load!((load)(userptr, c"glDeleteFramebuffers".as_ptr()) as PFNGLDELETEFRAMEBUFFERSPROC);
            self.glDeleteRenderbuffers = load!((load)(userptr, c"glDeleteRenderbuffers".as_ptr()) as PFNGLDELETERENDERBUFFERSPROC);
            self.glFramebufferRenderbuffer = load!((load)(userptr, c"glFramebufferRenderbuffer".as_ptr()) as PFNGLFRAMEBUFFERRENDERBUFFERPROC);
            self.glFramebufferTexture1D = load!((load)(userptr, c"glFramebufferTexture1D".as_ptr()) as PFNGLFRAMEBUFFERTEXTURE1DPROC);
            self.glFramebufferTexture2D = load!((load)(userptr, c"glFramebufferTexture2D".as_ptr()) as PFNGLFRAMEBUFFERTEXTURE2DPROC);
            self.glFramebufferTexture3D = load!((load)(userptr, c"glFramebufferTexture3D".as_ptr()) as PFNGLFRAMEBUFFERTEXTURE3DPROC);
            self.glFramebufferTextureLayer = load!((load)(userptr, c"glFramebufferTextureLayer".as_ptr()) as PFNGLFRAMEBUFFERTEXTURELAYERPROC);
            self.glGenFramebuffers = load!((load)(userptr, c"glGenFramebuffers".as_ptr()) as PFNGLGENFRAMEBUFFERSPROC);
            self.glGenRenderbuffers = load!((load)(userptr, c"glGenRenderbuffers".as_ptr()) as PFNGLGENRENDERBUFFERSPROC);
            self.glGenerateMipmap = load!((load)(userptr, c"glGenerateMipmap".as_ptr()) as PFNGLGENERATEMIPMAPPROC);
            self.glGetFramebufferAttachmentParameteriv = load!((load)(userptr, c"glGetFramebufferAttachmentParameteriv".as_ptr()) as PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC);
            self.glGetRenderbufferParameteriv = load!((load)(userptr, c"glGetRenderbufferParameteriv".as_ptr()) as PFNGLGETRENDERBUFFERPARAMETERIVPROC);
            self.glIsFramebuffer = load!((load)(userptr, c"glIsFramebuffer".as_ptr()) as PFNGLISFRAMEBUFFERPROC);
            self.glIsRenderbuffer = load!((load)(userptr, c"glIsRenderbuffer".as_ptr()) as PFNGLISRENDERBUFFERPROC);
            self.glRenderbufferStorage = load!((load)(userptr, c"glRenderbufferStorage".as_ptr()) as PFNGLRENDERBUFFERSTORAGEPROC);
            self.glRenderbufferStorageMultisample = load!((load)(userptr, c"glRenderbufferStorageMultisample".as_ptr()) as PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_geometry_shader4(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_geometry_shader4 { return; }
            self.glFramebufferTextureARB = load!((load)(userptr, c"glFramebufferTextureARB".as_ptr()) as PFNGLFRAMEBUFFERTEXTUREARBPROC);
            self.glFramebufferTextureFaceARB = load!((load)(userptr, c"glFramebufferTextureFaceARB".as_ptr()) as PFNGLFRAMEBUFFERTEXTUREFACEARBPROC);
            self.glFramebufferTextureLayerARB = load!((load)(userptr, c"glFramebufferTextureLayerARB".as_ptr()) as PFNGLFRAMEBUFFERTEXTURELAYERARBPROC);
            self.glProgramParameteriARB = load!((load)(userptr, c"glProgramParameteriARB".as_ptr()) as PFNGLPROGRAMPARAMETERIARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_get_program_binary(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_get_program_binary { return; }
            self.glGetProgramBinary = load!((load)(userptr, c"glGetProgramBinary".as_ptr()) as PFNGLGETPROGRAMBINARYPROC);
            self.glProgramBinary = load!((load)(userptr, c"glProgramBinary".as_ptr()) as PFNGLPROGRAMBINARYPROC);
            self.glProgramParameteri = load!((load)(userptr, c"glProgramParameteri".as_ptr()) as PFNGLPROGRAMPARAMETERIPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_get_texture_sub_image(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_get_texture_sub_image { return; }
            self.glGetCompressedTextureSubImage = load!((load)(userptr, c"glGetCompressedTextureSubImage".as_ptr()) as PFNGLGETCOMPRESSEDTEXTURESUBIMAGEPROC);
            self.glGetTextureSubImage = load!((load)(userptr, c"glGetTextureSubImage".as_ptr()) as PFNGLGETTEXTURESUBIMAGEPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_gl_spirv(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_gl_spirv { return; }
            self.glSpecializeShaderARB = load!((load)(userptr, c"glSpecializeShaderARB".as_ptr()) as PFNGLSPECIALIZESHADERARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_gpu_shader_fp64(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_gpu_shader_fp64 { return; }
            self.glGetUniformdv = load!((load)(userptr, c"glGetUniformdv".as_ptr()) as PFNGLGETUNIFORMDVPROC);
            self.glUniform1d = load!((load)(userptr, c"glUniform1d".as_ptr()) as PFNGLUNIFORM1DPROC);
            self.glUniform1dv = load!((load)(userptr, c"glUniform1dv".as_ptr()) as PFNGLUNIFORM1DVPROC);
            self.glUniform2d = load!((load)(userptr, c"glUniform2d".as_ptr()) as PFNGLUNIFORM2DPROC);
            self.glUniform2dv = load!((load)(userptr, c"glUniform2dv".as_ptr()) as PFNGLUNIFORM2DVPROC);
            self.glUniform3d = load!((load)(userptr, c"glUniform3d".as_ptr()) as PFNGLUNIFORM3DPROC);
            self.glUniform3dv = load!((load)(userptr, c"glUniform3dv".as_ptr()) as PFNGLUNIFORM3DVPROC);
            self.glUniform4d = load!((load)(userptr, c"glUniform4d".as_ptr()) as PFNGLUNIFORM4DPROC);
            self.glUniform4dv = load!((load)(userptr, c"glUniform4dv".as_ptr()) as PFNGLUNIFORM4DVPROC);
            self.glUniformMatrix2dv = load!((load)(userptr, c"glUniformMatrix2dv".as_ptr()) as PFNGLUNIFORMMATRIX2DVPROC);
            self.glUniformMatrix2x3dv = load!((load)(userptr, c"glUniformMatrix2x3dv".as_ptr()) as PFNGLUNIFORMMATRIX2X3DVPROC);
            self.glUniformMatrix2x4dv = load!((load)(userptr, c"glUniformMatrix2x4dv".as_ptr()) as PFNGLUNIFORMMATRIX2X4DVPROC);
            self.glUniformMatrix3dv = load!((load)(userptr, c"glUniformMatrix3dv".as_ptr()) as PFNGLUNIFORMMATRIX3DVPROC);
            self.glUniformMatrix3x2dv = load!((load)(userptr, c"glUniformMatrix3x2dv".as_ptr()) as PFNGLUNIFORMMATRIX3X2DVPROC);
            self.glUniformMatrix3x4dv = load!((load)(userptr, c"glUniformMatrix3x4dv".as_ptr()) as PFNGLUNIFORMMATRIX3X4DVPROC);
            self.glUniformMatrix4dv = load!((load)(userptr, c"glUniformMatrix4dv".as_ptr()) as PFNGLUNIFORMMATRIX4DVPROC);
            self.glUniformMatrix4x2dv = load!((load)(userptr, c"glUniformMatrix4x2dv".as_ptr()) as PFNGLUNIFORMMATRIX4X2DVPROC);
            self.glUniformMatrix4x3dv = load!((load)(userptr, c"glUniformMatrix4x3dv".as_ptr()) as PFNGLUNIFORMMATRIX4X3DVPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_gpu_shader_int64(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_gpu_shader_int64 { return; }
            self.glGetUniformi64vARB = load!((load)(userptr, c"glGetUniformi64vARB".as_ptr()) as PFNGLGETUNIFORMI64VARBPROC);
            self.glGetUniformui64vARB = load!((load)(userptr, c"glGetUniformui64vARB".as_ptr()) as PFNGLGETUNIFORMUI64VARBPROC);
            self.glGetnUniformi64vARB = load!((load)(userptr, c"glGetnUniformi64vARB".as_ptr()) as PFNGLGETNUNIFORMI64VARBPROC);
            self.glGetnUniformui64vARB = load!((load)(userptr, c"glGetnUniformui64vARB".as_ptr()) as PFNGLGETNUNIFORMUI64VARBPROC);
            self.glProgramUniform1i64ARB = load!((load)(userptr, c"glProgramUniform1i64ARB".as_ptr()) as PFNGLPROGRAMUNIFORM1I64ARBPROC);
            self.glProgramUniform1i64vARB = load!((load)(userptr, c"glProgramUniform1i64vARB".as_ptr()) as PFNGLPROGRAMUNIFORM1I64VARBPROC);
            self.glProgramUniform1ui64ARB = load!((load)(userptr, c"glProgramUniform1ui64ARB".as_ptr()) as PFNGLPROGRAMUNIFORM1UI64ARBPROC);
            self.glProgramUniform1ui64vARB = load!((load)(userptr, c"glProgramUniform1ui64vARB".as_ptr()) as PFNGLPROGRAMUNIFORM1UI64VARBPROC);
            self.glProgramUniform2i64ARB = load!((load)(userptr, c"glProgramUniform2i64ARB".as_ptr()) as PFNGLPROGRAMUNIFORM2I64ARBPROC);
            self.glProgramUniform2i64vARB = load!((load)(userptr, c"glProgramUniform2i64vARB".as_ptr()) as PFNGLPROGRAMUNIFORM2I64VARBPROC);
            self.glProgramUniform2ui64ARB = load!((load)(userptr, c"glProgramUniform2ui64ARB".as_ptr()) as PFNGLPROGRAMUNIFORM2UI64ARBPROC);
            self.glProgramUniform2ui64vARB = load!((load)(userptr, c"glProgramUniform2ui64vARB".as_ptr()) as PFNGLPROGRAMUNIFORM2UI64VARBPROC);
            self.glProgramUniform3i64ARB = load!((load)(userptr, c"glProgramUniform3i64ARB".as_ptr()) as PFNGLPROGRAMUNIFORM3I64ARBPROC);
            self.glProgramUniform3i64vARB = load!((load)(userptr, c"glProgramUniform3i64vARB".as_ptr()) as PFNGLPROGRAMUNIFORM3I64VARBPROC);
            self.glProgramUniform3ui64ARB = load!((load)(userptr, c"glProgramUniform3ui64ARB".as_ptr()) as PFNGLPROGRAMUNIFORM3UI64ARBPROC);
            self.glProgramUniform3ui64vARB = load!((load)(userptr, c"glProgramUniform3ui64vARB".as_ptr()) as PFNGLPROGRAMUNIFORM3UI64VARBPROC);
            self.glProgramUniform4i64ARB = load!((load)(userptr, c"glProgramUniform4i64ARB".as_ptr()) as PFNGLPROGRAMUNIFORM4I64ARBPROC);
            self.glProgramUniform4i64vARB = load!((load)(userptr, c"glProgramUniform4i64vARB".as_ptr()) as PFNGLPROGRAMUNIFORM4I64VARBPROC);
            self.glProgramUniform4ui64ARB = load!((load)(userptr, c"glProgramUniform4ui64ARB".as_ptr()) as PFNGLPROGRAMUNIFORM4UI64ARBPROC);
            self.glProgramUniform4ui64vARB = load!((load)(userptr, c"glProgramUniform4ui64vARB".as_ptr()) as PFNGLPROGRAMUNIFORM4UI64VARBPROC);
            self.glUniform1i64ARB = load!((load)(userptr, c"glUniform1i64ARB".as_ptr()) as PFNGLUNIFORM1I64ARBPROC);
            self.glUniform1i64vARB = load!((load)(userptr, c"glUniform1i64vARB".as_ptr()) as PFNGLUNIFORM1I64VARBPROC);
            self.glUniform1ui64ARB = load!((load)(userptr, c"glUniform1ui64ARB".as_ptr()) as PFNGLUNIFORM1UI64ARBPROC);
            self.glUniform1ui64vARB = load!((load)(userptr, c"glUniform1ui64vARB".as_ptr()) as PFNGLUNIFORM1UI64VARBPROC);
            self.glUniform2i64ARB = load!((load)(userptr, c"glUniform2i64ARB".as_ptr()) as PFNGLUNIFORM2I64ARBPROC);
            self.glUniform2i64vARB = load!((load)(userptr, c"glUniform2i64vARB".as_ptr()) as PFNGLUNIFORM2I64VARBPROC);
            self.glUniform2ui64ARB = load!((load)(userptr, c"glUniform2ui64ARB".as_ptr()) as PFNGLUNIFORM2UI64ARBPROC);
            self.glUniform2ui64vARB = load!((load)(userptr, c"glUniform2ui64vARB".as_ptr()) as PFNGLUNIFORM2UI64VARBPROC);
            self.glUniform3i64ARB = load!((load)(userptr, c"glUniform3i64ARB".as_ptr()) as PFNGLUNIFORM3I64ARBPROC);
            self.glUniform3i64vARB = load!((load)(userptr, c"glUniform3i64vARB".as_ptr()) as PFNGLUNIFORM3I64VARBPROC);
            self.glUniform3ui64ARB = load!((load)(userptr, c"glUniform3ui64ARB".as_ptr()) as PFNGLUNIFORM3UI64ARBPROC);
            self.glUniform3ui64vARB = load!((load)(userptr, c"glUniform3ui64vARB".as_ptr()) as PFNGLUNIFORM3UI64VARBPROC);
            self.glUniform4i64ARB = load!((load)(userptr, c"glUniform4i64ARB".as_ptr()) as PFNGLUNIFORM4I64ARBPROC);
            self.glUniform4i64vARB = load!((load)(userptr, c"glUniform4i64vARB".as_ptr()) as PFNGLUNIFORM4I64VARBPROC);
            self.glUniform4ui64ARB = load!((load)(userptr, c"glUniform4ui64ARB".as_ptr()) as PFNGLUNIFORM4UI64ARBPROC);
            self.glUniform4ui64vARB = load!((load)(userptr, c"glUniform4ui64vARB".as_ptr()) as PFNGLUNIFORM4UI64VARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_instanced_arrays(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_instanced_arrays { return; }
            self.glVertexAttribDivisorARB = load!((load)(userptr, c"glVertexAttribDivisorARB".as_ptr()) as PFNGLVERTEXATTRIBDIVISORARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_internalformat_query(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_internalformat_query { return; }
            self.glGetInternalformativ = load!((load)(userptr, c"glGetInternalformativ".as_ptr()) as PFNGLGETINTERNALFORMATIVPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_internalformat_query2(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_internalformat_query2 { return; }
            self.glGetInternalformati64v = load!((load)(userptr, c"glGetInternalformati64v".as_ptr()) as PFNGLGETINTERNALFORMATI64VPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_map_buffer_range(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_map_buffer_range { return; }
            self.glFlushMappedBufferRange = load!((load)(userptr, c"glFlushMappedBufferRange".as_ptr()) as PFNGLFLUSHMAPPEDBUFFERRANGEPROC);
            self.glMapBufferRange = load!((load)(userptr, c"glMapBufferRange".as_ptr()) as PFNGLMAPBUFFERRANGEPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_multi_bind(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_multi_bind { return; }
            self.glBindBuffersBase = load!((load)(userptr, c"glBindBuffersBase".as_ptr()) as PFNGLBINDBUFFERSBASEPROC);
            self.glBindBuffersRange = load!((load)(userptr, c"glBindBuffersRange".as_ptr()) as PFNGLBINDBUFFERSRANGEPROC);
            self.glBindImageTextures = load!((load)(userptr, c"glBindImageTextures".as_ptr()) as PFNGLBINDIMAGETEXTURESPROC);
            self.glBindSamplers = load!((load)(userptr, c"glBindSamplers".as_ptr()) as PFNGLBINDSAMPLERSPROC);
            self.glBindTextures = load!((load)(userptr, c"glBindTextures".as_ptr()) as PFNGLBINDTEXTURESPROC);
            self.glBindVertexBuffers = load!((load)(userptr, c"glBindVertexBuffers".as_ptr()) as PFNGLBINDVERTEXBUFFERSPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_multi_draw_indirect(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_multi_draw_indirect { return; }
            self.glMultiDrawArraysIndirect = load!((load)(userptr, c"glMultiDrawArraysIndirect".as_ptr()) as PFNGLMULTIDRAWARRAYSINDIRECTPROC);
            self.glMultiDrawElementsIndirect = load!((load)(userptr, c"glMultiDrawElementsIndirect".as_ptr()) as PFNGLMULTIDRAWELEMENTSINDIRECTPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_multisample(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_multisample { return; }
            self.glSampleCoverageARB = load!((load)(userptr, c"glSampleCoverageARB".as_ptr()) as PFNGLSAMPLECOVERAGEARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_multitexture(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_multitexture { return; }
            self.glActiveTextureARB = load!((load)(userptr, c"glActiveTextureARB".as_ptr()) as PFNGLACTIVETEXTUREARBPROC);
            self.glClientActiveTextureARB = load!((load)(userptr, c"glClientActiveTextureARB".as_ptr()) as PFNGLCLIENTACTIVETEXTUREARBPROC);
            self.glMultiTexCoord1dARB = load!((load)(userptr, c"glMultiTexCoord1dARB".as_ptr()) as PFNGLMULTITEXCOORD1DARBPROC);
            self.glMultiTexCoord1dvARB = load!((load)(userptr, c"glMultiTexCoord1dvARB".as_ptr()) as PFNGLMULTITEXCOORD1DVARBPROC);
            self.glMultiTexCoord1fARB = load!((load)(userptr, c"glMultiTexCoord1fARB".as_ptr()) as PFNGLMULTITEXCOORD1FARBPROC);
            self.glMultiTexCoord1fvARB = load!((load)(userptr, c"glMultiTexCoord1fvARB".as_ptr()) as PFNGLMULTITEXCOORD1FVARBPROC);
            self.glMultiTexCoord1iARB = load!((load)(userptr, c"glMultiTexCoord1iARB".as_ptr()) as PFNGLMULTITEXCOORD1IARBPROC);
            self.glMultiTexCoord1ivARB = load!((load)(userptr, c"glMultiTexCoord1ivARB".as_ptr()) as PFNGLMULTITEXCOORD1IVARBPROC);
            self.glMultiTexCoord1sARB = load!((load)(userptr, c"glMultiTexCoord1sARB".as_ptr()) as PFNGLMULTITEXCOORD1SARBPROC);
            self.glMultiTexCoord1svARB = load!((load)(userptr, c"glMultiTexCoord1svARB".as_ptr()) as PFNGLMULTITEXCOORD1SVARBPROC);
            self.glMultiTexCoord2dARB = load!((load)(userptr, c"glMultiTexCoord2dARB".as_ptr()) as PFNGLMULTITEXCOORD2DARBPROC);
            self.glMultiTexCoord2dvARB = load!((load)(userptr, c"glMultiTexCoord2dvARB".as_ptr()) as PFNGLMULTITEXCOORD2DVARBPROC);
            self.glMultiTexCoord2fARB = load!((load)(userptr, c"glMultiTexCoord2fARB".as_ptr()) as PFNGLMULTITEXCOORD2FARBPROC);
            self.glMultiTexCoord2fvARB = load!((load)(userptr, c"glMultiTexCoord2fvARB".as_ptr()) as PFNGLMULTITEXCOORD2FVARBPROC);
            self.glMultiTexCoord2iARB = load!((load)(userptr, c"glMultiTexCoord2iARB".as_ptr()) as PFNGLMULTITEXCOORD2IARBPROC);
            self.glMultiTexCoord2ivARB = load!((load)(userptr, c"glMultiTexCoord2ivARB".as_ptr()) as PFNGLMULTITEXCOORD2IVARBPROC);
            self.glMultiTexCoord2sARB = load!((load)(userptr, c"glMultiTexCoord2sARB".as_ptr()) as PFNGLMULTITEXCOORD2SARBPROC);
            self.glMultiTexCoord2svARB = load!((load)(userptr, c"glMultiTexCoord2svARB".as_ptr()) as PFNGLMULTITEXCOORD2SVARBPROC);
            self.glMultiTexCoord3dARB = load!((load)(userptr, c"glMultiTexCoord3dARB".as_ptr()) as PFNGLMULTITEXCOORD3DARBPROC);
            self.glMultiTexCoord3dvARB = load!((load)(userptr, c"glMultiTexCoord3dvARB".as_ptr()) as PFNGLMULTITEXCOORD3DVARBPROC);
            self.glMultiTexCoord3fARB = load!((load)(userptr, c"glMultiTexCoord3fARB".as_ptr()) as PFNGLMULTITEXCOORD3FARBPROC);
            self.glMultiTexCoord3fvARB = load!((load)(userptr, c"glMultiTexCoord3fvARB".as_ptr()) as PFNGLMULTITEXCOORD3FVARBPROC);
            self.glMultiTexCoord3iARB = load!((load)(userptr, c"glMultiTexCoord3iARB".as_ptr()) as PFNGLMULTITEXCOORD3IARBPROC);
            self.glMultiTexCoord3ivARB = load!((load)(userptr, c"glMultiTexCoord3ivARB".as_ptr()) as PFNGLMULTITEXCOORD3IVARBPROC);
            self.glMultiTexCoord3sARB = load!((load)(userptr, c"glMultiTexCoord3sARB".as_ptr()) as PFNGLMULTITEXCOORD3SARBPROC);
            self.glMultiTexCoord3svARB = load!((load)(userptr, c"glMultiTexCoord3svARB".as_ptr()) as PFNGLMULTITEXCOORD3SVARBPROC);
            self.glMultiTexCoord4dARB = load!((load)(userptr, c"glMultiTexCoord4dARB".as_ptr()) as PFNGLMULTITEXCOORD4DARBPROC);
            self.glMultiTexCoord4dvARB = load!((load)(userptr, c"glMultiTexCoord4dvARB".as_ptr()) as PFNGLMULTITEXCOORD4DVARBPROC);
            self.glMultiTexCoord4fARB = load!((load)(userptr, c"glMultiTexCoord4fARB".as_ptr()) as PFNGLMULTITEXCOORD4FARBPROC);
            self.glMultiTexCoord4fvARB = load!((load)(userptr, c"glMultiTexCoord4fvARB".as_ptr()) as PFNGLMULTITEXCOORD4FVARBPROC);
            self.glMultiTexCoord4iARB = load!((load)(userptr, c"glMultiTexCoord4iARB".as_ptr()) as PFNGLMULTITEXCOORD4IARBPROC);
            self.glMultiTexCoord4ivARB = load!((load)(userptr, c"glMultiTexCoord4ivARB".as_ptr()) as PFNGLMULTITEXCOORD4IVARBPROC);
            self.glMultiTexCoord4sARB = load!((load)(userptr, c"glMultiTexCoord4sARB".as_ptr()) as PFNGLMULTITEXCOORD4SARBPROC);
            self.glMultiTexCoord4svARB = load!((load)(userptr, c"glMultiTexCoord4svARB".as_ptr()) as PFNGLMULTITEXCOORD4SVARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_occlusion_query(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_occlusion_query { return; }
            self.glBeginQueryARB = load!((load)(userptr, c"glBeginQueryARB".as_ptr()) as PFNGLBEGINQUERYARBPROC);
            self.glDeleteQueriesARB = load!((load)(userptr, c"glDeleteQueriesARB".as_ptr()) as PFNGLDELETEQUERIESARBPROC);
            self.glEndQueryARB = load!((load)(userptr, c"glEndQueryARB".as_ptr()) as PFNGLENDQUERYARBPROC);
            self.glGenQueriesARB = load!((load)(userptr, c"glGenQueriesARB".as_ptr()) as PFNGLGENQUERIESARBPROC);
            self.glGetQueryObjectivARB = load!((load)(userptr, c"glGetQueryObjectivARB".as_ptr()) as PFNGLGETQUERYOBJECTIVARBPROC);
            self.glGetQueryObjectuivARB = load!((load)(userptr, c"glGetQueryObjectuivARB".as_ptr()) as PFNGLGETQUERYOBJECTUIVARBPROC);
            self.glGetQueryivARB = load!((load)(userptr, c"glGetQueryivARB".as_ptr()) as PFNGLGETQUERYIVARBPROC);
            self.glIsQueryARB = load!((load)(userptr, c"glIsQueryARB".as_ptr()) as PFNGLISQUERYARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_sample_locations(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_sample_locations { return; }
            self.glEvaluateDepthValuesARB = load!((load)(userptr, c"glEvaluateDepthValuesARB".as_ptr()) as PFNGLEVALUATEDEPTHVALUESARBPROC);
            self.glFramebufferSampleLocationsfvARB = load!((load)(userptr, c"glFramebufferSampleLocationsfvARB".as_ptr()) as PFNGLFRAMEBUFFERSAMPLELOCATIONSFVARBPROC);
            self.glNamedFramebufferSampleLocationsfvARB = load!((load)(userptr, c"glNamedFramebufferSampleLocationsfvARB".as_ptr()) as PFNGLNAMEDFRAMEBUFFERSAMPLELOCATIONSFVARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_sample_shading(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_sample_shading { return; }
            self.glMinSampleShadingARB = load!((load)(userptr, c"glMinSampleShadingARB".as_ptr()) as PFNGLMINSAMPLESHADINGARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_shader_atomic_counters(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_shader_atomic_counters { return; }
            self.glGetActiveAtomicCounterBufferiv = load!((load)(userptr, c"glGetActiveAtomicCounterBufferiv".as_ptr()) as PFNGLGETACTIVEATOMICCOUNTERBUFFERIVPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_shader_image_load_store(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_shader_image_load_store { return; }
            self.glBindImageTexture = load!((load)(userptr, c"glBindImageTexture".as_ptr()) as PFNGLBINDIMAGETEXTUREPROC);
            self.glMemoryBarrier = load!((load)(userptr, c"glMemoryBarrier".as_ptr()) as PFNGLMEMORYBARRIERPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_shader_objects(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_shader_objects { return; }
            self.glAttachObjectARB = load!((load)(userptr, c"glAttachObjectARB".as_ptr()) as PFNGLATTACHOBJECTARBPROC);
            self.glCompileShaderARB = load!((load)(userptr, c"glCompileShaderARB".as_ptr()) as PFNGLCOMPILESHADERARBPROC);
            self.glCreateProgramObjectARB = load!((load)(userptr, c"glCreateProgramObjectARB".as_ptr()) as PFNGLCREATEPROGRAMOBJECTARBPROC);
            self.glCreateShaderObjectARB = load!((load)(userptr, c"glCreateShaderObjectARB".as_ptr()) as PFNGLCREATESHADEROBJECTARBPROC);
            self.glDeleteObjectARB = load!((load)(userptr, c"glDeleteObjectARB".as_ptr()) as PFNGLDELETEOBJECTARBPROC);
            self.glDetachObjectARB = load!((load)(userptr, c"glDetachObjectARB".as_ptr()) as PFNGLDETACHOBJECTARBPROC);
            self.glGetActiveUniformARB = load!((load)(userptr, c"glGetActiveUniformARB".as_ptr()) as PFNGLGETACTIVEUNIFORMARBPROC);
            self.glGetAttachedObjectsARB = load!((load)(userptr, c"glGetAttachedObjectsARB".as_ptr()) as PFNGLGETATTACHEDOBJECTSARBPROC);
            self.glGetHandleARB = load!((load)(userptr, c"glGetHandleARB".as_ptr()) as PFNGLGETHANDLEARBPROC);
            self.glGetInfoLogARB = load!((load)(userptr, c"glGetInfoLogARB".as_ptr()) as PFNGLGETINFOLOGARBPROC);
            self.glGetObjectParameterfvARB = load!((load)(userptr, c"glGetObjectParameterfvARB".as_ptr()) as PFNGLGETOBJECTPARAMETERFVARBPROC);
            self.glGetObjectParameterivARB = load!((load)(userptr, c"glGetObjectParameterivARB".as_ptr()) as PFNGLGETOBJECTPARAMETERIVARBPROC);
            self.glGetShaderSourceARB = load!((load)(userptr, c"glGetShaderSourceARB".as_ptr()) as PFNGLGETSHADERSOURCEARBPROC);
            self.glGetUniformLocationARB = load!((load)(userptr, c"glGetUniformLocationARB".as_ptr()) as PFNGLGETUNIFORMLOCATIONARBPROC);
            self.glGetUniformfvARB = load!((load)(userptr, c"glGetUniformfvARB".as_ptr()) as PFNGLGETUNIFORMFVARBPROC);
            self.glGetUniformivARB = load!((load)(userptr, c"glGetUniformivARB".as_ptr()) as PFNGLGETUNIFORMIVARBPROC);
            self.glLinkProgramARB = load!((load)(userptr, c"glLinkProgramARB".as_ptr()) as PFNGLLINKPROGRAMARBPROC);
            self.glShaderSourceARB = load!((load)(userptr, c"glShaderSourceARB".as_ptr()) as PFNGLSHADERSOURCEARBPROC);
            self.glUniform1fARB = load!((load)(userptr, c"glUniform1fARB".as_ptr()) as PFNGLUNIFORM1FARBPROC);
            self.glUniform1fvARB = load!((load)(userptr, c"glUniform1fvARB".as_ptr()) as PFNGLUNIFORM1FVARBPROC);
            self.glUniform1iARB = load!((load)(userptr, c"glUniform1iARB".as_ptr()) as PFNGLUNIFORM1IARBPROC);
            self.glUniform1ivARB = load!((load)(userptr, c"glUniform1ivARB".as_ptr()) as PFNGLUNIFORM1IVARBPROC);
            self.glUniform2fARB = load!((load)(userptr, c"glUniform2fARB".as_ptr()) as PFNGLUNIFORM2FARBPROC);
            self.glUniform2fvARB = load!((load)(userptr, c"glUniform2fvARB".as_ptr()) as PFNGLUNIFORM2FVARBPROC);
            self.glUniform2iARB = load!((load)(userptr, c"glUniform2iARB".as_ptr()) as PFNGLUNIFORM2IARBPROC);
            self.glUniform2ivARB = load!((load)(userptr, c"glUniform2ivARB".as_ptr()) as PFNGLUNIFORM2IVARBPROC);
            self.glUniform3fARB = load!((load)(userptr, c"glUniform3fARB".as_ptr()) as PFNGLUNIFORM3FARBPROC);
            self.glUniform3fvARB = load!((load)(userptr, c"glUniform3fvARB".as_ptr()) as PFNGLUNIFORM3FVARBPROC);
            self.glUniform3iARB = load!((load)(userptr, c"glUniform3iARB".as_ptr()) as PFNGLUNIFORM3IARBPROC);
            self.glUniform3ivARB = load!((load)(userptr, c"glUniform3ivARB".as_ptr()) as PFNGLUNIFORM3IVARBPROC);
            self.glUniform4fARB = load!((load)(userptr, c"glUniform4fARB".as_ptr()) as PFNGLUNIFORM4FARBPROC);
            self.glUniform4fvARB = load!((load)(userptr, c"glUniform4fvARB".as_ptr()) as PFNGLUNIFORM4FVARBPROC);
            self.glUniform4iARB = load!((load)(userptr, c"glUniform4iARB".as_ptr()) as PFNGLUNIFORM4IARBPROC);
            self.glUniform4ivARB = load!((load)(userptr, c"glUniform4ivARB".as_ptr()) as PFNGLUNIFORM4IVARBPROC);
            self.glUniformMatrix2fvARB = load!((load)(userptr, c"glUniformMatrix2fvARB".as_ptr()) as PFNGLUNIFORMMATRIX2FVARBPROC);
            self.glUniformMatrix3fvARB = load!((load)(userptr, c"glUniformMatrix3fvARB".as_ptr()) as PFNGLUNIFORMMATRIX3FVARBPROC);
            self.glUniformMatrix4fvARB = load!((load)(userptr, c"glUniformMatrix4fvARB".as_ptr()) as PFNGLUNIFORMMATRIX4FVARBPROC);
            self.glUseProgramObjectARB = load!((load)(userptr, c"glUseProgramObjectARB".as_ptr()) as PFNGLUSEPROGRAMOBJECTARBPROC);
            self.glValidateProgramARB = load!((load)(userptr, c"glValidateProgramARB".as_ptr()) as PFNGLVALIDATEPROGRAMARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_shader_storage_buffer_object(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_shader_storage_buffer_object { return; }
            self.glShaderStorageBlockBinding = load!((load)(userptr, c"glShaderStorageBlockBinding".as_ptr()) as PFNGLSHADERSTORAGEBLOCKBINDINGPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_shading_language_include(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_shading_language_include { return; }
            self.glCompileShaderIncludeARB = load!((load)(userptr, c"glCompileShaderIncludeARB".as_ptr()) as PFNGLCOMPILESHADERINCLUDEARBPROC);
            self.glDeleteNamedStringARB = load!((load)(userptr, c"glDeleteNamedStringARB".as_ptr()) as PFNGLDELETENAMEDSTRINGARBPROC);
            self.glGetNamedStringARB = load!((load)(userptr, c"glGetNamedStringARB".as_ptr()) as PFNGLGETNAMEDSTRINGARBPROC);
            self.glGetNamedStringivARB = load!((load)(userptr, c"glGetNamedStringivARB".as_ptr()) as PFNGLGETNAMEDSTRINGIVARBPROC);
            self.glIsNamedStringARB = load!((load)(userptr, c"glIsNamedStringARB".as_ptr()) as PFNGLISNAMEDSTRINGARBPROC);
            self.glNamedStringARB = load!((load)(userptr, c"glNamedStringARB".as_ptr()) as PFNGLNAMEDSTRINGARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_tessellation_shader(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_tessellation_shader { return; }
            self.glPatchParameterfv = load!((load)(userptr, c"glPatchParameterfv".as_ptr()) as PFNGLPATCHPARAMETERFVPROC);
            self.glPatchParameteri = load!((load)(userptr, c"glPatchParameteri".as_ptr()) as PFNGLPATCHPARAMETERIPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_texture_compression(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_texture_compression { return; }
            self.glCompressedTexImage1DARB = load!((load)(userptr, c"glCompressedTexImage1DARB".as_ptr()) as PFNGLCOMPRESSEDTEXIMAGE1DARBPROC);
            self.glCompressedTexImage2DARB = load!((load)(userptr, c"glCompressedTexImage2DARB".as_ptr()) as PFNGLCOMPRESSEDTEXIMAGE2DARBPROC);
            self.glCompressedTexImage3DARB = load!((load)(userptr, c"glCompressedTexImage3DARB".as_ptr()) as PFNGLCOMPRESSEDTEXIMAGE3DARBPROC);
            self.glCompressedTexSubImage1DARB = load!((load)(userptr, c"glCompressedTexSubImage1DARB".as_ptr()) as PFNGLCOMPRESSEDTEXSUBIMAGE1DARBPROC);
            self.glCompressedTexSubImage2DARB = load!((load)(userptr, c"glCompressedTexSubImage2DARB".as_ptr()) as PFNGLCOMPRESSEDTEXSUBIMAGE2DARBPROC);
            self.glCompressedTexSubImage3DARB = load!((load)(userptr, c"glCompressedTexSubImage3DARB".as_ptr()) as PFNGLCOMPRESSEDTEXSUBIMAGE3DARBPROC);
            self.glGetCompressedTexImageARB = load!((load)(userptr, c"glGetCompressedTexImageARB".as_ptr()) as PFNGLGETCOMPRESSEDTEXIMAGEARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_texture_multisample(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_texture_multisample { return; }
            self.glGetMultisamplefv = load!((load)(userptr, c"glGetMultisamplefv".as_ptr()) as PFNGLGETMULTISAMPLEFVPROC);
            self.glSampleMaski = load!((load)(userptr, c"glSampleMaski".as_ptr()) as PFNGLSAMPLEMASKIPROC);
            self.glTexImage2DMultisample = load!((load)(userptr, c"glTexImage2DMultisample".as_ptr()) as PFNGLTEXIMAGE2DMULTISAMPLEPROC);
            self.glTexImage3DMultisample = load!((load)(userptr, c"glTexImage3DMultisample".as_ptr()) as PFNGLTEXIMAGE3DMULTISAMPLEPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_texture_storage(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_texture_storage { return; }
            self.glTexStorage1D = load!((load)(userptr, c"glTexStorage1D".as_ptr()) as PFNGLTEXSTORAGE1DPROC);
            self.glTexStorage2D = load!((load)(userptr, c"glTexStorage2D".as_ptr()) as PFNGLTEXSTORAGE2DPROC);
            self.glTexStorage3D = load!((load)(userptr, c"glTexStorage3D".as_ptr()) as PFNGLTEXSTORAGE3DPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_texture_view(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_texture_view { return; }
            self.glTextureView = load!((load)(userptr, c"glTextureView".as_ptr()) as PFNGLTEXTUREVIEWPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_timer_query(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_timer_query { return; }
            self.glGetQueryObjecti64v = load!((load)(userptr, c"glGetQueryObjecti64v".as_ptr()) as PFNGLGETQUERYOBJECTI64VPROC);
            self.glGetQueryObjectui64v = load!((load)(userptr, c"glGetQueryObjectui64v".as_ptr()) as PFNGLGETQUERYOBJECTUI64VPROC);
            self.glQueryCounter = load!((load)(userptr, c"glQueryCounter".as_ptr()) as PFNGLQUERYCOUNTERPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_transpose_matrix(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_transpose_matrix { return; }
            self.glLoadTransposeMatrixdARB = load!((load)(userptr, c"glLoadTransposeMatrixdARB".as_ptr()) as PFNGLLOADTRANSPOSEMATRIXDARBPROC);
            self.glLoadTransposeMatrixfARB = load!((load)(userptr, c"glLoadTransposeMatrixfARB".as_ptr()) as PFNGLLOADTRANSPOSEMATRIXFARBPROC);
            self.glMultTransposeMatrixdARB = load!((load)(userptr, c"glMultTransposeMatrixdARB".as_ptr()) as PFNGLMULTTRANSPOSEMATRIXDARBPROC);
            self.glMultTransposeMatrixfARB = load!((load)(userptr, c"glMultTransposeMatrixfARB".as_ptr()) as PFNGLMULTTRANSPOSEMATRIXFARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_uniform_buffer_object(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_uniform_buffer_object { return; }
            self.glBindBufferBase = load!((load)(userptr, c"glBindBufferBase".as_ptr()) as PFNGLBINDBUFFERBASEPROC);
            self.glBindBufferRange = load!((load)(userptr, c"glBindBufferRange".as_ptr()) as PFNGLBINDBUFFERRANGEPROC);
            self.glGetActiveUniformBlockName = load!((load)(userptr, c"glGetActiveUniformBlockName".as_ptr()) as PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC);
            self.glGetActiveUniformBlockiv = load!((load)(userptr, c"glGetActiveUniformBlockiv".as_ptr()) as PFNGLGETACTIVEUNIFORMBLOCKIVPROC);
            self.glGetActiveUniformName = load!((load)(userptr, c"glGetActiveUniformName".as_ptr()) as PFNGLGETACTIVEUNIFORMNAMEPROC);
            self.glGetActiveUniformsiv = load!((load)(userptr, c"glGetActiveUniformsiv".as_ptr()) as PFNGLGETACTIVEUNIFORMSIVPROC);
            self.glGetIntegeri_v = load!((load)(userptr, c"glGetIntegeri_v".as_ptr()) as PFNGLGETINTEGERI_VPROC);
            self.glGetUniformBlockIndex = load!((load)(userptr, c"glGetUniformBlockIndex".as_ptr()) as PFNGLGETUNIFORMBLOCKINDEXPROC);
            self.glGetUniformIndices = load!((load)(userptr, c"glGetUniformIndices".as_ptr()) as PFNGLGETUNIFORMINDICESPROC);
            self.glUniformBlockBinding = load!((load)(userptr, c"glUniformBlockBinding".as_ptr()) as PFNGLUNIFORMBLOCKBINDINGPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_vertex_array_object(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_vertex_array_object { return; }
            self.glBindVertexArray = load!((load)(userptr, c"glBindVertexArray".as_ptr()) as PFNGLBINDVERTEXARRAYPROC);
            self.glDeleteVertexArrays = load!((load)(userptr, c"glDeleteVertexArrays".as_ptr()) as PFNGLDELETEVERTEXARRAYSPROC);
            self.glGenVertexArrays = load!((load)(userptr, c"glGenVertexArrays".as_ptr()) as PFNGLGENVERTEXARRAYSPROC);
            self.glIsVertexArray = load!((load)(userptr, c"glIsVertexArray".as_ptr()) as PFNGLISVERTEXARRAYPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_vertex_attrib_binding(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_vertex_attrib_binding { return; }
            self.glBindVertexBuffer = load!((load)(userptr, c"glBindVertexBuffer".as_ptr()) as PFNGLBINDVERTEXBUFFERPROC);
            self.glVertexAttribBinding = load!((load)(userptr, c"glVertexAttribBinding".as_ptr()) as PFNGLVERTEXATTRIBBINDINGPROC);
            self.glVertexAttribFormat = load!((load)(userptr, c"glVertexAttribFormat".as_ptr()) as PFNGLVERTEXATTRIBFORMATPROC);
            self.glVertexAttribIFormat = load!((load)(userptr, c"glVertexAttribIFormat".as_ptr()) as PFNGLVERTEXATTRIBIFORMATPROC);
            self.glVertexAttribLFormat = load!((load)(userptr, c"glVertexAttribLFormat".as_ptr()) as PFNGLVERTEXATTRIBLFORMATPROC);
            self.glVertexBindingDivisor = load!((load)(userptr, c"glVertexBindingDivisor".as_ptr()) as PFNGLVERTEXBINDINGDIVISORPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_vertex_buffer_object(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_vertex_buffer_object { return; }
            self.glBindBufferARB = load!((load)(userptr, c"glBindBufferARB".as_ptr()) as PFNGLBINDBUFFERARBPROC);
            self.glBufferDataARB = load!((load)(userptr, c"glBufferDataARB".as_ptr()) as PFNGLBUFFERDATAARBPROC);
            self.glBufferSubDataARB = load!((load)(userptr, c"glBufferSubDataARB".as_ptr()) as PFNGLBUFFERSUBDATAARBPROC);
            self.glDeleteBuffersARB = load!((load)(userptr, c"glDeleteBuffersARB".as_ptr()) as PFNGLDELETEBUFFERSARBPROC);
            self.glGenBuffersARB = load!((load)(userptr, c"glGenBuffersARB".as_ptr()) as PFNGLGENBUFFERSARBPROC);
            self.glGetBufferParameterivARB = load!((load)(userptr, c"glGetBufferParameterivARB".as_ptr()) as PFNGLGETBUFFERPARAMETERIVARBPROC);
            self.glGetBufferPointervARB = load!((load)(userptr, c"glGetBufferPointervARB".as_ptr()) as PFNGLGETBUFFERPOINTERVARBPROC);
            self.glGetBufferSubDataARB = load!((load)(userptr, c"glGetBufferSubDataARB".as_ptr()) as PFNGLGETBUFFERSUBDATAARBPROC);
            self.glIsBufferARB = load!((load)(userptr, c"glIsBufferARB".as_ptr()) as PFNGLISBUFFERARBPROC);
            self.glMapBufferARB = load!((load)(userptr, c"glMapBufferARB".as_ptr()) as PFNGLMAPBUFFERARBPROC);
            self.glUnmapBufferARB = load!((load)(userptr, c"glUnmapBufferARB".as_ptr()) as PFNGLUNMAPBUFFERARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_vertex_program(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_vertex_program { return; }
            self.glBindProgramARB = load!((load)(userptr, c"glBindProgramARB".as_ptr()) as PFNGLBINDPROGRAMARBPROC);
            self.glDeleteProgramsARB = load!((load)(userptr, c"glDeleteProgramsARB".as_ptr()) as PFNGLDELETEPROGRAMSARBPROC);
            self.glDisableVertexAttribArrayARB = load!((load)(userptr, c"glDisableVertexAttribArrayARB".as_ptr()) as PFNGLDISABLEVERTEXATTRIBARRAYARBPROC);
            self.glEnableVertexAttribArrayARB = load!((load)(userptr, c"glEnableVertexAttribArrayARB".as_ptr()) as PFNGLENABLEVERTEXATTRIBARRAYARBPROC);
            self.glGenProgramsARB = load!((load)(userptr, c"glGenProgramsARB".as_ptr()) as PFNGLGENPROGRAMSARBPROC);
            self.glGetProgramEnvParameterdvARB = load!((load)(userptr, c"glGetProgramEnvParameterdvARB".as_ptr()) as PFNGLGETPROGRAMENVPARAMETERDVARBPROC);
            self.glGetProgramEnvParameterfvARB = load!((load)(userptr, c"glGetProgramEnvParameterfvARB".as_ptr()) as PFNGLGETPROGRAMENVPARAMETERFVARBPROC);
            self.glGetProgramLocalParameterdvARB = load!((load)(userptr, c"glGetProgramLocalParameterdvARB".as_ptr()) as PFNGLGETPROGRAMLOCALPARAMETERDVARBPROC);
            self.glGetProgramLocalParameterfvARB = load!((load)(userptr, c"glGetProgramLocalParameterfvARB".as_ptr()) as PFNGLGETPROGRAMLOCALPARAMETERFVARBPROC);
            self.glGetProgramStringARB = load!((load)(userptr, c"glGetProgramStringARB".as_ptr()) as PFNGLGETPROGRAMSTRINGARBPROC);
            self.glGetProgramivARB = load!((load)(userptr, c"glGetProgramivARB".as_ptr()) as PFNGLGETPROGRAMIVARBPROC);
            self.glGetVertexAttribPointervARB = load!((load)(userptr, c"glGetVertexAttribPointervARB".as_ptr()) as PFNGLGETVERTEXATTRIBPOINTERVARBPROC);
            self.glGetVertexAttribdvARB = load!((load)(userptr, c"glGetVertexAttribdvARB".as_ptr()) as PFNGLGETVERTEXATTRIBDVARBPROC);
            self.glGetVertexAttribfvARB = load!((load)(userptr, c"glGetVertexAttribfvARB".as_ptr()) as PFNGLGETVERTEXATTRIBFVARBPROC);
            self.glGetVertexAttribivARB = load!((load)(userptr, c"glGetVertexAttribivARB".as_ptr()) as PFNGLGETVERTEXATTRIBIVARBPROC);
            self.glIsProgramARB = load!((load)(userptr, c"glIsProgramARB".as_ptr()) as PFNGLISPROGRAMARBPROC);
            self.glProgramEnvParameter4dARB = load!((load)(userptr, c"glProgramEnvParameter4dARB".as_ptr()) as PFNGLPROGRAMENVPARAMETER4DARBPROC);
            self.glProgramEnvParameter4dvARB = load!((load)(userptr, c"glProgramEnvParameter4dvARB".as_ptr()) as PFNGLPROGRAMENVPARAMETER4DVARBPROC);
            self.glProgramEnvParameter4fARB = load!((load)(userptr, c"glProgramEnvParameter4fARB".as_ptr()) as PFNGLPROGRAMENVPARAMETER4FARBPROC);
            self.glProgramEnvParameter4fvARB = load!((load)(userptr, c"glProgramEnvParameter4fvARB".as_ptr()) as PFNGLPROGRAMENVPARAMETER4FVARBPROC);
            self.glProgramLocalParameter4dARB = load!((load)(userptr, c"glProgramLocalParameter4dARB".as_ptr()) as PFNGLPROGRAMLOCALPARAMETER4DARBPROC);
            self.glProgramLocalParameter4dvARB = load!((load)(userptr, c"glProgramLocalParameter4dvARB".as_ptr()) as PFNGLPROGRAMLOCALPARAMETER4DVARBPROC);
            self.glProgramLocalParameter4fARB = load!((load)(userptr, c"glProgramLocalParameter4fARB".as_ptr()) as PFNGLPROGRAMLOCALPARAMETER4FARBPROC);
            self.glProgramLocalParameter4fvARB = load!((load)(userptr, c"glProgramLocalParameter4fvARB".as_ptr()) as PFNGLPROGRAMLOCALPARAMETER4FVARBPROC);
            self.glProgramStringARB = load!((load)(userptr, c"glProgramStringARB".as_ptr()) as PFNGLPROGRAMSTRINGARBPROC);
            self.glVertexAttrib1dARB = load!((load)(userptr, c"glVertexAttrib1dARB".as_ptr()) as PFNGLVERTEXATTRIB1DARBPROC);
            self.glVertexAttrib1dvARB = load!((load)(userptr, c"glVertexAttrib1dvARB".as_ptr()) as PFNGLVERTEXATTRIB1DVARBPROC);
            self.glVertexAttrib1fARB = load!((load)(userptr, c"glVertexAttrib1fARB".as_ptr()) as PFNGLVERTEXATTRIB1FARBPROC);
            self.glVertexAttrib1fvARB = load!((load)(userptr, c"glVertexAttrib1fvARB".as_ptr()) as PFNGLVERTEXATTRIB1FVARBPROC);
            self.glVertexAttrib1sARB = load!((load)(userptr, c"glVertexAttrib1sARB".as_ptr()) as PFNGLVERTEXATTRIB1SARBPROC);
            self.glVertexAttrib1svARB = load!((load)(userptr, c"glVertexAttrib1svARB".as_ptr()) as PFNGLVERTEXATTRIB1SVARBPROC);
            self.glVertexAttrib2dARB = load!((load)(userptr, c"glVertexAttrib2dARB".as_ptr()) as PFNGLVERTEXATTRIB2DARBPROC);
            self.glVertexAttrib2dvARB = load!((load)(userptr, c"glVertexAttrib2dvARB".as_ptr()) as PFNGLVERTEXATTRIB2DVARBPROC);
            self.glVertexAttrib2fARB = load!((load)(userptr, c"glVertexAttrib2fARB".as_ptr()) as PFNGLVERTEXATTRIB2FARBPROC);
            self.glVertexAttrib2fvARB = load!((load)(userptr, c"glVertexAttrib2fvARB".as_ptr()) as PFNGLVERTEXATTRIB2FVARBPROC);
            self.glVertexAttrib2sARB = load!((load)(userptr, c"glVertexAttrib2sARB".as_ptr()) as PFNGLVERTEXATTRIB2SARBPROC);
            self.glVertexAttrib2svARB = load!((load)(userptr, c"glVertexAttrib2svARB".as_ptr()) as PFNGLVERTEXATTRIB2SVARBPROC);
            self.glVertexAttrib3dARB = load!((load)(userptr, c"glVertexAttrib3dARB".as_ptr()) as PFNGLVERTEXATTRIB3DARBPROC);
            self.glVertexAttrib3dvARB = load!((load)(userptr, c"glVertexAttrib3dvARB".as_ptr()) as PFNGLVERTEXATTRIB3DVARBPROC);
            self.glVertexAttrib3fARB = load!((load)(userptr, c"glVertexAttrib3fARB".as_ptr()) as PFNGLVERTEXATTRIB3FARBPROC);
            self.glVertexAttrib3fvARB = load!((load)(userptr, c"glVertexAttrib3fvARB".as_ptr()) as PFNGLVERTEXATTRIB3FVARBPROC);
            self.glVertexAttrib3sARB = load!((load)(userptr, c"glVertexAttrib3sARB".as_ptr()) as PFNGLVERTEXATTRIB3SARBPROC);
            self.glVertexAttrib3svARB = load!((load)(userptr, c"glVertexAttrib3svARB".as_ptr()) as PFNGLVERTEXATTRIB3SVARBPROC);
            self.glVertexAttrib4NbvARB = load!((load)(userptr, c"glVertexAttrib4NbvARB".as_ptr()) as PFNGLVERTEXATTRIB4NBVARBPROC);
            self.glVertexAttrib4NivARB = load!((load)(userptr, c"glVertexAttrib4NivARB".as_ptr()) as PFNGLVERTEXATTRIB4NIVARBPROC);
            self.glVertexAttrib4NsvARB = load!((load)(userptr, c"glVertexAttrib4NsvARB".as_ptr()) as PFNGLVERTEXATTRIB4NSVARBPROC);
            self.glVertexAttrib4NubARB = load!((load)(userptr, c"glVertexAttrib4NubARB".as_ptr()) as PFNGLVERTEXATTRIB4NUBARBPROC);
            self.glVertexAttrib4NubvARB = load!((load)(userptr, c"glVertexAttrib4NubvARB".as_ptr()) as PFNGLVERTEXATTRIB4NUBVARBPROC);
            self.glVertexAttrib4NuivARB = load!((load)(userptr, c"glVertexAttrib4NuivARB".as_ptr()) as PFNGLVERTEXATTRIB4NUIVARBPROC);
            self.glVertexAttrib4NusvARB = load!((load)(userptr, c"glVertexAttrib4NusvARB".as_ptr()) as PFNGLVERTEXATTRIB4NUSVARBPROC);
            self.glVertexAttrib4bvARB = load!((load)(userptr, c"glVertexAttrib4bvARB".as_ptr()) as PFNGLVERTEXATTRIB4BVARBPROC);
            self.glVertexAttrib4dARB = load!((load)(userptr, c"glVertexAttrib4dARB".as_ptr()) as PFNGLVERTEXATTRIB4DARBPROC);
            self.glVertexAttrib4dvARB = load!((load)(userptr, c"glVertexAttrib4dvARB".as_ptr()) as PFNGLVERTEXATTRIB4DVARBPROC);
            self.glVertexAttrib4fARB = load!((load)(userptr, c"glVertexAttrib4fARB".as_ptr()) as PFNGLVERTEXATTRIB4FARBPROC);
            self.glVertexAttrib4fvARB = load!((load)(userptr, c"glVertexAttrib4fvARB".as_ptr()) as PFNGLVERTEXATTRIB4FVARBPROC);
            self.glVertexAttrib4ivARB = load!((load)(userptr, c"glVertexAttrib4ivARB".as_ptr()) as PFNGLVERTEXATTRIB4IVARBPROC);
            self.glVertexAttrib4sARB = load!((load)(userptr, c"glVertexAttrib4sARB".as_ptr()) as PFNGLVERTEXATTRIB4SARBPROC);
            self.glVertexAttrib4svARB = load!((load)(userptr, c"glVertexAttrib4svARB".as_ptr()) as PFNGLVERTEXATTRIB4SVARBPROC);
            self.glVertexAttrib4ubvARB = load!((load)(userptr, c"glVertexAttrib4ubvARB".as_ptr()) as PFNGLVERTEXATTRIB4UBVARBPROC);
            self.glVertexAttrib4uivARB = load!((load)(userptr, c"glVertexAttrib4uivARB".as_ptr()) as PFNGLVERTEXATTRIB4UIVARBPROC);
            self.glVertexAttrib4usvARB = load!((load)(userptr, c"glVertexAttrib4usvARB".as_ptr()) as PFNGLVERTEXATTRIB4USVARBPROC);
            self.glVertexAttribPointerARB = load!((load)(userptr, c"glVertexAttribPointerARB".as_ptr()) as PFNGLVERTEXATTRIBPOINTERARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_ARB_vertex_shader(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_vertex_shader { return; }
            self.glBindAttribLocationARB = load!((load)(userptr, c"glBindAttribLocationARB".as_ptr()) as PFNGLBINDATTRIBLOCATIONARBPROC);
            self.glDisableVertexAttribArrayARB = load!((load)(userptr, c"glDisableVertexAttribArrayARB".as_ptr()) as PFNGLDISABLEVERTEXATTRIBARRAYARBPROC);
            self.glEnableVertexAttribArrayARB = load!((load)(userptr, c"glEnableVertexAttribArrayARB".as_ptr()) as PFNGLENABLEVERTEXATTRIBARRAYARBPROC);
            self.glGetActiveAttribARB = load!((load)(userptr, c"glGetActiveAttribARB".as_ptr()) as PFNGLGETACTIVEATTRIBARBPROC);
            self.glGetAttribLocationARB = load!((load)(userptr, c"glGetAttribLocationARB".as_ptr()) as PFNGLGETATTRIBLOCATIONARBPROC);
            self.glGetVertexAttribPointervARB = load!((load)(userptr, c"glGetVertexAttribPointervARB".as_ptr()) as PFNGLGETVERTEXATTRIBPOINTERVARBPROC);
            self.glGetVertexAttribdvARB = load!((load)(userptr, c"glGetVertexAttribdvARB".as_ptr()) as PFNGLGETVERTEXATTRIBDVARBPROC);
            self.glGetVertexAttribfvARB = load!((load)(userptr, c"glGetVertexAttribfvARB".as_ptr()) as PFNGLGETVERTEXATTRIBFVARBPROC);
            self.glGetVertexAttribivARB = load!((load)(userptr, c"glGetVertexAttribivARB".as_ptr()) as PFNGLGETVERTEXATTRIBIVARBPROC);
            self.glVertexAttrib1dARB = load!((load)(userptr, c"glVertexAttrib1dARB".as_ptr()) as PFNGLVERTEXATTRIB1DARBPROC);
            self.glVertexAttrib1dvARB = load!((load)(userptr, c"glVertexAttrib1dvARB".as_ptr()) as PFNGLVERTEXATTRIB1DVARBPROC);
            self.glVertexAttrib1fARB = load!((load)(userptr, c"glVertexAttrib1fARB".as_ptr()) as PFNGLVERTEXATTRIB1FARBPROC);
            self.glVertexAttrib1fvARB = load!((load)(userptr, c"glVertexAttrib1fvARB".as_ptr()) as PFNGLVERTEXATTRIB1FVARBPROC);
            self.glVertexAttrib1sARB = load!((load)(userptr, c"glVertexAttrib1sARB".as_ptr()) as PFNGLVERTEXATTRIB1SARBPROC);
            self.glVertexAttrib1svARB = load!((load)(userptr, c"glVertexAttrib1svARB".as_ptr()) as PFNGLVERTEXATTRIB1SVARBPROC);
            self.glVertexAttrib2dARB = load!((load)(userptr, c"glVertexAttrib2dARB".as_ptr()) as PFNGLVERTEXATTRIB2DARBPROC);
            self.glVertexAttrib2dvARB = load!((load)(userptr, c"glVertexAttrib2dvARB".as_ptr()) as PFNGLVERTEXATTRIB2DVARBPROC);
            self.glVertexAttrib2fARB = load!((load)(userptr, c"glVertexAttrib2fARB".as_ptr()) as PFNGLVERTEXATTRIB2FARBPROC);
            self.glVertexAttrib2fvARB = load!((load)(userptr, c"glVertexAttrib2fvARB".as_ptr()) as PFNGLVERTEXATTRIB2FVARBPROC);
            self.glVertexAttrib2sARB = load!((load)(userptr, c"glVertexAttrib2sARB".as_ptr()) as PFNGLVERTEXATTRIB2SARBPROC);
            self.glVertexAttrib2svARB = load!((load)(userptr, c"glVertexAttrib2svARB".as_ptr()) as PFNGLVERTEXATTRIB2SVARBPROC);
            self.glVertexAttrib3dARB = load!((load)(userptr, c"glVertexAttrib3dARB".as_ptr()) as PFNGLVERTEXATTRIB3DARBPROC);
            self.glVertexAttrib3dvARB = load!((load)(userptr, c"glVertexAttrib3dvARB".as_ptr()) as PFNGLVERTEXATTRIB3DVARBPROC);
            self.glVertexAttrib3fARB = load!((load)(userptr, c"glVertexAttrib3fARB".as_ptr()) as PFNGLVERTEXATTRIB3FARBPROC);
            self.glVertexAttrib3fvARB = load!((load)(userptr, c"glVertexAttrib3fvARB".as_ptr()) as PFNGLVERTEXATTRIB3FVARBPROC);
            self.glVertexAttrib3sARB = load!((load)(userptr, c"glVertexAttrib3sARB".as_ptr()) as PFNGLVERTEXATTRIB3SARBPROC);
            self.glVertexAttrib3svARB = load!((load)(userptr, c"glVertexAttrib3svARB".as_ptr()) as PFNGLVERTEXATTRIB3SVARBPROC);
            self.glVertexAttrib4NbvARB = load!((load)(userptr, c"glVertexAttrib4NbvARB".as_ptr()) as PFNGLVERTEXATTRIB4NBVARBPROC);
            self.glVertexAttrib4NivARB = load!((load)(userptr, c"glVertexAttrib4NivARB".as_ptr()) as PFNGLVERTEXATTRIB4NIVARBPROC);
            self.glVertexAttrib4NsvARB = load!((load)(userptr, c"glVertexAttrib4NsvARB".as_ptr()) as PFNGLVERTEXATTRIB4NSVARBPROC);
            self.glVertexAttrib4NubARB = load!((load)(userptr, c"glVertexAttrib4NubARB".as_ptr()) as PFNGLVERTEXATTRIB4NUBARBPROC);
            self.glVertexAttrib4NubvARB = load!((load)(userptr, c"glVertexAttrib4NubvARB".as_ptr()) as PFNGLVERTEXATTRIB4NUBVARBPROC);
            self.glVertexAttrib4NuivARB = load!((load)(userptr, c"glVertexAttrib4NuivARB".as_ptr()) as PFNGLVERTEXATTRIB4NUIVARBPROC);
            self.glVertexAttrib4NusvARB = load!((load)(userptr, c"glVertexAttrib4NusvARB".as_ptr()) as PFNGLVERTEXATTRIB4NUSVARBPROC);
            self.glVertexAttrib4bvARB = load!((load)(userptr, c"glVertexAttrib4bvARB".as_ptr()) as PFNGLVERTEXATTRIB4BVARBPROC);
            self.glVertexAttrib4dARB = load!((load)(userptr, c"glVertexAttrib4dARB".as_ptr()) as PFNGLVERTEXATTRIB4DARBPROC);
            self.glVertexAttrib4dvARB = load!((load)(userptr, c"glVertexAttrib4dvARB".as_ptr()) as PFNGLVERTEXATTRIB4DVARBPROC);
            self.glVertexAttrib4fARB = load!((load)(userptr, c"glVertexAttrib4fARB".as_ptr()) as PFNGLVERTEXATTRIB4FARBPROC);
            self.glVertexAttrib4fvARB = load!((load)(userptr, c"glVertexAttrib4fvARB".as_ptr()) as PFNGLVERTEXATTRIB4FVARBPROC);
            self.glVertexAttrib4ivARB = load!((load)(userptr, c"glVertexAttrib4ivARB".as_ptr()) as PFNGLVERTEXATTRIB4IVARBPROC);
            self.glVertexAttrib4sARB = load!((load)(userptr, c"glVertexAttrib4sARB".as_ptr()) as PFNGLVERTEXATTRIB4SARBPROC);
            self.glVertexAttrib4svARB = load!((load)(userptr, c"glVertexAttrib4svARB".as_ptr()) as PFNGLVERTEXATTRIB4SVARBPROC);
            self.glVertexAttrib4ubvARB = load!((load)(userptr, c"glVertexAttrib4ubvARB".as_ptr()) as PFNGLVERTEXATTRIB4UBVARBPROC);
            self.glVertexAttrib4uivARB = load!((load)(userptr, c"glVertexAttrib4uivARB".as_ptr()) as PFNGLVERTEXATTRIB4UIVARBPROC);
            self.glVertexAttrib4usvARB = load!((load)(userptr, c"glVertexAttrib4usvARB".as_ptr()) as PFNGLVERTEXATTRIB4USVARBPROC);
            self.glVertexAttribPointerARB = load!((load)(userptr, c"glVertexAttribPointerARB".as_ptr()) as PFNGLVERTEXATTRIBPOINTERARBPROC);
        }
    }
    unsafe fn glad_gl_load_GL_EXT_draw_instanced(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_EXT_draw_instanced { return; }
            self.glDrawArraysInstancedEXT = load!((load)(userptr, c"glDrawArraysInstancedEXT".as_ptr()) as PFNGLDRAWARRAYSINSTANCEDEXTPROC);
            self.glDrawElementsInstancedEXT = load!((load)(userptr, c"glDrawElementsInstancedEXT".as_ptr()) as PFNGLDRAWELEMENTSINSTANCEDEXTPROC);
        }
    }
    unsafe fn glad_gl_load_GL_EXT_fog_coord(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_EXT_fog_coord { return; }
            self.glFogCoordPointerEXT = load!((load)(userptr, c"glFogCoordPointerEXT".as_ptr()) as PFNGLFOGCOORDPOINTEREXTPROC);
            self.glFogCoorddEXT = load!((load)(userptr, c"glFogCoorddEXT".as_ptr()) as PFNGLFOGCOORDDEXTPROC);
            self.glFogCoorddvEXT = load!((load)(userptr, c"glFogCoorddvEXT".as_ptr()) as PFNGLFOGCOORDDVEXTPROC);
            self.glFogCoordfEXT = load!((load)(userptr, c"glFogCoordfEXT".as_ptr()) as PFNGLFOGCOORDFEXTPROC);
            self.glFogCoordfvEXT = load!((load)(userptr, c"glFogCoordfvEXT".as_ptr()) as PFNGLFOGCOORDFVEXTPROC);
        }
    }
    unsafe fn glad_gl_load_GL_EXT_framebuffer_blit(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_EXT_framebuffer_blit { return; }
            self.glBlitFramebufferEXT = load!((load)(userptr, c"glBlitFramebufferEXT".as_ptr()) as PFNGLBLITFRAMEBUFFEREXTPROC);
        }
    }
    unsafe fn glad_gl_load_GL_EXT_framebuffer_multisample(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_EXT_framebuffer_multisample { return; }
            self.glRenderbufferStorageMultisampleEXT = load!((load)(userptr, c"glRenderbufferStorageMultisampleEXT".as_ptr()) as PFNGLRENDERBUFFERSTORAGEMULTISAMPLEEXTPROC);
        }
    }
    unsafe fn glad_gl_load_GL_EXT_framebuffer_object(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_EXT_framebuffer_object { return; }
            self.glBindFramebufferEXT = load!((load)(userptr, c"glBindFramebufferEXT".as_ptr()) as PFNGLBINDFRAMEBUFFEREXTPROC);
            self.glBindRenderbufferEXT = load!((load)(userptr, c"glBindRenderbufferEXT".as_ptr()) as PFNGLBINDRENDERBUFFEREXTPROC);
            self.glCheckFramebufferStatusEXT = load!((load)(userptr, c"glCheckFramebufferStatusEXT".as_ptr()) as PFNGLCHECKFRAMEBUFFERSTATUSEXTPROC);
            self.glDeleteFramebuffersEXT = load!((load)(userptr, c"glDeleteFramebuffersEXT".as_ptr()) as PFNGLDELETEFRAMEBUFFERSEXTPROC);
            self.glDeleteRenderbuffersEXT = load!((load)(userptr, c"glDeleteRenderbuffersEXT".as_ptr()) as PFNGLDELETERENDERBUFFERSEXTPROC);
            self.glFramebufferRenderbufferEXT = load!((load)(userptr, c"glFramebufferRenderbufferEXT".as_ptr()) as PFNGLFRAMEBUFFERRENDERBUFFEREXTPROC);
            self.glFramebufferTexture1DEXT = load!((load)(userptr, c"glFramebufferTexture1DEXT".as_ptr()) as PFNGLFRAMEBUFFERTEXTURE1DEXTPROC);
            self.glFramebufferTexture2DEXT = load!((load)(userptr, c"glFramebufferTexture2DEXT".as_ptr()) as PFNGLFRAMEBUFFERTEXTURE2DEXTPROC);
            self.glFramebufferTexture3DEXT = load!((load)(userptr, c"glFramebufferTexture3DEXT".as_ptr()) as PFNGLFRAMEBUFFERTEXTURE3DEXTPROC);
            self.glGenFramebuffersEXT = load!((load)(userptr, c"glGenFramebuffersEXT".as_ptr()) as PFNGLGENFRAMEBUFFERSEXTPROC);
            self.glGenRenderbuffersEXT = load!((load)(userptr, c"glGenRenderbuffersEXT".as_ptr()) as PFNGLGENRENDERBUFFERSEXTPROC);
            self.glGenerateMipmapEXT = load!((load)(userptr, c"glGenerateMipmapEXT".as_ptr()) as PFNGLGENERATEMIPMAPEXTPROC);
            self.glGetFramebufferAttachmentParameterivEXT = load!((load)(userptr, c"glGetFramebufferAttachmentParameterivEXT".as_ptr()) as PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVEXTPROC);
            self.glGetRenderbufferParameterivEXT = load!((load)(userptr, c"glGetRenderbufferParameterivEXT".as_ptr()) as PFNGLGETRENDERBUFFERPARAMETERIVEXTPROC);
            self.glIsFramebufferEXT = load!((load)(userptr, c"glIsFramebufferEXT".as_ptr()) as PFNGLISFRAMEBUFFEREXTPROC);
            self.glIsRenderbufferEXT = load!((load)(userptr, c"glIsRenderbufferEXT".as_ptr()) as PFNGLISRENDERBUFFEREXTPROC);
            self.glRenderbufferStorageEXT = load!((load)(userptr, c"glRenderbufferStorageEXT".as_ptr()) as PFNGLRENDERBUFFERSTORAGEEXTPROC);
        }
    }
    unsafe fn glad_gl_load_GL_OES_fixed_point(&mut self, compat: &GladCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_OES_fixed_point { return; }
            self.glAccumxOES = load!((load)(userptr, c"glAccumxOES".as_ptr()) as PFNGLACCUMXOESPROC);
            self.glAlphaFuncxOES = load!((load)(userptr, c"glAlphaFuncxOES".as_ptr()) as PFNGLALPHAFUNCXOESPROC);
            self.glBitmapxOES = load!((load)(userptr, c"glBitmapxOES".as_ptr()) as PFNGLBITMAPXOESPROC);
            self.glBlendColorxOES = load!((load)(userptr, c"glBlendColorxOES".as_ptr()) as PFNGLBLENDCOLORXOESPROC);
            self.glClearAccumxOES = load!((load)(userptr, c"glClearAccumxOES".as_ptr()) as PFNGLCLEARACCUMXOESPROC);
            self.glClearColorxOES = load!((load)(userptr, c"glClearColorxOES".as_ptr()) as PFNGLCLEARCOLORXOESPROC);
            self.glClearDepthxOES = load!((load)(userptr, c"glClearDepthxOES".as_ptr()) as PFNGLCLEARDEPTHXOESPROC);
            self.glClipPlanexOES = load!((load)(userptr, c"glClipPlanexOES".as_ptr()) as PFNGLCLIPPLANEXOESPROC);
            self.glColor3xOES = load!((load)(userptr, c"glColor3xOES".as_ptr()) as PFNGLCOLOR3XOESPROC);
            self.glColor3xvOES = load!((load)(userptr, c"glColor3xvOES".as_ptr()) as PFNGLCOLOR3XVOESPROC);
            self.glColor4xOES = load!((load)(userptr, c"glColor4xOES".as_ptr()) as PFNGLCOLOR4XOESPROC);
            self.glColor4xvOES = load!((load)(userptr, c"glColor4xvOES".as_ptr()) as PFNGLCOLOR4XVOESPROC);
            self.glConvolutionParameterxOES = load!((load)(userptr, c"glConvolutionParameterxOES".as_ptr()) as PFNGLCONVOLUTIONPARAMETERXOESPROC);
            self.glConvolutionParameterxvOES = load!((load)(userptr, c"glConvolutionParameterxvOES".as_ptr()) as PFNGLCONVOLUTIONPARAMETERXVOESPROC);
            self.glDepthRangexOES = load!((load)(userptr, c"glDepthRangexOES".as_ptr()) as PFNGLDEPTHRANGEXOESPROC);
            self.glEvalCoord1xOES = load!((load)(userptr, c"glEvalCoord1xOES".as_ptr()) as PFNGLEVALCOORD1XOESPROC);
            self.glEvalCoord1xvOES = load!((load)(userptr, c"glEvalCoord1xvOES".as_ptr()) as PFNGLEVALCOORD1XVOESPROC);
            self.glEvalCoord2xOES = load!((load)(userptr, c"glEvalCoord2xOES".as_ptr()) as PFNGLEVALCOORD2XOESPROC);
            self.glEvalCoord2xvOES = load!((load)(userptr, c"glEvalCoord2xvOES".as_ptr()) as PFNGLEVALCOORD2XVOESPROC);
            self.glFeedbackBufferxOES = load!((load)(userptr, c"glFeedbackBufferxOES".as_ptr()) as PFNGLFEEDBACKBUFFERXOESPROC);
            self.glFogxOES = load!((load)(userptr, c"glFogxOES".as_ptr()) as PFNGLFOGXOESPROC);
            self.glFogxvOES = load!((load)(userptr, c"glFogxvOES".as_ptr()) as PFNGLFOGXVOESPROC);
            self.glFrustumxOES = load!((load)(userptr, c"glFrustumxOES".as_ptr()) as PFNGLFRUSTUMXOESPROC);
            self.glGetClipPlanexOES = load!((load)(userptr, c"glGetClipPlanexOES".as_ptr()) as PFNGLGETCLIPPLANEXOESPROC);
            self.glGetConvolutionParameterxvOES = load!((load)(userptr, c"glGetConvolutionParameterxvOES".as_ptr()) as PFNGLGETCONVOLUTIONPARAMETERXVOESPROC);
            self.glGetFixedvOES = load!((load)(userptr, c"glGetFixedvOES".as_ptr()) as PFNGLGETFIXEDVOESPROC);
            self.glGetHistogramParameterxvOES = load!((load)(userptr, c"glGetHistogramParameterxvOES".as_ptr()) as PFNGLGETHISTOGRAMPARAMETERXVOESPROC);
            self.glGetLightxOES = load!((load)(userptr, c"glGetLightxOES".as_ptr()) as PFNGLGETLIGHTXOESPROC);
            self.glGetMapxvOES = load!((load)(userptr, c"glGetMapxvOES".as_ptr()) as PFNGLGETMAPXVOESPROC);
            self.glGetMaterialxOES = load!((load)(userptr, c"glGetMaterialxOES".as_ptr()) as PFNGLGETMATERIALXOESPROC);
            self.glGetPixelMapxv = load!((load)(userptr, c"glGetPixelMapxv".as_ptr()) as PFNGLGETPIXELMAPXVPROC);
            self.glGetTexEnvxvOES = load!((load)(userptr, c"glGetTexEnvxvOES".as_ptr()) as PFNGLGETTEXENVXVOESPROC);
            self.glGetTexGenxvOES = load!((load)(userptr, c"glGetTexGenxvOES".as_ptr()) as PFNGLGETTEXGENXVOESPROC);
            self.glGetTexLevelParameterxvOES = load!((load)(userptr, c"glGetTexLevelParameterxvOES".as_ptr()) as PFNGLGETTEXLEVELPARAMETERXVOESPROC);
            self.glGetTexParameterxvOES = load!((load)(userptr, c"glGetTexParameterxvOES".as_ptr()) as PFNGLGETTEXPARAMETERXVOESPROC);
            self.glIndexxOES = load!((load)(userptr, c"glIndexxOES".as_ptr()) as PFNGLINDEXXOESPROC);
            self.glIndexxvOES = load!((load)(userptr, c"glIndexxvOES".as_ptr()) as PFNGLINDEXXVOESPROC);
            self.glLightModelxOES = load!((load)(userptr, c"glLightModelxOES".as_ptr()) as PFNGLLIGHTMODELXOESPROC);
            self.glLightModelxvOES = load!((load)(userptr, c"glLightModelxvOES".as_ptr()) as PFNGLLIGHTMODELXVOESPROC);
            self.glLightxOES = load!((load)(userptr, c"glLightxOES".as_ptr()) as PFNGLLIGHTXOESPROC);
            self.glLightxvOES = load!((load)(userptr, c"glLightxvOES".as_ptr()) as PFNGLLIGHTXVOESPROC);
            self.glLineWidthxOES = load!((load)(userptr, c"glLineWidthxOES".as_ptr()) as PFNGLLINEWIDTHXOESPROC);
            self.glLoadMatrixxOES = load!((load)(userptr, c"glLoadMatrixxOES".as_ptr()) as PFNGLLOADMATRIXXOESPROC);
            self.glLoadTransposeMatrixxOES = load!((load)(userptr, c"glLoadTransposeMatrixxOES".as_ptr()) as PFNGLLOADTRANSPOSEMATRIXXOESPROC);
            self.glMap1xOES = load!((load)(userptr, c"glMap1xOES".as_ptr()) as PFNGLMAP1XOESPROC);
            self.glMap2xOES = load!((load)(userptr, c"glMap2xOES".as_ptr()) as PFNGLMAP2XOESPROC);
            self.glMapGrid1xOES = load!((load)(userptr, c"glMapGrid1xOES".as_ptr()) as PFNGLMAPGRID1XOESPROC);
            self.glMapGrid2xOES = load!((load)(userptr, c"glMapGrid2xOES".as_ptr()) as PFNGLMAPGRID2XOESPROC);
            self.glMaterialxOES = load!((load)(userptr, c"glMaterialxOES".as_ptr()) as PFNGLMATERIALXOESPROC);
            self.glMaterialxvOES = load!((load)(userptr, c"glMaterialxvOES".as_ptr()) as PFNGLMATERIALXVOESPROC);
            self.glMultMatrixxOES = load!((load)(userptr, c"glMultMatrixxOES".as_ptr()) as PFNGLMULTMATRIXXOESPROC);
            self.glMultTransposeMatrixxOES = load!((load)(userptr, c"glMultTransposeMatrixxOES".as_ptr()) as PFNGLMULTTRANSPOSEMATRIXXOESPROC);
            self.glMultiTexCoord1xOES = load!((load)(userptr, c"glMultiTexCoord1xOES".as_ptr()) as PFNGLMULTITEXCOORD1XOESPROC);
            self.glMultiTexCoord1xvOES = load!((load)(userptr, c"glMultiTexCoord1xvOES".as_ptr()) as PFNGLMULTITEXCOORD1XVOESPROC);
            self.glMultiTexCoord2xOES = load!((load)(userptr, c"glMultiTexCoord2xOES".as_ptr()) as PFNGLMULTITEXCOORD2XOESPROC);
            self.glMultiTexCoord2xvOES = load!((load)(userptr, c"glMultiTexCoord2xvOES".as_ptr()) as PFNGLMULTITEXCOORD2XVOESPROC);
            self.glMultiTexCoord3xOES = load!((load)(userptr, c"glMultiTexCoord3xOES".as_ptr()) as PFNGLMULTITEXCOORD3XOESPROC);
            self.glMultiTexCoord3xvOES = load!((load)(userptr, c"glMultiTexCoord3xvOES".as_ptr()) as PFNGLMULTITEXCOORD3XVOESPROC);
            self.glMultiTexCoord4xOES = load!((load)(userptr, c"glMultiTexCoord4xOES".as_ptr()) as PFNGLMULTITEXCOORD4XOESPROC);
            self.glMultiTexCoord4xvOES = load!((load)(userptr, c"glMultiTexCoord4xvOES".as_ptr()) as PFNGLMULTITEXCOORD4XVOESPROC);
            self.glNormal3xOES = load!((load)(userptr, c"glNormal3xOES".as_ptr()) as PFNGLNORMAL3XOESPROC);
            self.glNormal3xvOES = load!((load)(userptr, c"glNormal3xvOES".as_ptr()) as PFNGLNORMAL3XVOESPROC);
            self.glOrthoxOES = load!((load)(userptr, c"glOrthoxOES".as_ptr()) as PFNGLORTHOXOESPROC);
            self.glPassThroughxOES = load!((load)(userptr, c"glPassThroughxOES".as_ptr()) as PFNGLPASSTHROUGHXOESPROC);
            self.glPixelMapx = load!((load)(userptr, c"glPixelMapx".as_ptr()) as PFNGLPIXELMAPXPROC);
            self.glPixelStorex = load!((load)(userptr, c"glPixelStorex".as_ptr()) as PFNGLPIXELSTOREXPROC);
            self.glPixelTransferxOES = load!((load)(userptr, c"glPixelTransferxOES".as_ptr()) as PFNGLPIXELTRANSFERXOESPROC);
            self.glPixelZoomxOES = load!((load)(userptr, c"glPixelZoomxOES".as_ptr()) as PFNGLPIXELZOOMXOESPROC);
            self.glPointParameterxvOES = load!((load)(userptr, c"glPointParameterxvOES".as_ptr()) as PFNGLPOINTPARAMETERXVOESPROC);
            self.glPointSizexOES = load!((load)(userptr, c"glPointSizexOES".as_ptr()) as PFNGLPOINTSIZEXOESPROC);
            self.glPolygonOffsetxOES = load!((load)(userptr, c"glPolygonOffsetxOES".as_ptr()) as PFNGLPOLYGONOFFSETXOESPROC);
            self.glPrioritizeTexturesxOES = load!((load)(userptr, c"glPrioritizeTexturesxOES".as_ptr()) as PFNGLPRIORITIZETEXTURESXOESPROC);
            self.glRasterPos2xOES = load!((load)(userptr, c"glRasterPos2xOES".as_ptr()) as PFNGLRASTERPOS2XOESPROC);
            self.glRasterPos2xvOES = load!((load)(userptr, c"glRasterPos2xvOES".as_ptr()) as PFNGLRASTERPOS2XVOESPROC);
            self.glRasterPos3xOES = load!((load)(userptr, c"glRasterPos3xOES".as_ptr()) as PFNGLRASTERPOS3XOESPROC);
            self.glRasterPos3xvOES = load!((load)(userptr, c"glRasterPos3xvOES".as_ptr()) as PFNGLRASTERPOS3XVOESPROC);
            self.glRasterPos4xOES = load!((load)(userptr, c"glRasterPos4xOES".as_ptr()) as PFNGLRASTERPOS4XOESPROC);
            self.glRasterPos4xvOES = load!((load)(userptr, c"glRasterPos4xvOES".as_ptr()) as PFNGLRASTERPOS4XVOESPROC);
            self.glRectxOES = load!((load)(userptr, c"glRectxOES".as_ptr()) as PFNGLRECTXOESPROC);
            self.glRectxvOES = load!((load)(userptr, c"glRectxvOES".as_ptr()) as PFNGLRECTXVOESPROC);
            self.glRotatexOES = load!((load)(userptr, c"glRotatexOES".as_ptr()) as PFNGLROTATEXOESPROC);
            self.glScalexOES = load!((load)(userptr, c"glScalexOES".as_ptr()) as PFNGLSCALEXOESPROC);
            self.glTexCoord1xOES = load!((load)(userptr, c"glTexCoord1xOES".as_ptr()) as PFNGLTEXCOORD1XOESPROC);
            self.glTexCoord1xvOES = load!((load)(userptr, c"glTexCoord1xvOES".as_ptr()) as PFNGLTEXCOORD1XVOESPROC);
            self.glTexCoord2xOES = load!((load)(userptr, c"glTexCoord2xOES".as_ptr()) as PFNGLTEXCOORD2XOESPROC);
            self.glTexCoord2xvOES = load!((load)(userptr, c"glTexCoord2xvOES".as_ptr()) as PFNGLTEXCOORD2XVOESPROC);
            self.glTexCoord3xOES = load!((load)(userptr, c"glTexCoord3xOES".as_ptr()) as PFNGLTEXCOORD3XOESPROC);
            self.glTexCoord3xvOES = load!((load)(userptr, c"glTexCoord3xvOES".as_ptr()) as PFNGLTEXCOORD3XVOESPROC);
            self.glTexCoord4xOES = load!((load)(userptr, c"glTexCoord4xOES".as_ptr()) as PFNGLTEXCOORD4XOESPROC);
            self.glTexCoord4xvOES = load!((load)(userptr, c"glTexCoord4xvOES".as_ptr()) as PFNGLTEXCOORD4XVOESPROC);
            self.glTexEnvxOES = load!((load)(userptr, c"glTexEnvxOES".as_ptr()) as PFNGLTEXENVXOESPROC);
            self.glTexEnvxvOES = load!((load)(userptr, c"glTexEnvxvOES".as_ptr()) as PFNGLTEXENVXVOESPROC);
            self.glTexGenxOES = load!((load)(userptr, c"glTexGenxOES".as_ptr()) as PFNGLTEXGENXOESPROC);
            self.glTexGenxvOES = load!((load)(userptr, c"glTexGenxvOES".as_ptr()) as PFNGLTEXGENXVOESPROC);
            self.glTexParameterxOES = load!((load)(userptr, c"glTexParameterxOES".as_ptr()) as PFNGLTEXPARAMETERXOESPROC);
            self.glTexParameterxvOES = load!((load)(userptr, c"glTexParameterxvOES".as_ptr()) as PFNGLTEXPARAMETERXVOESPROC);
            self.glTranslatexOES = load!((load)(userptr, c"glTranslatexOES".as_ptr()) as PFNGLTRANSLATEXOESPROC);
            self.glVertex2xOES = load!((load)(userptr, c"glVertex2xOES".as_ptr()) as PFNGLVERTEX2XOESPROC);
            self.glVertex2xvOES = load!((load)(userptr, c"glVertex2xvOES".as_ptr()) as PFNGLVERTEX2XVOESPROC);
            self.glVertex3xOES = load!((load)(userptr, c"glVertex3xOES".as_ptr()) as PFNGLVERTEX3XOESPROC);
            self.glVertex3xvOES = load!((load)(userptr, c"glVertex3xvOES".as_ptr()) as PFNGLVERTEX3XVOESPROC);
            self.glVertex4xOES = load!((load)(userptr, c"glVertex4xOES".as_ptr()) as PFNGLVERTEX4XOESPROC);
            self.glVertex4xvOES = load!((load)(userptr, c"glVertex4xvOES".as_ptr()) as PFNGLVERTEX4XVOESPROC);
        }
    }

    unsafe fn gladLoadGLUserPtr(load: GLADuserptrloadfunc, userptr: *mut c_void) -> c_int {
        unsafe {
            let mut version: c_int;

            glad_glGetString = load(userptr, c"glGetString".as_ptr()).map(|f| std::mem::transmute::<unsafe extern "C" fn(), PFNGLGETSTRINGPROC>());
            if(glad_glGetString == NULL) return 0;
            if(glad_glGetString(GL_VERSION) == NULL) return 0;
            version = glad_gl_find_core_gl();

            glad_gl_load_GL_VERSION_1_0(load, userptr);
            glad_gl_load_GL_VERSION_1_1(load, userptr);
            glad_gl_load_GL_VERSION_1_2(load, userptr);
            glad_gl_load_GL_VERSION_1_3(load, userptr);
            glad_gl_load_GL_VERSION_1_4(load, userptr);
            glad_gl_load_GL_VERSION_1_5(load, userptr);
            glad_gl_load_GL_VERSION_2_0(load, userptr);
            glad_gl_load_GL_VERSION_2_1(load, userptr);
            glad_gl_load_GL_VERSION_3_0(load, userptr);
            glad_gl_load_GL_VERSION_3_1(load, userptr);
            glad_gl_load_GL_VERSION_3_2(load, userptr);
            glad_gl_load_GL_VERSION_3_3(load, userptr);
            glad_gl_load_GL_VERSION_4_0(load, userptr);
            glad_gl_load_GL_VERSION_4_1(load, userptr);
            glad_gl_load_GL_VERSION_4_2(load, userptr);
            glad_gl_load_GL_VERSION_4_3(load, userptr);

            if (!glad_gl_find_extensions_gl(version)) return 0;
            glad_gl_load_GL_ARB_ES2_compatibility(load, userptr);
            glad_gl_load_GL_ARB_ES3_1_compatibility(load, userptr);
            glad_gl_load_GL_ARB_ES3_2_compatibility(load, userptr);
            glad_gl_load_GL_ARB_blend_func_extended(load, userptr);
            glad_gl_load_GL_ARB_buffer_storage(load, userptr);
            glad_gl_load_GL_ARB_clear_buffer_object(load, userptr);
            glad_gl_load_GL_ARB_clear_texture(load, userptr);
            glad_gl_load_GL_ARB_color_buffer_float(load, userptr);
            glad_gl_load_GL_ARB_compute_shader(load, userptr);
            glad_gl_load_GL_ARB_compute_variable_group_size(load, userptr);
            glad_gl_load_GL_ARB_copy_buffer(load, userptr);
            glad_gl_load_GL_ARB_copy_image(load, userptr);
            glad_gl_load_GL_ARB_debug_output(load, userptr);
            glad_gl_load_GL_ARB_direct_state_access(load, userptr);
            glad_gl_load_GL_ARB_draw_buffers(load, userptr);
            glad_gl_load_GL_ARB_draw_buffers_blend(load, userptr);
            glad_gl_load_GL_ARB_draw_elements_base_vertex(load, userptr);
            glad_gl_load_GL_ARB_draw_indirect(load, userptr);
            glad_gl_load_GL_ARB_draw_instanced(load, userptr);
            glad_gl_load_GL_ARB_fragment_program(load, userptr);
            glad_gl_load_GL_ARB_framebuffer_no_attachments(load, userptr);
            glad_gl_load_GL_ARB_framebuffer_object(load, userptr);
            glad_gl_load_GL_ARB_geometry_shader4(load, userptr);
            glad_gl_load_GL_ARB_get_program_binary(load, userptr);
            glad_gl_load_GL_ARB_get_texture_sub_image(load, userptr);
            glad_gl_load_GL_ARB_gl_spirv(load, userptr);
            glad_gl_load_GL_ARB_gpu_shader_fp64(load, userptr);
            glad_gl_load_GL_ARB_gpu_shader_int64(load, userptr);
            glad_gl_load_GL_ARB_instanced_arrays(load, userptr);
            glad_gl_load_GL_ARB_internalformat_query(load, userptr);
            glad_gl_load_GL_ARB_internalformat_query2(load, userptr);
            glad_gl_load_GL_ARB_map_buffer_range(load, userptr);
            glad_gl_load_GL_ARB_multi_bind(load, userptr);
            glad_gl_load_GL_ARB_multi_draw_indirect(load, userptr);
            glad_gl_load_GL_ARB_multisample(load, userptr);
            glad_gl_load_GL_ARB_multitexture(load, userptr);
            glad_gl_load_GL_ARB_occlusion_query(load, userptr);
            glad_gl_load_GL_ARB_sample_locations(load, userptr);
            glad_gl_load_GL_ARB_sample_shading(load, userptr);
            glad_gl_load_GL_ARB_shader_atomic_counters(load, userptr);
            glad_gl_load_GL_ARB_shader_image_load_store(load, userptr);
            glad_gl_load_GL_ARB_shader_objects(load, userptr);
            glad_gl_load_GL_ARB_shader_storage_buffer_object(load, userptr);
            glad_gl_load_GL_ARB_shading_language_include(load, userptr);
            glad_gl_load_GL_ARB_tessellation_shader(load, userptr);
            glad_gl_load_GL_ARB_texture_compression(load, userptr);
            glad_gl_load_GL_ARB_texture_multisample(load, userptr);
            glad_gl_load_GL_ARB_texture_storage(load, userptr);
            glad_gl_load_GL_ARB_texture_view(load, userptr);
            glad_gl_load_GL_ARB_timer_query(load, userptr);
            glad_gl_load_GL_ARB_transpose_matrix(load, userptr);
            glad_gl_load_GL_ARB_uniform_buffer_object(load, userptr);
            glad_gl_load_GL_ARB_vertex_array_object(load, userptr);
            glad_gl_load_GL_ARB_vertex_attrib_binding(load, userptr);
            glad_gl_load_GL_ARB_vertex_buffer_object(load, userptr);
            glad_gl_load_GL_ARB_vertex_program(load, userptr);
            glad_gl_load_GL_ARB_vertex_shader(load, userptr);
            glad_gl_load_GL_EXT_draw_instanced(load, userptr);
            glad_gl_load_GL_EXT_fog_coord(load, userptr);
            glad_gl_load_GL_EXT_framebuffer_blit(load, userptr);
            glad_gl_load_GL_EXT_framebuffer_multisample(load, userptr);
            glad_gl_load_GL_EXT_framebuffer_object(load, userptr);
            glad_gl_load_GL_OES_fixed_point(load, userptr);

            return version;
        }
    }

    pub fn load() -> Self {
        let mut glad = Self::default();
        gladLoadGLUserPtr( glad_gl_get_proc_from_userptr, GLAD_GNUC_EXTENSION (void*) load)
        glad
    }
}
