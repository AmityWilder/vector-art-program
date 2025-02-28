use super::*;

pub const GL_ACTIVE_ATOMIC_COUNTER_BUFFERS:                              u16 = 0x92D9;
pub const GL_ACTIVE_ATTRIBUTES:                                          u16 = 0x8B89;
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH:                                u16 = 0x8B8A;
pub const GL_ACTIVE_PROGRAM:                                             u16 = 0x8259;
pub const GL_ACTIVE_RESOURCES:                                           u16 = 0x92F5;
pub const GL_ACTIVE_SUBROUTINES:                                         u16 = 0x8DE5;
pub const GL_ACTIVE_SUBROUTINE_MAX_LENGTH:                               u16 = 0x8E48;
pub const GL_ACTIVE_SUBROUTINE_UNIFORMS:                                 u16 = 0x8DE6;
pub const GL_ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS:                        u16 = 0x8E47;
pub const GL_ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH:                       u16 = 0x8E49;
pub const GL_ACTIVE_TEXTURE:                                             u16 = 0x84E0;
pub const GL_ACTIVE_TEXTURE_ARB:                                         u16 = 0x84E0;
pub const GL_ACTIVE_UNIFORMS:                                            u16 = 0x8B86;
pub const GL_ACTIVE_UNIFORM_BLOCKS:                                      u16 = 0x8A36;
pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH:                       u16 = 0x8A35;
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH:                                  u16 = 0x8B87;
pub const GL_ACTIVE_VARIABLES:                                           u16 = 0x9305;
pub const GL_ALIASED_LINE_WIDTH_RANGE:                                   u16 = 0x846E;
pub const GL_ALL_BARRIER_BITS:                                           u32 = 0xFFFFFFFF;
pub const GL_ALL_SHADER_BITS:                                            u32 = 0xFFFFFFFF;
pub const GL_ALPHA:                                                      u16 = 0x1906;
pub const GL_ALPHA16F_ARB:                                               u16 = 0x881C;
pub const GL_ALPHA32F_ARB:                                               u16 = 0x8816;
pub const GL_ALREADY_SIGNALED:                                           u16 = 0x911A;
pub const GL_ALWAYS:                                                     u16 = 0x0207;
pub const GL_AND:                                                        u16 = 0x1501;
pub const GL_AND_INVERTED:                                               u16 = 0x1504;
pub const GL_AND_REVERSE:                                                u16 = 0x1502;
pub const GL_ANY_SAMPLES_PASSED:                                         u16 = 0x8C2F;
pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE:                            u16 = 0x8D6A;
pub const GL_ARRAY_BUFFER:                                               u16 = 0x8892;
pub const GL_ARRAY_BUFFER_ARB:                                           u16 = 0x8892;
pub const GL_ARRAY_BUFFER_BINDING:                                       u16 = 0x8894;
pub const GL_ARRAY_BUFFER_BINDING_ARB:                                   u16 = 0x8894;
pub const GL_ARRAY_SIZE:                                                 u16 = 0x92FB;
pub const GL_ARRAY_STRIDE:                                               u16 = 0x92FE;
pub const GL_ATOMIC_COUNTER_BARRIER_BIT:                                 u32 = 0x00001000;
pub const GL_ATOMIC_COUNTER_BUFFER:                                      u16 = 0x92C0;
pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS:               u16 = 0x92C5;
pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES:        u16 = 0x92C6;
pub const GL_ATOMIC_COUNTER_BUFFER_BINDING:                              u16 = 0x92C1;
pub const GL_ATOMIC_COUNTER_BUFFER_DATA_SIZE:                            u16 = 0x92C4;
pub const GL_ATOMIC_COUNTER_BUFFER_INDEX:                                u16 = 0x9301;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER:         u16 = 0x90ED;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER:        u16 = 0x92CB;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER:        u16 = 0x92CA;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER:    u16 = 0x92C8;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: u16 = 0x92C9;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER:          u16 = 0x92C7;
pub const GL_ATOMIC_COUNTER_BUFFER_SIZE:                                 u16 = 0x92C3;
pub const GL_ATOMIC_COUNTER_BUFFER_START:                                u16 = 0x92C2;
pub const GL_ATTACHED_SHADERS:                                           u16 = 0x8B85;
pub const GL_AUTO_GENERATE_MIPMAP:                                       u16 = 0x8295;
pub const GL_BACK:                                                       u16 = 0x0405;
pub const GL_BACK_LEFT:                                                  u16 = 0x0402;
pub const GL_BACK_RIGHT:                                                 u16 = 0x0403;
pub const GL_BGR:                                                        u16 = 0x80E0;
pub const GL_BGRA:                                                       u16 = 0x80E1;
pub const GL_BGRA_INTEGER:                                               u16 = 0x8D9B;
pub const GL_BGR_INTEGER:                                                u16 = 0x8D9A;
pub const GL_BLEND:                                                      u16 = 0x0BE2;
pub const GL_BLEND_COLOR:                                                u16 = 0x8005;
pub const GL_BLEND_DST:                                                  u16 = 0x0BE0;
pub const GL_BLEND_DST_ALPHA:                                            u16 = 0x80CA;
pub const GL_BLEND_DST_RGB:                                              u16 = 0x80C8;
pub const GL_BLEND_EQUATION:                                             u16 = 0x8009;
pub const GL_BLEND_EQUATION_ALPHA:                                       u16 = 0x883D;
pub const GL_BLEND_EQUATION_RGB:                                         u16 = 0x8009;
pub const GL_BLEND_SRC:                                                  u16 = 0x0BE1;
pub const GL_BLEND_SRC_ALPHA:                                            u16 = 0x80CB;
pub const GL_BLEND_SRC_RGB:                                              u16 = 0x80C9;
pub const GL_BLOCK_INDEX:                                                u16 = 0x92FD;
pub const GL_BLUE:                                                       u16 = 0x1905;
pub const GL_BLUE_INTEGER:                                               u16 = 0x8D96;
pub const GL_BOOL:                                                       u16 = 0x8B56;
pub const GL_BOOL_ARB:                                                   u16 = 0x8B56;
pub const GL_BOOL_VEC2:                                                  u16 = 0x8B57;
pub const GL_BOOL_VEC2_ARB:                                              u16 = 0x8B57;
pub const GL_BOOL_VEC3:                                                  u16 = 0x8B58;
pub const GL_BOOL_VEC3_ARB:                                              u16 = 0x8B58;
pub const GL_BOOL_VEC4:                                                  u16 = 0x8B59;
pub const GL_BOOL_VEC4_ARB:                                              u16 = 0x8B59;
pub const GL_BUFFER:                                                     u16 = 0x82E0;
pub const GL_BUFFER_ACCESS:                                              u16 = 0x88BB;
pub const GL_BUFFER_ACCESS_ARB:                                          u16 = 0x88BB;
pub const GL_BUFFER_ACCESS_FLAGS:                                        u16 = 0x911F;
pub const GL_BUFFER_BINDING:                                             u16 = 0x9302;
pub const GL_BUFFER_DATA_SIZE:                                           u16 = 0x9303;
pub const GL_BUFFER_IMMUTABLE_STORAGE:                                   u16 = 0x821F;
pub const GL_BUFFER_MAPPED:                                              u16 = 0x88BC;
pub const GL_BUFFER_MAPPED_ARB:                                          u16 = 0x88BC;
pub const GL_BUFFER_MAP_LENGTH:                                          u16 = 0x9120;
pub const GL_BUFFER_MAP_OFFSET:                                          u16 = 0x9121;
pub const GL_BUFFER_MAP_POINTER:                                         u16 = 0x88BD;
pub const GL_BUFFER_MAP_POINTER_ARB:                                     u16 = 0x88BD;
pub const GL_BUFFER_SIZE:                                                u16 = 0x8764;
pub const GL_BUFFER_SIZE_ARB:                                            u16 = 0x8764;
pub const GL_BUFFER_STORAGE_FLAGS:                                       u16 = 0x8220;
pub const GL_BUFFER_UPDATE_BARRIER_BIT:                                  u32 = 0x00000200;
pub const GL_BUFFER_USAGE:                                               u16 = 0x8765;
pub const GL_BUFFER_USAGE_ARB:                                           u16 = 0x8765;
pub const GL_BUFFER_VARIABLE:                                            u16 = 0x92E5;
pub const GL_BYTE:                                                       u16 = 0x1400;
pub const GL_CAVEAT_SUPPORT:                                             u16 = 0x82B8;
pub const GL_CCW:                                                        u16 = 0x0901;
pub const GL_CLAMP_FRAGMENT_COLOR_ARB:                                   u16 = 0x891B;
pub const GL_CLAMP_READ_COLOR:                                           u16 = 0x891C;
pub const GL_CLAMP_READ_COLOR_ARB:                                       u16 = 0x891C;
pub const GL_CLAMP_TO_BORDER:                                            u16 = 0x812D;
pub const GL_CLAMP_TO_BORDER_ARB:                                        u16 = 0x812D;
pub const GL_CLAMP_TO_EDGE:                                              u16 = 0x812F;
pub const GL_CLAMP_VERTEX_COLOR_ARB:                                     u16 = 0x891A;
pub const GL_CLEAR:                                                      u16 = 0x1500;
pub const GL_CLEAR_BUFFER:                                               u16 = 0x82B4;
pub const GL_CLEAR_TEXTURE:                                              u16 = 0x9365;
pub const GL_CLIENT_ACTIVE_TEXTURE_ARB:                                  u16 = 0x84E1;
pub const GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT:                           u32 = 0x00004000;
pub const GL_CLIENT_STORAGE_BIT:                                         u16 = 0x0200;
pub const GL_CLIPPING_INPUT_PRIMITIVES:                                  u16 = 0x82F6;
pub const GL_CLIPPING_INPUT_PRIMITIVES_ARB:                              u16 = 0x82F6;
pub const GL_CLIPPING_OUTPUT_PRIMITIVES:                                 u16 = 0x82F7;
pub const GL_CLIPPING_OUTPUT_PRIMITIVES_ARB:                             u16 = 0x82F7;
pub const GL_CLIP_DISTANCE0:                                             u16 = 0x3000;
pub const GL_CLIP_DISTANCE1:                                             u16 = 0x3001;
pub const GL_CLIP_DISTANCE2:                                             u16 = 0x3002;
pub const GL_CLIP_DISTANCE3:                                             u16 = 0x3003;
pub const GL_CLIP_DISTANCE4:                                             u16 = 0x3004;
pub const GL_CLIP_DISTANCE5:                                             u16 = 0x3005;
pub const GL_CLIP_DISTANCE6:                                             u16 = 0x3006;
pub const GL_CLIP_DISTANCE7:                                             u16 = 0x3007;
pub const GL_COLOR:                                                      u16 = 0x1800;
pub const GL_COLOR_ARRAY_BUFFER_BINDING_ARB:                             u16 = 0x8898;
pub const GL_COLOR_ATTACHMENT0:                                          u16 = 0x8CE0;
pub const GL_COLOR_ATTACHMENT0_EXT:                                      u16 = 0x8CE0;
pub const GL_COLOR_ATTACHMENT1:                                          u16 = 0x8CE1;
pub const GL_COLOR_ATTACHMENT10:                                         u16 = 0x8CEA;
pub const GL_COLOR_ATTACHMENT10_EXT:                                     u16 = 0x8CEA;
pub const GL_COLOR_ATTACHMENT11:                                         u16 = 0x8CEB;
pub const GL_COLOR_ATTACHMENT11_EXT:                                     u16 = 0x8CEB;
pub const GL_COLOR_ATTACHMENT12:                                         u16 = 0x8CEC;
pub const GL_COLOR_ATTACHMENT12_EXT:                                     u16 = 0x8CEC;
pub const GL_COLOR_ATTACHMENT13:                                         u16 = 0x8CED;
pub const GL_COLOR_ATTACHMENT13_EXT:                                     u16 = 0x8CED;
pub const GL_COLOR_ATTACHMENT14:                                         u16 = 0x8CEE;
pub const GL_COLOR_ATTACHMENT14_EXT:                                     u16 = 0x8CEE;
pub const GL_COLOR_ATTACHMENT15:                                         u16 = 0x8CEF;
pub const GL_COLOR_ATTACHMENT15_EXT:                                     u16 = 0x8CEF;
pub const GL_COLOR_ATTACHMENT16:                                         u16 = 0x8CF0;
pub const GL_COLOR_ATTACHMENT17:                                         u16 = 0x8CF1;
pub const GL_COLOR_ATTACHMENT18:                                         u16 = 0x8CF2;
pub const GL_COLOR_ATTACHMENT19:                                         u16 = 0x8CF3;
pub const GL_COLOR_ATTACHMENT1_EXT:                                      u16 = 0x8CE1;
pub const GL_COLOR_ATTACHMENT2:                                          u16 = 0x8CE2;
pub const GL_COLOR_ATTACHMENT20:                                         u16 = 0x8CF4;
pub const GL_COLOR_ATTACHMENT21:                                         u16 = 0x8CF5;
pub const GL_COLOR_ATTACHMENT22:                                         u16 = 0x8CF6;
pub const GL_COLOR_ATTACHMENT23:                                         u16 = 0x8CF7;
pub const GL_COLOR_ATTACHMENT24:                                         u16 = 0x8CF8;
pub const GL_COLOR_ATTACHMENT25:                                         u16 = 0x8CF9;
pub const GL_COLOR_ATTACHMENT26:                                         u16 = 0x8CFA;
pub const GL_COLOR_ATTACHMENT27:                                         u16 = 0x8CFB;
pub const GL_COLOR_ATTACHMENT28:                                         u16 = 0x8CFC;
pub const GL_COLOR_ATTACHMENT29:                                         u16 = 0x8CFD;
pub const GL_COLOR_ATTACHMENT2_EXT:                                      u16 = 0x8CE2;
pub const GL_COLOR_ATTACHMENT3:                                          u16 = 0x8CE3;
pub const GL_COLOR_ATTACHMENT30:                                         u16 = 0x8CFE;
pub const GL_COLOR_ATTACHMENT31:                                         u16 = 0x8CFF;
pub const GL_COLOR_ATTACHMENT3_EXT:                                      u16 = 0x8CE3;
pub const GL_COLOR_ATTACHMENT4:                                          u16 = 0x8CE4;
pub const GL_COLOR_ATTACHMENT4_EXT:                                      u16 = 0x8CE4;
pub const GL_COLOR_ATTACHMENT5:                                          u16 = 0x8CE5;
pub const GL_COLOR_ATTACHMENT5_EXT:                                      u16 = 0x8CE5;
pub const GL_COLOR_ATTACHMENT6:                                          u16 = 0x8CE6;
pub const GL_COLOR_ATTACHMENT6_EXT:                                      u16 = 0x8CE6;
pub const GL_COLOR_ATTACHMENT7:                                          u16 = 0x8CE7;
pub const GL_COLOR_ATTACHMENT7_EXT:                                      u16 = 0x8CE7;
pub const GL_COLOR_ATTACHMENT8:                                          u16 = 0x8CE8;
pub const GL_COLOR_ATTACHMENT8_EXT:                                      u16 = 0x8CE8;
pub const GL_COLOR_ATTACHMENT9:                                          u16 = 0x8CE9;
pub const GL_COLOR_ATTACHMENT9_EXT:                                      u16 = 0x8CE9;
pub const GL_COLOR_BUFFER_BIT:                                           u32 = 0x00004000;
pub const GL_COLOR_CLEAR_VALUE:                                          u16 = 0x0C22;
pub const GL_COLOR_COMPONENTS:                                           u16 = 0x8283;
pub const GL_COLOR_ENCODING:                                             u16 = 0x8296;
pub const GL_COLOR_LOGIC_OP:                                             u16 = 0x0BF2;
pub const GL_COLOR_RENDERABLE:                                           u16 = 0x8286;
pub const GL_COLOR_SUM_ARB:                                              u16 = 0x8458;
pub const GL_COLOR_WRITEMASK:                                            u16 = 0x0C23;
pub const GL_COMMAND_BARRIER_BIT:                                        u32 = 0x00000040;
pub const GL_COMPARE_REF_TO_TEXTURE:                                     u16 = 0x884E;
pub const GL_COMPATIBLE_SUBROUTINES:                                     u16 = 0x8E4B;
pub const GL_COMPILE_STATUS:                                             u16 = 0x8B81;
pub const GL_COMPRESSED_ALPHA_ARB:                                       u16 = 0x84E9;
pub const GL_COMPRESSED_INTENSITY_ARB:                                   u16 = 0x84EC;
pub const GL_COMPRESSED_LUMINANCE_ALPHA_ARB:                             u16 = 0x84EB;
pub const GL_COMPRESSED_LUMINANCE_ARB:                                   u16 = 0x84EA;
pub const GL_COMPRESSED_R11_EAC:                                         u16 = 0x9270;
pub const GL_COMPRESSED_RED:                                             u16 = 0x8225;
pub const GL_COMPRESSED_RED_RGTC1:                                       u16 = 0x8DBB;
pub const GL_COMPRESSED_RG:                                              u16 = 0x8226;
pub const GL_COMPRESSED_RG11_EAC:                                        u16 = 0x9272;
pub const GL_COMPRESSED_RGB:                                             u16 = 0x84ED;
pub const GL_COMPRESSED_RGB8_ETC2:                                       u16 = 0x9274;
pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2:                   u16 = 0x9276;
pub const GL_COMPRESSED_RGBA:                                            u16 = 0x84EE;
pub const GL_COMPRESSED_RGBA8_ETC2_EAC:                                  u16 = 0x9278;
pub const GL_COMPRESSED_RGBA_ARB:                                        u16 = 0x84EE;
pub const GL_COMPRESSED_RGBA_ASTC_10x10_KHR:                             u16 = 0x93BB;
pub const GL_COMPRESSED_RGBA_ASTC_10x5_KHR:                              u16 = 0x93B8;
pub const GL_COMPRESSED_RGBA_ASTC_10x6_KHR:                              u16 = 0x93B9;
pub const GL_COMPRESSED_RGBA_ASTC_10x8_KHR:                              u16 = 0x93BA;
pub const GL_COMPRESSED_RGBA_ASTC_12x10_KHR:                             u16 = 0x93BC;
pub const GL_COMPRESSED_RGBA_ASTC_12x12_KHR:                             u16 = 0x93BD;
pub const GL_COMPRESSED_RGBA_ASTC_4x4_KHR:                               u16 = 0x93B0;
pub const GL_COMPRESSED_RGBA_ASTC_5x4_KHR:                               u16 = 0x93B1;
pub const GL_COMPRESSED_RGBA_ASTC_5x5_KHR:                               u16 = 0x93B2;
pub const GL_COMPRESSED_RGBA_ASTC_6x5_KHR:                               u16 = 0x93B3;
pub const GL_COMPRESSED_RGBA_ASTC_6x6_KHR:                               u16 = 0x93B4;
pub const GL_COMPRESSED_RGBA_ASTC_8x5_KHR:                               u16 = 0x93B5;
pub const GL_COMPRESSED_RGBA_ASTC_8x6_KHR:                               u16 = 0x93B6;
pub const GL_COMPRESSED_RGBA_ASTC_8x8_KHR:                               u16 = 0x93B7;
pub const GL_COMPRESSED_RGBA_BPTC_UNORM:                                 u16 = 0x8E8C;
pub const GL_COMPRESSED_RGBA_S3TC_DXT1_EXT:                              u16 = 0x83F1;
pub const GL_COMPRESSED_RGBA_S3TC_DXT3_EXT:                              u16 = 0x83F2;
pub const GL_COMPRESSED_RGBA_S3TC_DXT5_EXT:                              u16 = 0x83F3;
pub const GL_COMPRESSED_RGB_ARB:                                         u16 = 0x84ED;
pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT:                           u16 = 0x8E8E;
pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT:                         u16 = 0x8E8F;
pub const GL_COMPRESSED_RGB_S3TC_DXT1_EXT:                               u16 = 0x83F0;
pub const GL_COMPRESSED_RG_RGTC2:                                        u16 = 0x8DBD;
pub const GL_COMPRESSED_SIGNED_R11_EAC:                                  u16 = 0x9271;
pub const GL_COMPRESSED_SIGNED_RED_RGTC1:                                u16 = 0x8DBC;
pub const GL_COMPRESSED_SIGNED_RG11_EAC:                                 u16 = 0x9273;
pub const GL_COMPRESSED_SIGNED_RG_RGTC2:                                 u16 = 0x8DBE;
pub const GL_COMPRESSED_SRGB:                                            u16 = 0x8C48;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR:                     u16 = 0x93DB;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR:                      u16 = 0x93D8;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR:                      u16 = 0x93D9;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR:                      u16 = 0x93DA;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR:                     u16 = 0x93DC;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR:                     u16 = 0x93DD;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR:                       u16 = 0x93D0;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR:                       u16 = 0x93D1;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR:                       u16 = 0x93D2;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR:                       u16 = 0x93D3;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR:                       u16 = 0x93D4;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR:                       u16 = 0x93D5;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR:                       u16 = 0x93D6;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR:                       u16 = 0x93D7;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC:                           u16 = 0x9279;
pub const GL_COMPRESSED_SRGB8_ETC2:                                      u16 = 0x9275;
pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2:                  u16 = 0x9277;
pub const GL_COMPRESSED_SRGB_ALPHA:                                      u16 = 0x8C49;
pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM:                           u16 = 0x8E8D;
pub const GL_COMPRESSED_TEXTURE_FORMATS:                                 u16 = 0x86A3;
pub const GL_COMPRESSED_TEXTURE_FORMATS_ARB:                             u16 = 0x86A3;
pub const GL_COMPUTE_SHADER:                                             u16 = 0x91B9;
pub const GL_COMPUTE_SHADER_BIT:                                         u32 = 0x00000020;
pub const GL_COMPUTE_SHADER_INVOCATIONS:                                 u16 = 0x82F5;
pub const GL_COMPUTE_SHADER_INVOCATIONS_ARB:                             u16 = 0x82F5;
pub const GL_COMPUTE_SUBROUTINE:                                         u16 = 0x92ED;
pub const GL_COMPUTE_SUBROUTINE_UNIFORM:                                 u16 = 0x92F3;
pub const GL_COMPUTE_TEXTURE:                                            u16 = 0x82A0;
pub const GL_COMPUTE_WORK_GROUP_SIZE:                                    u16 = 0x8267;
pub const GL_CONDITION_SATISFIED:                                        u16 = 0x911C;
pub const GL_CONSTANT_ALPHA:                                             u16 = 0x8003;
pub const GL_CONSTANT_COLOR:                                             u16 = 0x8001;
pub const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT:                          u32 = 0x00000002;
pub const GL_CONTEXT_CORE_PROFILE_BIT:                                   u32 = 0x00000001;
pub const GL_CONTEXT_FLAGS:                                              u16 = 0x821E;
pub const GL_CONTEXT_FLAG_DEBUG_BIT:                                     u32 = 0x00000002;
pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT:                        u32 = 0x00000001;
pub const GL_CONTEXT_PROFILE_MASK:                                       u16 = 0x9126;
pub const GL_COPY:                                                       u16 = 0x1503;
pub const GL_COPY_INVERTED:                                              u16 = 0x150C;
pub const GL_COPY_READ_BUFFER:                                           u16 = 0x8F36;
pub const GL_COPY_READ_BUFFER_BINDING:                                   u16 = 0x8F36;
pub const GL_COPY_WRITE_BUFFER:                                          u16 = 0x8F37;
pub const GL_COPY_WRITE_BUFFER_BINDING:                                  u16 = 0x8F37;
pub const GL_CULL_FACE:                                                  u16 = 0x0B44;
pub const GL_CULL_FACE_MODE:                                             u16 = 0x0B45;
pub const GL_CURRENT_FOG_COORDINATE_EXT:                                 u16 = 0x8453;
pub const GL_CURRENT_MATRIX_ARB:                                         u16 = 0x8641;
pub const GL_CURRENT_MATRIX_STACK_DEPTH_ARB:                             u16 = 0x8640;
pub const GL_CURRENT_PROGRAM:                                            u16 = 0x8B8D;
pub const GL_CURRENT_QUERY:                                              u16 = 0x8865;
pub const GL_CURRENT_QUERY_ARB:                                          u16 = 0x8865;
pub const GL_CURRENT_VERTEX_ATTRIB:                                      u16 = 0x8626;
pub const GL_CURRENT_VERTEX_ATTRIB_ARB:                                  u16 = 0x8626;
pub const GL_CW:                                                         u16 = 0x0900;
pub const GL_DEBUG_CALLBACK_FUNCTION:                                    u16 = 0x8244;
pub const GL_DEBUG_CALLBACK_FUNCTION_ARB:                                u16 = 0x8244;
pub const GL_DEBUG_CALLBACK_USER_PARAM:                                  u16 = 0x8245;
pub const GL_DEBUG_CALLBACK_USER_PARAM_ARB:                              u16 = 0x8245;
pub const GL_DEBUG_GROUP_STACK_DEPTH:                                    u16 = 0x826D;
pub const GL_DEBUG_LOGGED_MESSAGES:                                      u16 = 0x9145;
pub const GL_DEBUG_LOGGED_MESSAGES_ARB:                                  u16 = 0x9145;
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH:                           u16 = 0x8243;
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_ARB:                       u16 = 0x8243;
pub const GL_DEBUG_OUTPUT:                                               u16 = 0x92E0;
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS:                                   u16 = 0x8242;
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS_ARB:                               u16 = 0x8242;
pub const GL_DEBUG_SEVERITY_HIGH:                                        u16 = 0x9146;
pub const GL_DEBUG_SEVERITY_HIGH_ARB:                                    u16 = 0x9146;
pub const GL_DEBUG_SEVERITY_LOW:                                         u16 = 0x9148;
pub const GL_DEBUG_SEVERITY_LOW_ARB:                                     u16 = 0x9148;
pub const GL_DEBUG_SEVERITY_MEDIUM:                                      u16 = 0x9147;
pub const GL_DEBUG_SEVERITY_MEDIUM_ARB:                                  u16 = 0x9147;
pub const GL_DEBUG_SEVERITY_NOTIFICATION:                                u16 = 0x826B;
pub const GL_DEBUG_SOURCE_API:                                           u16 = 0x8246;
pub const GL_DEBUG_SOURCE_API_ARB:                                       u16 = 0x8246;
pub const GL_DEBUG_SOURCE_APPLICATION:                                   u16 = 0x824A;
pub const GL_DEBUG_SOURCE_APPLICATION_ARB:                               u16 = 0x824A;
pub const GL_DEBUG_SOURCE_OTHER:                                         u16 = 0x824B;
pub const GL_DEBUG_SOURCE_OTHER_ARB:                                     u16 = 0x824B;
pub const GL_DEBUG_SOURCE_SHADER_COMPILER:                               u16 = 0x8248;
pub const GL_DEBUG_SOURCE_SHADER_COMPILER_ARB:                           u16 = 0x8248;
pub const GL_DEBUG_SOURCE_THIRD_PARTY:                                   u16 = 0x8249;
pub const GL_DEBUG_SOURCE_THIRD_PARTY_ARB:                               u16 = 0x8249;
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM:                                 u16 = 0x8247;
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM_ARB:                             u16 = 0x8247;
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR:                             u16 = 0x824D;
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_ARB:                         u16 = 0x824D;
pub const GL_DEBUG_TYPE_ERROR:                                           u16 = 0x824C;
pub const GL_DEBUG_TYPE_ERROR_ARB:                                       u16 = 0x824C;
pub const GL_DEBUG_TYPE_MARKER:                                          u16 = 0x8268;
pub const GL_DEBUG_TYPE_OTHER:                                           u16 = 0x8251;
pub const GL_DEBUG_TYPE_OTHER_ARB:                                       u16 = 0x8251;
pub const GL_DEBUG_TYPE_PERFORMANCE:                                     u16 = 0x8250;
pub const GL_DEBUG_TYPE_PERFORMANCE_ARB:                                 u16 = 0x8250;
pub const GL_DEBUG_TYPE_POP_GROUP:                                       u16 = 0x826A;
pub const GL_DEBUG_TYPE_PORTABILITY:                                     u16 = 0x824F;
pub const GL_DEBUG_TYPE_PORTABILITY_ARB:                                 u16 = 0x824F;
pub const GL_DEBUG_TYPE_PUSH_GROUP:                                      u16 = 0x8269;
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR:                              u16 = 0x824E;
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_ARB:                          u16 = 0x824E;
pub const GL_DECR:                                                       u16 = 0x1E03;
pub const GL_DECR_WRAP:                                                  u16 = 0x8508;
pub const GL_DELETE_STATUS:                                              u16 = 0x8B80;
pub const GL_DEPTH:                                                      u16 = 0x1801;
pub const GL_DEPTH24_STENCIL8:                                           u16 = 0x88F0;
pub const GL_DEPTH32F_STENCIL8:                                          u16 = 0x8CAD;
pub const GL_DEPTH_ATTACHMENT:                                           u16 = 0x8D00;
pub const GL_DEPTH_ATTACHMENT_EXT:                                       u16 = 0x8D00;
pub const GL_DEPTH_BUFFER_BIT:                                           u32 = 0x00000100;
pub const GL_DEPTH_CLAMP:                                                u16 = 0x864F;
pub const GL_DEPTH_CLEAR_VALUE:                                          u16 = 0x0B73;
pub const GL_DEPTH_COMPONENT:                                            u16 = 0x1902;
pub const GL_DEPTH_COMPONENT16:                                          u16 = 0x81A5;
pub const GL_DEPTH_COMPONENT16_ARB:                                      u16 = 0x81A5;
pub const GL_DEPTH_COMPONENT24:                                          u16 = 0x81A6;
pub const GL_DEPTH_COMPONENT24_ARB:                                      u16 = 0x81A6;
pub const GL_DEPTH_COMPONENT32:                                          u16 = 0x81A7;
pub const GL_DEPTH_COMPONENT32F:                                         u16 = 0x8CAC;
pub const GL_DEPTH_COMPONENT32_ARB:                                      u16 = 0x81A7;
pub const GL_DEPTH_COMPONENTS:                                           u16 = 0x8284;
pub const GL_DEPTH_FUNC:                                                 u16 = 0x0B74;
pub const GL_DEPTH_RANGE:                                                u16 = 0x0B70;
pub const GL_DEPTH_RENDERABLE:                                           u16 = 0x8287;
pub const GL_DEPTH_STENCIL:                                              u16 = 0x84F9;
pub const GL_DEPTH_STENCIL_ATTACHMENT:                                   u16 = 0x821A;
pub const GL_DEPTH_STENCIL_TEXTURE_MODE:                                 u16 = 0x90EA;
pub const GL_DEPTH_TEST:                                                 u16 = 0x0B71;
pub const GL_DEPTH_TEXTURE_MODE_ARB:                                     u16 = 0x884B;
pub const GL_DEPTH_WRITEMASK:                                            u16 = 0x0B72;
pub const GL_DISPATCH_INDIRECT_BUFFER:                                   u16 = 0x90EE;
pub const GL_DISPATCH_INDIRECT_BUFFER_BINDING:                           u16 = 0x90EF;
pub const GL_DITHER:                                                     u16 = 0x0BD0;
pub const GL_DONT_CARE:                                                  u16 = 0x1100;
pub const GL_DOUBLE:                                                     u16 = 0x140A;
pub const GL_DOUBLEBUFFER:                                               u16 = 0x0C32;
pub const GL_DOUBLE_MAT2:                                                u16 = 0x8F46;
pub const GL_DOUBLE_MAT2x3:                                              u16 = 0x8F49;
pub const GL_DOUBLE_MAT2x4:                                              u16 = 0x8F4A;
pub const GL_DOUBLE_MAT3:                                                u16 = 0x8F47;
pub const GL_DOUBLE_MAT3x2:                                              u16 = 0x8F4B;
pub const GL_DOUBLE_MAT3x4:                                              u16 = 0x8F4C;
pub const GL_DOUBLE_MAT4:                                                u16 = 0x8F48;
pub const GL_DOUBLE_MAT4x2:                                              u16 = 0x8F4D;
pub const GL_DOUBLE_MAT4x3:                                              u16 = 0x8F4E;
pub const GL_DOUBLE_VEC2:                                                u16 = 0x8FFC;
pub const GL_DOUBLE_VEC3:                                                u16 = 0x8FFD;
pub const GL_DOUBLE_VEC4:                                                u16 = 0x8FFE;
pub const GL_DRAW_BUFFER:                                                u16 = 0x0C01;
pub const GL_DRAW_BUFFER0:                                               u16 = 0x8825;
pub const GL_DRAW_BUFFER0_ARB:                                           u16 = 0x8825;
pub const GL_DRAW_BUFFER1:                                               u16 = 0x8826;
pub const GL_DRAW_BUFFER10:                                              u16 = 0x882F;
pub const GL_DRAW_BUFFER10_ARB:                                          u16 = 0x882F;
pub const GL_DRAW_BUFFER11:                                              u16 = 0x8830;
pub const GL_DRAW_BUFFER11_ARB:                                          u16 = 0x8830;
pub const GL_DRAW_BUFFER12:                                              u16 = 0x8831;
pub const GL_DRAW_BUFFER12_ARB:                                          u16 = 0x8831;
pub const GL_DRAW_BUFFER13:                                              u16 = 0x8832;
pub const GL_DRAW_BUFFER13_ARB:                                          u16 = 0x8832;
pub const GL_DRAW_BUFFER14:                                              u16 = 0x8833;
pub const GL_DRAW_BUFFER14_ARB:                                          u16 = 0x8833;
pub const GL_DRAW_BUFFER15:                                              u16 = 0x8834;
pub const GL_DRAW_BUFFER15_ARB:                                          u16 = 0x8834;
pub const GL_DRAW_BUFFER1_ARB:                                           u16 = 0x8826;
pub const GL_DRAW_BUFFER2:                                               u16 = 0x8827;
pub const GL_DRAW_BUFFER2_ARB:                                           u16 = 0x8827;
pub const GL_DRAW_BUFFER3:                                               u16 = 0x8828;
pub const GL_DRAW_BUFFER3_ARB:                                           u16 = 0x8828;
pub const GL_DRAW_BUFFER4:                                               u16 = 0x8829;
pub const GL_DRAW_BUFFER4_ARB:                                           u16 = 0x8829;
pub const GL_DRAW_BUFFER5:                                               u16 = 0x882A;
pub const GL_DRAW_BUFFER5_ARB:                                           u16 = 0x882A;
pub const GL_DRAW_BUFFER6:                                               u16 = 0x882B;
pub const GL_DRAW_BUFFER6_ARB:                                           u16 = 0x882B;
pub const GL_DRAW_BUFFER7:                                               u16 = 0x882C;
pub const GL_DRAW_BUFFER7_ARB:                                           u16 = 0x882C;
pub const GL_DRAW_BUFFER8:                                               u16 = 0x882D;
pub const GL_DRAW_BUFFER8_ARB:                                           u16 = 0x882D;
pub const GL_DRAW_BUFFER9:                                               u16 = 0x882E;
pub const GL_DRAW_BUFFER9_ARB:                                           u16 = 0x882E;
pub const GL_DRAW_FRAMEBUFFER:                                           u16 = 0x8CA9;
pub const GL_DRAW_FRAMEBUFFER_BINDING:                                   u16 = 0x8CA6;
pub const GL_DRAW_FRAMEBUFFER_BINDING_EXT:                               u16 = 0x8CA6;
pub const GL_DRAW_FRAMEBUFFER_EXT:                                       u16 = 0x8CA9;
pub const GL_DRAW_INDIRECT_BUFFER:                                       u16 = 0x8F3F;
pub const GL_DRAW_INDIRECT_BUFFER_BINDING:                               u16 = 0x8F43;
pub const GL_DST_ALPHA:                                                  u16 = 0x0304;
pub const GL_DST_COLOR:                                                  u16 = 0x0306;
pub const GL_DYNAMIC_COPY:                                               u16 = 0x88EA;
pub const GL_DYNAMIC_COPY_ARB:                                           u16 = 0x88EA;
pub const GL_DYNAMIC_DRAW:                                               u16 = 0x88E8;
pub const GL_DYNAMIC_DRAW_ARB:                                           u16 = 0x88E8;
pub const GL_DYNAMIC_READ:                                               u16 = 0x88E9;
pub const GL_DYNAMIC_READ_ARB:                                           u16 = 0x88E9;
pub const GL_DYNAMIC_STORAGE_BIT:                                        u16 = 0x0100;
pub const GL_EDGE_FLAG_ARRAY_BUFFER_BINDING_ARB:                         u16 = 0x889B;
pub const GL_ELEMENT_ARRAY_BARRIER_BIT:                                  u32 = 0x00000002;
pub const GL_ELEMENT_ARRAY_BUFFER:                                       u16 = 0x8893;
pub const GL_ELEMENT_ARRAY_BUFFER_ARB:                                   u16 = 0x8893;
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING:                               u16 = 0x8895;
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING_ARB:                           u16 = 0x8895;
pub const GL_EQUAL:                                                      u16 = 0x0202;
pub const GL_EQUIV:                                                      u16 = 0x1509;
pub const GL_EXTENSIONS:                                                 u16 = 0x1F03;
pub const GL_FALSE:                                                      u8  = 0;
pub const GL_FASTEST:                                                    u16 = 0x1101;
pub const GL_FILL:                                                       u16 = 0x1B02;
pub const GL_FILTER:                                                     u16 = 0x829A;
pub const GL_FIRST_VERTEX_CONVENTION:                                    u16 = 0x8E4D;
pub const GL_FIXED:                                                      u16 = 0x140C;
pub const GL_FIXED_OES:                                                  u16 = 0x140C;
pub const GL_FIXED_ONLY:                                                 u16 = 0x891D;
pub const GL_FIXED_ONLY_ARB:                                             u16 = 0x891D;
pub const GL_FLOAT:                                                      u16 = 0x1406;
pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV:                             u16 = 0x8DAD;
pub const GL_FLOAT_MAT2:                                                 u16 = 0x8B5A;
pub const GL_FLOAT_MAT2_ARB:                                             u16 = 0x8B5A;
pub const GL_FLOAT_MAT2x3:                                               u16 = 0x8B65;
pub const GL_FLOAT_MAT2x4:                                               u16 = 0x8B66;
pub const GL_FLOAT_MAT3:                                                 u16 = 0x8B5B;
pub const GL_FLOAT_MAT3_ARB:                                             u16 = 0x8B5B;
pub const GL_FLOAT_MAT3x2:                                               u16 = 0x8B67;
pub const GL_FLOAT_MAT3x4:                                               u16 = 0x8B68;
pub const GL_FLOAT_MAT4:                                                 u16 = 0x8B5C;
pub const GL_FLOAT_MAT4_ARB:                                             u16 = 0x8B5C;
pub const GL_FLOAT_MAT4x2:                                               u16 = 0x8B69;
pub const GL_FLOAT_MAT4x3:                                               u16 = 0x8B6A;
pub const GL_FLOAT_VEC2:                                                 u16 = 0x8B50;
pub const GL_FLOAT_VEC2_ARB:                                             u16 = 0x8B50;
pub const GL_FLOAT_VEC3:                                                 u16 = 0x8B51;
pub const GL_FLOAT_VEC3_ARB:                                             u16 = 0x8B51;
pub const GL_FLOAT_VEC4:                                                 u16 = 0x8B52;
pub const GL_FLOAT_VEC4_ARB:                                             u16 = 0x8B52;
pub const GL_FOG_COORDINATE_ARRAY_BUFFER_BINDING_ARB:                    u16 = 0x889D;
pub const GL_FOG_COORDINATE_ARRAY_EXT:                                   u16 = 0x8457;
pub const GL_FOG_COORDINATE_ARRAY_POINTER_EXT:                           u16 = 0x8456;
pub const GL_FOG_COORDINATE_ARRAY_STRIDE_EXT:                            u16 = 0x8455;
pub const GL_FOG_COORDINATE_ARRAY_TYPE_EXT:                              u16 = 0x8454;
pub const GL_FOG_COORDINATE_EXT:                                         u16 = 0x8451;
pub const GL_FOG_COORDINATE_SOURCE_EXT:                                  u16 = 0x8450;
pub const GL_FRACTIONAL_EVEN:                                            u16 = 0x8E7C;
pub const GL_FRACTIONAL_ODD:                                             u16 = 0x8E7B;
pub const GL_FRAGMENT_DEPTH_EXT:                                         u16 = 0x8452;
pub const GL_FRAGMENT_INTERPOLATION_OFFSET_BITS:                         u16 = 0x8E5D;
pub const GL_FRAGMENT_PROGRAM_ARB:                                       u16 = 0x8804;
pub const GL_FRAGMENT_SHADER:                                            u16 = 0x8B30;
pub const GL_FRAGMENT_SHADER_ARB:                                        u16 = 0x8B30;
pub const GL_FRAGMENT_SHADER_BIT:                                        u32 = 0x00000002;
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT:                            u16 = 0x8B8B;
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT_ARB:                        u16 = 0x8B8B;
pub const GL_FRAGMENT_SHADER_INVOCATIONS:                                u16 = 0x82F4;
pub const GL_FRAGMENT_SHADER_INVOCATIONS_ARB:                            u16 = 0x82F4;
pub const GL_FRAGMENT_SUBROUTINE:                                        u16 = 0x92EC;
pub const GL_FRAGMENT_SUBROUTINE_UNIFORM:                                u16 = 0x92F2;
pub const GL_FRAGMENT_TEXTURE:                                           u16 = 0x829F;
pub const GL_FRAMEBUFFER:                                                u16 = 0x8D40;
pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE:                          u16 = 0x8215;
pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE:                           u16 = 0x8214;
pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING:                      u16 = 0x8210;
pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE:                      u16 = 0x8211;
pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE:                          u16 = 0x8216;
pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE:                          u16 = 0x8213;
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED:                             u16 = 0x8DA7;
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED_ARB:                         u16 = 0x8DA7;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME:                         u16 = 0x8CD1;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME_EXT:                     u16 = 0x8CD1;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE:                         u16 = 0x8CD0;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE_EXT:                     u16 = 0x8CD0;
pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE:                            u16 = 0x8212;
pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE:                        u16 = 0x8217;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_3D_ZOFFSET_EXT:              u16 = 0x8CD4;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE:               u16 = 0x8CD3;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE_EXT:           u16 = 0x8CD3;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER:                       u16 = 0x8CD4;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL:                       u16 = 0x8CD2;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL_EXT:                   u16 = 0x8CD2;
pub const GL_FRAMEBUFFER_BARRIER_BIT:                                    u32 = 0x00000400;
pub const GL_FRAMEBUFFER_BINDING:                                        u16 = 0x8CA6;
pub const GL_FRAMEBUFFER_BINDING_EXT:                                    u16 = 0x8CA6;
pub const GL_FRAMEBUFFER_BLEND:                                          u16 = 0x828B;
pub const GL_FRAMEBUFFER_COMPLETE:                                       u16 = 0x8CD5;
pub const GL_FRAMEBUFFER_COMPLETE_EXT:                                   u16 = 0x8CD5;
pub const GL_FRAMEBUFFER_DEFAULT:                                        u16 = 0x8218;
pub const GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS:                 u16 = 0x9314;
pub const GL_FRAMEBUFFER_DEFAULT_HEIGHT:                                 u16 = 0x9311;
pub const GL_FRAMEBUFFER_DEFAULT_LAYERS:                                 u16 = 0x9312;
pub const GL_FRAMEBUFFER_DEFAULT_SAMPLES:                                u16 = 0x9313;
pub const GL_FRAMEBUFFER_DEFAULT_WIDTH:                                  u16 = 0x9310;
pub const GL_FRAMEBUFFER_EXT:                                            u16 = 0x8D40;
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT:                          u16 = 0x8CD6;
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT_EXT:                      u16 = 0x8CD6;
pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS_EXT:                      u16 = 0x8CD9;
pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER:                         u16 = 0x8CDB;
pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER_EXT:                     u16 = 0x8CDB;
pub const GL_FRAMEBUFFER_INCOMPLETE_FORMATS_EXT:                         u16 = 0x8CDA;
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_COUNT_ARB:                     u16 = 0x8DA9;
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS:                       u16 = 0x8DA8;
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS_ARB:                   u16 = 0x8DA8;
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT:                  u16 = 0x8CD7;
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT_EXT:              u16 = 0x8CD7;
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE:                         u16 = 0x8D56;
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_EXT:                     u16 = 0x8D56;
pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER:                         u16 = 0x8CDC;
pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER_EXT:                     u16 = 0x8CDC;
pub const GL_FRAMEBUFFER_PROGRAMMABLE_SAMPLE_LOCATIONS_ARB:              u16 = 0x9342;
pub const GL_FRAMEBUFFER_RENDERABLE:                                     u16 = 0x8289;
pub const GL_FRAMEBUFFER_RENDERABLE_LAYERED:                             u16 = 0x828A;
pub const GL_FRAMEBUFFER_SAMPLE_LOCATION_PIXEL_GRID_ARB:                 u16 = 0x9343;
pub const GL_FRAMEBUFFER_SRGB:                                           u16 = 0x8DB9;
pub const GL_FRAMEBUFFER_SRGB_CAPABLE_EXT:                               u16 = 0x8DBA;
pub const GL_FRAMEBUFFER_SRGB_EXT:                                       u16 = 0x8DB9;
pub const GL_FRAMEBUFFER_UNDEFINED:                                      u16 = 0x8219;
pub const GL_FRAMEBUFFER_UNSUPPORTED:                                    u16 = 0x8CDD;
pub const GL_FRAMEBUFFER_UNSUPPORTED_EXT:                                u16 = 0x8CDD;
pub const GL_FRONT:                                                      u16 = 0x0404;
pub const GL_FRONT_AND_BACK:                                             u16 = 0x0408;
pub const GL_FRONT_FACE:                                                 u16 = 0x0B46;
pub const GL_FRONT_LEFT:                                                 u16 = 0x0400;
pub const GL_FRONT_RIGHT:                                                u16 = 0x0401;
pub const GL_FULL_SUPPORT:                                               u16 = 0x82B7;
pub const GL_FUNC_ADD:                                                   u16 = 0x8006;
pub const GL_FUNC_REVERSE_SUBTRACT:                                      u16 = 0x800B;
pub const GL_FUNC_SUBTRACT:                                              u16 = 0x800A;
pub const GL_GEOMETRY_INPUT_TYPE:                                        u16 = 0x8917;
pub const GL_GEOMETRY_INPUT_TYPE_ARB:                                    u16 = 0x8DDB;
pub const GL_GEOMETRY_OUTPUT_TYPE:                                       u16 = 0x8918;
pub const GL_GEOMETRY_OUTPUT_TYPE_ARB:                                   u16 = 0x8DDC;
pub const GL_GEOMETRY_SHADER:                                            u16 = 0x8DD9;
pub const GL_GEOMETRY_SHADER_ARB:                                        u16 = 0x8DD9;
pub const GL_GEOMETRY_SHADER_BIT:                                        u32 = 0x00000004;
pub const GL_GEOMETRY_SHADER_INVOCATIONS:                                u16 = 0x887F;
pub const GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED:                         u16 = 0x82F3;
pub const GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED_ARB:                     u16 = 0x82F3;
pub const GL_GEOMETRY_SUBROUTINE:                                        u16 = 0x92EB;
pub const GL_GEOMETRY_SUBROUTINE_UNIFORM:                                u16 = 0x92F1;
pub const GL_GEOMETRY_TEXTURE:                                           u16 = 0x829E;
pub const GL_GEOMETRY_VERTICES_OUT:                                      u16 = 0x8916;
pub const GL_GEOMETRY_VERTICES_OUT_ARB:                                  u16 = 0x8DDA;
pub const GL_GEQUAL:                                                     u16 = 0x0206;
pub const GL_GET_TEXTURE_IMAGE_FORMAT:                                   u16 = 0x8291;
pub const GL_GET_TEXTURE_IMAGE_TYPE:                                     u16 = 0x8292;
pub const GL_GREATER:                                                    u16 = 0x0204;
pub const GL_GREEN:                                                      u16 = 0x1904;
pub const GL_GREEN_INTEGER:                                              u16 = 0x8D95;
pub const GL_HALF_FLOAT:                                                 u16 = 0x140B;
pub const GL_HALF_FLOAT_ARB:                                             u16 = 0x140B;
pub const GL_HIGH_FLOAT:                                                 u16 = 0x8DF2;
pub const GL_HIGH_INT:                                                   u16 = 0x8DF5;
pub const GL_IMAGE_1D:                                                   u16 = 0x904C;
pub const GL_IMAGE_1D_ARRAY:                                             u16 = 0x9052;
pub const GL_IMAGE_2D:                                                   u16 = 0x904D;
pub const GL_IMAGE_2D_ARRAY:                                             u16 = 0x9053;
pub const GL_IMAGE_2D_MULTISAMPLE:                                       u16 = 0x9055;
pub const GL_IMAGE_2D_MULTISAMPLE_ARRAY:                                 u16 = 0x9056;
pub const GL_IMAGE_2D_RECT:                                              u16 = 0x904F;
pub const GL_IMAGE_3D:                                                   u16 = 0x904E;
pub const GL_IMAGE_BINDING_ACCESS:                                       u16 = 0x8F3E;
pub const GL_IMAGE_BINDING_FORMAT:                                       u16 = 0x906E;
pub const GL_IMAGE_BINDING_LAYER:                                        u16 = 0x8F3D;
pub const GL_IMAGE_BINDING_LAYERED:                                      u16 = 0x8F3C;
pub const GL_IMAGE_BINDING_LEVEL:                                        u16 = 0x8F3B;
pub const GL_IMAGE_BINDING_NAME:                                         u16 = 0x8F3A;
pub const GL_IMAGE_BUFFER:                                               u16 = 0x9051;
pub const GL_IMAGE_CLASS_10_10_10_2:                                     u16 = 0x82C3;
pub const GL_IMAGE_CLASS_11_11_10:                                       u16 = 0x82C2;
pub const GL_IMAGE_CLASS_1_X_16:                                         u16 = 0x82BE;
pub const GL_IMAGE_CLASS_1_X_32:                                         u16 = 0x82BB;
pub const GL_IMAGE_CLASS_1_X_8:                                          u16 = 0x82C1;
pub const GL_IMAGE_CLASS_2_X_16:                                         u16 = 0x82BD;
pub const GL_IMAGE_CLASS_2_X_32:                                         u16 = 0x82BA;
pub const GL_IMAGE_CLASS_2_X_8:                                          u16 = 0x82C0;
pub const GL_IMAGE_CLASS_4_X_16:                                         u16 = 0x82BC;
pub const GL_IMAGE_CLASS_4_X_32:                                         u16 = 0x82B9;
pub const GL_IMAGE_CLASS_4_X_8:                                          u16 = 0x82BF;
pub const GL_IMAGE_COMPATIBILITY_CLASS:                                  u16 = 0x82A8;
pub const GL_IMAGE_CUBE:                                                 u16 = 0x9050;
pub const GL_IMAGE_CUBE_MAP_ARRAY:                                       u16 = 0x9054;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS:                        u16 = 0x90C9;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE:                         u16 = 0x90C8;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_TYPE:                            u16 = 0x90C7;
pub const GL_IMAGE_PIXEL_FORMAT:                                         u16 = 0x82A9;
pub const GL_IMAGE_PIXEL_TYPE:                                           u16 = 0x82AA;
pub const GL_IMAGE_TEXEL_SIZE:                                           u16 = 0x82A7;
pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT:                           u16 = 0x8B9B;
pub const GL_IMPLEMENTATION_COLOR_READ_TYPE:                             u16 = 0x8B9A;
pub const GL_INCR:                                                       u16 = 0x1E02;
pub const GL_INCR_WRAP:                                                  u16 = 0x8507;
pub const GL_INDEX_ARRAY_BUFFER_BINDING_ARB:                             u16 = 0x8899;
pub const GL_INFO_LOG_LENGTH:                                            u16 = 0x8B84;
pub const GL_INT:                                                        u16 = 0x1404;
pub const GL_INT64_ARB:                                                  u16 = 0x140E;
pub const GL_INT64_VEC2_ARB:                                             u16 = 0x8FE9;
pub const GL_INT64_VEC3_ARB:                                             u16 = 0x8FEA;
pub const GL_INT64_VEC4_ARB:                                             u16 = 0x8FEB;
pub const GL_INTENSITY16F_ARB:                                           u16 = 0x881D;
pub const GL_INTENSITY32F_ARB:                                           u16 = 0x8817;
pub const GL_INTERLEAVED_ATTRIBS:                                        u16 = 0x8C8C;
pub const GL_INTERNALFORMAT_ALPHA_SIZE:                                  u16 = 0x8274;
pub const GL_INTERNALFORMAT_ALPHA_TYPE:                                  u16 = 0x827B;
pub const GL_INTERNALFORMAT_BLUE_SIZE:                                   u16 = 0x8273;
pub const GL_INTERNALFORMAT_BLUE_TYPE:                                   u16 = 0x827A;
pub const GL_INTERNALFORMAT_DEPTH_SIZE:                                  u16 = 0x8275;
pub const GL_INTERNALFORMAT_DEPTH_TYPE:                                  u16 = 0x827C;
pub const GL_INTERNALFORMAT_GREEN_SIZE:                                  u16 = 0x8272;
pub const GL_INTERNALFORMAT_GREEN_TYPE:                                  u16 = 0x8279;
pub const GL_INTERNALFORMAT_PREFERRED:                                   u16 = 0x8270;
pub const GL_INTERNALFORMAT_RED_SIZE:                                    u16 = 0x8271;
pub const GL_INTERNALFORMAT_RED_TYPE:                                    u16 = 0x8278;
pub const GL_INTERNALFORMAT_SHARED_SIZE:                                 u16 = 0x8277;
pub const GL_INTERNALFORMAT_STENCIL_SIZE:                                u16 = 0x8276;
pub const GL_INTERNALFORMAT_STENCIL_TYPE:                                u16 = 0x827D;
pub const GL_INTERNALFORMAT_SUPPORTED:                                   u16 = 0x826F;
pub const GL_INT_2_10_10_10_REV:                                         u16 = 0x8D9F;
pub const GL_INT_IMAGE_1D:                                               u16 = 0x9057;
pub const GL_INT_IMAGE_1D_ARRAY:                                         u16 = 0x905D;
pub const GL_INT_IMAGE_2D:                                               u16 = 0x9058;
pub const GL_INT_IMAGE_2D_ARRAY:                                         u16 = 0x905E;
pub const GL_INT_IMAGE_2D_MULTISAMPLE:                                   u16 = 0x9060;
pub const GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY:                             u16 = 0x9061;
pub const GL_INT_IMAGE_2D_RECT:                                          u16 = 0x905A;
pub const GL_INT_IMAGE_3D:                                               u16 = 0x9059;
pub const GL_INT_IMAGE_BUFFER:                                           u16 = 0x905C;
pub const GL_INT_IMAGE_CUBE:                                             u16 = 0x905B;
pub const GL_INT_IMAGE_CUBE_MAP_ARRAY:                                   u16 = 0x905F;
pub const GL_INT_SAMPLER_1D:                                             u16 = 0x8DC9;
pub const GL_INT_SAMPLER_1D_ARRAY:                                       u16 = 0x8DCE;
pub const GL_INT_SAMPLER_2D:                                             u16 = 0x8DCA;
pub const GL_INT_SAMPLER_2D_ARRAY:                                       u16 = 0x8DCF;
pub const GL_INT_SAMPLER_2D_MULTISAMPLE:                                 u16 = 0x9109;
pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY:                           u16 = 0x910C;
pub const GL_INT_SAMPLER_2D_RECT:                                        u16 = 0x8DCD;
pub const GL_INT_SAMPLER_3D:                                             u16 = 0x8DCB;
pub const GL_INT_SAMPLER_BUFFER:                                         u16 = 0x8DD0;
pub const GL_INT_SAMPLER_CUBE:                                           u16 = 0x8DCC;
pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY:                                 u16 = 0x900E;
pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY_ARB:                             u16 = 0x900E;
pub const GL_INT_VEC2:                                                   u16 = 0x8B53;
pub const GL_INT_VEC2_ARB:                                               u16 = 0x8B53;
pub const GL_INT_VEC3:                                                   u16 = 0x8B54;
pub const GL_INT_VEC3_ARB:                                               u16 = 0x8B54;
pub const GL_INT_VEC4:                                                   u16 = 0x8B55;
pub const GL_INT_VEC4_ARB:                                               u16 = 0x8B55;
pub const GL_INVALID_ENUM:                                               u16 = 0x0500;
pub const GL_INVALID_FRAMEBUFFER_OPERATION:                              u16 = 0x0506;
pub const GL_INVALID_FRAMEBUFFER_OPERATION_EXT:                          u16 = 0x0506;
pub const GL_INVALID_INDEX:                                              u32 = 0xFFFFFFFF;
pub const GL_INVALID_OPERATION:                                          u16 = 0x0502;
pub const GL_INVALID_VALUE:                                              u16 = 0x0501;
pub const GL_INVERT:                                                     u16 = 0x150A;
pub const GL_ISOLINES:                                                   u16 = 0x8E7A;
pub const GL_IS_PER_PATCH:                                               u16 = 0x92E7;
pub const GL_IS_ROW_MAJOR:                                               u16 = 0x9300;
pub const GL_KEEP:                                                       u16 = 0x1E00;
pub const GL_LAST_VERTEX_CONVENTION:                                     u16 = 0x8E4E;
pub const GL_LAYER_PROVOKING_VERTEX:                                     u16 = 0x825E;
pub const GL_LEFT:                                                       u16 = 0x0406;
pub const GL_LEQUAL:                                                     u16 = 0x0203;
pub const GL_LESS:                                                       u16 = 0x0201;
pub const GL_LINE:                                                       u16 = 0x1B01;
pub const GL_LINEAR:                                                     u16 = 0x2601;
pub const GL_LINEAR_MIPMAP_LINEAR:                                       u16 = 0x2703;
pub const GL_LINEAR_MIPMAP_NEAREST:                                      u16 = 0x2701;
pub const GL_LINES:                                                      u16 = 0x0001;
pub const GL_LINES_ADJACENCY:                                            u16 = 0x000A;
pub const GL_LINES_ADJACENCY_ARB:                                        u16 = 0x000A;
pub const GL_LINE_LOOP:                                                  u16 = 0x0002;
pub const GL_LINE_SMOOTH:                                                u16 = 0x0B20;
pub const GL_LINE_SMOOTH_HINT:                                           u16 = 0x0C52;
pub const GL_LINE_STRIP:                                                 u16 = 0x0003;
pub const GL_LINE_STRIP_ADJACENCY:                                       u16 = 0x000B;
pub const GL_LINE_STRIP_ADJACENCY_ARB:                                   u16 = 0x000B;
pub const GL_LINE_WIDTH:                                                 u16 = 0x0B21;
pub const GL_LINE_WIDTH_GRANULARITY:                                     u16 = 0x0B23;
pub const GL_LINE_WIDTH_RANGE:                                           u16 = 0x0B22;
pub const GL_LINK_STATUS:                                                u16 = 0x8B82;
pub const GL_LOCATION:                                                   u16 = 0x930E;
pub const GL_LOCATION_COMPONENT:                                         u16 = 0x934A;
pub const GL_LOCATION_INDEX:                                             u16 = 0x930F;
pub const GL_LOGIC_OP_MODE:                                              u16 = 0x0BF0;
pub const GL_LOWER_LEFT:                                                 u16 = 0x8CA1;
pub const GL_LOW_FLOAT:                                                  u16 = 0x8DF0;
pub const GL_LOW_INT:                                                    u16 = 0x8DF3;
pub const GL_LUMINANCE16F_ARB:                                           u16 = 0x881E;
pub const GL_LUMINANCE32F_ARB:                                           u16 = 0x8818;
pub const GL_LUMINANCE_ALPHA16F_ARB:                                     u16 = 0x881F;
pub const GL_LUMINANCE_ALPHA32F_ARB:                                     u16 = 0x8819;
pub const GL_MAJOR_VERSION:                                              u16 = 0x821B;
pub const GL_MANUAL_GENERATE_MIPMAP:                                     u16 = 0x8294;
pub const GL_MAP_COHERENT_BIT:                                           u16 = 0x0080;
pub const GL_MAP_FLUSH_EXPLICIT_BIT:                                     u16 = 0x0010;
pub const GL_MAP_INVALIDATE_BUFFER_BIT:                                  u16 = 0x0008;
pub const GL_MAP_INVALIDATE_RANGE_BIT:                                   u16 = 0x0004;
pub const GL_MAP_PERSISTENT_BIT:                                         u16 = 0x0040;
pub const GL_MAP_READ_BIT:                                               u16 = 0x0001;
pub const GL_MAP_UNSYNCHRONIZED_BIT:                                     u16 = 0x0020;
pub const GL_MAP_WRITE_BIT:                                              u16 = 0x0002;
pub const GL_MATRIX0_ARB:                                                u16 = 0x88C0;
pub const GL_MATRIX10_ARB:                                               u16 = 0x88CA;
pub const GL_MATRIX11_ARB:                                               u16 = 0x88CB;
pub const GL_MATRIX12_ARB:                                               u16 = 0x88CC;
pub const GL_MATRIX13_ARB:                                               u16 = 0x88CD;
pub const GL_MATRIX14_ARB:                                               u16 = 0x88CE;
pub const GL_MATRIX15_ARB:                                               u16 = 0x88CF;
pub const GL_MATRIX16_ARB:                                               u16 = 0x88D0;
pub const GL_MATRIX17_ARB:                                               u16 = 0x88D1;
pub const GL_MATRIX18_ARB:                                               u16 = 0x88D2;
pub const GL_MATRIX19_ARB:                                               u16 = 0x88D3;
pub const GL_MATRIX1_ARB:                                                u16 = 0x88C1;
pub const GL_MATRIX20_ARB:                                               u16 = 0x88D4;
pub const GL_MATRIX21_ARB:                                               u16 = 0x88D5;
pub const GL_MATRIX22_ARB:                                               u16 = 0x88D6;
pub const GL_MATRIX23_ARB:                                               u16 = 0x88D7;
pub const GL_MATRIX24_ARB:                                               u16 = 0x88D8;
pub const GL_MATRIX25_ARB:                                               u16 = 0x88D9;
pub const GL_MATRIX26_ARB:                                               u16 = 0x88DA;
pub const GL_MATRIX27_ARB:                                               u16 = 0x88DB;
pub const GL_MATRIX28_ARB:                                               u16 = 0x88DC;
pub const GL_MATRIX29_ARB:                                               u16 = 0x88DD;
pub const GL_MATRIX2_ARB:                                                u16 = 0x88C2;
pub const GL_MATRIX30_ARB:                                               u16 = 0x88DE;
pub const GL_MATRIX31_ARB:                                               u16 = 0x88DF;
pub const GL_MATRIX3_ARB:                                                u16 = 0x88C3;
pub const GL_MATRIX4_ARB:                                                u16 = 0x88C4;
pub const GL_MATRIX5_ARB:                                                u16 = 0x88C5;
pub const GL_MATRIX6_ARB:                                                u16 = 0x88C6;
pub const GL_MATRIX7_ARB:                                                u16 = 0x88C7;
pub const GL_MATRIX8_ARB:                                                u16 = 0x88C8;
pub const GL_MATRIX9_ARB:                                                u16 = 0x88C9;
pub const GL_MATRIX_STRIDE:                                              u16 = 0x92FF;
pub const GL_MAX:                                                        u16 = 0x8008;
pub const GL_MAX_3D_TEXTURE_SIZE:                                        u16 = 0x8073;
pub const GL_MAX_ARRAY_TEXTURE_LAYERS:                                   u16 = 0x88FF;
pub const GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS:                         u16 = 0x92DC;
pub const GL_MAX_ATOMIC_COUNTER_BUFFER_SIZE:                             u16 = 0x92D8;
pub const GL_MAX_CLIP_DISTANCES:                                         u16 = 0x0D32;
pub const GL_MAX_COLOR_ATTACHMENTS:                                      u16 = 0x8CDF;
pub const GL_MAX_COLOR_ATTACHMENTS_EXT:                                  u16 = 0x8CDF;
pub const GL_MAX_COLOR_TEXTURE_SAMPLES:                                  u16 = 0x910E;
pub const GL_MAX_COMBINED_ATOMIC_COUNTERS:                               u16 = 0x92D7;
pub const GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS:                        u16 = 0x92D1;
pub const GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS:                    u16 = 0x8266;
pub const GL_MAX_COMBINED_DIMENSIONS:                                    u16 = 0x8282;
pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS:                   u16 = 0x8A33;
pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS:                   u16 = 0x8A32;
pub const GL_MAX_COMBINED_IMAGE_UNIFORMS:                                u16 = 0x90CF;
pub const GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS:              u16 = 0x8F39;
pub const GL_MAX_COMBINED_SHADER_OUTPUT_RESOURCES:                       u16 = 0x8F39;
pub const GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS:                         u16 = 0x90DC;
pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS:               u16 = 0x8E1E;
pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS:            u16 = 0x8E1F;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS:                           u16 = 0x8B4D;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS_ARB:                       u16 = 0x8B4D;
pub const GL_MAX_COMBINED_UNIFORM_BLOCKS:                                u16 = 0x8A2E;
pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS:                     u16 = 0x8A31;
pub const GL_MAX_COMPUTE_ATOMIC_COUNTERS:                                u16 = 0x8265;
pub const GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS:                         u16 = 0x8264;
pub const GL_MAX_COMPUTE_FIXED_GROUP_INVOCATIONS_ARB:                    u16 = 0x90EB;
pub const GL_MAX_COMPUTE_FIXED_GROUP_SIZE_ARB:                           u16 = 0x91BF;
pub const GL_MAX_COMPUTE_IMAGE_UNIFORMS:                                 u16 = 0x91BD;
pub const GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS:                          u16 = 0x90DB;
pub const GL_MAX_COMPUTE_SHARED_MEMORY_SIZE:                             u16 = 0x8262;
pub const GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS:                            u16 = 0x91BC;
pub const GL_MAX_COMPUTE_UNIFORM_BLOCKS:                                 u16 = 0x91BB;
pub const GL_MAX_COMPUTE_UNIFORM_COMPONENTS:                             u16 = 0x8263;
pub const GL_MAX_COMPUTE_VARIABLE_GROUP_INVOCATIONS_ARB:                 u16 = 0x9344;
pub const GL_MAX_COMPUTE_VARIABLE_GROUP_SIZE_ARB:                        u16 = 0x9345;
pub const GL_MAX_COMPUTE_WORK_GROUP_COUNT:                               u16 = 0x91BE;
pub const GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS:                         u16 = 0x90EB;
pub const GL_MAX_COMPUTE_WORK_GROUP_SIZE:                                u16 = 0x91BF;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE:                                  u16 = 0x851C;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE_ARB:                              u16 = 0x851C;
pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH:                                u16 = 0x826C;
pub const GL_MAX_DEBUG_LOGGED_MESSAGES:                                  u16 = 0x9144;
pub const GL_MAX_DEBUG_LOGGED_MESSAGES_ARB:                              u16 = 0x9144;
pub const GL_MAX_DEBUG_MESSAGE_LENGTH:                                   u16 = 0x9143;
pub const GL_MAX_DEBUG_MESSAGE_LENGTH_ARB:                               u16 = 0x9143;
pub const GL_MAX_DEPTH:                                                  u16 = 0x8280;
pub const GL_MAX_DEPTH_TEXTURE_SAMPLES:                                  u16 = 0x910F;
pub const GL_MAX_DRAW_BUFFERS:                                           u16 = 0x8824;
pub const GL_MAX_DRAW_BUFFERS_ARB:                                       u16 = 0x8824;
pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS:                               u16 = 0x88FC;
pub const GL_MAX_ELEMENTS_INDICES:                                       u16 = 0x80E9;
pub const GL_MAX_ELEMENTS_VERTICES:                                      u16 = 0x80E8;
pub const GL_MAX_ELEMENT_INDEX:                                          u16 = 0x8D6B;
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTERS:                               u16 = 0x92D6;
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS:                        u16 = 0x92D0;
pub const GL_MAX_FRAGMENT_IMAGE_UNIFORMS:                                u16 = 0x90CE;
pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS:                              u16 = 0x9125;
pub const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET:                          u16 = 0x8E5C;
pub const GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS:                         u16 = 0x90DA;
pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS:                                u16 = 0x8A2D;
pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS:                            u16 = 0x8B49;
pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS_ARB:                        u16 = 0x8B49;
pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS:                               u16 = 0x8DFD;
pub const GL_MAX_FRAMEBUFFER_HEIGHT:                                     u16 = 0x9316;
pub const GL_MAX_FRAMEBUFFER_LAYERS:                                     u16 = 0x9317;
pub const GL_MAX_FRAMEBUFFER_SAMPLES:                                    u16 = 0x9318;
pub const GL_MAX_FRAMEBUFFER_WIDTH:                                      u16 = 0x9315;
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS:                               u16 = 0x92D5;
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS:                        u16 = 0x92CF;
pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS:                                u16 = 0x90CD;
pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS:                              u16 = 0x9123;
pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS:                             u16 = 0x9124;
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES:                               u16 = 0x8DE0;
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES_ARB:                           u16 = 0x8DE0;
pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS:                            u16 = 0x8E5A;
pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS:                         u16 = 0x90D7;
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS:                           u16 = 0x8C29;
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS_ARB:                       u16 = 0x8C29;
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS:                       u16 = 0x8DE1;
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS_ARB:                   u16 = 0x8DE1;
pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS:                                u16 = 0x8A2C;
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS:                            u16 = 0x8DDF;
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS_ARB:                        u16 = 0x8DDF;
pub const GL_MAX_GEOMETRY_VARYING_COMPONENTS_ARB:                        u16 = 0x8DDD;
pub const GL_MAX_HEIGHT:                                                 u16 = 0x827F;
pub const GL_MAX_IMAGE_SAMPLES:                                          u16 = 0x906D;
pub const GL_MAX_IMAGE_UNITS:                                            u16 = 0x8F38;
pub const GL_MAX_INTEGER_SAMPLES:                                        u16 = 0x9110;
pub const GL_MAX_LABEL_LENGTH:                                           u16 = 0x82E8;
pub const GL_MAX_LAYERS:                                                 u16 = 0x8281;
pub const GL_MAX_NAME_LENGTH:                                            u16 = 0x92F6;
pub const GL_MAX_NUM_ACTIVE_VARIABLES:                                   u16 = 0x92F7;
pub const GL_MAX_NUM_COMPATIBLE_SUBROUTINES:                             u16 = 0x92F8;
pub const GL_MAX_PATCH_VERTICES:                                         u16 = 0x8E7D;
pub const GL_MAX_PROGRAM_ADDRESS_REGISTERS_ARB:                          u16 = 0x88B1;
pub const GL_MAX_PROGRAM_ALU_INSTRUCTIONS_ARB:                           u16 = 0x880B;
pub const GL_MAX_PROGRAM_ATTRIBS_ARB:                                    u16 = 0x88AD;
pub const GL_MAX_PROGRAM_ENV_PARAMETERS_ARB:                             u16 = 0x88B5;
pub const GL_MAX_PROGRAM_INSTRUCTIONS_ARB:                               u16 = 0x88A1;
pub const GL_MAX_PROGRAM_LOCAL_PARAMETERS_ARB:                           u16 = 0x88B4;
pub const GL_MAX_PROGRAM_MATRICES_ARB:                                   u16 = 0x862F;
pub const GL_MAX_PROGRAM_MATRIX_STACK_DEPTH_ARB:                         u16 = 0x862E;
pub const GL_MAX_PROGRAM_NATIVE_ADDRESS_REGISTERS_ARB:                   u16 = 0x88B3;
pub const GL_MAX_PROGRAM_NATIVE_ALU_INSTRUCTIONS_ARB:                    u16 = 0x880E;
pub const GL_MAX_PROGRAM_NATIVE_ATTRIBS_ARB:                             u16 = 0x88AF;
pub const GL_MAX_PROGRAM_NATIVE_INSTRUCTIONS_ARB:                        u16 = 0x88A3;
pub const GL_MAX_PROGRAM_NATIVE_PARAMETERS_ARB:                          u16 = 0x88AB;
pub const GL_MAX_PROGRAM_NATIVE_TEMPORARIES_ARB:                         u16 = 0x88A7;
pub const GL_MAX_PROGRAM_NATIVE_TEX_INDIRECTIONS_ARB:                    u16 = 0x8810;
pub const GL_MAX_PROGRAM_NATIVE_TEX_INSTRUCTIONS_ARB:                    u16 = 0x880F;
pub const GL_MAX_PROGRAM_PARAMETERS_ARB:                                 u16 = 0x88A9;
pub const GL_MAX_PROGRAM_TEMPORARIES_ARB:                                u16 = 0x88A5;
pub const GL_MAX_PROGRAM_TEXEL_OFFSET:                                   u16 = 0x8905;
pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET:                          u16 = 0x8E5F;
pub const GL_MAX_PROGRAM_TEX_INDIRECTIONS_ARB:                           u16 = 0x880D;
pub const GL_MAX_PROGRAM_TEX_INSTRUCTIONS_ARB:                           u16 = 0x880C;
pub const GL_MAX_RECTANGLE_TEXTURE_SIZE:                                 u16 = 0x84F8;
pub const GL_MAX_RENDERBUFFER_SIZE:                                      u16 = 0x84E8;
pub const GL_MAX_RENDERBUFFER_SIZE_EXT:                                  u16 = 0x84E8;
pub const GL_MAX_SAMPLES:                                                u16 = 0x8D57;
pub const GL_MAX_SAMPLES_EXT:                                            u16 = 0x8D57;
pub const GL_MAX_SAMPLE_MASK_WORDS:                                      u16 = 0x8E59;
pub const GL_MAX_SERVER_WAIT_TIMEOUT:                                    u16 = 0x9111;
pub const GL_MAX_SHADER_STORAGE_BLOCK_SIZE:                              u16 = 0x90DE;
pub const GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS:                         u16 = 0x90DD;
pub const GL_MAX_SUBROUTINES:                                            u16 = 0x8DE7;
pub const GL_MAX_SUBROUTINE_UNIFORM_LOCATIONS:                           u16 = 0x8DE8;
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS:                           u16 = 0x92D3;
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS:                    u16 = 0x92CD;
pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS:                            u16 = 0x90CB;
pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS:                          u16 = 0x886C;
pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS:                         u16 = 0x8E83;
pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS:                     u16 = 0x90D8;
pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS:                       u16 = 0x8E81;
pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS:                   u16 = 0x8E85;
pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS:                            u16 = 0x8E89;
pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS:                        u16 = 0x8E7F;
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS:                        u16 = 0x92D4;
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS:                 u16 = 0x92CE;
pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS:                         u16 = 0x90CC;
pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS:                       u16 = 0x886D;
pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS:                      u16 = 0x8E86;
pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS:                  u16 = 0x90D9;
pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS:                    u16 = 0x8E82;
pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS:                         u16 = 0x8E8A;
pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS:                     u16 = 0x8E80;
pub const GL_MAX_TESS_GEN_LEVEL:                                         u16 = 0x8E7E;
pub const GL_MAX_TESS_PATCH_COMPONENTS:                                  u16 = 0x8E84;
pub const GL_MAX_TEXTURE_BUFFER_SIZE:                                    u16 = 0x8C2B;
pub const GL_MAX_TEXTURE_COORDS_ARB:                                     u16 = 0x8871;
pub const GL_MAX_TEXTURE_IMAGE_UNITS:                                    u16 = 0x8872;
pub const GL_MAX_TEXTURE_IMAGE_UNITS_ARB:                                u16 = 0x8872;
pub const GL_MAX_TEXTURE_LOD_BIAS:                                       u16 = 0x84FD;
pub const GL_MAX_TEXTURE_MAX_ANISOTROPY:                                 u16 = 0x84FF;
pub const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT:                             u16 = 0x84FF;
pub const GL_MAX_TEXTURE_SIZE:                                           u16 = 0x0D33;
pub const GL_MAX_TEXTURE_UNITS_ARB:                                      u16 = 0x84E2;
pub const GL_MAX_TRANSFORM_FEEDBACK_BUFFERS:                             u16 = 0x8E70;
pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS:              u16 = 0x8C8A;
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS:                    u16 = 0x8C8B;
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS:                 u16 = 0x8C80;
pub const GL_MAX_UNIFORM_BLOCK_SIZE:                                     u16 = 0x8A30;
pub const GL_MAX_UNIFORM_BUFFER_BINDINGS:                                u16 = 0x8A2F;
pub const GL_MAX_UNIFORM_LOCATIONS:                                      u16 = 0x826E;
pub const GL_MAX_VARYING_COMPONENTS:                                     u16 = 0x8B4B;
pub const GL_MAX_VARYING_FLOATS:                                         u16 = 0x8B4B;
pub const GL_MAX_VARYING_FLOATS_ARB:                                     u16 = 0x8B4B;
pub const GL_MAX_VARYING_VECTORS:                                        u16 = 0x8DFC;
pub const GL_MAX_VERTEX_ATOMIC_COUNTERS:                                 u16 = 0x92D2;
pub const GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS:                          u16 = 0x92CC;
pub const GL_MAX_VERTEX_ATTRIBS:                                         u16 = 0x8869;
pub const GL_MAX_VERTEX_ATTRIBS_ARB:                                     u16 = 0x8869;
pub const GL_MAX_VERTEX_ATTRIB_BINDINGS:                                 u16 = 0x82DA;
pub const GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET:                          u16 = 0x82D9;
pub const GL_MAX_VERTEX_IMAGE_UNIFORMS:                                  u16 = 0x90CA;
pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS:                               u16 = 0x9122;
pub const GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS:                           u16 = 0x90D6;
pub const GL_MAX_VERTEX_STREAMS:                                         u16 = 0x8E71;
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS:                             u16 = 0x8B4C;
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS_ARB:                         u16 = 0x8B4C;
pub const GL_MAX_VERTEX_UNIFORM_BLOCKS:                                  u16 = 0x8A2B;
pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS:                              u16 = 0x8B4A;
pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS_ARB:                          u16 = 0x8B4A;
pub const GL_MAX_VERTEX_UNIFORM_VECTORS:                                 u16 = 0x8DFB;
pub const GL_MAX_VERTEX_VARYING_COMPONENTS_ARB:                          u16 = 0x8DDE;
pub const GL_MAX_VIEWPORTS:                                              u16 = 0x825B;
pub const GL_MAX_VIEWPORT_DIMS:                                          u16 = 0x0D3A;
pub const GL_MAX_WIDTH:                                                  u16 = 0x827E;
pub const GL_MEDIUM_FLOAT:                                               u16 = 0x8DF1;
pub const GL_MEDIUM_INT:                                                 u16 = 0x8DF4;
pub const GL_MIN:                                                        u16 = 0x8007;
pub const GL_MINOR_VERSION:                                              u16 = 0x821C;
pub const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET:                          u16 = 0x8E5B;
pub const GL_MIN_MAP_BUFFER_ALIGNMENT:                                   u16 = 0x90BC;
pub const GL_MIN_PROGRAM_TEXEL_OFFSET:                                   u16 = 0x8904;
pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET:                          u16 = 0x8E5E;
pub const GL_MIN_SAMPLE_SHADING_VALUE:                                   u16 = 0x8C37;
pub const GL_MIN_SAMPLE_SHADING_VALUE_ARB:                               u16 = 0x8C37;
pub const GL_MIPMAP:                                                     u16 = 0x8293;
pub const GL_MIRRORED_REPEAT:                                            u16 = 0x8370;
pub const GL_MIRRORED_REPEAT_ARB:                                        u16 = 0x8370;
pub const GL_MIRROR_CLAMP_EXT:                                           u16 = 0x8742;
pub const GL_MIRROR_CLAMP_TO_BORDER_EXT:                                 u16 = 0x8912;
pub const GL_MIRROR_CLAMP_TO_EDGE:                                       u16 = 0x8743;
pub const GL_MIRROR_CLAMP_TO_EDGE_EXT:                                   u16 = 0x8743;
pub const GL_MULTISAMPLE:                                                u16 = 0x809D;
pub const GL_MULTISAMPLE_ARB:                                            u16 = 0x809D;
pub const GL_MULTISAMPLE_BIT_ARB:                                        u32 = 0x20000000;
pub const GL_MULTISAMPLE_LINE_WIDTH_GRANULARITY_ARB:                     u16 = 0x9382;
pub const GL_MULTISAMPLE_LINE_WIDTH_RANGE_ARB:                           u16 = 0x9381;
pub const GL_NAMED_STRING_LENGTH_ARB:                                    u16 = 0x8DE9;
pub const GL_NAMED_STRING_TYPE_ARB:                                      u16 = 0x8DEA;
pub const GL_NAME_LENGTH:                                                u16 = 0x92F9;
pub const GL_NAND:                                                       u16 = 0x150E;
pub const GL_NEAREST:                                                    u16 = 0x2600;
pub const GL_NEAREST_MIPMAP_LINEAR:                                      u16 = 0x2702;
pub const GL_NEAREST_MIPMAP_NEAREST:                                     u16 = 0x2700;
pub const GL_NEVER:                                                      u16 = 0x0200;
pub const GL_NICEST:                                                     u16 = 0x1102;
pub const GL_NONE:                                                       u8  = 0;
pub const GL_NOOP:                                                       u16 = 0x1505;
pub const GL_NOR:                                                        u16 = 0x1508;
pub const GL_NORMAL_ARRAY_BUFFER_BINDING_ARB:                            u16 = 0x8897;
pub const GL_NORMAL_MAP_ARB:                                             u16 = 0x8511;
pub const GL_NOTEQUAL:                                                   u16 = 0x0205;
pub const GL_NO_ERROR:                                                   u8  = 0;
pub const GL_NUM_ACTIVE_VARIABLES:                                       u16 = 0x9304;
pub const GL_NUM_COMPATIBLE_SUBROUTINES:                                 u16 = 0x8E4A;
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS:                             u16 = 0x86A2;
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS_ARB:                         u16 = 0x86A2;
pub const GL_NUM_EXTENSIONS:                                             u16 = 0x821D;
pub const GL_NUM_PROGRAM_BINARY_FORMATS:                                 u16 = 0x87FE;
pub const GL_NUM_SAMPLE_COUNTS:                                          u16 = 0x9380;
pub const GL_NUM_SHADER_BINARY_FORMATS:                                  u16 = 0x8DF9;
pub const GL_NUM_SHADING_LANGUAGE_VERSIONS:                              u16 = 0x82E9;
pub const GL_NUM_SPIR_V_EXTENSIONS:                                      u16 = 0x9554;
pub const GL_OBJECT_ACTIVE_ATTRIBUTES_ARB:                               u16 = 0x8B89;
pub const GL_OBJECT_ACTIVE_ATTRIBUTE_MAX_LENGTH_ARB:                     u16 = 0x8B8A;
pub const GL_OBJECT_ACTIVE_UNIFORMS_ARB:                                 u16 = 0x8B86;
pub const GL_OBJECT_ACTIVE_UNIFORM_MAX_LENGTH_ARB:                       u16 = 0x8B87;
pub const GL_OBJECT_ATTACHED_OBJECTS_ARB:                                u16 = 0x8B85;
pub const GL_OBJECT_COMPILE_STATUS_ARB:                                  u16 = 0x8B81;
pub const GL_OBJECT_DELETE_STATUS_ARB:                                   u16 = 0x8B80;
pub const GL_OBJECT_INFO_LOG_LENGTH_ARB:                                 u16 = 0x8B84;
pub const GL_OBJECT_LINK_STATUS_ARB:                                     u16 = 0x8B82;
pub const GL_OBJECT_SHADER_SOURCE_LENGTH_ARB:                            u16 = 0x8B88;
pub const GL_OBJECT_SUBTYPE_ARB:                                         u16 = 0x8B4F;
pub const GL_OBJECT_TYPE:                                                u16 = 0x9112;
pub const GL_OBJECT_TYPE_ARB:                                            u16 = 0x8B4E;
pub const GL_OBJECT_VALIDATE_STATUS_ARB:                                 u16 = 0x8B83;
pub const GL_OFFSET:                                                     u16 = 0x92FC;
pub const GL_ONE:                                                        u8  = 1;
pub const GL_ONE_MINUS_CONSTANT_ALPHA:                                   u16 = 0x8004;
pub const GL_ONE_MINUS_CONSTANT_COLOR:                                   u16 = 0x8002;
pub const GL_ONE_MINUS_DST_ALPHA:                                        u16 = 0x0305;
pub const GL_ONE_MINUS_DST_COLOR:                                        u16 = 0x0307;
pub const GL_ONE_MINUS_SRC1_ALPHA:                                       u16 = 0x88FB;
pub const GL_ONE_MINUS_SRC1_COLOR:                                       u16 = 0x88FA;
pub const GL_ONE_MINUS_SRC_ALPHA:                                        u16 = 0x0303;
pub const GL_ONE_MINUS_SRC_COLOR:                                        u16 = 0x0301;
pub const GL_OR:                                                         u16 = 0x1507;
pub const GL_OR_INVERTED:                                                u16 = 0x150D;
pub const GL_OR_REVERSE:                                                 u16 = 0x150B;
pub const GL_OUT_OF_MEMORY:                                              u16 = 0x0505;
pub const GL_PACK_ALIGNMENT:                                             u16 = 0x0D05;
pub const GL_PACK_COMPRESSED_BLOCK_DEPTH:                                u16 = 0x912D;
pub const GL_PACK_COMPRESSED_BLOCK_HEIGHT:                               u16 = 0x912C;
pub const GL_PACK_COMPRESSED_BLOCK_SIZE:                                 u16 = 0x912E;
pub const GL_PACK_COMPRESSED_BLOCK_WIDTH:                                u16 = 0x912B;
pub const GL_PACK_IMAGE_HEIGHT:                                          u16 = 0x806C;
pub const GL_PACK_LSB_FIRST:                                             u16 = 0x0D01;
pub const GL_PACK_ROW_LENGTH:                                            u16 = 0x0D02;
pub const GL_PACK_SKIP_IMAGES:                                           u16 = 0x806B;
pub const GL_PACK_SKIP_PIXELS:                                           u16 = 0x0D04;
pub const GL_PACK_SKIP_ROWS:                                             u16 = 0x0D03;
pub const GL_PACK_SWAP_BYTES:                                            u16 = 0x0D00;
pub const GL_PALETTE4_R5_G6_B5_OES:                                      u16 = 0x8B92;
pub const GL_PALETTE4_RGB5_A1_OES:                                       u16 = 0x8B94;
pub const GL_PALETTE4_RGB8_OES:                                          u16 = 0x8B90;
pub const GL_PALETTE4_RGBA4_OES:                                         u16 = 0x8B93;
pub const GL_PALETTE4_RGBA8_OES:                                         u16 = 0x8B91;
pub const GL_PALETTE8_R5_G6_B5_OES:                                      u16 = 0x8B97;
pub const GL_PALETTE8_RGB5_A1_OES:                                       u16 = 0x8B99;
pub const GL_PALETTE8_RGB8_OES:                                          u16 = 0x8B95;
pub const GL_PALETTE8_RGBA4_OES:                                         u16 = 0x8B98;
pub const GL_PALETTE8_RGBA8_OES:                                         u16 = 0x8B96;
pub const GL_PATCHES:                                                    u16 = 0x000E;
pub const GL_PATCH_DEFAULT_INNER_LEVEL:                                  u16 = 0x8E73;
pub const GL_PATCH_DEFAULT_OUTER_LEVEL:                                  u16 = 0x8E74;
pub const GL_PATCH_VERTICES:                                             u16 = 0x8E72;
pub const GL_PIXEL_BUFFER_BARRIER_BIT:                                   u32 = 0x00000080;
pub const GL_PIXEL_PACK_BUFFER:                                          u16 = 0x88EB;
pub const GL_PIXEL_PACK_BUFFER_BINDING:                                  u16 = 0x88ED;
pub const GL_PIXEL_UNPACK_BUFFER:                                        u16 = 0x88EC;
pub const GL_PIXEL_UNPACK_BUFFER_BINDING:                                u16 = 0x88EF;
pub const GL_POINT:                                                      u16 = 0x1B00;
pub const GL_POINTS:                                                     u16 = 0x0000;
pub const GL_POINT_FADE_THRESHOLD_SIZE:                                  u16 = 0x8128;
pub const GL_POINT_SIZE:                                                 u16 = 0x0B11;
pub const GL_POINT_SIZE_GRANULARITY:                                     u16 = 0x0B13;
pub const GL_POINT_SIZE_RANGE:                                           u16 = 0x0B12;
pub const GL_POINT_SPRITE_COORD_ORIGIN:                                  u16 = 0x8CA0;
pub const GL_POLYGON_MODE:                                               u16 = 0x0B40;
pub const GL_POLYGON_OFFSET_FACTOR:                                      u16 = 0x8038;
pub const GL_POLYGON_OFFSET_FILL:                                        u16 = 0x8037;
pub const GL_POLYGON_OFFSET_LINE:                                        u16 = 0x2A02;
pub const GL_POLYGON_OFFSET_POINT:                                       u16 = 0x2A01;
pub const GL_POLYGON_OFFSET_UNITS:                                       u16 = 0x2A00;
pub const GL_POLYGON_SMOOTH:                                             u16 = 0x0B41;
pub const GL_POLYGON_SMOOTH_HINT:                                        u16 = 0x0C53;
pub const GL_PRIMITIVES_GENERATED:                                       u16 = 0x8C87;
pub const GL_PRIMITIVES_SUBMITTED:                                       u16 = 0x82EF;
pub const GL_PRIMITIVES_SUBMITTED_ARB:                                   u16 = 0x82EF;
pub const GL_PRIMITIVE_BOUNDING_BOX_ARB:                                 u16 = 0x92BE;
pub const GL_PRIMITIVE_RESTART:                                          u16 = 0x8F9D;
pub const GL_PRIMITIVE_RESTART_FIXED_INDEX:                              u16 = 0x8D69;
pub const GL_PRIMITIVE_RESTART_INDEX:                                    u16 = 0x8F9E;
pub const GL_PROGRAM:                                                    u16 = 0x82E2;
pub const GL_PROGRAMMABLE_SAMPLE_LOCATION_ARB:                           u16 = 0x9341;
pub const GL_PROGRAMMABLE_SAMPLE_LOCATION_TABLE_SIZE_ARB:                u16 = 0x9340;
pub const GL_PROGRAM_ADDRESS_REGISTERS_ARB:                              u16 = 0x88B0;
pub const GL_PROGRAM_ALU_INSTRUCTIONS_ARB:                               u16 = 0x8805;
pub const GL_PROGRAM_ATTRIBS_ARB:                                        u16 = 0x88AC;
pub const GL_PROGRAM_BINARY_FORMATS:                                     u16 = 0x87FF;
pub const GL_PROGRAM_BINARY_LENGTH:                                      u16 = 0x8741;
pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT:                            u16 = 0x8257;
pub const GL_PROGRAM_BINDING_ARB:                                        u16 = 0x8677;
pub const GL_PROGRAM_ERROR_POSITION_ARB:                                 u16 = 0x864B;
pub const GL_PROGRAM_ERROR_STRING_ARB:                                   u16 = 0x8874;
pub const GL_PROGRAM_FORMAT_ARB:                                         u16 = 0x8876;
pub const GL_PROGRAM_FORMAT_ASCII_ARB:                                   u16 = 0x8875;
pub const GL_PROGRAM_INPUT:                                              u16 = 0x92E3;
pub const GL_PROGRAM_INSTRUCTIONS_ARB:                                   u16 = 0x88A0;
pub const GL_PROGRAM_LENGTH_ARB:                                         u16 = 0x8627;
pub const GL_PROGRAM_NATIVE_ADDRESS_REGISTERS_ARB:                       u16 = 0x88B2;
pub const GL_PROGRAM_NATIVE_ALU_INSTRUCTIONS_ARB:                        u16 = 0x8808;
pub const GL_PROGRAM_NATIVE_ATTRIBS_ARB:                                 u16 = 0x88AE;
pub const GL_PROGRAM_NATIVE_INSTRUCTIONS_ARB:                            u16 = 0x88A2;
pub const GL_PROGRAM_NATIVE_PARAMETERS_ARB:                              u16 = 0x88AA;
pub const GL_PROGRAM_NATIVE_TEMPORARIES_ARB:                             u16 = 0x88A6;
pub const GL_PROGRAM_NATIVE_TEX_INDIRECTIONS_ARB:                        u16 = 0x880A;
pub const GL_PROGRAM_NATIVE_TEX_INSTRUCTIONS_ARB:                        u16 = 0x8809;
pub const GL_PROGRAM_OBJECT_ARB:                                         u16 = 0x8B40;
pub const GL_PROGRAM_OUTPUT:                                             u16 = 0x92E4;
pub const GL_PROGRAM_PARAMETERS_ARB:                                     u16 = 0x88A8;
pub const GL_PROGRAM_PIPELINE:                                           u16 = 0x82E4;
pub const GL_PROGRAM_PIPELINE_BINDING:                                   u16 = 0x825A;
pub const GL_PROGRAM_POINT_SIZE:                                         u16 = 0x8642;
pub const GL_PROGRAM_POINT_SIZE_ARB:                                     u16 = 0x8642;
pub const GL_PROGRAM_SEPARABLE:                                          u16 = 0x8258;
pub const GL_PROGRAM_STRING_ARB:                                         u16 = 0x8628;
pub const GL_PROGRAM_TEMPORARIES_ARB:                                    u16 = 0x88A4;
pub const GL_PROGRAM_TEX_INDIRECTIONS_ARB:                               u16 = 0x8807;
pub const GL_PROGRAM_TEX_INSTRUCTIONS_ARB:                               u16 = 0x8806;
pub const GL_PROGRAM_UNDER_NATIVE_LIMITS_ARB:                            u16 = 0x88B6;
pub const GL_PROVOKING_VERTEX:                                           u16 = 0x8E4F;
pub const GL_PROXY_TEXTURE_1D:                                           u16 = 0x8063;
pub const GL_PROXY_TEXTURE_1D_ARRAY:                                     u16 = 0x8C19;
pub const GL_PROXY_TEXTURE_2D:                                           u16 = 0x8064;
pub const GL_PROXY_TEXTURE_2D_ARRAY:                                     u16 = 0x8C1B;
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE:                               u16 = 0x9101;
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY:                         u16 = 0x9103;
pub const GL_PROXY_TEXTURE_3D:                                           u16 = 0x8070;
pub const GL_PROXY_TEXTURE_CUBE_MAP:                                     u16 = 0x851B;
pub const GL_PROXY_TEXTURE_CUBE_MAP_ARB:                                 u16 = 0x851B;
pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY:                               u16 = 0x900B;
pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY_ARB:                           u16 = 0x900B;
pub const GL_PROXY_TEXTURE_RECTANGLE:                                    u16 = 0x84F7;
pub const GL_QUADS:                                                      u16 = 0x0007;
pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION:                   u16 = 0x8E4C;
pub const GL_QUERY:                                                      u16 = 0x82E3;
pub const GL_QUERY_BUFFER:                                               u16 = 0x9192;
pub const GL_QUERY_BUFFER_BARRIER_BIT:                                   u32 = 0x00008000;
pub const GL_QUERY_BUFFER_BINDING:                                       u16 = 0x9193;
pub const GL_QUERY_BY_REGION_NO_WAIT:                                    u16 = 0x8E16;
pub const GL_QUERY_BY_REGION_WAIT:                                       u16 = 0x8E15;
pub const GL_QUERY_COUNTER_BITS:                                         u16 = 0x8864;
pub const GL_QUERY_COUNTER_BITS_ARB:                                     u16 = 0x8864;
pub const GL_QUERY_NO_WAIT:                                              u16 = 0x8E14;
pub const GL_QUERY_RESULT:                                               u16 = 0x8866;
pub const GL_QUERY_RESULT_ARB:                                           u16 = 0x8866;
pub const GL_QUERY_RESULT_AVAILABLE:                                     u16 = 0x8867;
pub const GL_QUERY_RESULT_AVAILABLE_ARB:                                 u16 = 0x8867;
pub const GL_QUERY_RESULT_NO_WAIT:                                       u16 = 0x9194;
pub const GL_QUERY_TARGET:                                               u16 = 0x82EA;
pub const GL_QUERY_WAIT:                                                 u16 = 0x8E13;
pub const GL_R11F_G11F_B10F:                                             u16 = 0x8C3A;
pub const GL_R16:                                                        u16 = 0x822A;
pub const GL_R16F:                                                       u16 = 0x822D;
pub const GL_R16I:                                                       u16 = 0x8233;
pub const GL_R16UI:                                                      u16 = 0x8234;
pub const GL_R16_SNORM:                                                  u16 = 0x8F98;
pub const GL_R32F:                                                       u16 = 0x822E;
pub const GL_R32I:                                                       u16 = 0x8235;
pub const GL_R32UI:                                                      u16 = 0x8236;
pub const GL_R3_G3_B2:                                                   u16 = 0x2A10;
pub const GL_R8:                                                         u16 = 0x8229;
pub const GL_R8I:                                                        u16 = 0x8231;
pub const GL_R8UI:                                                       u16 = 0x8232;
pub const GL_R8_SNORM:                                                   u16 = 0x8F94;
pub const GL_RASTERIZER_DISCARD:                                         u16 = 0x8C89;
pub const GL_READ_BUFFER:                                                u16 = 0x0C02;
pub const GL_READ_FRAMEBUFFER:                                           u16 = 0x8CA8;
pub const GL_READ_FRAMEBUFFER_BINDING:                                   u16 = 0x8CAA;
pub const GL_READ_FRAMEBUFFER_BINDING_EXT:                               u16 = 0x8CAA;
pub const GL_READ_FRAMEBUFFER_EXT:                                       u16 = 0x8CA8;
pub const GL_READ_ONLY:                                                  u16 = 0x88B8;
pub const GL_READ_ONLY_ARB:                                              u16 = 0x88B8;
pub const GL_READ_PIXELS:                                                u16 = 0x828C;
pub const GL_READ_PIXELS_FORMAT:                                         u16 = 0x828D;
pub const GL_READ_PIXELS_TYPE:                                           u16 = 0x828E;
pub const GL_READ_WRITE:                                                 u16 = 0x88BA;
pub const GL_READ_WRITE_ARB:                                             u16 = 0x88BA;
pub const GL_RED:                                                        u16 = 0x1903;
pub const GL_RED_INTEGER:                                                u16 = 0x8D94;
pub const GL_REFERENCED_BY_COMPUTE_SHADER:                               u16 = 0x930B;
pub const GL_REFERENCED_BY_FRAGMENT_SHADER:                              u16 = 0x930A;
pub const GL_REFERENCED_BY_GEOMETRY_SHADER:                              u16 = 0x9309;
pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER:                          u16 = 0x9307;
pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER:                       u16 = 0x9308;
pub const GL_REFERENCED_BY_VERTEX_SHADER:                                u16 = 0x9306;
pub const GL_REFLECTION_MAP_ARB:                                         u16 = 0x8512;
pub const GL_RENDERBUFFER:                                               u16 = 0x8D41;
pub const GL_RENDERBUFFER_ALPHA_SIZE:                                    u16 = 0x8D53;
pub const GL_RENDERBUFFER_ALPHA_SIZE_EXT:                                u16 = 0x8D53;
pub const GL_RENDERBUFFER_BINDING:                                       u16 = 0x8CA7;
pub const GL_RENDERBUFFER_BINDING_EXT:                                   u16 = 0x8CA7;
pub const GL_RENDERBUFFER_BLUE_SIZE:                                     u16 = 0x8D52;
pub const GL_RENDERBUFFER_BLUE_SIZE_EXT:                                 u16 = 0x8D52;
pub const GL_RENDERBUFFER_DEPTH_SIZE:                                    u16 = 0x8D54;
pub const GL_RENDERBUFFER_DEPTH_SIZE_EXT:                                u16 = 0x8D54;
pub const GL_RENDERBUFFER_EXT:                                           u16 = 0x8D41;
pub const GL_RENDERBUFFER_GREEN_SIZE:                                    u16 = 0x8D51;
pub const GL_RENDERBUFFER_GREEN_SIZE_EXT:                                u16 = 0x8D51;
pub const GL_RENDERBUFFER_HEIGHT:                                        u16 = 0x8D43;
pub const GL_RENDERBUFFER_HEIGHT_EXT:                                    u16 = 0x8D43;
pub const GL_RENDERBUFFER_INTERNAL_FORMAT:                               u16 = 0x8D44;
pub const GL_RENDERBUFFER_INTERNAL_FORMAT_EXT:                           u16 = 0x8D44;
pub const GL_RENDERBUFFER_RED_SIZE:                                      u16 = 0x8D50;
pub const GL_RENDERBUFFER_RED_SIZE_EXT:                                  u16 = 0x8D50;
pub const GL_RENDERBUFFER_SAMPLES:                                       u16 = 0x8CAB;
pub const GL_RENDERBUFFER_SAMPLES_EXT:                                   u16 = 0x8CAB;
pub const GL_RENDERBUFFER_STENCIL_SIZE:                                  u16 = 0x8D55;
pub const GL_RENDERBUFFER_STENCIL_SIZE_EXT:                              u16 = 0x8D55;
pub const GL_RENDERBUFFER_WIDTH:                                         u16 = 0x8D42;
pub const GL_RENDERBUFFER_WIDTH_EXT:                                     u16 = 0x8D42;
pub const GL_RENDERER:                                                   u16 = 0x1F01;
pub const GL_REPEAT:                                                     u16 = 0x2901;
pub const GL_REPLACE:                                                    u16 = 0x1E01;
pub const GL_RG:                                                         u16 = 0x8227;
pub const GL_RG16:                                                       u16 = 0x822C;
pub const GL_RG16F:                                                      u16 = 0x822F;
pub const GL_RG16I:                                                      u16 = 0x8239;
pub const GL_RG16UI:                                                     u16 = 0x823A;
pub const GL_RG16_SNORM:                                                 u16 = 0x8F99;
pub const GL_RG32F:                                                      u16 = 0x8230;
pub const GL_RG32I:                                                      u16 = 0x823B;
pub const GL_RG32UI:                                                     u16 = 0x823C;
pub const GL_RG8:                                                        u16 = 0x822B;
pub const GL_RG8I:                                                       u16 = 0x8237;
pub const GL_RG8UI:                                                      u16 = 0x8238;
pub const GL_RG8_SNORM:                                                  u16 = 0x8F95;
pub const GL_RGB:                                                        u16 = 0x1907;
pub const GL_RGB10:                                                      u16 = 0x8052;
pub const GL_RGB10_A2:                                                   u16 = 0x8059;
pub const GL_RGB10_A2UI:                                                 u16 = 0x906F;
pub const GL_RGB12:                                                      u16 = 0x8053;
pub const GL_RGB16:                                                      u16 = 0x8054;
pub const GL_RGB16F:                                                     u16 = 0x881B;
pub const GL_RGB16F_ARB:                                                 u16 = 0x881B;
pub const GL_RGB16I:                                                     u16 = 0x8D89;
pub const GL_RGB16UI:                                                    u16 = 0x8D77;
pub const GL_RGB16_SNORM:                                                u16 = 0x8F9A;
pub const GL_RGB32F:                                                     u16 = 0x8815;
pub const GL_RGB32F_ARB:                                                 u16 = 0x8815;
pub const GL_RGB32I:                                                     u16 = 0x8D83;
pub const GL_RGB32UI:                                                    u16 = 0x8D71;
pub const GL_RGB4:                                                       u16 = 0x804F;
pub const GL_RGB5:                                                       u16 = 0x8050;
pub const GL_RGB565:                                                     u16 = 0x8D62;
pub const GL_RGB5_A1:                                                    u16 = 0x8057;
pub const GL_RGB8:                                                       u16 = 0x8051;
pub const GL_RGB8I:                                                      u16 = 0x8D8F;
pub const GL_RGB8UI:                                                     u16 = 0x8D7D;
pub const GL_RGB8_SNORM:                                                 u16 = 0x8F96;
pub const GL_RGB9_E5:                                                    u16 = 0x8C3D;
pub const GL_RGBA:                                                       u16 = 0x1908;
pub const GL_RGBA12:                                                     u16 = 0x805A;
pub const GL_RGBA16:                                                     u16 = 0x805B;
pub const GL_RGBA16F:                                                    u16 = 0x881A;
pub const GL_RGBA16F_ARB:                                                u16 = 0x881A;
pub const GL_RGBA16I:                                                    u16 = 0x8D88;
pub const GL_RGBA16UI:                                                   u16 = 0x8D76;
pub const GL_RGBA16_SNORM:                                               u16 = 0x8F9B;
pub const GL_RGBA2:                                                      u16 = 0x8055;
pub const GL_RGBA32F:                                                    u16 = 0x8814;
pub const GL_RGBA32F_ARB:                                                u16 = 0x8814;
pub const GL_RGBA32I:                                                    u16 = 0x8D82;
pub const GL_RGBA32UI:                                                   u16 = 0x8D70;
pub const GL_RGBA4:                                                      u16 = 0x8056;
pub const GL_RGBA8:                                                      u16 = 0x8058;
pub const GL_RGBA8I:                                                     u16 = 0x8D8E;
pub const GL_RGBA8UI:                                                    u16 = 0x8D7C;
pub const GL_RGBA8_SNORM:                                                u16 = 0x8F97;
pub const GL_RGBA_FLOAT_MODE_ARB:                                        u16 = 0x8820;
pub const GL_RGBA_INTEGER:                                               u16 = 0x8D99;
pub const GL_RGB_INTEGER:                                                u16 = 0x8D98;
pub const GL_RG_INTEGER:                                                 u16 = 0x8228;
pub const GL_RIGHT:                                                      u16 = 0x0407;
pub const GL_SAMPLER:                                                    u16 = 0x82E6;
pub const GL_SAMPLER_1D:                                                 u16 = 0x8B5D;
pub const GL_SAMPLER_1D_ARB:                                             u16 = 0x8B5D;
pub const GL_SAMPLER_1D_ARRAY:                                           u16 = 0x8DC0;
pub const GL_SAMPLER_1D_ARRAY_SHADOW:                                    u16 = 0x8DC3;
pub const GL_SAMPLER_1D_SHADOW:                                          u16 = 0x8B61;
pub const GL_SAMPLER_1D_SHADOW_ARB:                                      u16 = 0x8B61;
pub const GL_SAMPLER_2D:                                                 u16 = 0x8B5E;
pub const GL_SAMPLER_2D_ARB:                                             u16 = 0x8B5E;
pub const GL_SAMPLER_2D_ARRAY:                                           u16 = 0x8DC1;
pub const GL_SAMPLER_2D_ARRAY_SHADOW:                                    u16 = 0x8DC4;
pub const GL_SAMPLER_2D_MULTISAMPLE:                                     u16 = 0x9108;
pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY:                               u16 = 0x910B;
pub const GL_SAMPLER_2D_RECT:                                            u16 = 0x8B63;
pub const GL_SAMPLER_2D_RECT_ARB:                                        u16 = 0x8B63;
pub const GL_SAMPLER_2D_RECT_SHADOW:                                     u16 = 0x8B64;
pub const GL_SAMPLER_2D_RECT_SHADOW_ARB:                                 u16 = 0x8B64;
pub const GL_SAMPLER_2D_SHADOW:                                          u16 = 0x8B62;
pub const GL_SAMPLER_2D_SHADOW_ARB:                                      u16 = 0x8B62;
pub const GL_SAMPLER_3D:                                                 u16 = 0x8B5F;
pub const GL_SAMPLER_3D_ARB:                                             u16 = 0x8B5F;
pub const GL_SAMPLER_BINDING:                                            u16 = 0x8919;
pub const GL_SAMPLER_BUFFER:                                             u16 = 0x8DC2;
pub const GL_SAMPLER_CUBE:                                               u16 = 0x8B60;
pub const GL_SAMPLER_CUBE_ARB:                                           u16 = 0x8B60;
pub const GL_SAMPLER_CUBE_MAP_ARRAY:                                     u16 = 0x900C;
pub const GL_SAMPLER_CUBE_MAP_ARRAY_ARB:                                 u16 = 0x900C;
pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW:                              u16 = 0x900D;
pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW_ARB:                          u16 = 0x900D;
pub const GL_SAMPLER_CUBE_SHADOW:                                        u16 = 0x8DC5;
pub const GL_SAMPLES:                                                    u16 = 0x80A9;
pub const GL_SAMPLES_ARB:                                                u16 = 0x80A9;
pub const GL_SAMPLES_PASSED:                                             u16 = 0x8914;
pub const GL_SAMPLES_PASSED_ARB:                                         u16 = 0x8914;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE:                                   u16 = 0x809E;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE_ARB:                               u16 = 0x809E;
pub const GL_SAMPLE_ALPHA_TO_ONE:                                        u16 = 0x809F;
pub const GL_SAMPLE_ALPHA_TO_ONE_ARB:                                    u16 = 0x809F;
pub const GL_SAMPLE_BUFFERS:                                             u16 = 0x80A8;
pub const GL_SAMPLE_BUFFERS_ARB:                                         u16 = 0x80A8;
pub const GL_SAMPLE_COVERAGE:                                            u16 = 0x80A0;
pub const GL_SAMPLE_COVERAGE_ARB:                                        u16 = 0x80A0;
pub const GL_SAMPLE_COVERAGE_INVERT:                                     u16 = 0x80AB;
pub const GL_SAMPLE_COVERAGE_INVERT_ARB:                                 u16 = 0x80AB;
pub const GL_SAMPLE_COVERAGE_VALUE:                                      u16 = 0x80AA;
pub const GL_SAMPLE_COVERAGE_VALUE_ARB:                                  u16 = 0x80AA;
pub const GL_SAMPLE_LOCATION_ARB:                                        u16 = 0x8E50;
pub const GL_SAMPLE_LOCATION_PIXEL_GRID_HEIGHT_ARB:                      u16 = 0x933F;
pub const GL_SAMPLE_LOCATION_PIXEL_GRID_WIDTH_ARB:                       u16 = 0x933E;
pub const GL_SAMPLE_LOCATION_SUBPIXEL_BITS_ARB:                          u16 = 0x933D;
pub const GL_SAMPLE_MASK:                                                u16 = 0x8E51;
pub const GL_SAMPLE_MASK_VALUE:                                          u16 = 0x8E52;
pub const GL_SAMPLE_POSITION:                                            u16 = 0x8E50;
pub const GL_SAMPLE_SHADING:                                             u16 = 0x8C36;
pub const GL_SAMPLE_SHADING_ARB:                                         u16 = 0x8C36;
pub const GL_SCISSOR_BOX:                                                u16 = 0x0C10;
pub const GL_SCISSOR_TEST:                                               u16 = 0x0C11;
pub const GL_SECONDARY_COLOR_ARRAY_BUFFER_BINDING_ARB:                   u16 = 0x889C;
pub const GL_SEPARATE_ATTRIBS:                                           u16 = 0x8C8D;
pub const GL_SET:                                                        u16 = 0x150F;
pub const GL_SHADER:                                                     u16 = 0x82E1;
pub const GL_SHADER_BINARY_FORMATS:                                      u16 = 0x8DF8;
pub const GL_SHADER_BINARY_FORMAT_SPIR_V:                                u16 = 0x9551;
pub const GL_SHADER_BINARY_FORMAT_SPIR_V_ARB:                            u16 = 0x9551;
pub const GL_SHADER_COMPILER:                                            u16 = 0x8DFA;
pub const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT:                            u32 = 0x00000020;
pub const GL_SHADER_IMAGE_ATOMIC:                                        u16 = 0x82A6;
pub const GL_SHADER_IMAGE_LOAD:                                          u16 = 0x82A4;
pub const GL_SHADER_IMAGE_STORE:                                         u16 = 0x82A5;
pub const GL_SHADER_INCLUDE_ARB:                                         u16 = 0x8DAE;
pub const GL_SHADER_OBJECT_ARB:                                          u16 = 0x8B48;
pub const GL_SHADER_SOURCE_LENGTH:                                       u16 = 0x8B88;
pub const GL_SHADER_STORAGE_BARRIER_BIT:                                 u32 = 0x00002000;
pub const GL_SHADER_STORAGE_BLOCK:                                       u16 = 0x92E6;
pub const GL_SHADER_STORAGE_BUFFER:                                      u16 = 0x90D2;
pub const GL_SHADER_STORAGE_BUFFER_BINDING:                              u16 = 0x90D3;
pub const GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT:                     u16 = 0x90DF;
pub const GL_SHADER_STORAGE_BUFFER_SIZE:                                 u16 = 0x90D5;
pub const GL_SHADER_STORAGE_BUFFER_START:                                u16 = 0x90D4;
pub const GL_SHADER_TYPE:                                                u16 = 0x8B4F;
pub const GL_SHADING_LANGUAGE_VERSION:                                   u16 = 0x8B8C;
pub const GL_SHADING_LANGUAGE_VERSION_ARB:                               u16 = 0x8B8C;
pub const GL_SHORT:                                                      u16 = 0x1402;
pub const GL_SIGNALED:                                                   u16 = 0x9119;
pub const GL_SIGNED_NORMALIZED:                                          u16 = 0x8F9C;
pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST:                        u16 = 0x82AC;
pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE:                       u16 = 0x82AE;
pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST:                      u16 = 0x82AD;
pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE:                     u16 = 0x82AF;
pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY:                              u16 = 0x0B23;
pub const GL_SMOOTH_LINE_WIDTH_RANGE:                                    u16 = 0x0B22;
pub const GL_SMOOTH_POINT_SIZE_GRANULARITY:                              u16 = 0x0B13;
pub const GL_SMOOTH_POINT_SIZE_RANGE:                                    u16 = 0x0B12;
pub const GL_SOURCE1_ALPHA:                                              u16 = 0x8589;
pub const GL_SPIR_V_BINARY:                                              u16 = 0x9552;
pub const GL_SPIR_V_BINARY_ARB:                                          u16 = 0x9552;
pub const GL_SPIR_V_EXTENSIONS:                                          u16 = 0x9553;
pub const GL_SRC1_ALPHA:                                                 u16 = 0x8589;
pub const GL_SRC1_COLOR:                                                 u16 = 0x88F9;
pub const GL_SRC_ALPHA:                                                  u16 = 0x0302;
pub const GL_SRC_ALPHA_SATURATE:                                         u16 = 0x0308;
pub const GL_SRC_COLOR:                                                  u16 = 0x0300;
pub const GL_SRGB:                                                       u16 = 0x8C40;
pub const GL_SRGB8:                                                      u16 = 0x8C41;
pub const GL_SRGB8_ALPHA8:                                               u16 = 0x8C43;
pub const GL_SRGB_ALPHA:                                                 u16 = 0x8C42;
pub const GL_SRGB_DECODE_ARB:                                            u16 = 0x8299;
pub const GL_SRGB_READ:                                                  u16 = 0x8297;
pub const GL_SRGB_WRITE:                                                 u16 = 0x8298;
pub const GL_STACK_OVERFLOW:                                             u16 = 0x0503;
pub const GL_STACK_UNDERFLOW:                                            u16 = 0x0504;
pub const GL_STATIC_COPY:                                                u16 = 0x88E6;
pub const GL_STATIC_COPY_ARB:                                            u16 = 0x88E6;
pub const GL_STATIC_DRAW:                                                u16 = 0x88E4;
pub const GL_STATIC_DRAW_ARB:                                            u16 = 0x88E4;
pub const GL_STATIC_READ:                                                u16 = 0x88E5;
pub const GL_STATIC_READ_ARB:                                            u16 = 0x88E5;
pub const GL_STENCIL:                                                    u16 = 0x1802;
pub const GL_STENCIL_ATTACHMENT:                                         u16 = 0x8D20;
pub const GL_STENCIL_ATTACHMENT_EXT:                                     u16 = 0x8D20;
pub const GL_STENCIL_BACK_FAIL:                                          u16 = 0x8801;
pub const GL_STENCIL_BACK_FUNC:                                          u16 = 0x8800;
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL:                               u16 = 0x8802;
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS:                               u16 = 0x8803;
pub const GL_STENCIL_BACK_REF:                                           u16 = 0x8CA3;
pub const GL_STENCIL_BACK_VALUE_MASK:                                    u16 = 0x8CA4;
pub const GL_STENCIL_BACK_WRITEMASK:                                     u16 = 0x8CA5;
pub const GL_STENCIL_BUFFER_BIT:                                         u32 = 0x00000400;
pub const GL_STENCIL_CLEAR_VALUE:                                        u16 = 0x0B91;
pub const GL_STENCIL_COMPONENTS:                                         u16 = 0x8285;
pub const GL_STENCIL_FAIL:                                               u16 = 0x0B94;
pub const GL_STENCIL_FUNC:                                               u16 = 0x0B92;
pub const GL_STENCIL_INDEX:                                              u16 = 0x1901;
pub const GL_STENCIL_INDEX1:                                             u16 = 0x8D46;
pub const GL_STENCIL_INDEX16:                                            u16 = 0x8D49;
pub const GL_STENCIL_INDEX16_EXT:                                        u16 = 0x8D49;
pub const GL_STENCIL_INDEX1_EXT:                                         u16 = 0x8D46;
pub const GL_STENCIL_INDEX4:                                             u16 = 0x8D47;
pub const GL_STENCIL_INDEX4_EXT:                                         u16 = 0x8D47;
pub const GL_STENCIL_INDEX8:                                             u16 = 0x8D48;
pub const GL_STENCIL_INDEX8_EXT:                                         u16 = 0x8D48;
pub const GL_STENCIL_PASS_DEPTH_FAIL:                                    u16 = 0x0B95;
pub const GL_STENCIL_PASS_DEPTH_PASS:                                    u16 = 0x0B96;
pub const GL_STENCIL_REF:                                                u16 = 0x0B97;
pub const GL_STENCIL_RENDERABLE:                                         u16 = 0x8288;
pub const GL_STENCIL_TEST:                                               u16 = 0x0B90;
pub const GL_STENCIL_VALUE_MASK:                                         u16 = 0x0B93;
pub const GL_STENCIL_WRITEMASK:                                          u16 = 0x0B98;
pub const GL_STEREO:                                                     u16 = 0x0C33;
pub const GL_STREAM_COPY:                                                u16 = 0x88E2;
pub const GL_STREAM_COPY_ARB:                                            u16 = 0x88E2;
pub const GL_STREAM_DRAW:                                                u16 = 0x88E0;
pub const GL_STREAM_DRAW_ARB:                                            u16 = 0x88E0;
pub const GL_STREAM_READ:                                                u16 = 0x88E1;
pub const GL_STREAM_READ_ARB:                                            u16 = 0x88E1;
pub const GL_SUBPIXEL_BITS:                                              u16 = 0x0D50;
pub const GL_SYNC_CONDITION:                                             u16 = 0x9113;
pub const GL_SYNC_FENCE:                                                 u16 = 0x9116;
pub const GL_SYNC_FLAGS:                                                 u16 = 0x9115;
pub const GL_SYNC_FLUSH_COMMANDS_BIT:                                    u32 = 0x00000001;
pub const GL_SYNC_GPU_COMMANDS_COMPLETE:                                 u16 = 0x9117;
pub const GL_SYNC_STATUS:                                                u16 = 0x9114;
pub const GL_TESS_CONTROL_OUTPUT_VERTICES:                               u16 = 0x8E75;
pub const GL_TESS_CONTROL_SHADER:                                        u16 = 0x8E88;
pub const GL_TESS_CONTROL_SHADER_BIT:                                    u32 = 0x00000008;
pub const GL_TESS_CONTROL_SHADER_PATCHES:                                u16 = 0x82F1;
pub const GL_TESS_CONTROL_SHADER_PATCHES_ARB:                            u16 = 0x82F1;
pub const GL_TESS_CONTROL_SUBROUTINE:                                    u16 = 0x92E9;
pub const GL_TESS_CONTROL_SUBROUTINE_UNIFORM:                            u16 = 0x92EF;
pub const GL_TESS_CONTROL_TEXTURE:                                       u16 = 0x829C;
pub const GL_TESS_EVALUATION_SHADER:                                     u16 = 0x8E87;
pub const GL_TESS_EVALUATION_SHADER_BIT:                                 u32 = 0x00000010;
pub const GL_TESS_EVALUATION_SHADER_INVOCATIONS:                         u16 = 0x82F2;
pub const GL_TESS_EVALUATION_SHADER_INVOCATIONS_ARB:                     u16 = 0x82F2;
pub const GL_TESS_EVALUATION_SUBROUTINE:                                 u16 = 0x92EA;
pub const GL_TESS_EVALUATION_SUBROUTINE_UNIFORM:                         u16 = 0x92F0;
pub const GL_TESS_EVALUATION_TEXTURE:                                    u16 = 0x829D;
pub const GL_TESS_GEN_MODE:                                              u16 = 0x8E76;
pub const GL_TESS_GEN_POINT_MODE:                                        u16 = 0x8E79;
pub const GL_TESS_GEN_SPACING:                                           u16 = 0x8E77;
pub const GL_TESS_GEN_VERTEX_ORDER:                                      u16 = 0x8E78;
pub const GL_TEXTURE:                                                    u16 = 0x1702;
pub const GL_TEXTURE0:                                                   u16 = 0x84C0;
pub const GL_TEXTURE0_ARB:                                               u16 = 0x84C0;
pub const GL_TEXTURE1:                                                   u16 = 0x84C1;
pub const GL_TEXTURE10:                                                  u16 = 0x84CA;
pub const GL_TEXTURE10_ARB:                                              u16 = 0x84CA;
pub const GL_TEXTURE11:                                                  u16 = 0x84CB;
pub const GL_TEXTURE11_ARB:                                              u16 = 0x84CB;
pub const GL_TEXTURE12:                                                  u16 = 0x84CC;
pub const GL_TEXTURE12_ARB:                                              u16 = 0x84CC;
pub const GL_TEXTURE13:                                                  u16 = 0x84CD;
pub const GL_TEXTURE13_ARB:                                              u16 = 0x84CD;
pub const GL_TEXTURE14:                                                  u16 = 0x84CE;
pub const GL_TEXTURE14_ARB:                                              u16 = 0x84CE;
pub const GL_TEXTURE15:                                                  u16 = 0x84CF;
pub const GL_TEXTURE15_ARB:                                              u16 = 0x84CF;
pub const GL_TEXTURE16:                                                  u16 = 0x84D0;
pub const GL_TEXTURE16_ARB:                                              u16 = 0x84D0;
pub const GL_TEXTURE17:                                                  u16 = 0x84D1;
pub const GL_TEXTURE17_ARB:                                              u16 = 0x84D1;
pub const GL_TEXTURE18:                                                  u16 = 0x84D2;
pub const GL_TEXTURE18_ARB:                                              u16 = 0x84D2;
pub const GL_TEXTURE19:                                                  u16 = 0x84D3;
pub const GL_TEXTURE19_ARB:                                              u16 = 0x84D3;
pub const GL_TEXTURE1_ARB:                                               u16 = 0x84C1;
pub const GL_TEXTURE2:                                                   u16 = 0x84C2;
pub const GL_TEXTURE20:                                                  u16 = 0x84D4;
pub const GL_TEXTURE20_ARB:                                              u16 = 0x84D4;
pub const GL_TEXTURE21:                                                  u16 = 0x84D5;
pub const GL_TEXTURE21_ARB:                                              u16 = 0x84D5;
pub const GL_TEXTURE22:                                                  u16 = 0x84D6;
pub const GL_TEXTURE22_ARB:                                              u16 = 0x84D6;
pub const GL_TEXTURE23:                                                  u16 = 0x84D7;
pub const GL_TEXTURE23_ARB:                                              u16 = 0x84D7;
pub const GL_TEXTURE24:                                                  u16 = 0x84D8;
pub const GL_TEXTURE24_ARB:                                              u16 = 0x84D8;
pub const GL_TEXTURE25:                                                  u16 = 0x84D9;
pub const GL_TEXTURE25_ARB:                                              u16 = 0x84D9;
pub const GL_TEXTURE26:                                                  u16 = 0x84DA;
pub const GL_TEXTURE26_ARB:                                              u16 = 0x84DA;
pub const GL_TEXTURE27:                                                  u16 = 0x84DB;
pub const GL_TEXTURE27_ARB:                                              u16 = 0x84DB;
pub const GL_TEXTURE28:                                                  u16 = 0x84DC;
pub const GL_TEXTURE28_ARB:                                              u16 = 0x84DC;
pub const GL_TEXTURE29:                                                  u16 = 0x84DD;
pub const GL_TEXTURE29_ARB:                                              u16 = 0x84DD;
pub const GL_TEXTURE2_ARB:                                               u16 = 0x84C2;
pub const GL_TEXTURE3:                                                   u16 = 0x84C3;
pub const GL_TEXTURE30:                                                  u16 = 0x84DE;
pub const GL_TEXTURE30_ARB:                                              u16 = 0x84DE;
pub const GL_TEXTURE31:                                                  u16 = 0x84DF;
pub const GL_TEXTURE31_ARB:                                              u16 = 0x84DF;
pub const GL_TEXTURE3_ARB:                                               u16 = 0x84C3;
pub const GL_TEXTURE4:                                                   u16 = 0x84C4;
pub const GL_TEXTURE4_ARB:                                               u16 = 0x84C4;
pub const GL_TEXTURE5:                                                   u16 = 0x84C5;
pub const GL_TEXTURE5_ARB:                                               u16 = 0x84C5;
pub const GL_TEXTURE6:                                                   u16 = 0x84C6;
pub const GL_TEXTURE6_ARB:                                               u16 = 0x84C6;
pub const GL_TEXTURE7:                                                   u16 = 0x84C7;
pub const GL_TEXTURE7_ARB:                                               u16 = 0x84C7;
pub const GL_TEXTURE8:                                                   u16 = 0x84C8;
pub const GL_TEXTURE8_ARB:                                               u16 = 0x84C8;
pub const GL_TEXTURE9:                                                   u16 = 0x84C9;
pub const GL_TEXTURE9_ARB:                                               u16 = 0x84C9;
pub const GL_TEXTURE_1D:                                                 u16 = 0x0DE0;
pub const GL_TEXTURE_1D_ARRAY:                                           u16 = 0x8C18;
pub const GL_TEXTURE_2D:                                                 u16 = 0x0DE1;
pub const GL_TEXTURE_2D_ARRAY:                                           u16 = 0x8C1A;
pub const GL_TEXTURE_2D_MULTISAMPLE:                                     u16 = 0x9100;
pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY:                               u16 = 0x9102;
pub const GL_TEXTURE_3D:                                                 u16 = 0x806F;
pub const GL_TEXTURE_ALPHA_SIZE:                                         u16 = 0x805F;
pub const GL_TEXTURE_ALPHA_TYPE:                                         u16 = 0x8C13;
pub const GL_TEXTURE_ALPHA_TYPE_ARB:                                     u16 = 0x8C13;
pub const GL_TEXTURE_BASE_LEVEL:                                         u16 = 0x813C;
pub const GL_TEXTURE_BINDING_1D:                                         u16 = 0x8068;
pub const GL_TEXTURE_BINDING_1D_ARRAY:                                   u16 = 0x8C1C;
pub const GL_TEXTURE_BINDING_2D:                                         u16 = 0x8069;
pub const GL_TEXTURE_BINDING_2D_ARRAY:                                   u16 = 0x8C1D;
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE:                             u16 = 0x9104;
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY:                       u16 = 0x9105;
pub const GL_TEXTURE_BINDING_3D:                                         u16 = 0x806A;
pub const GL_TEXTURE_BINDING_BUFFER:                                     u16 = 0x8C2C;
pub const GL_TEXTURE_BINDING_CUBE_MAP:                                   u16 = 0x8514;
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARB:                               u16 = 0x8514;
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY:                             u16 = 0x900A;
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY_ARB:                         u16 = 0x900A;
pub const GL_TEXTURE_BINDING_RECTANGLE:                                  u16 = 0x84F6;
pub const GL_TEXTURE_BLUE_SIZE:                                          u16 = 0x805E;
pub const GL_TEXTURE_BLUE_TYPE:                                          u16 = 0x8C12;
pub const GL_TEXTURE_BLUE_TYPE_ARB:                                      u16 = 0x8C12;
pub const GL_TEXTURE_BORDER_COLOR:                                       u16 = 0x1004;
pub const GL_TEXTURE_BUFFER:                                             u16 = 0x8C2A;
pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING:                          u16 = 0x8C2D;
pub const GL_TEXTURE_BUFFER_OFFSET:                                      u16 = 0x919D;
pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT:                            u16 = 0x919F;
pub const GL_TEXTURE_BUFFER_SIZE:                                        u16 = 0x919E;
pub const GL_TEXTURE_COMPARE_FUNC:                                       u16 = 0x884D;
pub const GL_TEXTURE_COMPARE_MODE:                                       u16 = 0x884C;
pub const GL_TEXTURE_COMPRESSED:                                         u16 = 0x86A1;
pub const GL_TEXTURE_COMPRESSED_ARB:                                     u16 = 0x86A1;
pub const GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT:                            u16 = 0x82B2;
pub const GL_TEXTURE_COMPRESSED_BLOCK_SIZE:                              u16 = 0x82B3;
pub const GL_TEXTURE_COMPRESSED_BLOCK_WIDTH:                             u16 = 0x82B1;
pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE:                              u16 = 0x86A0;
pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE_ARB:                          u16 = 0x86A0;
pub const GL_TEXTURE_COMPRESSION_HINT:                                   u16 = 0x84EF;
pub const GL_TEXTURE_COMPRESSION_HINT_ARB:                               u16 = 0x84EF;
pub const GL_TEXTURE_COORD_ARRAY_BUFFER_BINDING_ARB:                     u16 = 0x889A;
pub const GL_TEXTURE_CUBE_MAP:                                           u16 = 0x8513;
pub const GL_TEXTURE_CUBE_MAP_ARB:                                       u16 = 0x8513;
pub const GL_TEXTURE_CUBE_MAP_ARRAY:                                     u16 = 0x9009;
pub const GL_TEXTURE_CUBE_MAP_ARRAY_ARB:                                 u16 = 0x9009;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X:                                u16 = 0x8516;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X_ARB:                            u16 = 0x8516;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y:                                u16 = 0x8518;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y_ARB:                            u16 = 0x8518;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z:                                u16 = 0x851A;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z_ARB:                            u16 = 0x851A;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X:                                u16 = 0x8515;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X_ARB:                            u16 = 0x8515;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y:                                u16 = 0x8517;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y_ARB:                            u16 = 0x8517;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z:                                u16 = 0x8519;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z_ARB:                            u16 = 0x8519;
pub const GL_TEXTURE_CUBE_MAP_SEAMLESS:                                  u16 = 0x884F;
pub const GL_TEXTURE_DEPTH:                                              u16 = 0x8071;
pub const GL_TEXTURE_DEPTH_SIZE:                                         u16 = 0x884A;
pub const GL_TEXTURE_DEPTH_SIZE_ARB:                                     u16 = 0x884A;
pub const GL_TEXTURE_DEPTH_TYPE:                                         u16 = 0x8C16;
pub const GL_TEXTURE_DEPTH_TYPE_ARB:                                     u16 = 0x8C16;
pub const GL_TEXTURE_FETCH_BARRIER_BIT:                                  u32 = 0x00000008;
pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS:                             u16 = 0x9107;
pub const GL_TEXTURE_GATHER:                                             u16 = 0x82A2;
pub const GL_TEXTURE_GATHER_SHADOW:                                      u16 = 0x82A3;
pub const GL_TEXTURE_GREEN_SIZE:                                         u16 = 0x805D;
pub const GL_TEXTURE_GREEN_TYPE:                                         u16 = 0x8C11;
pub const GL_TEXTURE_GREEN_TYPE_ARB:                                     u16 = 0x8C11;
pub const GL_TEXTURE_HEIGHT:                                             u16 = 0x1001;
pub const GL_TEXTURE_IMAGE_FORMAT:                                       u16 = 0x828F;
pub const GL_TEXTURE_IMAGE_TYPE:                                         u16 = 0x8290;
pub const GL_TEXTURE_IMMUTABLE_FORMAT:                                   u16 = 0x912F;
pub const GL_TEXTURE_IMMUTABLE_LEVELS:                                   u16 = 0x82DF;
pub const GL_TEXTURE_INTENSITY_TYPE_ARB:                                 u16 = 0x8C15;
pub const GL_TEXTURE_INTERNAL_FORMAT:                                    u16 = 0x1003;
pub const GL_TEXTURE_LOD_BIAS:                                           u16 = 0x8501;
pub const GL_TEXTURE_LUMINANCE_TYPE_ARB:                                 u16 = 0x8C14;
pub const GL_TEXTURE_MAG_FILTER:                                         u16 = 0x2800;
pub const GL_TEXTURE_MAX_ANISOTROPY:                                     u16 = 0x84FE;
pub const GL_TEXTURE_MAX_ANISOTROPY_EXT:                                 u16 = 0x84FE;
pub const GL_TEXTURE_MAX_LEVEL:                                          u16 = 0x813D;
pub const GL_TEXTURE_MAX_LOD:                                            u16 = 0x813B;
pub const GL_TEXTURE_MIN_FILTER:                                         u16 = 0x2801;
pub const GL_TEXTURE_MIN_LOD:                                            u16 = 0x813A;
pub const GL_TEXTURE_RECTANGLE:                                          u16 = 0x84F5;
pub const GL_TEXTURE_REDUCTION_MODE_ARB:                                 u16 = 0x9366;
pub const GL_TEXTURE_RED_SIZE:                                           u16 = 0x805C;
pub const GL_TEXTURE_RED_TYPE:                                           u16 = 0x8C10;
pub const GL_TEXTURE_RED_TYPE_ARB:                                       u16 = 0x8C10;
pub const GL_TEXTURE_SAMPLES:                                            u16 = 0x9106;
pub const GL_TEXTURE_SHADOW:                                             u16 = 0x82A1;
pub const GL_TEXTURE_SHARED_SIZE:                                        u16 = 0x8C3F;
pub const GL_TEXTURE_STENCIL_SIZE:                                       u16 = 0x88F1;
pub const GL_TEXTURE_SWIZZLE_A:                                          u16 = 0x8E45;
pub const GL_TEXTURE_SWIZZLE_B:                                          u16 = 0x8E44;
pub const GL_TEXTURE_SWIZZLE_G:                                          u16 = 0x8E43;
pub const GL_TEXTURE_SWIZZLE_R:                                          u16 = 0x8E42;
pub const GL_TEXTURE_SWIZZLE_RGBA:                                       u16 = 0x8E46;
pub const GL_TEXTURE_TARGET:                                             u16 = 0x1006;
pub const GL_TEXTURE_UPDATE_BARRIER_BIT:                                 u32 = 0x00000100;
pub const GL_TEXTURE_VIEW:                                               u16 = 0x82B5;
pub const GL_TEXTURE_VIEW_MIN_LAYER:                                     u16 = 0x82DD;
pub const GL_TEXTURE_VIEW_MIN_LEVEL:                                     u16 = 0x82DB;
pub const GL_TEXTURE_VIEW_NUM_LAYERS:                                    u16 = 0x82DE;
pub const GL_TEXTURE_VIEW_NUM_LEVELS:                                    u16 = 0x82DC;
pub const GL_TEXTURE_WIDTH:                                              u16 = 0x1000;
pub const GL_TEXTURE_WRAP_R:                                             u16 = 0x8072;
pub const GL_TEXTURE_WRAP_S:                                             u16 = 0x2802;
pub const GL_TEXTURE_WRAP_T:                                             u16 = 0x2803;
pub const GL_TIMEOUT_EXPIRED:                                            u64 = 0x911B;
pub const GL_TIMEOUT_IGNORED:                                            u64 = 0xFFFFFFFFFFFFFFFF;
pub const GL_TIMESTAMP:                                                  u16 = 0x8E28;
pub const GL_TIME_ELAPSED:                                               u16 = 0x88BF;
pub const GL_TOP_LEVEL_ARRAY_SIZE:                                       u16 = 0x930C;
pub const GL_TOP_LEVEL_ARRAY_STRIDE:                                     u16 = 0x930D;
pub const GL_TRANSFORM_FEEDBACK:                                         u16 = 0x8E22;
pub const GL_TRANSFORM_FEEDBACK_ACTIVE:                                  u16 = 0x8E24;
pub const GL_TRANSFORM_FEEDBACK_BARRIER_BIT:                             u32 = 0x00000800;
pub const GL_TRANSFORM_FEEDBACK_BINDING:                                 u16 = 0x8E25;
pub const GL_TRANSFORM_FEEDBACK_BUFFER:                                  u16 = 0x8C8E;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE:                           u16 = 0x8E24;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING:                          u16 = 0x8C8F;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_INDEX:                            u16 = 0x934B;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE:                             u16 = 0x8C7F;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED:                           u16 = 0x8E23;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE:                             u16 = 0x8C85;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_START:                            u16 = 0x8C84;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_STRIDE:                           u16 = 0x934C;
pub const GL_TRANSFORM_FEEDBACK_PAUSED:                                  u16 = 0x8E23;
pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN:                      u16 = 0x8C88;
pub const GL_TRANSFORM_FEEDBACK_VARYING:                                 u16 = 0x92F4;
pub const GL_TRANSFORM_FEEDBACK_VARYINGS:                                u16 = 0x8C83;
pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH:                      u16 = 0x8C76;
pub const GL_TRANSPOSE_COLOR_MATRIX_ARB:                                 u16 = 0x84E6;
pub const GL_TRANSPOSE_CURRENT_MATRIX_ARB:                               u16 = 0x88B7;
pub const GL_TRANSPOSE_MODELVIEW_MATRIX_ARB:                             u16 = 0x84E3;
pub const GL_TRANSPOSE_PROJECTION_MATRIX_ARB:                            u16 = 0x84E4;
pub const GL_TRANSPOSE_TEXTURE_MATRIX_ARB:                               u16 = 0x84E5;
pub const GL_TRIANGLES:                                                  u16 = 0x0004;
pub const GL_TRIANGLES_ADJACENCY:                                        u16 = 0x000C;
pub const GL_TRIANGLES_ADJACENCY_ARB:                                    u16 = 0x000C;
pub const GL_TRIANGLE_FAN:                                               u16 = 0x0006;
pub const GL_TRIANGLE_STRIP:                                             u16 = 0x0005;
pub const GL_TRIANGLE_STRIP_ADJACENCY:                                   u16 = 0x000D;
pub const GL_TRIANGLE_STRIP_ADJACENCY_ARB:                               u16 = 0x000D;
pub const GL_TRUE:                                                       u8  = 1;
pub const GL_TYPE:                                                       u16 = 0x92FA;
pub const GL_UNDEFINED_VERTEX:                                           u16 = 0x8260;
pub const GL_UNIFORM:                                                    u16 = 0x92E1;
pub const GL_UNIFORM_ARRAY_STRIDE:                                       u16 = 0x8A3C;
pub const GL_UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX:                        u16 = 0x92DA;
pub const GL_UNIFORM_BARRIER_BIT:                                        u32 = 0x00000004;
pub const GL_UNIFORM_BLOCK:                                              u16 = 0x92E2;
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS:                              u16 = 0x8A42;
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES:                       u16 = 0x8A43;
pub const GL_UNIFORM_BLOCK_BINDING:                                      u16 = 0x8A3F;
pub const GL_UNIFORM_BLOCK_DATA_SIZE:                                    u16 = 0x8A40;
pub const GL_UNIFORM_BLOCK_INDEX:                                        u16 = 0x8A3A;
pub const GL_UNIFORM_BLOCK_NAME_LENGTH:                                  u16 = 0x8A41;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER:                 u16 = 0x90EC;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER:                u16 = 0x8A46;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER:                u16 = 0x8A45;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER:            u16 = 0x84F0;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER:         u16 = 0x84F1;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER:                  u16 = 0x8A44;
pub const GL_UNIFORM_BUFFER:                                             u16 = 0x8A11;
pub const GL_UNIFORM_BUFFER_BINDING:                                     u16 = 0x8A28;
pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT:                            u16 = 0x8A34;
pub const GL_UNIFORM_BUFFER_SIZE:                                        u16 = 0x8A2A;
pub const GL_UNIFORM_BUFFER_START:                                       u16 = 0x8A29;
pub const GL_UNIFORM_IS_ROW_MAJOR:                                       u16 = 0x8A3E;
pub const GL_UNIFORM_MATRIX_STRIDE:                                      u16 = 0x8A3D;
pub const GL_UNIFORM_NAME_LENGTH:                                        u16 = 0x8A39;
pub const GL_UNIFORM_OFFSET:                                             u16 = 0x8A3B;
pub const GL_UNIFORM_SIZE:                                               u16 = 0x8A38;
pub const GL_UNIFORM_TYPE:                                               u16 = 0x8A37;
pub const GL_UNPACK_ALIGNMENT:                                           u16 = 0x0CF5;
pub const GL_UNPACK_COMPRESSED_BLOCK_DEPTH:                              u16 = 0x9129;
pub const GL_UNPACK_COMPRESSED_BLOCK_HEIGHT:                             u16 = 0x9128;
pub const GL_UNPACK_COMPRESSED_BLOCK_SIZE:                               u16 = 0x912A;
pub const GL_UNPACK_COMPRESSED_BLOCK_WIDTH:                              u16 = 0x9127;
pub const GL_UNPACK_IMAGE_HEIGHT:                                        u16 = 0x806E;
pub const GL_UNPACK_LSB_FIRST:                                           u16 = 0x0CF1;
pub const GL_UNPACK_ROW_LENGTH:                                          u16 = 0x0CF2;
pub const GL_UNPACK_SKIP_IMAGES:                                         u16 = 0x806D;
pub const GL_UNPACK_SKIP_PIXELS:                                         u16 = 0x0CF4;
pub const GL_UNPACK_SKIP_ROWS:                                           u16 = 0x0CF3;
pub const GL_UNPACK_SWAP_BYTES:                                          u16 = 0x0CF0;
pub const GL_UNSIGNALED:                                                 u16 = 0x9118;
pub const GL_UNSIGNED_BYTE:                                              u16 = 0x1401;
pub const GL_UNSIGNED_BYTE_2_3_3_REV:                                    u16 = 0x8362;
pub const GL_UNSIGNED_BYTE_3_3_2:                                        u16 = 0x8032;
pub const GL_UNSIGNED_INT:                                               u16 = 0x1405;
pub const GL_UNSIGNED_INT64_ARB:                                         u16 = 0x140F;
pub const GL_UNSIGNED_INT64_VEC2_ARB:                                    u16 = 0x8FF5;
pub const GL_UNSIGNED_INT64_VEC3_ARB:                                    u16 = 0x8FF6;
pub const GL_UNSIGNED_INT64_VEC4_ARB:                                    u16 = 0x8FF7;
pub const GL_UNSIGNED_INT_10F_11F_11F_REV:                               u16 = 0x8C3B;
pub const GL_UNSIGNED_INT_10_10_10_2:                                    u16 = 0x8036;
pub const GL_UNSIGNED_INT_24_8:                                          u16 = 0x84FA;
pub const GL_UNSIGNED_INT_2_10_10_10_REV:                                u16 = 0x8368;
pub const GL_UNSIGNED_INT_5_9_9_9_REV:                                   u16 = 0x8C3E;
pub const GL_UNSIGNED_INT_8_8_8_8:                                       u16 = 0x8035;
pub const GL_UNSIGNED_INT_8_8_8_8_REV:                                   u16 = 0x8367;
pub const GL_UNSIGNED_INT_ATOMIC_COUNTER:                                u16 = 0x92DB;
pub const GL_UNSIGNED_INT_IMAGE_1D:                                      u16 = 0x9062;
pub const GL_UNSIGNED_INT_IMAGE_1D_ARRAY:                                u16 = 0x9068;
pub const GL_UNSIGNED_INT_IMAGE_2D:                                      u16 = 0x9063;
pub const GL_UNSIGNED_INT_IMAGE_2D_ARRAY:                                u16 = 0x9069;
pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE:                          u16 = 0x906B;
pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY:                    u16 = 0x906C;
pub const GL_UNSIGNED_INT_IMAGE_2D_RECT:                                 u16 = 0x9065;
pub const GL_UNSIGNED_INT_IMAGE_3D:                                      u16 = 0x9064;
pub const GL_UNSIGNED_INT_IMAGE_BUFFER:                                  u16 = 0x9067;
pub const GL_UNSIGNED_INT_IMAGE_CUBE:                                    u16 = 0x9066;
pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY:                          u16 = 0x906A;
pub const GL_UNSIGNED_INT_SAMPLER_1D:                                    u16 = 0x8DD1;
pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY:                              u16 = 0x8DD6;
pub const GL_UNSIGNED_INT_SAMPLER_2D:                                    u16 = 0x8DD2;
pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY:                              u16 = 0x8DD7;
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE:                        u16 = 0x910A;
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY:                  u16 = 0x910D;
pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT:                               u16 = 0x8DD5;
pub const GL_UNSIGNED_INT_SAMPLER_3D:                                    u16 = 0x8DD3;
pub const GL_UNSIGNED_INT_SAMPLER_BUFFER:                                u16 = 0x8DD8;
pub const GL_UNSIGNED_INT_SAMPLER_CUBE:                                  u16 = 0x8DD4;
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY:                        u16 = 0x900F;
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY_ARB:                    u16 = 0x900F;
pub const GL_UNSIGNED_INT_VEC2:                                          u16 = 0x8DC6;
pub const GL_UNSIGNED_INT_VEC3:                                          u16 = 0x8DC7;
pub const GL_UNSIGNED_INT_VEC4:                                          u16 = 0x8DC8;
pub const GL_UNSIGNED_NORMALIZED:                                        u16 = 0x8C17;
pub const GL_UNSIGNED_NORMALIZED_ARB:                                    u16 = 0x8C17;
pub const GL_UNSIGNED_SHORT:                                             u16 = 0x1403;
pub const GL_UNSIGNED_SHORT_1_5_5_5_REV:                                 u16 = 0x8366;
pub const GL_UNSIGNED_SHORT_4_4_4_4:                                     u16 = 0x8033;
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV:                                 u16 = 0x8365;
pub const GL_UNSIGNED_SHORT_5_5_5_1:                                     u16 = 0x8034;
pub const GL_UNSIGNED_SHORT_5_6_5:                                       u16 = 0x8363;
pub const GL_UNSIGNED_SHORT_5_6_5_REV:                                   u16 = 0x8364;
pub const GL_UPPER_LEFT:                                                 u16 = 0x8CA2;
pub const GL_VALIDATE_STATUS:                                            u16 = 0x8B83;
pub const GL_VENDOR:                                                     u16 = 0x1F00;
pub const GL_VERSION:                                                    u16 = 0x1F02;
pub const GL_VERTEX_ARRAY:                                               u16 = 0x8074;
pub const GL_VERTEX_ARRAY_BINDING:                                       u16 = 0x85B5;
pub const GL_VERTEX_ARRAY_BUFFER_BINDING_ARB:                            u16 = 0x8896;
pub const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT:                            u32 = 0x00000001;
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING:                         u16 = 0x889F;
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING_ARB:                     u16 = 0x889F;
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR:                                u16 = 0x88FE;
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR_ARB:                            u16 = 0x88FE;
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED:                                u16 = 0x8622;
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED_ARB:                            u16 = 0x8622;
pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER:                                u16 = 0x88FD;
pub const GL_VERTEX_ATTRIB_ARRAY_LONG:                                   u16 = 0x874E;
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED:                             u16 = 0x886A;
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED_ARB:                         u16 = 0x886A;
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER:                                u16 = 0x8645;
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER_ARB:                            u16 = 0x8645;
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE:                                   u16 = 0x8623;
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE_ARB:                               u16 = 0x8623;
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE:                                 u16 = 0x8624;
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE_ARB:                             u16 = 0x8624;
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE:                                   u16 = 0x8625;
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE_ARB:                               u16 = 0x8625;
pub const GL_VERTEX_ATTRIB_BINDING:                                      u16 = 0x82D4;
pub const GL_VERTEX_ATTRIB_RELATIVE_OFFSET:                              u16 = 0x82D5;
pub const GL_VERTEX_BINDING_BUFFER:                                      u16 = 0x8F4F;
pub const GL_VERTEX_BINDING_DIVISOR:                                     u16 = 0x82D6;
pub const GL_VERTEX_BINDING_OFFSET:                                      u16 = 0x82D7;
pub const GL_VERTEX_BINDING_STRIDE:                                      u16 = 0x82D8;
pub const GL_VERTEX_PROGRAM_ARB:                                         u16 = 0x8620;
pub const GL_VERTEX_PROGRAM_POINT_SIZE:                                  u16 = 0x8642;
pub const GL_VERTEX_PROGRAM_POINT_SIZE_ARB:                              u16 = 0x8642;
pub const GL_VERTEX_PROGRAM_TWO_SIDE_ARB:                                u16 = 0x8643;
pub const GL_VERTEX_SHADER:                                              u16 = 0x8B31;
pub const GL_VERTEX_SHADER_ARB:                                          u16 = 0x8B31;
pub const GL_VERTEX_SHADER_BIT:                                          u32 = 0x00000001;
pub const GL_VERTEX_SHADER_INVOCATIONS:                                  u16 = 0x82F0;
pub const GL_VERTEX_SHADER_INVOCATIONS_ARB:                              u16 = 0x82F0;
pub const GL_VERTEX_SUBROUTINE:                                          u16 = 0x92E8;
pub const GL_VERTEX_SUBROUTINE_UNIFORM:                                  u16 = 0x92EE;
pub const GL_VERTEX_TEXTURE:                                             u16 = 0x829B;
pub const GL_VERTICES_SUBMITTED:                                         u16 = 0x82EE;
pub const GL_VERTICES_SUBMITTED_ARB:                                     u16 = 0x82EE;
pub const GL_VIEWPORT:                                                   u16 = 0x0BA2;
pub const GL_VIEWPORT_BOUNDS_RANGE:                                      u16 = 0x825D;
pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX:                            u16 = 0x825F;
pub const GL_VIEWPORT_SUBPIXEL_BITS:                                     u16 = 0x825C;
pub const GL_VIEW_CLASS_128_BITS:                                        u16 = 0x82C4;
pub const GL_VIEW_CLASS_16_BITS:                                         u16 = 0x82CA;
pub const GL_VIEW_CLASS_24_BITS:                                         u16 = 0x82C9;
pub const GL_VIEW_CLASS_32_BITS:                                         u16 = 0x82C8;
pub const GL_VIEW_CLASS_48_BITS:                                         u16 = 0x82C7;
pub const GL_VIEW_CLASS_64_BITS:                                         u16 = 0x82C6;
pub const GL_VIEW_CLASS_8_BITS:                                          u16 = 0x82CB;
pub const GL_VIEW_CLASS_96_BITS:                                         u16 = 0x82C5;
pub const GL_VIEW_CLASS_ASTC_10x10_RGBA:                                 u16 = 0x9393;
pub const GL_VIEW_CLASS_ASTC_10x5_RGBA:                                  u16 = 0x9390;
pub const GL_VIEW_CLASS_ASTC_10x6_RGBA:                                  u16 = 0x9391;
pub const GL_VIEW_CLASS_ASTC_10x8_RGBA:                                  u16 = 0x9392;
pub const GL_VIEW_CLASS_ASTC_12x10_RGBA:                                 u16 = 0x9394;
pub const GL_VIEW_CLASS_ASTC_12x12_RGBA:                                 u16 = 0x9395;
pub const GL_VIEW_CLASS_ASTC_4x4_RGBA:                                   u16 = 0x9388;
pub const GL_VIEW_CLASS_ASTC_5x4_RGBA:                                   u16 = 0x9389;
pub const GL_VIEW_CLASS_ASTC_5x5_RGBA:                                   u16 = 0x938A;
pub const GL_VIEW_CLASS_ASTC_6x5_RGBA:                                   u16 = 0x938B;
pub const GL_VIEW_CLASS_ASTC_6x6_RGBA:                                   u16 = 0x938C;
pub const GL_VIEW_CLASS_ASTC_8x5_RGBA:                                   u16 = 0x938D;
pub const GL_VIEW_CLASS_ASTC_8x6_RGBA:                                   u16 = 0x938E;
pub const GL_VIEW_CLASS_ASTC_8x8_RGBA:                                   u16 = 0x938F;
pub const GL_VIEW_CLASS_BPTC_FLOAT:                                      u16 = 0x82D3;
pub const GL_VIEW_CLASS_BPTC_UNORM:                                      u16 = 0x82D2;
pub const GL_VIEW_CLASS_EAC_R11:                                         u16 = 0x9383;
pub const GL_VIEW_CLASS_EAC_RG11:                                        u16 = 0x9384;
pub const GL_VIEW_CLASS_ETC2_EAC_RGBA:                                   u16 = 0x9387;
pub const GL_VIEW_CLASS_ETC2_RGB:                                        u16 = 0x9385;
pub const GL_VIEW_CLASS_ETC2_RGBA:                                       u16 = 0x9386;
pub const GL_VIEW_CLASS_RGTC1_RED:                                       u16 = 0x82D0;
pub const GL_VIEW_CLASS_RGTC2_RG:                                        u16 = 0x82D1;
pub const GL_VIEW_CLASS_S3TC_DXT1_RGB:                                   u16 = 0x82CC;
pub const GL_VIEW_CLASS_S3TC_DXT1_RGBA:                                  u16 = 0x82CD;
pub const GL_VIEW_CLASS_S3TC_DXT3_RGBA:                                  u16 = 0x82CE;
pub const GL_VIEW_CLASS_S3TC_DXT5_RGBA:                                  u16 = 0x82CF;
pub const GL_VIEW_COMPATIBILITY_CLASS:                                   u16 = 0x82B6;
pub const GL_WAIT_FAILED:                                                u16 = 0x911D;
pub const GL_WEIGHTED_AVERAGE_ARB:                                       u16 = 0x9367;
pub const GL_WEIGHT_ARRAY_BUFFER_BINDING_ARB:                            u16 = 0x889E;
pub const GL_WRITE_ONLY:                                                 u16 = 0x88B9;
pub const GL_WRITE_ONLY_ARB:                                             u16 = 0x88B9;
pub const GL_XOR:                                                        u16 = 0x1506;
pub const GL_ZERO:                                                       u8  = 0;

#[derive(Default, Clone, Copy)]
pub struct GladVersionCompat {
    pub GL_VERSION_1_0: bool,
    pub GL_VERSION_1_1: bool,
    pub GL_VERSION_1_2: bool,
    pub GL_VERSION_1_3: bool,
    pub GL_VERSION_1_4: bool,
    pub GL_VERSION_1_5: bool,

    pub GL_VERSION_2_0: bool,
    pub GL_VERSION_2_1: bool,

    pub GL_VERSION_3_0: bool,
    pub GL_VERSION_3_1: bool,
    pub GL_VERSION_3_2: bool,
    pub GL_VERSION_3_3: bool,

    pub GL_VERSION_4_0: bool,
    pub GL_VERSION_4_1: bool,
    pub GL_VERSION_4_2: bool,
    pub GL_VERSION_4_3: bool,
}

#[derive(Default, Clone, Copy)]
pub struct GladExtCompat {
    pub GL_ARB_ES2_compatibility:                bool,
    pub GL_ARB_ES3_1_compatibility:              bool,
    pub GL_ARB_ES3_2_compatibility:              bool,
    pub GL_ARB_ES3_compatibility:                bool,
    pub GL_ARB_blend_func_extended:              bool,
    pub GL_ARB_buffer_storage:                   bool,
    pub GL_ARB_clear_buffer_object:              bool,
    pub GL_ARB_clear_texture:                    bool,
    pub GL_ARB_color_buffer_float:               bool,
    pub GL_ARB_compatibility:                    bool,
    pub GL_ARB_compressed_texture_pixel_storage: bool,
    pub GL_ARB_compute_shader:                   bool,
    pub GL_ARB_compute_variable_group_size:      bool,
    pub GL_ARB_copy_buffer:                      bool,
    pub GL_ARB_copy_image:                       bool,
    pub GL_ARB_debug_output:                     bool,
    pub GL_ARB_depth_buffer_float:               bool,
    pub GL_ARB_depth_clamp:                      bool,
    pub GL_ARB_depth_texture:                    bool,
    pub GL_ARB_direct_state_access:              bool,
    pub GL_ARB_draw_buffers:                     bool,
    pub GL_ARB_draw_buffers_blend:               bool,
    pub GL_ARB_draw_elements_base_vertex:        bool,
    pub GL_ARB_draw_indirect:                    bool,
    pub GL_ARB_draw_instanced:                   bool,
    pub GL_ARB_enhanced_layouts:                 bool,
    pub GL_ARB_explicit_attrib_location:         bool,
    pub GL_ARB_explicit_uniform_location:        bool,
    pub GL_ARB_fragment_coord_conventions:       bool,
    pub GL_ARB_fragment_layer_viewport:          bool,
    pub GL_ARB_fragment_program:                 bool,
    pub GL_ARB_fragment_program_shadow:          bool,
    pub GL_ARB_fragment_shader:                  bool,
    pub GL_ARB_fragment_shader_interlock:        bool,
    pub GL_ARB_framebuffer_no_attachments:       bool,
    pub GL_ARB_framebuffer_object:               bool,
    pub GL_ARB_framebuffer_sRGB:                 bool,
    pub GL_ARB_geometry_shader4:                 bool,
    pub GL_ARB_get_program_binary:               bool,
    pub GL_ARB_get_texture_sub_image:            bool,
    pub GL_ARB_gl_spirv:                         bool,
    pub GL_ARB_gpu_shader5:                      bool,
    pub GL_ARB_gpu_shader_fp64:                  bool,
    pub GL_ARB_gpu_shader_int64:                 bool,
    pub GL_ARB_half_float_pixel:                 bool,
    pub GL_ARB_half_float_vertex:                bool,
    pub GL_ARB_instanced_arrays:                 bool,
    pub GL_ARB_internalformat_query:             bool,
    pub GL_ARB_internalformat_query2:            bool,
    pub GL_ARB_map_buffer_range:                 bool,
    pub GL_ARB_multi_bind:                       bool,
    pub GL_ARB_multi_draw_indirect:              bool,
    pub GL_ARB_multisample:                      bool,
    pub GL_ARB_multitexture:                     bool,
    pub GL_ARB_occlusion_query:                  bool,
    pub GL_ARB_occlusion_query2:                 bool,
    pub GL_ARB_pipeline_statistics_query:        bool,
    pub GL_ARB_query_buffer_object:              bool,
    pub GL_ARB_sample_locations:                 bool,
    pub GL_ARB_sample_shading:                   bool,
    pub GL_ARB_seamless_cube_map:                bool,
    pub GL_ARB_seamless_cubemap_per_texture:     bool,
    pub GL_ARB_shader_atomic_counter_ops:        bool,
    pub GL_ARB_shader_atomic_counters:           bool,
    pub GL_ARB_shader_bit_encoding:              bool,
    pub GL_ARB_shader_clock:                     bool,
    pub GL_ARB_shader_image_load_store:          bool,
    pub GL_ARB_shader_image_size:                bool,
    pub GL_ARB_shader_objects:                   bool,
    pub GL_ARB_shader_storage_buffer_object:     bool,
    pub GL_ARB_shader_texture_lod:               bool,
    pub GL_ARB_shading_language_100:             bool,
    pub GL_ARB_shading_language_420pack:         bool,
    pub GL_ARB_shading_language_include:         bool,
    pub GL_ARB_shading_language_packing:         bool,
    pub GL_ARB_spirv_extensions:                 bool,
    pub GL_ARB_tessellation_shader:              bool,
    pub GL_ARB_texture_border_clamp:             bool,
    pub GL_ARB_texture_buffer_object_rgb32:      bool,
    pub GL_ARB_texture_compression:              bool,
    pub GL_ARB_texture_cube_map:                 bool,
    pub GL_ARB_texture_cube_map_array:           bool,
    pub GL_ARB_texture_env_add:                  bool,
    pub GL_ARB_texture_filter_anisotropic:       bool,
    pub GL_ARB_texture_filter_minmax:            bool,
    pub GL_ARB_texture_float:                    bool,
    pub GL_ARB_texture_mirror_clamp_to_edge:     bool,
    pub GL_ARB_texture_mirrored_repeat:          bool,
    pub GL_ARB_texture_multisample:              bool,
    pub GL_ARB_texture_non_power_of_two:         bool,
    pub GL_ARB_texture_rg:                       bool,
    pub GL_ARB_texture_storage:                  bool,
    pub GL_ARB_texture_swizzle:                  bool,
    pub GL_ARB_texture_view:                     bool,
    pub GL_ARB_timer_query:                      bool,
    pub GL_ARB_transpose_matrix:                 bool,
    pub GL_ARB_uniform_buffer_object:            bool,
    pub GL_ARB_vertex_array_bgra:                bool,
    pub GL_ARB_vertex_array_object:              bool,
    pub GL_ARB_vertex_attrib_binding:            bool,
    pub GL_ARB_vertex_buffer_object:             bool,
    pub GL_ARB_vertex_program:                   bool,
    pub GL_ARB_vertex_shader:                    bool,

    pub GL_EXT_draw_instanced:                   bool,
    pub GL_EXT_fog_coord:                        bool,
    pub GL_EXT_framebuffer_blit:                 bool,
    pub GL_EXT_framebuffer_multisample:          bool,
    pub GL_EXT_framebuffer_object:               bool,
    pub GL_EXT_framebuffer_sRGB:                 bool,
    pub GL_EXT_texture_compression_s3tc:         bool,
    pub GL_EXT_texture_filter_anisotropic:       bool,
    pub GL_EXT_texture_mirror_clamp:             bool,

    pub GL_KHR_texture_compression_astc_hdr:     bool,
    pub GL_KHR_texture_compression_astc_ldr:     bool,

    pub GL_OES_compressed_paletted_texture:      bool,
    pub GL_OES_fixed_point:                      bool,
}

#[derive(Default)]
pub struct GladCompat(pub GladVersionCompat, pub GladExtCompat);

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

// unsafe extern "C" {
//     pub(super) static glad_glAccumxOES:                                   Option<PFNGLACCUMXOESPROC>;
//     pub(super) static glad_glActiveShaderProgram:                         Option<PFNGLACTIVESHADERPROGRAMPROC>;
//     pub(super) static glad_glActiveTexture:                               Option<PFNGLACTIVETEXTUREPROC>;
//     pub(super) static glad_glActiveTextureARB:                            Option<PFNGLACTIVETEXTUREARBPROC>;
//     pub(super) static glad_glAlphaFuncxOES:                               Option<PFNGLALPHAFUNCXOESPROC>;
//     pub(super) static glad_glAttachObjectARB:                             Option<PFNGLATTACHOBJECTARBPROC>;
//     pub(super) static glad_glAttachShader:                                Option<PFNGLATTACHSHADERPROC>;
//     pub(super) static glad_glBeginConditionalRender:                      Option<PFNGLBEGINCONDITIONALRENDERPROC>;
//     pub(super) static glad_glBeginQuery:                                  Option<PFNGLBEGINQUERYPROC>;
//     pub(super) static glad_glBeginQueryARB:                               Option<PFNGLBEGINQUERYARBPROC>;
//     pub(super) static glad_glBeginQueryIndexed:                           Option<PFNGLBEGINQUERYINDEXEDPROC>;
//     pub(super) static glad_glBeginTransformFeedback:                      Option<PFNGLBEGINTRANSFORMFEEDBACKPROC>;
//     pub(super) static glad_glBindAttribLocation:                          Option<PFNGLBINDATTRIBLOCATIONPROC>;
//     pub(super) static glad_glBindAttribLocationARB:                       Option<PFNGLBINDATTRIBLOCATIONARBPROC>;
//     pub(super) static glad_glBindBuffer:                                  Option<PFNGLBINDBUFFERPROC>;
//     pub(super) static glad_glBindBufferARB:                               Option<PFNGLBINDBUFFERARBPROC>;
//     pub(super) static glad_glBindBufferBase:                              Option<PFNGLBINDBUFFERBASEPROC>;
//     pub(super) static glad_glBindBufferRange:                             Option<PFNGLBINDBUFFERRANGEPROC>;
//     pub(super) static glad_glBindBuffersBase:                             Option<PFNGLBINDBUFFERSBASEPROC>;
//     pub(super) static glad_glBindBuffersRange:                            Option<PFNGLBINDBUFFERSRANGEPROC>;
//     pub(super) static glad_glBindFragDataLocation:                        Option<PFNGLBINDFRAGDATALOCATIONPROC>;
//     pub(super) static glad_glBindFragDataLocationIndexed:                 Option<PFNGLBINDFRAGDATALOCATIONINDEXEDPROC>;
//     pub(super) static glad_glBindFramebuffer:                             Option<PFNGLBINDFRAMEBUFFERPROC>;
//     pub(super) static glad_glBindFramebufferEXT:                          Option<PFNGLBINDFRAMEBUFFEREXTPROC>;
//     pub(super) static glad_glBindImageTexture:                            Option<PFNGLBINDIMAGETEXTUREPROC>;
//     pub(super) static glad_glBindImageTextures:                           Option<PFNGLBINDIMAGETEXTURESPROC>;
//     pub(super) static glad_glBindProgramARB:                              Option<PFNGLBINDPROGRAMARBPROC>;
//     pub(super) static glad_glBindProgramPipeline:                         Option<PFNGLBINDPROGRAMPIPELINEPROC>;
//     pub(super) static glad_glBindRenderbuffer:                            Option<PFNGLBINDRENDERBUFFERPROC>;
//     pub(super) static glad_glBindRenderbufferEXT:                         Option<PFNGLBINDRENDERBUFFEREXTPROC>;
//     pub(super) static glad_glBindSampler:                                 Option<PFNGLBINDSAMPLERPROC>;
//     pub(super) static glad_glBindSamplers:                                Option<PFNGLBINDSAMPLERSPROC>;
//     pub(super) static glad_glBindTexture:                                 Option<PFNGLBINDTEXTUREPROC>;
//     pub(super) static glad_glBindTextureUnit:                             Option<PFNGLBINDTEXTUREUNITPROC>;
//     pub(super) static glad_glBindTextures:                                Option<PFNGLBINDTEXTURESPROC>;
//     pub(super) static glad_glBindTransformFeedback:                       Option<PFNGLBINDTRANSFORMFEEDBACKPROC>;
//     pub(super) static glad_glBindVertexArray:                             Option<PFNGLBINDVERTEXARRAYPROC>;
//     pub(super) static glad_glBindVertexBuffer:                            Option<PFNGLBINDVERTEXBUFFERPROC>;
//     pub(super) static glad_glBindVertexBuffers:                           Option<PFNGLBINDVERTEXBUFFERSPROC>;
//     pub(super) static glad_glBitmapxOES:                                  Option<PFNGLBITMAPXOESPROC>;
//     pub(super) static glad_glBlendColor:                                  Option<PFNGLBLENDCOLORPROC>;
//     pub(super) static glad_glBlendColorxOES:                              Option<PFNGLBLENDCOLORXOESPROC>;
//     pub(super) static glad_glBlendEquation:                               Option<PFNGLBLENDEQUATIONPROC>;
//     pub(super) static glad_glBlendEquationSeparate:                       Option<PFNGLBLENDEQUATIONSEPARATEPROC>;
//     pub(super) static glad_glBlendEquationSeparatei:                      Option<PFNGLBLENDEQUATIONSEPARATEIPROC>;
//     pub(super) static glad_glBlendEquationSeparateiARB:                   Option<PFNGLBLENDEQUATIONSEPARATEIARBPROC>;
//     pub(super) static glad_glBlendEquationi:                              Option<PFNGLBLENDEQUATIONIPROC>;
//     pub(super) static glad_glBlendEquationiARB:                           Option<PFNGLBLENDEQUATIONIARBPROC>;
//     pub(super) static glad_glBlendFunc:                                   Option<PFNGLBLENDFUNCPROC>;
//     pub(super) static glad_glBlendFuncSeparate:                           Option<PFNGLBLENDFUNCSEPARATEPROC>;
//     pub(super) static glad_glBlendFuncSeparatei:                          Option<PFNGLBLENDFUNCSEPARATEIPROC>;
//     pub(super) static glad_glBlendFuncSeparateiARB:                       Option<PFNGLBLENDFUNCSEPARATEIARBPROC>;
//     pub(super) static glad_glBlendFunci:                                  Option<PFNGLBLENDFUNCIPROC>;
//     pub(super) static glad_glBlendFunciARB:                               Option<PFNGLBLENDFUNCIARBPROC>;
//     pub(super) static glad_glBlitFramebuffer:                             Option<PFNGLBLITFRAMEBUFFERPROC>;
//     pub(super) static glad_glBlitFramebufferEXT:                          Option<PFNGLBLITFRAMEBUFFEREXTPROC>;
//     pub(super) static glad_glBlitNamedFramebuffer:                        Option<PFNGLBLITNAMEDFRAMEBUFFERPROC>;
//     pub(super) static glad_glBufferData:                                  Option<PFNGLBUFFERDATAPROC>;
//     pub(super) static glad_glBufferDataARB:                               Option<PFNGLBUFFERDATAARBPROC>;
//     pub(super) static glad_glBufferStorage:                               Option<PFNGLBUFFERSTORAGEPROC>;
//     pub(super) static glad_glBufferSubData:                               Option<PFNGLBUFFERSUBDATAPROC>;
//     pub(super) static glad_glBufferSubDataARB:                            Option<PFNGLBUFFERSUBDATAARBPROC>;
//     pub(super) static glad_glCheckFramebufferStatus:                      Option<PFNGLCHECKFRAMEBUFFERSTATUSPROC>;
//     pub(super) static glad_glCheckFramebufferStatusEXT:                   Option<PFNGLCHECKFRAMEBUFFERSTATUSEXTPROC>;
//     pub(super) static glad_glCheckNamedFramebufferStatus:                 Option<PFNGLCHECKNAMEDFRAMEBUFFERSTATUSPROC>;
//     pub(super) static glad_glClampColor:                                  Option<PFNGLCLAMPCOLORPROC>;
//     pub(super) static glad_glClampColorARB:                               Option<PFNGLCLAMPCOLORARBPROC>;
//     pub(super) static glad_glClear:                                       Option<PFNGLCLEARPROC>;
//     pub(super) static glad_glClearAccumxOES:                              Option<PFNGLCLEARACCUMXOESPROC>;
//     pub(super) static glad_glClearBufferData:                             Option<PFNGLCLEARBUFFERDATAPROC>;
//     pub(super) static glad_glClearBufferSubData:                          Option<PFNGLCLEARBUFFERSUBDATAPROC>;
//     pub(super) static glad_glClearBufferfi:                               Option<PFNGLCLEARBUFFERFIPROC>;
//     pub(super) static glad_glClearBufferfv:                               Option<PFNGLCLEARBUFFERFVPROC>;
//     pub(super) static glad_glClearBufferiv:                               Option<PFNGLCLEARBUFFERIVPROC>;
//     pub(super) static glad_glClearBufferuiv:                              Option<PFNGLCLEARBUFFERUIVPROC>;
//     pub(super) static glad_glClearColor:                                  Option<PFNGLCLEARCOLORPROC>;
//     pub(super) static glad_glClearColorxOES:                              Option<PFNGLCLEARCOLORXOESPROC>;
//     pub(super) static glad_glClearDepth:                                  Option<PFNGLCLEARDEPTHPROC>;
//     pub(super) static glad_glClearDepthf:                                 Option<PFNGLCLEARDEPTHFPROC>;
//     pub(super) static glad_glClearDepthxOES:                              Option<PFNGLCLEARDEPTHXOESPROC>;
//     pub(super) static glad_glClearNamedBufferData:                        Option<PFNGLCLEARNAMEDBUFFERDATAPROC>;
//     pub(super) static glad_glClearNamedBufferSubData:                     Option<PFNGLCLEARNAMEDBUFFERSUBDATAPROC>;
//     pub(super) static glad_glClearNamedFramebufferfi:                     Option<PFNGLCLEARNAMEDFRAMEBUFFERFIPROC>;
//     pub(super) static glad_glClearNamedFramebufferfv:                     Option<PFNGLCLEARNAMEDFRAMEBUFFERFVPROC>;
//     pub(super) static glad_glClearNamedFramebufferiv:                     Option<PFNGLCLEARNAMEDFRAMEBUFFERIVPROC>;
//     pub(super) static glad_glClearNamedFramebufferuiv:                    Option<PFNGLCLEARNAMEDFRAMEBUFFERUIVPROC>;
//     pub(super) static glad_glClearStencil:                                Option<PFNGLCLEARSTENCILPROC>;
//     pub(super) static glad_glClearTexImage:                               Option<PFNGLCLEARTEXIMAGEPROC>;
//     pub(super) static glad_glClearTexSubImage:                            Option<PFNGLCLEARTEXSUBIMAGEPROC>;
//     pub(super) static glad_glClientActiveTextureARB:                      Option<PFNGLCLIENTACTIVETEXTUREARBPROC>;
//     pub(super) static glad_glClientWaitSync:                              Option<PFNGLCLIENTWAITSYNCPROC>;
//     pub(super) static glad_glClipPlanexOES:                               Option<PFNGLCLIPPLANEXOESPROC>;
//     pub(super) static glad_glColor3xOES:                                  Option<PFNGLCOLOR3XOESPROC>;
//     pub(super) static glad_glColor3xvOES:                                 Option<PFNGLCOLOR3XVOESPROC>;
//     pub(super) static glad_glColor4xOES:                                  Option<PFNGLCOLOR4XOESPROC>;
//     pub(super) static glad_glColor4xvOES:                                 Option<PFNGLCOLOR4XVOESPROC>;
//     pub(super) static glad_glColorMask:                                   Option<PFNGLCOLORMASKPROC>;
//     pub(super) static glad_glColorMaski:                                  Option<PFNGLCOLORMASKIPROC>;
//     pub(super) static glad_glCompileShader:                               Option<PFNGLCOMPILESHADERPROC>;
//     pub(super) static glad_glCompileShaderARB:                            Option<PFNGLCOMPILESHADERARBPROC>;
//     pub(super) static glad_glCompileShaderIncludeARB:                     Option<PFNGLCOMPILESHADERINCLUDEARBPROC>;
//     pub(super) static glad_glCompressedTexImage1D:                        Option<PFNGLCOMPRESSEDTEXIMAGE1DPROC>;
//     pub(super) static glad_glCompressedTexImage1DARB:                     Option<PFNGLCOMPRESSEDTEXIMAGE1DARBPROC>;
//     pub(super) static glad_glCompressedTexImage2D:                        Option<PFNGLCOMPRESSEDTEXIMAGE2DPROC>;
//     pub(super) static glad_glCompressedTexImage2DARB:                     Option<PFNGLCOMPRESSEDTEXIMAGE2DARBPROC>;
//     pub(super) static glad_glCompressedTexImage3D:                        Option<PFNGLCOMPRESSEDTEXIMAGE3DPROC>;
//     pub(super) static glad_glCompressedTexImage3DARB:                     Option<PFNGLCOMPRESSEDTEXIMAGE3DARBPROC>;
//     pub(super) static glad_glCompressedTexSubImage1D:                     Option<PFNGLCOMPRESSEDTEXSUBIMAGE1DPROC>;
//     pub(super) static glad_glCompressedTexSubImage1DARB:                  Option<PFNGLCOMPRESSEDTEXSUBIMAGE1DARBPROC>;
//     pub(super) static glad_glCompressedTexSubImage2D:                     Option<PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC>;
//     pub(super) static glad_glCompressedTexSubImage2DARB:                  Option<PFNGLCOMPRESSEDTEXSUBIMAGE2DARBPROC>;
//     pub(super) static glad_glCompressedTexSubImage3D:                     Option<PFNGLCOMPRESSEDTEXSUBIMAGE3DPROC>;
//     pub(super) static glad_glCompressedTexSubImage3DARB:                  Option<PFNGLCOMPRESSEDTEXSUBIMAGE3DARBPROC>;
//     pub(super) static glad_glCompressedTextureSubImage1D:                 Option<PFNGLCOMPRESSEDTEXTURESUBIMAGE1DPROC>;
//     pub(super) static glad_glCompressedTextureSubImage2D:                 Option<PFNGLCOMPRESSEDTEXTURESUBIMAGE2DPROC>;
//     pub(super) static glad_glCompressedTextureSubImage3D:                 Option<PFNGLCOMPRESSEDTEXTURESUBIMAGE3DPROC>;
//     pub(super) static glad_glConvolutionParameterxOES:                    Option<PFNGLCONVOLUTIONPARAMETERXOESPROC>;
//     pub(super) static glad_glConvolutionParameterxvOES:                   Option<PFNGLCONVOLUTIONPARAMETERXVOESPROC>;
//     pub(super) static glad_glCopyBufferSubData:                           Option<PFNGLCOPYBUFFERSUBDATAPROC>;
//     pub(super) static glad_glCopyImageSubData:                            Option<PFNGLCOPYIMAGESUBDATAPROC>;
//     pub(super) static glad_glCopyNamedBufferSubData:                      Option<PFNGLCOPYNAMEDBUFFERSUBDATAPROC>;
//     pub(super) static glad_glCopyTexImage1D:                              Option<PFNGLCOPYTEXIMAGE1DPROC>;
//     pub(super) static glad_glCopyTexImage2D:                              Option<PFNGLCOPYTEXIMAGE2DPROC>;
//     pub(super) static glad_glCopyTexSubImage1D:                           Option<PFNGLCOPYTEXSUBIMAGE1DPROC>;
//     pub(super) static glad_glCopyTexSubImage2D:                           Option<PFNGLCOPYTEXSUBIMAGE2DPROC>;
//     pub(super) static glad_glCopyTexSubImage3D:                           Option<PFNGLCOPYTEXSUBIMAGE3DPROC>;
//     pub(super) static glad_glCopyTextureSubImage1D:                       Option<PFNGLCOPYTEXTURESUBIMAGE1DPROC>;
//     pub(super) static glad_glCopyTextureSubImage2D:                       Option<PFNGLCOPYTEXTURESUBIMAGE2DPROC>;
//     pub(super) static glad_glCopyTextureSubImage3D:                       Option<PFNGLCOPYTEXTURESUBIMAGE3DPROC>;
//     pub(super) static glad_glCreateBuffers:                               Option<PFNGLCREATEBUFFERSPROC>;
//     pub(super) static glad_glCreateFramebuffers:                          Option<PFNGLCREATEFRAMEBUFFERSPROC>;
//     pub(super) static glad_glCreateProgram:                               Option<PFNGLCREATEPROGRAMPROC>;
//     pub(super) static glad_glCreateProgramObjectARB:                      Option<PFNGLCREATEPROGRAMOBJECTARBPROC>;
//     pub(super) static glad_glCreateProgramPipelines:                      Option<PFNGLCREATEPROGRAMPIPELINESPROC>;
//     pub(super) static glad_glCreateQueries:                               Option<PFNGLCREATEQUERIESPROC>;
//     pub(super) static glad_glCreateRenderbuffers:                         Option<PFNGLCREATERENDERBUFFERSPROC>;
//     pub(super) static glad_glCreateSamplers:                              Option<PFNGLCREATESAMPLERSPROC>;
//     pub(super) static glad_glCreateShader:                                Option<PFNGLCREATESHADERPROC>;
//     pub(super) static glad_glCreateShaderObjectARB:                       Option<PFNGLCREATESHADEROBJECTARBPROC>;
//     pub(super) static glad_glCreateShaderProgramv:                        Option<PFNGLCREATESHADERPROGRAMVPROC>;
//     pub(super) static glad_glCreateTextures:                              Option<PFNGLCREATETEXTURESPROC>;
//     pub(super) static glad_glCreateTransformFeedbacks:                    Option<PFNGLCREATETRANSFORMFEEDBACKSPROC>;
//     pub(super) static glad_glCreateVertexArrays:                          Option<PFNGLCREATEVERTEXARRAYSPROC>;
//     pub(super) static glad_glCullFace:                                    Option<PFNGLCULLFACEPROC>;
//     pub(super) static glad_glDebugMessageCallback:                        Option<PFNGLDEBUGMESSAGECALLBACKPROC>;
//     pub(super) static glad_glDebugMessageCallbackARB:                     Option<PFNGLDEBUGMESSAGECALLBACKARBPROC>;
//     pub(super) static glad_glDebugMessageControl:                         Option<PFNGLDEBUGMESSAGECONTROLPROC>;
//     pub(super) static glad_glDebugMessageControlARB:                      Option<PFNGLDEBUGMESSAGECONTROLARBPROC>;
//     pub(super) static glad_glDebugMessageInsert:                          Option<PFNGLDEBUGMESSAGEINSERTPROC>;
//     pub(super) static glad_glDebugMessageInsertARB:                       Option<PFNGLDEBUGMESSAGEINSERTARBPROC>;
//     pub(super) static glad_glDeleteBuffers:                               Option<PFNGLDELETEBUFFERSPROC>;
//     pub(super) static glad_glDeleteBuffersARB:                            Option<PFNGLDELETEBUFFERSARBPROC>;
//     pub(super) static glad_glDeleteFramebuffers:                          Option<PFNGLDELETEFRAMEBUFFERSPROC>;
//     pub(super) static glad_glDeleteFramebuffersEXT:                       Option<PFNGLDELETEFRAMEBUFFERSEXTPROC>;
//     pub(super) static glad_glDeleteNamedStringARB:                        Option<PFNGLDELETENAMEDSTRINGARBPROC>;
//     pub(super) static glad_glDeleteObjectARB:                             Option<PFNGLDELETEOBJECTARBPROC>;
//     pub(super) static glad_glDeleteProgram:                               Option<PFNGLDELETEPROGRAMPROC>;
//     pub(super) static glad_glDeleteProgramPipelines:                      Option<PFNGLDELETEPROGRAMPIPELINESPROC>;
//     pub(super) static glad_glDeleteProgramsARB:                           Option<PFNGLDELETEPROGRAMSARBPROC>;
//     pub(super) static glad_glDeleteQueries:                               Option<PFNGLDELETEQUERIESPROC>;
//     pub(super) static glad_glDeleteQueriesARB:                            Option<PFNGLDELETEQUERIESARBPROC>;
//     pub(super) static glad_glDeleteRenderbuffers:                         Option<PFNGLDELETERENDERBUFFERSPROC>;
//     pub(super) static glad_glDeleteRenderbuffersEXT:                      Option<PFNGLDELETERENDERBUFFERSEXTPROC>;
//     pub(super) static glad_glDeleteSamplers:                              Option<PFNGLDELETESAMPLERSPROC>;
//     pub(super) static glad_glDeleteShader:                                Option<PFNGLDELETESHADERPROC>;
//     pub(super) static glad_glDeleteSync:                                  Option<PFNGLDELETESYNCPROC>;
//     pub(super) static glad_glDeleteTextures:                              Option<PFNGLDELETETEXTURESPROC>;
//     pub(super) static glad_glDeleteTransformFeedbacks:                    Option<PFNGLDELETETRANSFORMFEEDBACKSPROC>;
//     pub(super) static glad_glDeleteVertexArrays:                          Option<PFNGLDELETEVERTEXARRAYSPROC>;
//     pub(super) static glad_glDepthFunc:                                   Option<PFNGLDEPTHFUNCPROC>;
//     pub(super) static glad_glDepthMask:                                   Option<PFNGLDEPTHMASKPROC>;
//     pub(super) static glad_glDepthRange:                                  Option<PFNGLDEPTHRANGEPROC>;
//     pub(super) static glad_glDepthRangeArrayv:                            Option<PFNGLDEPTHRANGEARRAYVPROC>;
//     pub(super) static glad_glDepthRangeIndexed:                           Option<PFNGLDEPTHRANGEINDEXEDPROC>;
//     pub(super) static glad_glDepthRangef:                                 Option<PFNGLDEPTHRANGEFPROC>;
//     pub(super) static glad_glDepthRangexOES:                              Option<PFNGLDEPTHRANGEXOESPROC>;
//     pub(super) static glad_glDetachObjectARB:                             Option<PFNGLDETACHOBJECTARBPROC>;
//     pub(super) static glad_glDetachShader:                                Option<PFNGLDETACHSHADERPROC>;
//     pub(super) static glad_glDisable:                                     Option<PFNGLDISABLEPROC>;
//     pub(super) static glad_glDisableVertexArrayAttrib:                    Option<PFNGLDISABLEVERTEXARRAYATTRIBPROC>;
//     pub(super) static glad_glDisableVertexAttribArray:                    Option<PFNGLDISABLEVERTEXATTRIBARRAYPROC>;
//     pub(super) static glad_glDisableVertexAttribArrayARB:                 Option<PFNGLDISABLEVERTEXATTRIBARRAYARBPROC>;
//     pub(super) static glad_glDisablei:                                    Option<PFNGLDISABLEIPROC>;
//     pub(super) static glad_glDispatchCompute:                             Option<PFNGLDISPATCHCOMPUTEPROC>;
//     pub(super) static glad_glDispatchComputeGroupSizeARB:                 Option<PFNGLDISPATCHCOMPUTEGROUPSIZEARBPROC>;
//     pub(super) static glad_glDispatchComputeIndirect:                     Option<PFNGLDISPATCHCOMPUTEINDIRECTPROC>;
//     pub(super) static glad_glDrawArrays:                                  Option<PFNGLDRAWARRAYSPROC>;
//     pub(super) static glad_glDrawArraysIndirect:                          Option<PFNGLDRAWARRAYSINDIRECTPROC>;
//     pub(super) static glad_glDrawArraysInstanced:                         Option<PFNGLDRAWARRAYSINSTANCEDPROC>;
//     pub(super) static glad_glDrawArraysInstancedARB:                      Option<PFNGLDRAWARRAYSINSTANCEDARBPROC>;
//     pub(super) static glad_glDrawArraysInstancedBaseInstance:             Option<PFNGLDRAWARRAYSINSTANCEDBASEINSTANCEPROC>;
//     pub(super) static glad_glDrawArraysInstancedEXT:                      Option<PFNGLDRAWARRAYSINSTANCEDEXTPROC>;
//     pub(super) static glad_glDrawBuffer:                                  Option<PFNGLDRAWBUFFERPROC>;
//     pub(super) static glad_glDrawBuffers:                                 Option<PFNGLDRAWBUFFERSPROC>;
//     pub(super) static glad_glDrawBuffersARB:                              Option<PFNGLDRAWBUFFERSARBPROC>;
//     pub(super) static glad_glDrawElements:                                Option<PFNGLDRAWELEMENTSPROC>;
//     pub(super) static glad_glDrawElementsBaseVertex:                      Option<PFNGLDRAWELEMENTSBASEVERTEXPROC>;
//     pub(super) static glad_glDrawElementsIndirect:                        Option<PFNGLDRAWELEMENTSINDIRECTPROC>;
//     pub(super) static glad_glDrawElementsInstanced:                       Option<PFNGLDRAWELEMENTSINSTANCEDPROC>;
//     pub(super) static glad_glDrawElementsInstancedARB:                    Option<PFNGLDRAWELEMENTSINSTANCEDARBPROC>;
//     pub(super) static glad_glDrawElementsInstancedBaseInstance:           Option<PFNGLDRAWELEMENTSINSTANCEDBASEINSTANCEPROC>;
//     pub(super) static glad_glDrawElementsInstancedBaseVertex:             Option<PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC>;
//     pub(super) static glad_glDrawElementsInstancedBaseVertexBaseInstance: Option<PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXBASEINSTANCEPROC>;
//     pub(super) static glad_glDrawElementsInstancedEXT:                    Option<PFNGLDRAWELEMENTSINSTANCEDEXTPROC>;
//     pub(super) static glad_glDrawRangeElements:                           Option<PFNGLDRAWRANGEELEMENTSPROC>;
//     pub(super) static glad_glDrawRangeElementsBaseVertex:                 Option<PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC>;
//     pub(super) static glad_glDrawTransformFeedback:                       Option<PFNGLDRAWTRANSFORMFEEDBACKPROC>;
//     pub(super) static glad_glDrawTransformFeedbackInstanced:              Option<PFNGLDRAWTRANSFORMFEEDBACKINSTANCEDPROC>;
//     pub(super) static glad_glDrawTransformFeedbackStream:                 Option<PFNGLDRAWTRANSFORMFEEDBACKSTREAMPROC>;
//     pub(super) static glad_glDrawTransformFeedbackStreamInstanced:        Option<PFNGLDRAWTRANSFORMFEEDBACKSTREAMINSTANCEDPROC>;
//     pub(super) static glad_glEnable:                                      Option<PFNGLENABLEPROC>;
//     pub(super) static glad_glEnableVertexArrayAttrib:                     Option<PFNGLENABLEVERTEXARRAYATTRIBPROC>;
//     pub(super) static glad_glEnableVertexAttribArray:                     Option<PFNGLENABLEVERTEXATTRIBARRAYPROC>;
//     pub(super) static glad_glEnableVertexAttribArrayARB:                  Option<PFNGLENABLEVERTEXATTRIBARRAYARBPROC>;
//     pub(super) static glad_glEnablei:                                     Option<PFNGLENABLEIPROC>;
//     pub(super) static glad_glEndConditionalRender:                        Option<PFNGLENDCONDITIONALRENDERPROC>;
//     pub(super) static glad_glEndQuery:                                    Option<PFNGLENDQUERYPROC>;
//     pub(super) static glad_glEndQueryARB:                                 Option<PFNGLENDQUERYARBPROC>;
//     pub(super) static glad_glEndQueryIndexed:                             Option<PFNGLENDQUERYINDEXEDPROC>;
//     pub(super) static glad_glEndTransformFeedback:                        Option<PFNGLENDTRANSFORMFEEDBACKPROC>;
//     pub(super) static glad_glEvalCoord1xOES:                              Option<PFNGLEVALCOORD1XOESPROC>;
//     pub(super) static glad_glEvalCoord1xvOES:                             Option<PFNGLEVALCOORD1XVOESPROC>;
//     pub(super) static glad_glEvalCoord2xOES:                              Option<PFNGLEVALCOORD2XOESPROC>;
//     pub(super) static glad_glEvalCoord2xvOES:                             Option<PFNGLEVALCOORD2XVOESPROC>;
//     pub(super) static glad_glEvaluateDepthValuesARB:                      Option<PFNGLEVALUATEDEPTHVALUESARBPROC>;
//     pub(super) static glad_glFeedbackBufferxOES:                          Option<PFNGLFEEDBACKBUFFERXOESPROC>;
//     pub(super) static glad_glFenceSync:                                   Option<PFNGLFENCESYNCPROC>;
//     pub(super) static glad_glFinish:                                      Option<PFNGLFINISHPROC>;
//     pub(super) static glad_glFlush:                                       Option<PFNGLFLUSHPROC>;
//     pub(super) static glad_glFlushMappedBufferRange:                      Option<PFNGLFLUSHMAPPEDBUFFERRANGEPROC>;
//     pub(super) static glad_glFlushMappedNamedBufferRange:                 Option<PFNGLFLUSHMAPPEDNAMEDBUFFERRANGEPROC>;
//     pub(super) static glad_glFogCoordPointerEXT:                          Option<PFNGLFOGCOORDPOINTEREXTPROC>;
//     pub(super) static glad_glFogCoorddEXT:                                Option<PFNGLFOGCOORDDEXTPROC>;
//     pub(super) static glad_glFogCoorddvEXT:                               Option<PFNGLFOGCOORDDVEXTPROC>;
//     pub(super) static glad_glFogCoordfEXT:                                Option<PFNGLFOGCOORDFEXTPROC>;
//     pub(super) static glad_glFogCoordfvEXT:                               Option<PFNGLFOGCOORDFVEXTPROC>;
//     pub(super) static glad_glFogxOES:                                     Option<PFNGLFOGXOESPROC>;
//     pub(super) static glad_glFogxvOES:                                    Option<PFNGLFOGXVOESPROC>;
//     pub(super) static glad_glFramebufferParameteri:                       Option<PFNGLFRAMEBUFFERPARAMETERIPROC>;
//     pub(super) static glad_glFramebufferRenderbuffer:                     Option<PFNGLFRAMEBUFFERRENDERBUFFERPROC>;
//     pub(super) static glad_glFramebufferRenderbufferEXT:                  Option<PFNGLFRAMEBUFFERRENDERBUFFEREXTPROC>;
//     pub(super) static glad_glFramebufferSampleLocationsfvARB:             Option<PFNGLFRAMEBUFFERSAMPLELOCATIONSFVARBPROC>;
//     pub(super) static glad_glFramebufferTexture:                          Option<PFNGLFRAMEBUFFERTEXTUREPROC>;
//     pub(super) static glad_glFramebufferTexture1D:                        Option<PFNGLFRAMEBUFFERTEXTURE1DPROC>;
//     pub(super) static glad_glFramebufferTexture1DEXT:                     Option<PFNGLFRAMEBUFFERTEXTURE1DEXTPROC>;
//     pub(super) static glad_glFramebufferTexture2D:                        Option<PFNGLFRAMEBUFFERTEXTURE2DPROC>;
//     pub(super) static glad_glFramebufferTexture2DEXT:                     Option<PFNGLFRAMEBUFFERTEXTURE2DEXTPROC>;
//     pub(super) static glad_glFramebufferTexture3D:                        Option<PFNGLFRAMEBUFFERTEXTURE3DPROC>;
//     pub(super) static glad_glFramebufferTexture3DEXT:                     Option<PFNGLFRAMEBUFFERTEXTURE3DEXTPROC>;
//     pub(super) static glad_glFramebufferTextureARB:                       Option<PFNGLFRAMEBUFFERTEXTUREARBPROC>;
//     pub(super) static glad_glFramebufferTextureFaceARB:                   Option<PFNGLFRAMEBUFFERTEXTUREFACEARBPROC>;
//     pub(super) static glad_glFramebufferTextureLayer:                     Option<PFNGLFRAMEBUFFERTEXTURELAYERPROC>;
//     pub(super) static glad_glFramebufferTextureLayerARB:                  Option<PFNGLFRAMEBUFFERTEXTURELAYERARBPROC>;
//     pub(super) static glad_glFrontFace:                                   Option<PFNGLFRONTFACEPROC>;
//     pub(super) static glad_glFrustumxOES:                                 Option<PFNGLFRUSTUMXOESPROC>;
//     pub(super) static glad_glGenBuffers:                                  Option<PFNGLGENBUFFERSPROC>;
//     pub(super) static glad_glGenBuffersARB:                               Option<PFNGLGENBUFFERSARBPROC>;
//     pub(super) static glad_glGenFramebuffers:                             Option<PFNGLGENFRAMEBUFFERSPROC>;
//     pub(super) static glad_glGenFramebuffersEXT:                          Option<PFNGLGENFRAMEBUFFERSEXTPROC>;
//     pub(super) static glad_glGenProgramPipelines:                         Option<PFNGLGENPROGRAMPIPELINESPROC>;
//     pub(super) static glad_glGenProgramsARB:                              Option<PFNGLGENPROGRAMSARBPROC>;
//     pub(super) static glad_glGenQueries:                                  Option<PFNGLGENQUERIESPROC>;
//     pub(super) static glad_glGenQueriesARB:                               Option<PFNGLGENQUERIESARBPROC>;
//     pub(super) static glad_glGenRenderbuffers:                            Option<PFNGLGENRENDERBUFFERSPROC>;
//     pub(super) static glad_glGenRenderbuffersEXT:                         Option<PFNGLGENRENDERBUFFERSEXTPROC>;
//     pub(super) static glad_glGenSamplers:                                 Option<PFNGLGENSAMPLERSPROC>;
//     pub(super) static glad_glGenTextures:                                 Option<PFNGLGENTEXTURESPROC>;
//     pub(super) static glad_glGenTransformFeedbacks:                       Option<PFNGLGENTRANSFORMFEEDBACKSPROC>;
//     pub(super) static glad_glGenVertexArrays:                             Option<PFNGLGENVERTEXARRAYSPROC>;
//     pub(super) static glad_glGenerateMipmap:                              Option<PFNGLGENERATEMIPMAPPROC>;
//     pub(super) static glad_glGenerateMipmapEXT:                           Option<PFNGLGENERATEMIPMAPEXTPROC>;
//     pub(super) static glad_glGenerateTextureMipmap:                       Option<PFNGLGENERATETEXTUREMIPMAPPROC>;
//     pub(super) static glad_glGetActiveAtomicCounterBufferiv:              Option<PFNGLGETACTIVEATOMICCOUNTERBUFFERIVPROC>;
//     pub(super) static glad_glGetActiveAttrib:                             Option<PFNGLGETACTIVEATTRIBPROC>;
//     pub(super) static glad_glGetActiveAttribARB:                          Option<PFNGLGETACTIVEATTRIBARBPROC>;
//     pub(super) static glad_glGetActiveSubroutineName:                     Option<PFNGLGETACTIVESUBROUTINENAMEPROC>;
//     pub(super) static glad_glGetActiveSubroutineUniformName:              Option<PFNGLGETACTIVESUBROUTINEUNIFORMNAMEPROC>;
//     pub(super) static glad_glGetActiveSubroutineUniformiv:                Option<PFNGLGETACTIVESUBROUTINEUNIFORMIVPROC>;
//     pub(super) static glad_glGetActiveUniform:                            Option<PFNGLGETACTIVEUNIFORMPROC>;
//     pub(super) static glad_glGetActiveUniformARB:                         Option<PFNGLGETACTIVEUNIFORMARBPROC>;
//     pub(super) static glad_glGetActiveUniformBlockName:                   Option<PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC>;
//     pub(super) static glad_glGetActiveUniformBlockiv:                     Option<PFNGLGETACTIVEUNIFORMBLOCKIVPROC>;
//     pub(super) static glad_glGetActiveUniformName:                        Option<PFNGLGETACTIVEUNIFORMNAMEPROC>;
//     pub(super) static glad_glGetActiveUniformsiv:                         Option<PFNGLGETACTIVEUNIFORMSIVPROC>;
//     pub(super) static glad_glGetAttachedObjectsARB:                       Option<PFNGLGETATTACHEDOBJECTSARBPROC>;
//     pub(super) static glad_glGetAttachedShaders:                          Option<PFNGLGETATTACHEDSHADERSPROC>;
//     pub(super) static glad_glGetAttribLocation:                           Option<PFNGLGETATTRIBLOCATIONPROC>;
//     pub(super) static glad_glGetAttribLocationARB:                        Option<PFNGLGETATTRIBLOCATIONARBPROC>;
//     pub(super) static glad_glGetBooleani_v:                               Option<PFNGLGETBOOLEANI_VPROC>;
//     pub(super) static glad_glGetBooleanv:                                 Option<PFNGLGETBOOLEANVPROC>;
//     pub(super) static glad_glGetBufferParameteri64v:                      Option<PFNGLGETBUFFERPARAMETERI64VPROC>;
//     pub(super) static glad_glGetBufferParameteriv:                        Option<PFNGLGETBUFFERPARAMETERIVPROC>;
//     pub(super) static glad_glGetBufferParameterivARB:                     Option<PFNGLGETBUFFERPARAMETERIVARBPROC>;
//     pub(super) static glad_glGetBufferPointerv:                           Option<PFNGLGETBUFFERPOINTERVPROC>;
//     pub(super) static glad_glGetBufferPointervARB:                        Option<PFNGLGETBUFFERPOINTERVARBPROC>;
//     pub(super) static glad_glGetBufferSubData:                            Option<PFNGLGETBUFFERSUBDATAPROC>;
//     pub(super) static glad_glGetBufferSubDataARB:                         Option<PFNGLGETBUFFERSUBDATAARBPROC>;
//     pub(super) static glad_glGetClipPlanexOES:                            Option<PFNGLGETCLIPPLANEXOESPROC>;
//     pub(super) static glad_glGetCompressedTexImage:                       Option<PFNGLGETCOMPRESSEDTEXIMAGEPROC>;
//     pub(super) static glad_glGetCompressedTexImageARB:                    Option<PFNGLGETCOMPRESSEDTEXIMAGEARBPROC>;
//     pub(super) static glad_glGetCompressedTextureImage:                   Option<PFNGLGETCOMPRESSEDTEXTUREIMAGEPROC>;
//     pub(super) static glad_glGetCompressedTextureSubImage:                Option<PFNGLGETCOMPRESSEDTEXTURESUBIMAGEPROC>;
//     pub(super) static glad_glGetConvolutionParameterxvOES:                Option<PFNGLGETCONVOLUTIONPARAMETERXVOESPROC>;
//     pub(super) static glad_glGetDebugMessageLog:                          Option<PFNGLGETDEBUGMESSAGELOGPROC>;
//     pub(super) static glad_glGetDebugMessageLogARB:                       Option<PFNGLGETDEBUGMESSAGELOGARBPROC>;
//     pub(super) static glad_glGetDoublei_v:                                Option<PFNGLGETDOUBLEI_VPROC>;
//     pub(super) static glad_glGetDoublev:                                  Option<PFNGLGETDOUBLEVPROC>;
//     pub(super) static glad_glGetError:                                    Option<PFNGLGETERRORPROC>;
//     pub(super) static glad_glGetFixedvOES:                                Option<PFNGLGETFIXEDVOESPROC>;
//     pub(super) static glad_glGetFloati_v:                                 Option<PFNGLGETFLOATI_VPROC>;
//     pub(super) static glad_glGetFloatv:                                   Option<PFNGLGETFLOATVPROC>;
//     pub(super) static glad_glGetFragDataIndex:                            Option<PFNGLGETFRAGDATAINDEXPROC>;
//     pub(super) static glad_glGetFragDataLocation:                         Option<PFNGLGETFRAGDATALOCATIONPROC>;
//     pub(super) static glad_glGetFramebufferAttachmentParameteriv:         Option<PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC>;
//     pub(super) static glad_glGetFramebufferAttachmentParameterivEXT:      Option<PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVEXTPROC>;
//     pub(super) static glad_glGetFramebufferParameteriv:                   Option<PFNGLGETFRAMEBUFFERPARAMETERIVPROC>;
//     pub(super) static glad_glGetHandleARB:                                Option<PFNGLGETHANDLEARBPROC>;
//     pub(super) static glad_glGetHistogramParameterxvOES:                  Option<PFNGLGETHISTOGRAMPARAMETERXVOESPROC>;
//     pub(super) static glad_glGetInfoLogARB:                               Option<PFNGLGETINFOLOGARBPROC>;
//     pub(super) static glad_glGetInteger64i_v:                             Option<PFNGLGETINTEGER64I_VPROC>;
//     pub(super) static glad_glGetInteger64v:                               Option<PFNGLGETINTEGER64VPROC>;
//     pub(super) static glad_glGetIntegeri_v:                               Option<PFNGLGETINTEGERI_VPROC>;
//     pub(super) static glad_glGetIntegerv:                                 Option<PFNGLGETINTEGERVPROC>;
//     pub(super) static glad_glGetInternalformati64v:                       Option<PFNGLGETINTERNALFORMATI64VPROC>;
//     pub(super) static glad_glGetInternalformativ:                         Option<PFNGLGETINTERNALFORMATIVPROC>;
//     pub(super) static glad_glGetLightxOES:                                Option<PFNGLGETLIGHTXOESPROC>;
//     pub(super) static glad_glGetMapxvOES:                                 Option<PFNGLGETMAPXVOESPROC>;
//     pub(super) static glad_glGetMaterialxOES:                             Option<PFNGLGETMATERIALXOESPROC>;
//     pub(super) static glad_glGetMultisamplefv:                            Option<PFNGLGETMULTISAMPLEFVPROC>;
//     pub(super) static glad_glGetNamedBufferParameteri64v:                 Option<PFNGLGETNAMEDBUFFERPARAMETERI64VPROC>;
//     pub(super) static glad_glGetNamedBufferParameteriv:                   Option<PFNGLGETNAMEDBUFFERPARAMETERIVPROC>;
//     pub(super) static glad_glGetNamedBufferPointerv:                      Option<PFNGLGETNAMEDBUFFERPOINTERVPROC>;
//     pub(super) static glad_glGetNamedBufferSubData:                       Option<PFNGLGETNAMEDBUFFERSUBDATAPROC>;
//     pub(super) static glad_glGetNamedFramebufferAttachmentParameteriv:    Option<PFNGLGETNAMEDFRAMEBUFFERATTACHMENTPARAMETERIVPROC>;
//     pub(super) static glad_glGetNamedFramebufferParameteriv:              Option<PFNGLGETNAMEDFRAMEBUFFERPARAMETERIVPROC>;
//     pub(super) static glad_glGetNamedRenderbufferParameteriv:             Option<PFNGLGETNAMEDRENDERBUFFERPARAMETERIVPROC>;
//     pub(super) static glad_glGetNamedStringARB:                           Option<PFNGLGETNAMEDSTRINGARBPROC>;
//     pub(super) static glad_glGetNamedStringivARB:                         Option<PFNGLGETNAMEDSTRINGIVARBPROC>;
//     pub(super) static glad_glGetObjectLabel:                              Option<PFNGLGETOBJECTLABELPROC>;
//     pub(super) static glad_glGetObjectParameterfvARB:                     Option<PFNGLGETOBJECTPARAMETERFVARBPROC>;
//     pub(super) static glad_glGetObjectParameterivARB:                     Option<PFNGLGETOBJECTPARAMETERIVARBPROC>;
//     pub(super) static glad_glGetObjectPtrLabel:                           Option<PFNGLGETOBJECTPTRLABELPROC>;
//     pub(super) static glad_glGetPixelMapxv:                               Option<PFNGLGETPIXELMAPXVPROC>;
//     pub(super) static glad_glGetPointerv:                                 Option<PFNGLGETPOINTERVPROC>;
//     pub(super) static glad_glGetProgramBinary:                            Option<PFNGLGETPROGRAMBINARYPROC>;
//     pub(super) static glad_glGetProgramEnvParameterdvARB:                 Option<PFNGLGETPROGRAMENVPARAMETERDVARBPROC>;
//     pub(super) static glad_glGetProgramEnvParameterfvARB:                 Option<PFNGLGETPROGRAMENVPARAMETERFVARBPROC>;
//     pub(super) static glad_glGetProgramInfoLog:                           Option<PFNGLGETPROGRAMINFOLOGPROC>;
//     pub(super) static glad_glGetProgramInterfaceiv:                       Option<PFNGLGETPROGRAMINTERFACEIVPROC>;
//     pub(super) static glad_glGetProgramLocalParameterdvARB:               Option<PFNGLGETPROGRAMLOCALPARAMETERDVARBPROC>;
//     pub(super) static glad_glGetProgramLocalParameterfvARB:               Option<PFNGLGETPROGRAMLOCALPARAMETERFVARBPROC>;
//     pub(super) static glad_glGetProgramPipelineInfoLog:                   Option<PFNGLGETPROGRAMPIPELINEINFOLOGPROC>;
//     pub(super) static glad_glGetProgramPipelineiv:                        Option<PFNGLGETPROGRAMPIPELINEIVPROC>;
//     pub(super) static glad_glGetProgramResourceIndex:                     Option<PFNGLGETPROGRAMRESOURCEINDEXPROC>;
//     pub(super) static glad_glGetProgramResourceLocation:                  Option<PFNGLGETPROGRAMRESOURCELOCATIONPROC>;
//     pub(super) static glad_glGetProgramResourceLocationIndex:             Option<PFNGLGETPROGRAMRESOURCELOCATIONINDEXPROC>;
//     pub(super) static glad_glGetProgramResourceName:                      Option<PFNGLGETPROGRAMRESOURCENAMEPROC>;
//     pub(super) static glad_glGetProgramResourceiv:                        Option<PFNGLGETPROGRAMRESOURCEIVPROC>;
//     pub(super) static glad_glGetProgramStageiv:                           Option<PFNGLGETPROGRAMSTAGEIVPROC>;
//     pub(super) static glad_glGetProgramStringARB:                         Option<PFNGLGETPROGRAMSTRINGARBPROC>;
//     pub(super) static glad_glGetProgramiv:                                Option<PFNGLGETPROGRAMIVPROC>;
//     pub(super) static glad_glGetProgramivARB:                             Option<PFNGLGETPROGRAMIVARBPROC>;
//     pub(super) static glad_glGetQueryBufferObjecti64v:                    Option<PFNGLGETQUERYBUFFEROBJECTI64VPROC>;
//     pub(super) static glad_glGetQueryBufferObjectiv:                      Option<PFNGLGETQUERYBUFFEROBJECTIVPROC>;
//     pub(super) static glad_glGetQueryBufferObjectui64v:                   Option<PFNGLGETQUERYBUFFEROBJECTUI64VPROC>;
//     pub(super) static glad_glGetQueryBufferObjectuiv:                     Option<PFNGLGETQUERYBUFFEROBJECTUIVPROC>;
//     pub(super) static glad_glGetQueryIndexediv:                           Option<PFNGLGETQUERYINDEXEDIVPROC>;
//     pub(super) static glad_glGetQueryObjecti64v:                          Option<PFNGLGETQUERYOBJECTI64VPROC>;
//     pub(super) static glad_glGetQueryObjectiv:                            Option<PFNGLGETQUERYOBJECTIVPROC>;
//     pub(super) static glad_glGetQueryObjectivARB:                         Option<PFNGLGETQUERYOBJECTIVARBPROC>;
//     pub(super) static glad_glGetQueryObjectui64v:                         Option<PFNGLGETQUERYOBJECTUI64VPROC>;
//     pub(super) static glad_glGetQueryObjectuiv:                           Option<PFNGLGETQUERYOBJECTUIVPROC>;
//     pub(super) static glad_glGetQueryObjectuivARB:                        Option<PFNGLGETQUERYOBJECTUIVARBPROC>;
//     pub(super) static glad_glGetQueryiv:                                  Option<PFNGLGETQUERYIVPROC>;
//     pub(super) static glad_glGetQueryivARB:                               Option<PFNGLGETQUERYIVARBPROC>;
//     pub(super) static glad_glGetRenderbufferParameteriv:                  Option<PFNGLGETRENDERBUFFERPARAMETERIVPROC>;
//     pub(super) static glad_glGetRenderbufferParameterivEXT:               Option<PFNGLGETRENDERBUFFERPARAMETERIVEXTPROC>;
//     pub(super) static glad_glGetSamplerParameterIiv:                      Option<PFNGLGETSAMPLERPARAMETERIIVPROC>;
//     pub(super) static glad_glGetSamplerParameterIuiv:                     Option<PFNGLGETSAMPLERPARAMETERIUIVPROC>;
//     pub(super) static glad_glGetSamplerParameterfv:                       Option<PFNGLGETSAMPLERPARAMETERFVPROC>;
//     pub(super) static glad_glGetSamplerParameteriv:                       Option<PFNGLGETSAMPLERPARAMETERIVPROC>;
//     pub(super) static glad_glGetShaderInfoLog:                            Option<PFNGLGETSHADERINFOLOGPROC>;
//     pub(super) static glad_glGetShaderPrecisionFormat:                    Option<PFNGLGETSHADERPRECISIONFORMATPROC>;
//     pub(super) static glad_glGetShaderSource:                             Option<PFNGLGETSHADERSOURCEPROC>;
//     pub(super) static glad_glGetShaderSourceARB:                          Option<PFNGLGETSHADERSOURCEARBPROC>;
//     pub(super) static glad_glGetShaderiv:                                 Option<PFNGLGETSHADERIVPROC>;
//     pub(super) static glad_glGetString:                                   Option<PFNGLGETSTRINGPROC>;
//     pub(super) static glad_glGetStringi:                                  Option<PFNGLGETSTRINGIPROC>;
//     pub(super) static glad_glGetSubroutineIndex:                          Option<PFNGLGETSUBROUTINEINDEXPROC>;
//     pub(super) static glad_glGetSubroutineUniformLocation:                Option<PFNGLGETSUBROUTINEUNIFORMLOCATIONPROC>;
//     pub(super) static glad_glGetSynciv:                                   Option<PFNGLGETSYNCIVPROC>;
//     pub(super) static glad_glGetTexEnvxvOES:                              Option<PFNGLGETTEXENVXVOESPROC>;
//     pub(super) static glad_glGetTexGenxvOES:                              Option<PFNGLGETTEXGENXVOESPROC>;
//     pub(super) static glad_glGetTexImage:                                 Option<PFNGLGETTEXIMAGEPROC>;
//     pub(super) static glad_glGetTexLevelParameterfv:                      Option<PFNGLGETTEXLEVELPARAMETERFVPROC>;
//     pub(super) static glad_glGetTexLevelParameteriv:                      Option<PFNGLGETTEXLEVELPARAMETERIVPROC>;
//     pub(super) static glad_glGetTexLevelParameterxvOES:                   Option<PFNGLGETTEXLEVELPARAMETERXVOESPROC>;
//     pub(super) static glad_glGetTexParameterIiv:                          Option<PFNGLGETTEXPARAMETERIIVPROC>;
//     pub(super) static glad_glGetTexParameterIuiv:                         Option<PFNGLGETTEXPARAMETERIUIVPROC>;
//     pub(super) static glad_glGetTexParameterfv:                           Option<PFNGLGETTEXPARAMETERFVPROC>;
//     pub(super) static glad_glGetTexParameteriv:                           Option<PFNGLGETTEXPARAMETERIVPROC>;
//     pub(super) static glad_glGetTexParameterxvOES:                        Option<PFNGLGETTEXPARAMETERXVOESPROC>;
//     pub(super) static glad_glGetTextureImage:                             Option<PFNGLGETTEXTUREIMAGEPROC>;
//     pub(super) static glad_glGetTextureLevelParameterfv:                  Option<PFNGLGETTEXTURELEVELPARAMETERFVPROC>;
//     pub(super) static glad_glGetTextureLevelParameteriv:                  Option<PFNGLGETTEXTURELEVELPARAMETERIVPROC>;
//     pub(super) static glad_glGetTextureParameterIiv:                      Option<PFNGLGETTEXTUREPARAMETERIIVPROC>;
//     pub(super) static glad_glGetTextureParameterIuiv:                     Option<PFNGLGETTEXTUREPARAMETERIUIVPROC>;
//     pub(super) static glad_glGetTextureParameterfv:                       Option<PFNGLGETTEXTUREPARAMETERFVPROC>;
//     pub(super) static glad_glGetTextureParameteriv:                       Option<PFNGLGETTEXTUREPARAMETERIVPROC>;
//     pub(super) static glad_glGetTextureSubImage:                          Option<PFNGLGETTEXTURESUBIMAGEPROC>;
//     pub(super) static glad_glGetTransformFeedbackVarying:                 Option<PFNGLGETTRANSFORMFEEDBACKVARYINGPROC>;
//     pub(super) static glad_glGetTransformFeedbacki64_v:                   Option<PFNGLGETTRANSFORMFEEDBACKI64_VPROC>;
//     pub(super) static glad_glGetTransformFeedbacki_v:                     Option<PFNGLGETTRANSFORMFEEDBACKI_VPROC>;
//     pub(super) static glad_glGetTransformFeedbackiv:                      Option<PFNGLGETTRANSFORMFEEDBACKIVPROC>;
//     pub(super) static glad_glGetUniformBlockIndex:                        Option<PFNGLGETUNIFORMBLOCKINDEXPROC>;
//     pub(super) static glad_glGetUniformIndices:                           Option<PFNGLGETUNIFORMINDICESPROC>;
//     pub(super) static glad_glGetUniformLocation:                          Option<PFNGLGETUNIFORMLOCATIONPROC>;
//     pub(super) static glad_glGetUniformLocationARB:                       Option<PFNGLGETUNIFORMLOCATIONARBPROC>;
//     pub(super) static glad_glGetUniformSubroutineuiv:                     Option<PFNGLGETUNIFORMSUBROUTINEUIVPROC>;
//     pub(super) static glad_glGetUniformdv:                                Option<PFNGLGETUNIFORMDVPROC>;
//     pub(super) static glad_glGetUniformfv:                                Option<PFNGLGETUNIFORMFVPROC>;
//     pub(super) static glad_glGetUniformfvARB:                             Option<PFNGLGETUNIFORMFVARBPROC>;
//     pub(super) static glad_glGetUniformi64vARB:                           Option<PFNGLGETUNIFORMI64VARBPROC>;
//     pub(super) static glad_glGetUniformiv:                                Option<PFNGLGETUNIFORMIVPROC>;
//     pub(super) static glad_glGetUniformivARB:                             Option<PFNGLGETUNIFORMIVARBPROC>;
//     pub(super) static glad_glGetUniformui64vARB:                          Option<PFNGLGETUNIFORMUI64VARBPROC>;
//     pub(super) static glad_glGetUniformuiv:                               Option<PFNGLGETUNIFORMUIVPROC>;
//     pub(super) static glad_glGetVertexArrayIndexed64iv:                   Option<PFNGLGETVERTEXARRAYINDEXED64IVPROC>;
//     pub(super) static glad_glGetVertexArrayIndexediv:                     Option<PFNGLGETVERTEXARRAYINDEXEDIVPROC>;
//     pub(super) static glad_glGetVertexArrayiv:                            Option<PFNGLGETVERTEXARRAYIVPROC>;
//     pub(super) static glad_glGetVertexAttribIiv:                          Option<PFNGLGETVERTEXATTRIBIIVPROC>;
//     pub(super) static glad_glGetVertexAttribIuiv:                         Option<PFNGLGETVERTEXATTRIBIUIVPROC>;
//     pub(super) static glad_glGetVertexAttribLdv:                          Option<PFNGLGETVERTEXATTRIBLDVPROC>;
//     pub(super) static glad_glGetVertexAttribPointerv:                     Option<PFNGLGETVERTEXATTRIBPOINTERVPROC>;
//     pub(super) static glad_glGetVertexAttribPointervARB:                  Option<PFNGLGETVERTEXATTRIBPOINTERVARBPROC>;
//     pub(super) static glad_glGetVertexAttribdv:                           Option<PFNGLGETVERTEXATTRIBDVPROC>;
//     pub(super) static glad_glGetVertexAttribdvARB:                        Option<PFNGLGETVERTEXATTRIBDVARBPROC>;
//     pub(super) static glad_glGetVertexAttribfv:                           Option<PFNGLGETVERTEXATTRIBFVPROC>;
//     pub(super) static glad_glGetVertexAttribfvARB:                        Option<PFNGLGETVERTEXATTRIBFVARBPROC>;
//     pub(super) static glad_glGetVertexAttribiv:                           Option<PFNGLGETVERTEXATTRIBIVPROC>;
//     pub(super) static glad_glGetVertexAttribivARB:                        Option<PFNGLGETVERTEXATTRIBIVARBPROC>;
//     pub(super) static glad_glGetnUniformi64vARB:                          Option<PFNGLGETNUNIFORMI64VARBPROC>;
//     pub(super) static glad_glGetnUniformui64vARB:                         Option<PFNGLGETNUNIFORMUI64VARBPROC>;
//     pub(super) static glad_glHint:                                        Option<PFNGLHINTPROC>;
//     pub(super) static glad_glIndexxOES:                                   Option<PFNGLINDEXXOESPROC>;
//     pub(super) static glad_glIndexxvOES:                                  Option<PFNGLINDEXXVOESPROC>;
//     pub(super) static glad_glInvalidateBufferData:                        Option<PFNGLINVALIDATEBUFFERDATAPROC>;
//     pub(super) static glad_glInvalidateBufferSubData:                     Option<PFNGLINVALIDATEBUFFERSUBDATAPROC>;
//     pub(super) static glad_glInvalidateFramebuffer:                       Option<PFNGLINVALIDATEFRAMEBUFFERPROC>;
//     pub(super) static glad_glInvalidateNamedFramebufferData:              Option<PFNGLINVALIDATENAMEDFRAMEBUFFERDATAPROC>;
//     pub(super) static glad_glInvalidateNamedFramebufferSubData:           Option<PFNGLINVALIDATENAMEDFRAMEBUFFERSUBDATAPROC>;
//     pub(super) static glad_glInvalidateSubFramebuffer:                    Option<PFNGLINVALIDATESUBFRAMEBUFFERPROC>;
//     pub(super) static glad_glInvalidateTexImage:                          Option<PFNGLINVALIDATETEXIMAGEPROC>;
//     pub(super) static glad_glInvalidateTexSubImage:                       Option<PFNGLINVALIDATETEXSUBIMAGEPROC>;
//     pub(super) static glad_glIsBuffer:                                    Option<PFNGLISBUFFERPROC>;
//     pub(super) static glad_glIsBufferARB:                                 Option<PFNGLISBUFFERARBPROC>;
//     pub(super) static glad_glIsEnabled:                                   Option<PFNGLISENABLEDPROC>;
//     pub(super) static glad_glIsEnabledi:                                  Option<PFNGLISENABLEDIPROC>;
//     pub(super) static glad_glIsFramebuffer:                               Option<PFNGLISFRAMEBUFFERPROC>;
//     pub(super) static glad_glIsFramebufferEXT:                            Option<PFNGLISFRAMEBUFFEREXTPROC>;
//     pub(super) static glad_glIsNamedStringARB:                            Option<PFNGLISNAMEDSTRINGARBPROC>;
//     pub(super) static glad_glIsProgram:                                   Option<PFNGLISPROGRAMPROC>;
//     pub(super) static glad_glIsProgramARB:                                Option<PFNGLISPROGRAMARBPROC>;
//     pub(super) static glad_glIsProgramPipeline:                           Option<PFNGLISPROGRAMPIPELINEPROC>;
//     pub(super) static glad_glIsQuery:                                     Option<PFNGLISQUERYPROC>;
//     pub(super) static glad_glIsQueryARB:                                  Option<PFNGLISQUERYARBPROC>;
//     pub(super) static glad_glIsRenderbuffer:                              Option<PFNGLISRENDERBUFFERPROC>;
//     pub(super) static glad_glIsRenderbufferEXT:                           Option<PFNGLISRENDERBUFFEREXTPROC>;
//     pub(super) static glad_glIsSampler:                                   Option<PFNGLISSAMPLERPROC>;
//     pub(super) static glad_glIsShader:                                    Option<PFNGLISSHADERPROC>;
//     pub(super) static glad_glIsSync:                                      Option<PFNGLISSYNCPROC>;
//     pub(super) static glad_glIsTexture:                                   Option<PFNGLISTEXTUREPROC>;
//     pub(super) static glad_glIsTransformFeedback:                         Option<PFNGLISTRANSFORMFEEDBACKPROC>;
//     pub(super) static glad_glIsVertexArray:                               Option<PFNGLISVERTEXARRAYPROC>;
//     pub(super) static glad_glLightModelxOES:                              Option<PFNGLLIGHTMODELXOESPROC>;
//     pub(super) static glad_glLightModelxvOES:                             Option<PFNGLLIGHTMODELXVOESPROC>;
//     pub(super) static glad_glLightxOES:                                   Option<PFNGLLIGHTXOESPROC>;
//     pub(super) static glad_glLightxvOES:                                  Option<PFNGLLIGHTXVOESPROC>;
//     pub(super) static glad_glLineWidth:                                   Option<PFNGLLINEWIDTHPROC>;
//     pub(super) static glad_glLineWidthxOES:                               Option<PFNGLLINEWIDTHXOESPROC>;
//     pub(super) static glad_glLinkProgram:                                 Option<PFNGLLINKPROGRAMPROC>;
//     pub(super) static glad_glLinkProgramARB:                              Option<PFNGLLINKPROGRAMARBPROC>;
//     pub(super) static glad_glLoadMatrixxOES:                              Option<PFNGLLOADMATRIXXOESPROC>;
//     pub(super) static glad_glLoadTransposeMatrixdARB:                     Option<PFNGLLOADTRANSPOSEMATRIXDARBPROC>;
//     pub(super) static glad_glLoadTransposeMatrixfARB:                     Option<PFNGLLOADTRANSPOSEMATRIXFARBPROC>;
//     pub(super) static glad_glLoadTransposeMatrixxOES:                     Option<PFNGLLOADTRANSPOSEMATRIXXOESPROC>;
//     pub(super) static glad_glLogicOp:                                     Option<PFNGLLOGICOPPROC>;
//     pub(super) static glad_glMap1xOES:                                    Option<PFNGLMAP1XOESPROC>;
//     pub(super) static glad_glMap2xOES:                                    Option<PFNGLMAP2XOESPROC>;
//     pub(super) static glad_glMapBuffer:                                   Option<PFNGLMAPBUFFERPROC>;
//     pub(super) static glad_glMapBufferARB:                                Option<PFNGLMAPBUFFERARBPROC>;
//     pub(super) static glad_glMapBufferRange:                              Option<PFNGLMAPBUFFERRANGEPROC>;
//     pub(super) static glad_glMapGrid1xOES:                                Option<PFNGLMAPGRID1XOESPROC>;
//     pub(super) static glad_glMapGrid2xOES:                                Option<PFNGLMAPGRID2XOESPROC>;
//     pub(super) static glad_glMapNamedBuffer:                              Option<PFNGLMAPNAMEDBUFFERPROC>;
//     pub(super) static glad_glMapNamedBufferRange:                         Option<PFNGLMAPNAMEDBUFFERRANGEPROC>;
//     pub(super) static glad_glMaterialxOES:                                Option<PFNGLMATERIALXOESPROC>;
//     pub(super) static glad_glMaterialxvOES:                               Option<PFNGLMATERIALXVOESPROC>;
//     pub(super) static glad_glMemoryBarrier:                               Option<PFNGLMEMORYBARRIERPROC>;
//     pub(super) static glad_glMemoryBarrierByRegion:                       Option<PFNGLMEMORYBARRIERBYREGIONPROC>;
//     pub(super) static glad_glMinSampleShading:                            Option<PFNGLMINSAMPLESHADINGPROC>;
//     pub(super) static glad_glMinSampleShadingARB:                         Option<PFNGLMINSAMPLESHADINGARBPROC>;
//     pub(super) static glad_glMultMatrixxOES:                              Option<PFNGLMULTMATRIXXOESPROC>;
//     pub(super) static glad_glMultTransposeMatrixdARB:                     Option<PFNGLMULTTRANSPOSEMATRIXDARBPROC>;
//     pub(super) static glad_glMultTransposeMatrixfARB:                     Option<PFNGLMULTTRANSPOSEMATRIXFARBPROC>;
//     pub(super) static glad_glMultTransposeMatrixxOES:                     Option<PFNGLMULTTRANSPOSEMATRIXXOESPROC>;
//     pub(super) static glad_glMultiDrawArrays:                             Option<PFNGLMULTIDRAWARRAYSPROC>;
//     pub(super) static glad_glMultiDrawArraysIndirect:                     Option<PFNGLMULTIDRAWARRAYSINDIRECTPROC>;
//     pub(super) static glad_glMultiDrawElements:                           Option<PFNGLMULTIDRAWELEMENTSPROC>;
//     pub(super) static glad_glMultiDrawElementsBaseVertex:                 Option<PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC>;
//     pub(super) static glad_glMultiDrawElementsIndirect:                   Option<PFNGLMULTIDRAWELEMENTSINDIRECTPROC>;
//     pub(super) static glad_glMultiTexCoord1dARB:                          Option<PFNGLMULTITEXCOORD1DARBPROC>;
//     pub(super) static glad_glMultiTexCoord1dvARB:                         Option<PFNGLMULTITEXCOORD1DVARBPROC>;
//     pub(super) static glad_glMultiTexCoord1fARB:                          Option<PFNGLMULTITEXCOORD1FARBPROC>;
//     pub(super) static glad_glMultiTexCoord1fvARB:                         Option<PFNGLMULTITEXCOORD1FVARBPROC>;
//     pub(super) static glad_glMultiTexCoord1iARB:                          Option<PFNGLMULTITEXCOORD1IARBPROC>;
//     pub(super) static glad_glMultiTexCoord1ivARB:                         Option<PFNGLMULTITEXCOORD1IVARBPROC>;
//     pub(super) static glad_glMultiTexCoord1sARB:                          Option<PFNGLMULTITEXCOORD1SARBPROC>;
//     pub(super) static glad_glMultiTexCoord1svARB:                         Option<PFNGLMULTITEXCOORD1SVARBPROC>;
//     pub(super) static glad_glMultiTexCoord1xOES:                          Option<PFNGLMULTITEXCOORD1XOESPROC>;
//     pub(super) static glad_glMultiTexCoord1xvOES:                         Option<PFNGLMULTITEXCOORD1XVOESPROC>;
//     pub(super) static glad_glMultiTexCoord2dARB:                          Option<PFNGLMULTITEXCOORD2DARBPROC>;
//     pub(super) static glad_glMultiTexCoord2dvARB:                         Option<PFNGLMULTITEXCOORD2DVARBPROC>;
//     pub(super) static glad_glMultiTexCoord2fARB:                          Option<PFNGLMULTITEXCOORD2FARBPROC>;
//     pub(super) static glad_glMultiTexCoord2fvARB:                         Option<PFNGLMULTITEXCOORD2FVARBPROC>;
//     pub(super) static glad_glMultiTexCoord2iARB:                          Option<PFNGLMULTITEXCOORD2IARBPROC>;
//     pub(super) static glad_glMultiTexCoord2ivARB:                         Option<PFNGLMULTITEXCOORD2IVARBPROC>;
//     pub(super) static glad_glMultiTexCoord2sARB:                          Option<PFNGLMULTITEXCOORD2SARBPROC>;
//     pub(super) static glad_glMultiTexCoord2svARB:                         Option<PFNGLMULTITEXCOORD2SVARBPROC>;
//     pub(super) static glad_glMultiTexCoord2xOES:                          Option<PFNGLMULTITEXCOORD2XOESPROC>;
//     pub(super) static glad_glMultiTexCoord2xvOES:                         Option<PFNGLMULTITEXCOORD2XVOESPROC>;
//     pub(super) static glad_glMultiTexCoord3dARB:                          Option<PFNGLMULTITEXCOORD3DARBPROC>;
//     pub(super) static glad_glMultiTexCoord3dvARB:                         Option<PFNGLMULTITEXCOORD3DVARBPROC>;
//     pub(super) static glad_glMultiTexCoord3fARB:                          Option<PFNGLMULTITEXCOORD3FARBPROC>;
//     pub(super) static glad_glMultiTexCoord3fvARB:                         Option<PFNGLMULTITEXCOORD3FVARBPROC>;
//     pub(super) static glad_glMultiTexCoord3iARB:                          Option<PFNGLMULTITEXCOORD3IARBPROC>;
//     pub(super) static glad_glMultiTexCoord3ivARB:                         Option<PFNGLMULTITEXCOORD3IVARBPROC>;
//     pub(super) static glad_glMultiTexCoord3sARB:                          Option<PFNGLMULTITEXCOORD3SARBPROC>;
//     pub(super) static glad_glMultiTexCoord3svARB:                         Option<PFNGLMULTITEXCOORD3SVARBPROC>;
//     pub(super) static glad_glMultiTexCoord3xOES:                          Option<PFNGLMULTITEXCOORD3XOESPROC>;
//     pub(super) static glad_glMultiTexCoord3xvOES:                         Option<PFNGLMULTITEXCOORD3XVOESPROC>;
//     pub(super) static glad_glMultiTexCoord4dARB:                          Option<PFNGLMULTITEXCOORD4DARBPROC>;
//     pub(super) static glad_glMultiTexCoord4dvARB:                         Option<PFNGLMULTITEXCOORD4DVARBPROC>;
//     pub(super) static glad_glMultiTexCoord4fARB:                          Option<PFNGLMULTITEXCOORD4FARBPROC>;
//     pub(super) static glad_glMultiTexCoord4fvARB:                         Option<PFNGLMULTITEXCOORD4FVARBPROC>;
//     pub(super) static glad_glMultiTexCoord4iARB:                          Option<PFNGLMULTITEXCOORD4IARBPROC>;
//     pub(super) static glad_glMultiTexCoord4ivARB:                         Option<PFNGLMULTITEXCOORD4IVARBPROC>;
//     pub(super) static glad_glMultiTexCoord4sARB:                          Option<PFNGLMULTITEXCOORD4SARBPROC>;
//     pub(super) static glad_glMultiTexCoord4svARB:                         Option<PFNGLMULTITEXCOORD4SVARBPROC>;
//     pub(super) static glad_glMultiTexCoord4xOES:                          Option<PFNGLMULTITEXCOORD4XOESPROC>;
//     pub(super) static glad_glMultiTexCoord4xvOES:                         Option<PFNGLMULTITEXCOORD4XVOESPROC>;
//     pub(super) static glad_glNamedBufferData:                             Option<PFNGLNAMEDBUFFERDATAPROC>;
//     pub(super) static glad_glNamedBufferStorage:                          Option<PFNGLNAMEDBUFFERSTORAGEPROC>;
//     pub(super) static glad_glNamedBufferSubData:                          Option<PFNGLNAMEDBUFFERSUBDATAPROC>;
//     pub(super) static glad_glNamedFramebufferDrawBuffer:                  Option<PFNGLNAMEDFRAMEBUFFERDRAWBUFFERPROC>;
//     pub(super) static glad_glNamedFramebufferDrawBuffers:                 Option<PFNGLNAMEDFRAMEBUFFERDRAWBUFFERSPROC>;
//     pub(super) static glad_glNamedFramebufferParameteri:                  Option<PFNGLNAMEDFRAMEBUFFERPARAMETERIPROC>;
//     pub(super) static glad_glNamedFramebufferReadBuffer:                  Option<PFNGLNAMEDFRAMEBUFFERREADBUFFERPROC>;
//     pub(super) static glad_glNamedFramebufferRenderbuffer:                Option<PFNGLNAMEDFRAMEBUFFERRENDERBUFFERPROC>;
//     pub(super) static glad_glNamedFramebufferSampleLocationsfvARB:        Option<PFNGLNAMEDFRAMEBUFFERSAMPLELOCATIONSFVARBPROC>;
//     pub(super) static glad_glNamedFramebufferTexture:                     Option<PFNGLNAMEDFRAMEBUFFERTEXTUREPROC>;
//     pub(super) static glad_glNamedFramebufferTextureLayer:                Option<PFNGLNAMEDFRAMEBUFFERTEXTURELAYERPROC>;
//     pub(super) static glad_glNamedRenderbufferStorage:                    Option<PFNGLNAMEDRENDERBUFFERSTORAGEPROC>;
//     pub(super) static glad_glNamedRenderbufferStorageMultisample:         Option<PFNGLNAMEDRENDERBUFFERSTORAGEMULTISAMPLEPROC>;
//     pub(super) static glad_glNamedStringARB:                              Option<PFNGLNAMEDSTRINGARBPROC>;
//     pub(super) static glad_glNormal3xOES:                                 Option<PFNGLNORMAL3XOESPROC>;
//     pub(super) static glad_glNormal3xvOES:                                Option<PFNGLNORMAL3XVOESPROC>;
//     pub(super) static glad_glObjectLabel:                                 Option<PFNGLOBJECTLABELPROC>;
//     pub(super) static glad_glObjectPtrLabel:                              Option<PFNGLOBJECTPTRLABELPROC>;
//     pub(super) static glad_glOrthoxOES:                                   Option<PFNGLORTHOXOESPROC>;
//     pub(super) static glad_glPassThroughxOES:                             Option<PFNGLPASSTHROUGHXOESPROC>;
//     pub(super) static glad_glPatchParameterfv:                            Option<PFNGLPATCHPARAMETERFVPROC>;
//     pub(super) static glad_glPatchParameteri:                             Option<PFNGLPATCHPARAMETERIPROC>;
//     pub(super) static glad_glPauseTransformFeedback:                      Option<PFNGLPAUSETRANSFORMFEEDBACKPROC>;
//     pub(super) static glad_glPixelMapx:                                   Option<PFNGLPIXELMAPXPROC>;
//     pub(super) static glad_glPixelStoref:                                 Option<PFNGLPIXELSTOREFPROC>;
//     pub(super) static glad_glPixelStorei:                                 Option<PFNGLPIXELSTOREIPROC>;
//     pub(super) static glad_glPixelStorex:                                 Option<PFNGLPIXELSTOREXPROC>;
//     pub(super) static glad_glPixelTransferxOES:                           Option<PFNGLPIXELTRANSFERXOESPROC>;
//     pub(super) static glad_glPixelZoomxOES:                               Option<PFNGLPIXELZOOMXOESPROC>;
//     pub(super) static glad_glPointParameterf:                             Option<PFNGLPOINTPARAMETERFPROC>;
//     pub(super) static glad_glPointParameterfv:                            Option<PFNGLPOINTPARAMETERFVPROC>;
//     pub(super) static glad_glPointParameteri:                             Option<PFNGLPOINTPARAMETERIPROC>;
//     pub(super) static glad_glPointParameteriv:                            Option<PFNGLPOINTPARAMETERIVPROC>;
//     pub(super) static glad_glPointParameterxvOES:                         Option<PFNGLPOINTPARAMETERXVOESPROC>;
//     pub(super) static glad_glPointSize:                                   Option<PFNGLPOINTSIZEPROC>;
//     pub(super) static glad_glPointSizexOES:                               Option<PFNGLPOINTSIZEXOESPROC>;
//     pub(super) static glad_glPolygonMode:                                 Option<PFNGLPOLYGONMODEPROC>;
//     pub(super) static glad_glPolygonOffset:                               Option<PFNGLPOLYGONOFFSETPROC>;
//     pub(super) static glad_glPolygonOffsetxOES:                           Option<PFNGLPOLYGONOFFSETXOESPROC>;
//     pub(super) static glad_glPopDebugGroup:                               Option<PFNGLPOPDEBUGGROUPPROC>;
//     pub(super) static glad_glPrimitiveBoundingBoxARB:                     Option<PFNGLPRIMITIVEBOUNDINGBOXARBPROC>;
//     pub(super) static glad_glPrimitiveRestartIndex:                       Option<PFNGLPRIMITIVERESTARTINDEXPROC>;
//     pub(super) static glad_glPrioritizeTexturesxOES:                      Option<PFNGLPRIORITIZETEXTURESXOESPROC>;
//     pub(super) static glad_glProgramBinary:                               Option<PFNGLPROGRAMBINARYPROC>;
//     pub(super) static glad_glProgramEnvParameter4dARB:                    Option<PFNGLPROGRAMENVPARAMETER4DARBPROC>;
//     pub(super) static glad_glProgramEnvParameter4dvARB:                   Option<PFNGLPROGRAMENVPARAMETER4DVARBPROC>;
//     pub(super) static glad_glProgramEnvParameter4fARB:                    Option<PFNGLPROGRAMENVPARAMETER4FARBPROC>;
//     pub(super) static glad_glProgramEnvParameter4fvARB:                   Option<PFNGLPROGRAMENVPARAMETER4FVARBPROC>;
//     pub(super) static glad_glProgramLocalParameter4dARB:                  Option<PFNGLPROGRAMLOCALPARAMETER4DARBPROC>;
//     pub(super) static glad_glProgramLocalParameter4dvARB:                 Option<PFNGLPROGRAMLOCALPARAMETER4DVARBPROC>;
//     pub(super) static glad_glProgramLocalParameter4fARB:                  Option<PFNGLPROGRAMLOCALPARAMETER4FARBPROC>;
//     pub(super) static glad_glProgramLocalParameter4fvARB:                 Option<PFNGLPROGRAMLOCALPARAMETER4FVARBPROC>;
//     pub(super) static glad_glProgramParameteri:                           Option<PFNGLPROGRAMPARAMETERIPROC>;
//     pub(super) static glad_glProgramParameteriARB:                        Option<PFNGLPROGRAMPARAMETERIARBPROC>;
//     pub(super) static glad_glProgramStringARB:                            Option<PFNGLPROGRAMSTRINGARBPROC>;
//     pub(super) static glad_glProgramUniform1d:                            Option<PFNGLPROGRAMUNIFORM1DPROC>;
//     pub(super) static glad_glProgramUniform1dv:                           Option<PFNGLPROGRAMUNIFORM1DVPROC>;
//     pub(super) static glad_glProgramUniform1f:                            Option<PFNGLPROGRAMUNIFORM1FPROC>;
//     pub(super) static glad_glProgramUniform1fv:                           Option<PFNGLPROGRAMUNIFORM1FVPROC>;
//     pub(super) static glad_glProgramUniform1i:                            Option<PFNGLPROGRAMUNIFORM1IPROC>;
//     pub(super) static glad_glProgramUniform1i64ARB:                       Option<PFNGLPROGRAMUNIFORM1I64ARBPROC>;
//     pub(super) static glad_glProgramUniform1i64vARB:                      Option<PFNGLPROGRAMUNIFORM1I64VARBPROC>;
//     pub(super) static glad_glProgramUniform1iv:                           Option<PFNGLPROGRAMUNIFORM1IVPROC>;
//     pub(super) static glad_glProgramUniform1ui:                           Option<PFNGLPROGRAMUNIFORM1UIPROC>;
//     pub(super) static glad_glProgramUniform1ui64ARB:                      Option<PFNGLPROGRAMUNIFORM1UI64ARBPROC>;
//     pub(super) static glad_glProgramUniform1ui64vARB:                     Option<PFNGLPROGRAMUNIFORM1UI64VARBPROC>;
//     pub(super) static glad_glProgramUniform1uiv:                          Option<PFNGLPROGRAMUNIFORM1UIVPROC>;
//     pub(super) static glad_glProgramUniform2d:                            Option<PFNGLPROGRAMUNIFORM2DPROC>;
//     pub(super) static glad_glProgramUniform2dv:                           Option<PFNGLPROGRAMUNIFORM2DVPROC>;
//     pub(super) static glad_glProgramUniform2f:                            Option<PFNGLPROGRAMUNIFORM2FPROC>;
//     pub(super) static glad_glProgramUniform2fv:                           Option<PFNGLPROGRAMUNIFORM2FVPROC>;
//     pub(super) static glad_glProgramUniform2i:                            Option<PFNGLPROGRAMUNIFORM2IPROC>;
//     pub(super) static glad_glProgramUniform2i64ARB:                       Option<PFNGLPROGRAMUNIFORM2I64ARBPROC>;
//     pub(super) static glad_glProgramUniform2i64vARB:                      Option<PFNGLPROGRAMUNIFORM2I64VARBPROC>;
//     pub(super) static glad_glProgramUniform2iv:                           Option<PFNGLPROGRAMUNIFORM2IVPROC>;
//     pub(super) static glad_glProgramUniform2ui:                           Option<PFNGLPROGRAMUNIFORM2UIPROC>;
//     pub(super) static glad_glProgramUniform2ui64ARB:                      Option<PFNGLPROGRAMUNIFORM2UI64ARBPROC>;
//     pub(super) static glad_glProgramUniform2ui64vARB:                     Option<PFNGLPROGRAMUNIFORM2UI64VARBPROC>;
//     pub(super) static glad_glProgramUniform2uiv:                          Option<PFNGLPROGRAMUNIFORM2UIVPROC>;
//     pub(super) static glad_glProgramUniform3d:                            Option<PFNGLPROGRAMUNIFORM3DPROC>;
//     pub(super) static glad_glProgramUniform3dv:                           Option<PFNGLPROGRAMUNIFORM3DVPROC>;
//     pub(super) static glad_glProgramUniform3f:                            Option<PFNGLPROGRAMUNIFORM3FPROC>;
//     pub(super) static glad_glProgramUniform3fv:                           Option<PFNGLPROGRAMUNIFORM3FVPROC>;
//     pub(super) static glad_glProgramUniform3i:                            Option<PFNGLPROGRAMUNIFORM3IPROC>;
//     pub(super) static glad_glProgramUniform3i64ARB:                       Option<PFNGLPROGRAMUNIFORM3I64ARBPROC>;
//     pub(super) static glad_glProgramUniform3i64vARB:                      Option<PFNGLPROGRAMUNIFORM3I64VARBPROC>;
//     pub(super) static glad_glProgramUniform3iv:                           Option<PFNGLPROGRAMUNIFORM3IVPROC>;
//     pub(super) static glad_glProgramUniform3ui:                           Option<PFNGLPROGRAMUNIFORM3UIPROC>;
//     pub(super) static glad_glProgramUniform3ui64ARB:                      Option<PFNGLPROGRAMUNIFORM3UI64ARBPROC>;
//     pub(super) static glad_glProgramUniform3ui64vARB:                     Option<PFNGLPROGRAMUNIFORM3UI64VARBPROC>;
//     pub(super) static glad_glProgramUniform3uiv:                          Option<PFNGLPROGRAMUNIFORM3UIVPROC>;
//     pub(super) static glad_glProgramUniform4d:                            Option<PFNGLPROGRAMUNIFORM4DPROC>;
//     pub(super) static glad_glProgramUniform4dv:                           Option<PFNGLPROGRAMUNIFORM4DVPROC>;
//     pub(super) static glad_glProgramUniform4f:                            Option<PFNGLPROGRAMUNIFORM4FPROC>;
//     pub(super) static glad_glProgramUniform4fv:                           Option<PFNGLPROGRAMUNIFORM4FVPROC>;
//     pub(super) static glad_glProgramUniform4i:                            Option<PFNGLPROGRAMUNIFORM4IPROC>;
//     pub(super) static glad_glProgramUniform4i64ARB:                       Option<PFNGLPROGRAMUNIFORM4I64ARBPROC>;
//     pub(super) static glad_glProgramUniform4i64vARB:                      Option<PFNGLPROGRAMUNIFORM4I64VARBPROC>;
//     pub(super) static glad_glProgramUniform4iv:                           Option<PFNGLPROGRAMUNIFORM4IVPROC>;
//     pub(super) static glad_glProgramUniform4ui:                           Option<PFNGLPROGRAMUNIFORM4UIPROC>;
//     pub(super) static glad_glProgramUniform4ui64ARB:                      Option<PFNGLPROGRAMUNIFORM4UI64ARBPROC>;
//     pub(super) static glad_glProgramUniform4ui64vARB:                     Option<PFNGLPROGRAMUNIFORM4UI64VARBPROC>;
//     pub(super) static glad_glProgramUniform4uiv:                          Option<PFNGLPROGRAMUNIFORM4UIVPROC>;
//     pub(super) static glad_glProgramUniformMatrix2dv:                     Option<PFNGLPROGRAMUNIFORMMATRIX2DVPROC>;
//     pub(super) static glad_glProgramUniformMatrix2fv:                     Option<PFNGLPROGRAMUNIFORMMATRIX2FVPROC>;
//     pub(super) static glad_glProgramUniformMatrix2x3dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX2X3DVPROC>;
//     pub(super) static glad_glProgramUniformMatrix2x3fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX2X3FVPROC>;
//     pub(super) static glad_glProgramUniformMatrix2x4dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX2X4DVPROC>;
//     pub(super) static glad_glProgramUniformMatrix2x4fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX2X4FVPROC>;
//     pub(super) static glad_glProgramUniformMatrix3dv:                     Option<PFNGLPROGRAMUNIFORMMATRIX3DVPROC>;
//     pub(super) static glad_glProgramUniformMatrix3fv:                     Option<PFNGLPROGRAMUNIFORMMATRIX3FVPROC>;
//     pub(super) static glad_glProgramUniformMatrix3x2dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX3X2DVPROC>;
//     pub(super) static glad_glProgramUniformMatrix3x2fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX3X2FVPROC>;
//     pub(super) static glad_glProgramUniformMatrix3x4dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX3X4DVPROC>;
//     pub(super) static glad_glProgramUniformMatrix3x4fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX3X4FVPROC>;
//     pub(super) static glad_glProgramUniformMatrix4dv:                     Option<PFNGLPROGRAMUNIFORMMATRIX4DVPROC>;
//     pub(super) static glad_glProgramUniformMatrix4fv:                     Option<PFNGLPROGRAMUNIFORMMATRIX4FVPROC>;
//     pub(super) static glad_glProgramUniformMatrix4x2dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX4X2DVPROC>;
//     pub(super) static glad_glProgramUniformMatrix4x2fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX4X2FVPROC>;
//     pub(super) static glad_glProgramUniformMatrix4x3dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX4X3DVPROC>;
//     pub(super) static glad_glProgramUniformMatrix4x3fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX4X3FVPROC>;
//     pub(super) static glad_glProvokingVertex:                             Option<PFNGLPROVOKINGVERTEXPROC>;
//     pub(super) static glad_glPushDebugGroup:                              Option<PFNGLPUSHDEBUGGROUPPROC>;
//     pub(super) static glad_glQueryCounter:                                Option<PFNGLQUERYCOUNTERPROC>;
//     pub(super) static glad_glRasterPos2xOES:                              Option<PFNGLRASTERPOS2XOESPROC>;
//     pub(super) static glad_glRasterPos2xvOES:                             Option<PFNGLRASTERPOS2XVOESPROC>;
//     pub(super) static glad_glRasterPos3xOES:                              Option<PFNGLRASTERPOS3XOESPROC>;
//     pub(super) static glad_glRasterPos3xvOES:                             Option<PFNGLRASTERPOS3XVOESPROC>;
//     pub(super) static glad_glRasterPos4xOES:                              Option<PFNGLRASTERPOS4XOESPROC>;
//     pub(super) static glad_glRasterPos4xvOES:                             Option<PFNGLRASTERPOS4XVOESPROC>;
//     pub(super) static glad_glReadBuffer:                                  Option<PFNGLREADBUFFERPROC>;
//     pub(super) static glad_glReadPixels:                                  Option<PFNGLREADPIXELSPROC>;
//     pub(super) static glad_glRectxOES:                                    Option<PFNGLRECTXOESPROC>;
//     pub(super) static glad_glRectxvOES:                                   Option<PFNGLRECTXVOESPROC>;
//     pub(super) static glad_glReleaseShaderCompiler:                       Option<PFNGLRELEASESHADERCOMPILERPROC>;
//     pub(super) static glad_glRenderbufferStorage:                         Option<PFNGLRENDERBUFFERSTORAGEPROC>;
//     pub(super) static glad_glRenderbufferStorageEXT:                      Option<PFNGLRENDERBUFFERSTORAGEEXTPROC>;
//     pub(super) static glad_glRenderbufferStorageMultisample:              Option<PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC>;
//     pub(super) static glad_glRenderbufferStorageMultisampleEXT:           Option<PFNGLRENDERBUFFERSTORAGEMULTISAMPLEEXTPROC>;
//     pub(super) static glad_glResumeTransformFeedback:                     Option<PFNGLRESUMETRANSFORMFEEDBACKPROC>;
//     pub(super) static glad_glRotatexOES:                                  Option<PFNGLROTATEXOESPROC>;
//     pub(super) static glad_glSampleCoverage:                              Option<PFNGLSAMPLECOVERAGEPROC>;
//     pub(super) static glad_glSampleCoverageARB:                           Option<PFNGLSAMPLECOVERAGEARBPROC>;
//     pub(super) static glad_glSampleMaski:                                 Option<PFNGLSAMPLEMASKIPROC>;
//     pub(super) static glad_glSamplerParameterIiv:                         Option<PFNGLSAMPLERPARAMETERIIVPROC>;
//     pub(super) static glad_glSamplerParameterIuiv:                        Option<PFNGLSAMPLERPARAMETERIUIVPROC>;
//     pub(super) static glad_glSamplerParameterf:                           Option<PFNGLSAMPLERPARAMETERFPROC>;
//     pub(super) static glad_glSamplerParameterfv:                          Option<PFNGLSAMPLERPARAMETERFVPROC>;
//     pub(super) static glad_glSamplerParameteri:                           Option<PFNGLSAMPLERPARAMETERIPROC>;
//     pub(super) static glad_glSamplerParameteriv:                          Option<PFNGLSAMPLERPARAMETERIVPROC>;
//     pub(super) static glad_glScalexOES:                                   Option<PFNGLSCALEXOESPROC>;
//     pub(super) static glad_glScissor:                                     Option<PFNGLSCISSORPROC>;
//     pub(super) static glad_glScissorArrayv:                               Option<PFNGLSCISSORARRAYVPROC>;
//     pub(super) static glad_glScissorIndexed:                              Option<PFNGLSCISSORINDEXEDPROC>;
//     pub(super) static glad_glScissorIndexedv:                             Option<PFNGLSCISSORINDEXEDVPROC>;
//     pub(super) static glad_glShaderBinary:                                Option<PFNGLSHADERBINARYPROC>;
//     pub(super) static glad_glShaderSource:                                Option<PFNGLSHADERSOURCEPROC>;
//     pub(super) static glad_glShaderSourceARB:                             Option<PFNGLSHADERSOURCEARBPROC>;
//     pub(super) static glad_glShaderStorageBlockBinding:                   Option<PFNGLSHADERSTORAGEBLOCKBINDINGPROC>;
//     pub(super) static glad_glSpecializeShaderARB:                         Option<PFNGLSPECIALIZESHADERARBPROC>;
//     pub(super) static glad_glStencilFunc:                                 Option<PFNGLSTENCILFUNCPROC>;
//     pub(super) static glad_glStencilFuncSeparate:                         Option<PFNGLSTENCILFUNCSEPARATEPROC>;
//     pub(super) static glad_glStencilMask:                                 Option<PFNGLSTENCILMASKPROC>;
//     pub(super) static glad_glStencilMaskSeparate:                         Option<PFNGLSTENCILMASKSEPARATEPROC>;
//     pub(super) static glad_glStencilOp:                                   Option<PFNGLSTENCILOPPROC>;
//     pub(super) static glad_glStencilOpSeparate:                           Option<PFNGLSTENCILOPSEPARATEPROC>;
//     pub(super) static glad_glTexBuffer:                                   Option<PFNGLTEXBUFFERPROC>;
//     pub(super) static glad_glTexBufferRange:                              Option<PFNGLTEXBUFFERRANGEPROC>;
//     pub(super) static glad_glTexCoord1xOES:                               Option<PFNGLTEXCOORD1XOESPROC>;
//     pub(super) static glad_glTexCoord1xvOES:                              Option<PFNGLTEXCOORD1XVOESPROC>;
//     pub(super) static glad_glTexCoord2xOES:                               Option<PFNGLTEXCOORD2XOESPROC>;
//     pub(super) static glad_glTexCoord2xvOES:                              Option<PFNGLTEXCOORD2XVOESPROC>;
//     pub(super) static glad_glTexCoord3xOES:                               Option<PFNGLTEXCOORD3XOESPROC>;
//     pub(super) static glad_glTexCoord3xvOES:                              Option<PFNGLTEXCOORD3XVOESPROC>;
//     pub(super) static glad_glTexCoord4xOES:                               Option<PFNGLTEXCOORD4XOESPROC>;
//     pub(super) static glad_glTexCoord4xvOES:                              Option<PFNGLTEXCOORD4XVOESPROC>;
//     pub(super) static glad_glTexEnvxOES:                                  Option<PFNGLTEXENVXOESPROC>;
//     pub(super) static glad_glTexEnvxvOES:                                 Option<PFNGLTEXENVXVOESPROC>;
//     pub(super) static glad_glTexGenxOES:                                  Option<PFNGLTEXGENXOESPROC>;
//     pub(super) static glad_glTexGenxvOES:                                 Option<PFNGLTEXGENXVOESPROC>;
//     pub(super) static glad_glTexImage1D:                                  Option<PFNGLTEXIMAGE1DPROC>;
//     pub(super) static glad_glTexImage2D:                                  Option<PFNGLTEXIMAGE2DPROC>;
//     pub(super) static glad_glTexImage2DMultisample:                       Option<PFNGLTEXIMAGE2DMULTISAMPLEPROC>;
//     pub(super) static glad_glTexImage3D:                                  Option<PFNGLTEXIMAGE3DPROC>;
//     pub(super) static glad_glTexImage3DMultisample:                       Option<PFNGLTEXIMAGE3DMULTISAMPLEPROC>;
//     pub(super) static glad_glTexParameterIiv:                             Option<PFNGLTEXPARAMETERIIVPROC>;
//     pub(super) static glad_glTexParameterIuiv:                            Option<PFNGLTEXPARAMETERIUIVPROC>;
//     pub(super) static glad_glTexParameterf:                               Option<PFNGLTEXPARAMETERFPROC>;
//     pub(super) static glad_glTexParameterfv:                              Option<PFNGLTEXPARAMETERFVPROC>;
//     pub(super) static glad_glTexParameteri:                               Option<PFNGLTEXPARAMETERIPROC>;
//     pub(super) static glad_glTexParameteriv:                              Option<PFNGLTEXPARAMETERIVPROC>;
//     pub(super) static glad_glTexParameterxOES:                            Option<PFNGLTEXPARAMETERXOESPROC>;
//     pub(super) static glad_glTexParameterxvOES:                           Option<PFNGLTEXPARAMETERXVOESPROC>;
//     pub(super) static glad_glTexStorage1D:                                Option<PFNGLTEXSTORAGE1DPROC>;
//     pub(super) static glad_glTexStorage2D:                                Option<PFNGLTEXSTORAGE2DPROC>;
//     pub(super) static glad_glTexStorage2DMultisample:                     Option<PFNGLTEXSTORAGE2DMULTISAMPLEPROC>;
//     pub(super) static glad_glTexStorage3D:                                Option<PFNGLTEXSTORAGE3DPROC>;
//     pub(super) static glad_glTexStorage3DMultisample:                     Option<PFNGLTEXSTORAGE3DMULTISAMPLEPROC>;
//     pub(super) static glad_glTexSubImage1D:                               Option<PFNGLTEXSUBIMAGE1DPROC>;
//     pub(super) static glad_glTexSubImage2D:                               Option<PFNGLTEXSUBIMAGE2DPROC>;
//     pub(super) static glad_glTexSubImage3D:                               Option<PFNGLTEXSUBIMAGE3DPROC>;
//     pub(super) static glad_glTextureBuffer:                               Option<PFNGLTEXTUREBUFFERPROC>;
//     pub(super) static glad_glTextureBufferRange:                          Option<PFNGLTEXTUREBUFFERRANGEPROC>;
//     pub(super) static glad_glTextureParameterIiv:                         Option<PFNGLTEXTUREPARAMETERIIVPROC>;
//     pub(super) static glad_glTextureParameterIuiv:                        Option<PFNGLTEXTUREPARAMETERIUIVPROC>;
//     pub(super) static glad_glTextureParameterf:                           Option<PFNGLTEXTUREPARAMETERFPROC>;
//     pub(super) static glad_glTextureParameterfv:                          Option<PFNGLTEXTUREPARAMETERFVPROC>;
//     pub(super) static glad_glTextureParameteri:                           Option<PFNGLTEXTUREPARAMETERIPROC>;
//     pub(super) static glad_glTextureParameteriv:                          Option<PFNGLTEXTUREPARAMETERIVPROC>;
//     pub(super) static glad_glTextureStorage1D:                            Option<PFNGLTEXTURESTORAGE1DPROC>;
//     pub(super) static glad_glTextureStorage2D:                            Option<PFNGLTEXTURESTORAGE2DPROC>;
//     pub(super) static glad_glTextureStorage2DMultisample:                 Option<PFNGLTEXTURESTORAGE2DMULTISAMPLEPROC>;
//     pub(super) static glad_glTextureStorage3D:                            Option<PFNGLTEXTURESTORAGE3DPROC>;
//     pub(super) static glad_glTextureStorage3DMultisample:                 Option<PFNGLTEXTURESTORAGE3DMULTISAMPLEPROC>;
//     pub(super) static glad_glTextureSubImage1D:                           Option<PFNGLTEXTURESUBIMAGE1DPROC>;
//     pub(super) static glad_glTextureSubImage2D:                           Option<PFNGLTEXTURESUBIMAGE2DPROC>;
//     pub(super) static glad_glTextureSubImage3D:                           Option<PFNGLTEXTURESUBIMAGE3DPROC>;
//     pub(super) static glad_glTextureView:                                 Option<PFNGLTEXTUREVIEWPROC>;
//     pub(super) static glad_glTransformFeedbackBufferBase:                 Option<PFNGLTRANSFORMFEEDBACKBUFFERBASEPROC>;
//     pub(super) static glad_glTransformFeedbackBufferRange:                Option<PFNGLTRANSFORMFEEDBACKBUFFERRANGEPROC>;
//     pub(super) static glad_glTransformFeedbackVaryings:                   Option<PFNGLTRANSFORMFEEDBACKVARYINGSPROC>;
//     pub(super) static glad_glTranslatexOES:                               Option<PFNGLTRANSLATEXOESPROC>;
//     pub(super) static glad_glUniform1d:                                   Option<PFNGLUNIFORM1DPROC>;
//     pub(super) static glad_glUniform1dv:                                  Option<PFNGLUNIFORM1DVPROC>;
//     pub(super) static glad_glUniform1f:                                   Option<PFNGLUNIFORM1FPROC>;
//     pub(super) static glad_glUniform1fARB:                                Option<PFNGLUNIFORM1FARBPROC>;
//     pub(super) static glad_glUniform1fv:                                  Option<PFNGLUNIFORM1FVPROC>;
//     pub(super) static glad_glUniform1fvARB:                               Option<PFNGLUNIFORM1FVARBPROC>;
//     pub(super) static glad_glUniform1i:                                   Option<PFNGLUNIFORM1IPROC>;
//     pub(super) static glad_glUniform1i64ARB:                              Option<PFNGLUNIFORM1I64ARBPROC>;
//     pub(super) static glad_glUniform1i64vARB:                             Option<PFNGLUNIFORM1I64VARBPROC>;
//     pub(super) static glad_glUniform1iARB:                                Option<PFNGLUNIFORM1IARBPROC>;
//     pub(super) static glad_glUniform1iv:                                  Option<PFNGLUNIFORM1IVPROC>;
//     pub(super) static glad_glUniform1ivARB:                               Option<PFNGLUNIFORM1IVARBPROC>;
//     pub(super) static glad_glUniform1ui:                                  Option<PFNGLUNIFORM1UIPROC>;
//     pub(super) static glad_glUniform1ui64ARB:                             Option<PFNGLUNIFORM1UI64ARBPROC>;
//     pub(super) static glad_glUniform1ui64vARB:                            Option<PFNGLUNIFORM1UI64VARBPROC>;
//     pub(super) static glad_glUniform1uiv:                                 Option<PFNGLUNIFORM1UIVPROC>;
//     pub(super) static glad_glUniform2d:                                   Option<PFNGLUNIFORM2DPROC>;
//     pub(super) static glad_glUniform2dv:                                  Option<PFNGLUNIFORM2DVPROC>;
//     pub(super) static glad_glUniform2f:                                   Option<PFNGLUNIFORM2FPROC>;
//     pub(super) static glad_glUniform2fARB:                                Option<PFNGLUNIFORM2FARBPROC>;
//     pub(super) static glad_glUniform2fv:                                  Option<PFNGLUNIFORM2FVPROC>;
//     pub(super) static glad_glUniform2fvARB:                               Option<PFNGLUNIFORM2FVARBPROC>;
//     pub(super) static glad_glUniform2i:                                   Option<PFNGLUNIFORM2IPROC>;
//     pub(super) static glad_glUniform2i64ARB:                              Option<PFNGLUNIFORM2I64ARBPROC>;
//     pub(super) static glad_glUniform2i64vARB:                             Option<PFNGLUNIFORM2I64VARBPROC>;
//     pub(super) static glad_glUniform2iARB:                                Option<PFNGLUNIFORM2IARBPROC>;
//     pub(super) static glad_glUniform2iv:                                  Option<PFNGLUNIFORM2IVPROC>;
//     pub(super) static glad_glUniform2ivARB:                               Option<PFNGLUNIFORM2IVARBPROC>;
//     pub(super) static glad_glUniform2ui:                                  Option<PFNGLUNIFORM2UIPROC>;
//     pub(super) static glad_glUniform2ui64ARB:                             Option<PFNGLUNIFORM2UI64ARBPROC>;
//     pub(super) static glad_glUniform2ui64vARB:                            Option<PFNGLUNIFORM2UI64VARBPROC>;
//     pub(super) static glad_glUniform2uiv:                                 Option<PFNGLUNIFORM2UIVPROC>;
//     pub(super) static glad_glUniform3d:                                   Option<PFNGLUNIFORM3DPROC>;
//     pub(super) static glad_glUniform3dv:                                  Option<PFNGLUNIFORM3DVPROC>;
//     pub(super) static glad_glUniform3f:                                   Option<PFNGLUNIFORM3FPROC>;
//     pub(super) static glad_glUniform3fARB:                                Option<PFNGLUNIFORM3FARBPROC>;
//     pub(super) static glad_glUniform3fv:                                  Option<PFNGLUNIFORM3FVPROC>;
//     pub(super) static glad_glUniform3fvARB:                               Option<PFNGLUNIFORM3FVARBPROC>;
//     pub(super) static glad_glUniform3i:                                   Option<PFNGLUNIFORM3IPROC>;
//     pub(super) static glad_glUniform3i64ARB:                              Option<PFNGLUNIFORM3I64ARBPROC>;
//     pub(super) static glad_glUniform3i64vARB:                             Option<PFNGLUNIFORM3I64VARBPROC>;
//     pub(super) static glad_glUniform3iARB:                                Option<PFNGLUNIFORM3IARBPROC>;
//     pub(super) static glad_glUniform3iv:                                  Option<PFNGLUNIFORM3IVPROC>;
//     pub(super) static glad_glUniform3ivARB:                               Option<PFNGLUNIFORM3IVARBPROC>;
//     pub(super) static glad_glUniform3ui:                                  Option<PFNGLUNIFORM3UIPROC>;
//     pub(super) static glad_glUniform3ui64ARB:                             Option<PFNGLUNIFORM3UI64ARBPROC>;
//     pub(super) static glad_glUniform3ui64vARB:                            Option<PFNGLUNIFORM3UI64VARBPROC>;
//     pub(super) static glad_glUniform3uiv:                                 Option<PFNGLUNIFORM3UIVPROC>;
//     pub(super) static glad_glUniform4d:                                   Option<PFNGLUNIFORM4DPROC>;
//     pub(super) static glad_glUniform4dv:                                  Option<PFNGLUNIFORM4DVPROC>;
//     pub(super) static glad_glUniform4f:                                   Option<PFNGLUNIFORM4FPROC>;
//     pub(super) static glad_glUniform4fARB:                                Option<PFNGLUNIFORM4FARBPROC>;
//     pub(super) static glad_glUniform4fv:                                  Option<PFNGLUNIFORM4FVPROC>;
//     pub(super) static glad_glUniform4fvARB:                               Option<PFNGLUNIFORM4FVARBPROC>;
//     pub(super) static glad_glUniform4i:                                   Option<PFNGLUNIFORM4IPROC>;
//     pub(super) static glad_glUniform4i64ARB:                              Option<PFNGLUNIFORM4I64ARBPROC>;
//     pub(super) static glad_glUniform4i64vARB:                             Option<PFNGLUNIFORM4I64VARBPROC>;
//     pub(super) static glad_glUniform4iARB:                                Option<PFNGLUNIFORM4IARBPROC>;
//     pub(super) static glad_glUniform4iv:                                  Option<PFNGLUNIFORM4IVPROC>;
//     pub(super) static glad_glUniform4ivARB:                               Option<PFNGLUNIFORM4IVARBPROC>;
//     pub(super) static glad_glUniform4ui:                                  Option<PFNGLUNIFORM4UIPROC>;
//     pub(super) static glad_glUniform4ui64ARB:                             Option<PFNGLUNIFORM4UI64ARBPROC>;
//     pub(super) static glad_glUniform4ui64vARB:                            Option<PFNGLUNIFORM4UI64VARBPROC>;
//     pub(super) static glad_glUniform4uiv:                                 Option<PFNGLUNIFORM4UIVPROC>;
//     pub(super) static glad_glUniformBlockBinding:                         Option<PFNGLUNIFORMBLOCKBINDINGPROC>;
//     pub(super) static glad_glUniformMatrix2dv:                            Option<PFNGLUNIFORMMATRIX2DVPROC>;
//     pub(super) static glad_glUniformMatrix2fv:                            Option<PFNGLUNIFORMMATRIX2FVPROC>;
//     pub(super) static glad_glUniformMatrix2fvARB:                         Option<PFNGLUNIFORMMATRIX2FVARBPROC>;
//     pub(super) static glad_glUniformMatrix2x3dv:                          Option<PFNGLUNIFORMMATRIX2X3DVPROC>;
//     pub(super) static glad_glUniformMatrix2x3fv:                          Option<PFNGLUNIFORMMATRIX2X3FVPROC>;
//     pub(super) static glad_glUniformMatrix2x4dv:                          Option<PFNGLUNIFORMMATRIX2X4DVPROC>;
//     pub(super) static glad_glUniformMatrix2x4fv:                          Option<PFNGLUNIFORMMATRIX2X4FVPROC>;
//     pub(super) static glad_glUniformMatrix3dv:                            Option<PFNGLUNIFORMMATRIX3DVPROC>;
//     pub(super) static glad_glUniformMatrix3fv:                            Option<PFNGLUNIFORMMATRIX3FVPROC>;
//     pub(super) static glad_glUniformMatrix3fvARB:                         Option<PFNGLUNIFORMMATRIX3FVARBPROC>;
//     pub(super) static glad_glUniformMatrix3x2dv:                          Option<PFNGLUNIFORMMATRIX3X2DVPROC>;
//     pub(super) static glad_glUniformMatrix3x2fv:                          Option<PFNGLUNIFORMMATRIX3X2FVPROC>;
//     pub(super) static glad_glUniformMatrix3x4dv:                          Option<PFNGLUNIFORMMATRIX3X4DVPROC>;
//     pub(super) static glad_glUniformMatrix3x4fv:                          Option<PFNGLUNIFORMMATRIX3X4FVPROC>;
//     pub(super) static glad_glUniformMatrix4dv:                            Option<PFNGLUNIFORMMATRIX4DVPROC>;
//     pub(super) static glad_glUniformMatrix4fv:                            Option<PFNGLUNIFORMMATRIX4FVPROC>;
//     pub(super) static glad_glUniformMatrix4fvARB:                         Option<PFNGLUNIFORMMATRIX4FVARBPROC>;
//     pub(super) static glad_glUniformMatrix4x2dv:                          Option<PFNGLUNIFORMMATRIX4X2DVPROC>;
//     pub(super) static glad_glUniformMatrix4x2fv:                          Option<PFNGLUNIFORMMATRIX4X2FVPROC>;
//     pub(super) static glad_glUniformMatrix4x3dv:                          Option<PFNGLUNIFORMMATRIX4X3DVPROC>;
//     pub(super) static glad_glUniformMatrix4x3fv:                          Option<PFNGLUNIFORMMATRIX4X3FVPROC>;
//     pub(super) static glad_glUniformSubroutinesuiv:                       Option<PFNGLUNIFORMSUBROUTINESUIVPROC>;
//     pub(super) static glad_glUnmapBuffer:                                 Option<PFNGLUNMAPBUFFERPROC>;
//     pub(super) static glad_glUnmapBufferARB:                              Option<PFNGLUNMAPBUFFERARBPROC>;
//     pub(super) static glad_glUnmapNamedBuffer:                            Option<PFNGLUNMAPNAMEDBUFFERPROC>;
//     pub(super) static glad_glUseProgram:                                  Option<PFNGLUSEPROGRAMPROC>;
//     pub(super) static glad_glUseProgramObjectARB:                         Option<PFNGLUSEPROGRAMOBJECTARBPROC>;
//     pub(super) static glad_glUseProgramStages:                            Option<PFNGLUSEPROGRAMSTAGESPROC>;
//     pub(super) static glad_glValidateProgram:                             Option<PFNGLVALIDATEPROGRAMPROC>;
//     pub(super) static glad_glValidateProgramARB:                          Option<PFNGLVALIDATEPROGRAMARBPROC>;
//     pub(super) static glad_glValidateProgramPipeline:                     Option<PFNGLVALIDATEPROGRAMPIPELINEPROC>;
//     pub(super) static glad_glVertex2xOES:                                 Option<PFNGLVERTEX2XOESPROC>;
//     pub(super) static glad_glVertex2xvOES:                                Option<PFNGLVERTEX2XVOESPROC>;
//     pub(super) static glad_glVertex3xOES:                                 Option<PFNGLVERTEX3XOESPROC>;
//     pub(super) static glad_glVertex3xvOES:                                Option<PFNGLVERTEX3XVOESPROC>;
//     pub(super) static glad_glVertex4xOES:                                 Option<PFNGLVERTEX4XOESPROC>;
//     pub(super) static glad_glVertex4xvOES:                                Option<PFNGLVERTEX4XVOESPROC>;
//     pub(super) static glad_glVertexArrayAttribBinding:                    Option<PFNGLVERTEXARRAYATTRIBBINDINGPROC>;
//     pub(super) static glad_glVertexArrayAttribFormat:                     Option<PFNGLVERTEXARRAYATTRIBFORMATPROC>;
//     pub(super) static glad_glVertexArrayAttribIFormat:                    Option<PFNGLVERTEXARRAYATTRIBIFORMATPROC>;
//     pub(super) static glad_glVertexArrayAttribLFormat:                    Option<PFNGLVERTEXARRAYATTRIBLFORMATPROC>;
//     pub(super) static glad_glVertexArrayBindingDivisor:                   Option<PFNGLVERTEXARRAYBINDINGDIVISORPROC>;
//     pub(super) static glad_glVertexArrayElementBuffer:                    Option<PFNGLVERTEXARRAYELEMENTBUFFERPROC>;
//     pub(super) static glad_glVertexArrayVertexBuffer:                     Option<PFNGLVERTEXARRAYVERTEXBUFFERPROC>;
//     pub(super) static glad_glVertexArrayVertexBuffers:                    Option<PFNGLVERTEXARRAYVERTEXBUFFERSPROC>;
//     pub(super) static glad_glVertexAttrib1d:                              Option<PFNGLVERTEXATTRIB1DPROC>;
//     pub(super) static glad_glVertexAttrib1dARB:                           Option<PFNGLVERTEXATTRIB1DARBPROC>;
//     pub(super) static glad_glVertexAttrib1dv:                             Option<PFNGLVERTEXATTRIB1DVPROC>;
//     pub(super) static glad_glVertexAttrib1dvARB:                          Option<PFNGLVERTEXATTRIB1DVARBPROC>;
//     pub(super) static glad_glVertexAttrib1f:                              Option<PFNGLVERTEXATTRIB1FPROC>;
//     pub(super) static glad_glVertexAttrib1fARB:                           Option<PFNGLVERTEXATTRIB1FARBPROC>;
//     pub(super) static glad_glVertexAttrib1fv:                             Option<PFNGLVERTEXATTRIB1FVPROC>;
//     pub(super) static glad_glVertexAttrib1fvARB:                          Option<PFNGLVERTEXATTRIB1FVARBPROC>;
//     pub(super) static glad_glVertexAttrib1s:                              Option<PFNGLVERTEXATTRIB1SPROC>;
//     pub(super) static glad_glVertexAttrib1sARB:                           Option<PFNGLVERTEXATTRIB1SARBPROC>;
//     pub(super) static glad_glVertexAttrib1sv:                             Option<PFNGLVERTEXATTRIB1SVPROC>;
//     pub(super) static glad_glVertexAttrib1svARB:                          Option<PFNGLVERTEXATTRIB1SVARBPROC>;
//     pub(super) static glad_glVertexAttrib2d:                              Option<PFNGLVERTEXATTRIB2DPROC>;
//     pub(super) static glad_glVertexAttrib2dARB:                           Option<PFNGLVERTEXATTRIB2DARBPROC>;
//     pub(super) static glad_glVertexAttrib2dv:                             Option<PFNGLVERTEXATTRIB2DVPROC>;
//     pub(super) static glad_glVertexAttrib2dvARB:                          Option<PFNGLVERTEXATTRIB2DVARBPROC>;
//     pub(super) static glad_glVertexAttrib2f:                              Option<PFNGLVERTEXATTRIB2FPROC>;
//     pub(super) static glad_glVertexAttrib2fARB:                           Option<PFNGLVERTEXATTRIB2FARBPROC>;
//     pub(super) static glad_glVertexAttrib2fv:                             Option<PFNGLVERTEXATTRIB2FVPROC>;
//     pub(super) static glad_glVertexAttrib2fvARB:                          Option<PFNGLVERTEXATTRIB2FVARBPROC>;
//     pub(super) static glad_glVertexAttrib2s:                              Option<PFNGLVERTEXATTRIB2SPROC>;
//     pub(super) static glad_glVertexAttrib2sARB:                           Option<PFNGLVERTEXATTRIB2SARBPROC>;
//     pub(super) static glad_glVertexAttrib2sv:                             Option<PFNGLVERTEXATTRIB2SVPROC>;
//     pub(super) static glad_glVertexAttrib2svARB:                          Option<PFNGLVERTEXATTRIB2SVARBPROC>;
//     pub(super) static glad_glVertexAttrib3d:                              Option<PFNGLVERTEXATTRIB3DPROC>;
//     pub(super) static glad_glVertexAttrib3dARB:                           Option<PFNGLVERTEXATTRIB3DARBPROC>;
//     pub(super) static glad_glVertexAttrib3dv:                             Option<PFNGLVERTEXATTRIB3DVPROC>;
//     pub(super) static glad_glVertexAttrib3dvARB:                          Option<PFNGLVERTEXATTRIB3DVARBPROC>;
//     pub(super) static glad_glVertexAttrib3f:                              Option<PFNGLVERTEXATTRIB3FPROC>;
//     pub(super) static glad_glVertexAttrib3fARB:                           Option<PFNGLVERTEXATTRIB3FARBPROC>;
//     pub(super) static glad_glVertexAttrib3fv:                             Option<PFNGLVERTEXATTRIB3FVPROC>;
//     pub(super) static glad_glVertexAttrib3fvARB:                          Option<PFNGLVERTEXATTRIB3FVARBPROC>;
//     pub(super) static glad_glVertexAttrib3s:                              Option<PFNGLVERTEXATTRIB3SPROC>;
//     pub(super) static glad_glVertexAttrib3sARB:                           Option<PFNGLVERTEXATTRIB3SARBPROC>;
//     pub(super) static glad_glVertexAttrib3sv:                             Option<PFNGLVERTEXATTRIB3SVPROC>;
//     pub(super) static glad_glVertexAttrib3svARB:                          Option<PFNGLVERTEXATTRIB3SVARBPROC>;
//     pub(super) static glad_glVertexAttrib4Nbv:                            Option<PFNGLVERTEXATTRIB4NBVPROC>;
//     pub(super) static glad_glVertexAttrib4NbvARB:                         Option<PFNGLVERTEXATTRIB4NBVARBPROC>;
//     pub(super) static glad_glVertexAttrib4Niv:                            Option<PFNGLVERTEXATTRIB4NIVPROC>;
//     pub(super) static glad_glVertexAttrib4NivARB:                         Option<PFNGLVERTEXATTRIB4NIVARBPROC>;
//     pub(super) static glad_glVertexAttrib4Nsv:                            Option<PFNGLVERTEXATTRIB4NSVPROC>;
//     pub(super) static glad_glVertexAttrib4NsvARB:                         Option<PFNGLVERTEXATTRIB4NSVARBPROC>;
//     pub(super) static glad_glVertexAttrib4Nub:                            Option<PFNGLVERTEXATTRIB4NUBPROC>;
//     pub(super) static glad_glVertexAttrib4NubARB:                         Option<PFNGLVERTEXATTRIB4NUBARBPROC>;
//     pub(super) static glad_glVertexAttrib4Nubv:                           Option<PFNGLVERTEXATTRIB4NUBVPROC>;
//     pub(super) static glad_glVertexAttrib4NubvARB:                        Option<PFNGLVERTEXATTRIB4NUBVARBPROC>;
//     pub(super) static glad_glVertexAttrib4Nuiv:                           Option<PFNGLVERTEXATTRIB4NUIVPROC>;
//     pub(super) static glad_glVertexAttrib4NuivARB:                        Option<PFNGLVERTEXATTRIB4NUIVARBPROC>;
//     pub(super) static glad_glVertexAttrib4Nusv:                           Option<PFNGLVERTEXATTRIB4NUSVPROC>;
//     pub(super) static glad_glVertexAttrib4NusvARB:                        Option<PFNGLVERTEXATTRIB4NUSVARBPROC>;
//     pub(super) static glad_glVertexAttrib4bv:                             Option<PFNGLVERTEXATTRIB4BVPROC>;
//     pub(super) static glad_glVertexAttrib4bvARB:                          Option<PFNGLVERTEXATTRIB4BVARBPROC>;
//     pub(super) static glad_glVertexAttrib4d:                              Option<PFNGLVERTEXATTRIB4DPROC>;
//     pub(super) static glad_glVertexAttrib4dARB:                           Option<PFNGLVERTEXATTRIB4DARBPROC>;
//     pub(super) static glad_glVertexAttrib4dv:                             Option<PFNGLVERTEXATTRIB4DVPROC>;
//     pub(super) static glad_glVertexAttrib4dvARB:                          Option<PFNGLVERTEXATTRIB4DVARBPROC>;
//     pub(super) static glad_glVertexAttrib4f:                              Option<PFNGLVERTEXATTRIB4FPROC>;
//     pub(super) static glad_glVertexAttrib4fARB:                           Option<PFNGLVERTEXATTRIB4FARBPROC>;
//     pub(super) static glad_glVertexAttrib4fv:                             Option<PFNGLVERTEXATTRIB4FVPROC>;
//     pub(super) static glad_glVertexAttrib4fvARB:                          Option<PFNGLVERTEXATTRIB4FVARBPROC>;
//     pub(super) static glad_glVertexAttrib4iv:                             Option<PFNGLVERTEXATTRIB4IVPROC>;
//     pub(super) static glad_glVertexAttrib4ivARB:                          Option<PFNGLVERTEXATTRIB4IVARBPROC>;
//     pub(super) static glad_glVertexAttrib4s:                              Option<PFNGLVERTEXATTRIB4SPROC>;
//     pub(super) static glad_glVertexAttrib4sARB:                           Option<PFNGLVERTEXATTRIB4SARBPROC>;
//     pub(super) static glad_glVertexAttrib4sv:                             Option<PFNGLVERTEXATTRIB4SVPROC>;
//     pub(super) static glad_glVertexAttrib4svARB:                          Option<PFNGLVERTEXATTRIB4SVARBPROC>;
//     pub(super) static glad_glVertexAttrib4ubv:                            Option<PFNGLVERTEXATTRIB4UBVPROC>;
//     pub(super) static glad_glVertexAttrib4ubvARB:                         Option<PFNGLVERTEXATTRIB4UBVARBPROC>;
//     pub(super) static glad_glVertexAttrib4uiv:                            Option<PFNGLVERTEXATTRIB4UIVPROC>;
//     pub(super) static glad_glVertexAttrib4uivARB:                         Option<PFNGLVERTEXATTRIB4UIVARBPROC>;
//     pub(super) static glad_glVertexAttrib4usv:                            Option<PFNGLVERTEXATTRIB4USVPROC>;
//     pub(super) static glad_glVertexAttrib4usvARB:                         Option<PFNGLVERTEXATTRIB4USVARBPROC>;
//     pub(super) static glad_glVertexAttribBinding:                         Option<PFNGLVERTEXATTRIBBINDINGPROC>;
//     pub(super) static glad_glVertexAttribDivisor:                         Option<PFNGLVERTEXATTRIBDIVISORPROC>;
//     pub(super) static glad_glVertexAttribDivisorARB:                      Option<PFNGLVERTEXATTRIBDIVISORARBPROC>;
//     pub(super) static glad_glVertexAttribFormat:                          Option<PFNGLVERTEXATTRIBFORMATPROC>;
//     pub(super) static glad_glVertexAttribI1i:                             Option<PFNGLVERTEXATTRIBI1IPROC>;
//     pub(super) static glad_glVertexAttribI1iv:                            Option<PFNGLVERTEXATTRIBI1IVPROC>;
//     pub(super) static glad_glVertexAttribI1ui:                            Option<PFNGLVERTEXATTRIBI1UIPROC>;
//     pub(super) static glad_glVertexAttribI1uiv:                           Option<PFNGLVERTEXATTRIBI1UIVPROC>;
//     pub(super) static glad_glVertexAttribI2i:                             Option<PFNGLVERTEXATTRIBI2IPROC>;
//     pub(super) static glad_glVertexAttribI2iv:                            Option<PFNGLVERTEXATTRIBI2IVPROC>;
//     pub(super) static glad_glVertexAttribI2ui:                            Option<PFNGLVERTEXATTRIBI2UIPROC>;
//     pub(super) static glad_glVertexAttribI2uiv:                           Option<PFNGLVERTEXATTRIBI2UIVPROC>;
//     pub(super) static glad_glVertexAttribI3i:                             Option<PFNGLVERTEXATTRIBI3IPROC>;
//     pub(super) static glad_glVertexAttribI3iv:                            Option<PFNGLVERTEXATTRIBI3IVPROC>;
//     pub(super) static glad_glVertexAttribI3ui:                            Option<PFNGLVERTEXATTRIBI3UIPROC>;
//     pub(super) static glad_glVertexAttribI3uiv:                           Option<PFNGLVERTEXATTRIBI3UIVPROC>;
//     pub(super) static glad_glVertexAttribI4bv:                            Option<PFNGLVERTEXATTRIBI4BVPROC>;
//     pub(super) static glad_glVertexAttribI4i:                             Option<PFNGLVERTEXATTRIBI4IPROC>;
//     pub(super) static glad_glVertexAttribI4iv:                            Option<PFNGLVERTEXATTRIBI4IVPROC>;
//     pub(super) static glad_glVertexAttribI4sv:                            Option<PFNGLVERTEXATTRIBI4SVPROC>;
//     pub(super) static glad_glVertexAttribI4ubv:                           Option<PFNGLVERTEXATTRIBI4UBVPROC>;
//     pub(super) static glad_glVertexAttribI4ui:                            Option<PFNGLVERTEXATTRIBI4UIPROC>;
//     pub(super) static glad_glVertexAttribI4uiv:                           Option<PFNGLVERTEXATTRIBI4UIVPROC>;
//     pub(super) static glad_glVertexAttribI4usv:                           Option<PFNGLVERTEXATTRIBI4USVPROC>;
//     pub(super) static glad_glVertexAttribIFormat:                         Option<PFNGLVERTEXATTRIBIFORMATPROC>;
//     pub(super) static glad_glVertexAttribIPointer:                        Option<PFNGLVERTEXATTRIBIPOINTERPROC>;
//     pub(super) static glad_glVertexAttribL1d:                             Option<PFNGLVERTEXATTRIBL1DPROC>;
//     pub(super) static glad_glVertexAttribL1dv:                            Option<PFNGLVERTEXATTRIBL1DVPROC>;
//     pub(super) static glad_glVertexAttribL2d:                             Option<PFNGLVERTEXATTRIBL2DPROC>;
//     pub(super) static glad_glVertexAttribL2dv:                            Option<PFNGLVERTEXATTRIBL2DVPROC>;
//     pub(super) static glad_glVertexAttribL3d:                             Option<PFNGLVERTEXATTRIBL3DPROC>;
//     pub(super) static glad_glVertexAttribL3dv:                            Option<PFNGLVERTEXATTRIBL3DVPROC>;
//     pub(super) static glad_glVertexAttribL4d:                             Option<PFNGLVERTEXATTRIBL4DPROC>;
//     pub(super) static glad_glVertexAttribL4dv:                            Option<PFNGLVERTEXATTRIBL4DVPROC>;
//     pub(super) static glad_glVertexAttribLFormat:                         Option<PFNGLVERTEXATTRIBLFORMATPROC>;
//     pub(super) static glad_glVertexAttribLPointer:                        Option<PFNGLVERTEXATTRIBLPOINTERPROC>;
//     pub(super) static glad_glVertexAttribP1ui:                            Option<PFNGLVERTEXATTRIBP1UIPROC>;
//     pub(super) static glad_glVertexAttribP1uiv:                           Option<PFNGLVERTEXATTRIBP1UIVPROC>;
//     pub(super) static glad_glVertexAttribP2ui:                            Option<PFNGLVERTEXATTRIBP2UIPROC>;
//     pub(super) static glad_glVertexAttribP2uiv:                           Option<PFNGLVERTEXATTRIBP2UIVPROC>;
//     pub(super) static glad_glVertexAttribP3ui:                            Option<PFNGLVERTEXATTRIBP3UIPROC>;
//     pub(super) static glad_glVertexAttribP3uiv:                           Option<PFNGLVERTEXATTRIBP3UIVPROC>;
//     pub(super) static glad_glVertexAttribP4ui:                            Option<PFNGLVERTEXATTRIBP4UIPROC>;
//     pub(super) static glad_glVertexAttribP4uiv:                           Option<PFNGLVERTEXATTRIBP4UIVPROC>;
//     pub(super) static glad_glVertexAttribPointer:                         Option<PFNGLVERTEXATTRIBPOINTERPROC>;
//     pub(super) static glad_glVertexAttribPointerARB:                      Option<PFNGLVERTEXATTRIBPOINTERARBPROC>;
//     pub(super) static glad_glVertexBindingDivisor:                        Option<PFNGLVERTEXBINDINGDIVISORPROC>;
//     pub(super) static glad_glViewport:                                    Option<PFNGLVIEWPORTPROC>;
//     pub(super) static glad_glViewportArrayv:                              Option<PFNGLVIEWPORTARRAYVPROC>;
//     pub(super) static glad_glViewportIndexedf:                            Option<PFNGLVIEWPORTINDEXEDFPROC>;
//     pub(super) static glad_glViewportIndexedfv:                           Option<PFNGLVIEWPORTINDEXEDFVPROC>;
//     pub(super) static glad_glWaitSync:                                    Option<PFNGLWAITSYNCPROC>;
// }

#[derive(Default)]
pub struct Glad {
    pub glAccumxOES:                                   Option<PFNGLACCUMXOESPROC>,
    pub glActiveShaderProgram:                         Option<PFNGLACTIVESHADERPROGRAMPROC>,
    pub glActiveTexture:                               Option<PFNGLACTIVETEXTUREPROC>,
    pub glActiveTextureARB:                            Option<PFNGLACTIVETEXTUREARBPROC>,
    pub glAlphaFuncxOES:                               Option<PFNGLALPHAFUNCXOESPROC>,
    pub glAttachObjectARB:                             Option<PFNGLATTACHOBJECTARBPROC>,
    pub glAttachShader:                                Option<PFNGLATTACHSHADERPROC>,
    pub glBeginConditionalRender:                      Option<PFNGLBEGINCONDITIONALRENDERPROC>,
    pub glBeginQuery:                                  Option<PFNGLBEGINQUERYPROC>,
    pub glBeginQueryARB:                               Option<PFNGLBEGINQUERYARBPROC>,
    pub glBeginQueryIndexed:                           Option<PFNGLBEGINQUERYINDEXEDPROC>,
    pub glBeginTransformFeedback:                      Option<PFNGLBEGINTRANSFORMFEEDBACKPROC>,
    pub glBindAttribLocation:                          Option<PFNGLBINDATTRIBLOCATIONPROC>,
    pub glBindAttribLocationARB:                       Option<PFNGLBINDATTRIBLOCATIONARBPROC>,
    pub glBindBuffer:                                  Option<PFNGLBINDBUFFERPROC>,
    pub glBindBufferARB:                               Option<PFNGLBINDBUFFERARBPROC>,
    pub glBindBufferBase:                              Option<PFNGLBINDBUFFERBASEPROC>,
    pub glBindBufferRange:                             Option<PFNGLBINDBUFFERRANGEPROC>,
    pub glBindBuffersBase:                             Option<PFNGLBINDBUFFERSBASEPROC>,
    pub glBindBuffersRange:                            Option<PFNGLBINDBUFFERSRANGEPROC>,
    pub glBindFragDataLocation:                        Option<PFNGLBINDFRAGDATALOCATIONPROC>,
    pub glBindFragDataLocationIndexed:                 Option<PFNGLBINDFRAGDATALOCATIONINDEXEDPROC>,
    pub glBindFramebuffer:                             Option<PFNGLBINDFRAMEBUFFERPROC>,
    pub glBindFramebufferEXT:                          Option<PFNGLBINDFRAMEBUFFEREXTPROC>,
    pub glBindImageTexture:                            Option<PFNGLBINDIMAGETEXTUREPROC>,
    pub glBindImageTextures:                           Option<PFNGLBINDIMAGETEXTURESPROC>,
    pub glBindProgramARB:                              Option<PFNGLBINDPROGRAMARBPROC>,
    pub glBindProgramPipeline:                         Option<PFNGLBINDPROGRAMPIPELINEPROC>,
    pub glBindRenderbuffer:                            Option<PFNGLBINDRENDERBUFFERPROC>,
    pub glBindRenderbufferEXT:                         Option<PFNGLBINDRENDERBUFFEREXTPROC>,
    pub glBindSampler:                                 Option<PFNGLBINDSAMPLERPROC>,
    pub glBindSamplers:                                Option<PFNGLBINDSAMPLERSPROC>,
    pub glBindTexture:                                 Option<PFNGLBINDTEXTUREPROC>,
    pub glBindTextureUnit:                             Option<PFNGLBINDTEXTUREUNITPROC>,
    pub glBindTextures:                                Option<PFNGLBINDTEXTURESPROC>,
    pub glBindTransformFeedback:                       Option<PFNGLBINDTRANSFORMFEEDBACKPROC>,
    pub glBindVertexArray:                             Option<PFNGLBINDVERTEXARRAYPROC>,
    pub glBindVertexBuffer:                            Option<PFNGLBINDVERTEXBUFFERPROC>,
    pub glBindVertexBuffers:                           Option<PFNGLBINDVERTEXBUFFERSPROC>,
    pub glBitmapxOES:                                  Option<PFNGLBITMAPXOESPROC>,
    pub glBlendColor:                                  Option<PFNGLBLENDCOLORPROC>,
    pub glBlendColorxOES:                              Option<PFNGLBLENDCOLORXOESPROC>,
    pub glBlendEquation:                               Option<PFNGLBLENDEQUATIONPROC>,
    pub glBlendEquationSeparate:                       Option<PFNGLBLENDEQUATIONSEPARATEPROC>,
    pub glBlendEquationSeparatei:                      Option<PFNGLBLENDEQUATIONSEPARATEIPROC>,
    pub glBlendEquationSeparateiARB:                   Option<PFNGLBLENDEQUATIONSEPARATEIARBPROC>,
    pub glBlendEquationi:                              Option<PFNGLBLENDEQUATIONIPROC>,
    pub glBlendEquationiARB:                           Option<PFNGLBLENDEQUATIONIARBPROC>,
    pub glBlendFunc:                                   Option<PFNGLBLENDFUNCPROC>,
    pub glBlendFuncSeparate:                           Option<PFNGLBLENDFUNCSEPARATEPROC>,
    pub glBlendFuncSeparatei:                          Option<PFNGLBLENDFUNCSEPARATEIPROC>,
    pub glBlendFuncSeparateiARB:                       Option<PFNGLBLENDFUNCSEPARATEIARBPROC>,
    pub glBlendFunci:                                  Option<PFNGLBLENDFUNCIPROC>,
    pub glBlendFunciARB:                               Option<PFNGLBLENDFUNCIARBPROC>,
    pub glBlitFramebuffer:                             Option<PFNGLBLITFRAMEBUFFERPROC>,
    pub glBlitFramebufferEXT:                          Option<PFNGLBLITFRAMEBUFFEREXTPROC>,
    pub glBlitNamedFramebuffer:                        Option<PFNGLBLITNAMEDFRAMEBUFFERPROC>,
    pub glBufferData:                                  Option<PFNGLBUFFERDATAPROC>,
    pub glBufferDataARB:                               Option<PFNGLBUFFERDATAARBPROC>,
    pub glBufferStorage:                               Option<PFNGLBUFFERSTORAGEPROC>,
    pub glBufferSubData:                               Option<PFNGLBUFFERSUBDATAPROC>,
    pub glBufferSubDataARB:                            Option<PFNGLBUFFERSUBDATAARBPROC>,
    pub glCheckFramebufferStatus:                      Option<PFNGLCHECKFRAMEBUFFERSTATUSPROC>,
    pub glCheckFramebufferStatusEXT:                   Option<PFNGLCHECKFRAMEBUFFERSTATUSEXTPROC>,
    pub glCheckNamedFramebufferStatus:                 Option<PFNGLCHECKNAMEDFRAMEBUFFERSTATUSPROC>,
    pub glClampColor:                                  Option<PFNGLCLAMPCOLORPROC>,
    pub glClampColorARB:                               Option<PFNGLCLAMPCOLORARBPROC>,
    pub glClear:                                       Option<PFNGLCLEARPROC>,
    pub glClearAccumxOES:                              Option<PFNGLCLEARACCUMXOESPROC>,
    pub glClearBufferData:                             Option<PFNGLCLEARBUFFERDATAPROC>,
    pub glClearBufferSubData:                          Option<PFNGLCLEARBUFFERSUBDATAPROC>,
    pub glClearBufferfi:                               Option<PFNGLCLEARBUFFERFIPROC>,
    pub glClearBufferfv:                               Option<PFNGLCLEARBUFFERFVPROC>,
    pub glClearBufferiv:                               Option<PFNGLCLEARBUFFERIVPROC>,
    pub glClearBufferuiv:                              Option<PFNGLCLEARBUFFERUIVPROC>,
    pub glClearColor:                                  Option<PFNGLCLEARCOLORPROC>,
    pub glClearColorxOES:                              Option<PFNGLCLEARCOLORXOESPROC>,
    pub glClearDepth:                                  Option<PFNGLCLEARDEPTHPROC>,
    pub glClearDepthf:                                 Option<PFNGLCLEARDEPTHFPROC>,
    pub glClearDepthxOES:                              Option<PFNGLCLEARDEPTHXOESPROC>,
    pub glClearNamedBufferData:                        Option<PFNGLCLEARNAMEDBUFFERDATAPROC>,
    pub glClearNamedBufferSubData:                     Option<PFNGLCLEARNAMEDBUFFERSUBDATAPROC>,
    pub glClearNamedFramebufferfi:                     Option<PFNGLCLEARNAMEDFRAMEBUFFERFIPROC>,
    pub glClearNamedFramebufferfv:                     Option<PFNGLCLEARNAMEDFRAMEBUFFERFVPROC>,
    pub glClearNamedFramebufferiv:                     Option<PFNGLCLEARNAMEDFRAMEBUFFERIVPROC>,
    pub glClearNamedFramebufferuiv:                    Option<PFNGLCLEARNAMEDFRAMEBUFFERUIVPROC>,
    pub glClearStencil:                                Option<PFNGLCLEARSTENCILPROC>,
    pub glClearTexImage:                               Option<PFNGLCLEARTEXIMAGEPROC>,
    pub glClearTexSubImage:                            Option<PFNGLCLEARTEXSUBIMAGEPROC>,
    pub glClientActiveTextureARB:                      Option<PFNGLCLIENTACTIVETEXTUREARBPROC>,
    pub glClientWaitSync:                              Option<PFNGLCLIENTWAITSYNCPROC>,
    pub glClipPlanexOES:                               Option<PFNGLCLIPPLANEXOESPROC>,
    pub glColor3xOES:                                  Option<PFNGLCOLOR3XOESPROC>,
    pub glColor3xvOES:                                 Option<PFNGLCOLOR3XVOESPROC>,
    pub glColor4xOES:                                  Option<PFNGLCOLOR4XOESPROC>,
    pub glColor4xvOES:                                 Option<PFNGLCOLOR4XVOESPROC>,
    pub glColorMask:                                   Option<PFNGLCOLORMASKPROC>,
    pub glColorMaski:                                  Option<PFNGLCOLORMASKIPROC>,
    pub glCompileShader:                               Option<PFNGLCOMPILESHADERPROC>,
    pub glCompileShaderARB:                            Option<PFNGLCOMPILESHADERARBPROC>,
    pub glCompileShaderIncludeARB:                     Option<PFNGLCOMPILESHADERINCLUDEARBPROC>,
    pub glCompressedTexImage1D:                        Option<PFNGLCOMPRESSEDTEXIMAGE1DPROC>,
    pub glCompressedTexImage1DARB:                     Option<PFNGLCOMPRESSEDTEXIMAGE1DARBPROC>,
    pub glCompressedTexImage2D:                        Option<PFNGLCOMPRESSEDTEXIMAGE2DPROC>,
    pub glCompressedTexImage2DARB:                     Option<PFNGLCOMPRESSEDTEXIMAGE2DARBPROC>,
    pub glCompressedTexImage3D:                        Option<PFNGLCOMPRESSEDTEXIMAGE3DPROC>,
    pub glCompressedTexImage3DARB:                     Option<PFNGLCOMPRESSEDTEXIMAGE3DARBPROC>,
    pub glCompressedTexSubImage1D:                     Option<PFNGLCOMPRESSEDTEXSUBIMAGE1DPROC>,
    pub glCompressedTexSubImage1DARB:                  Option<PFNGLCOMPRESSEDTEXSUBIMAGE1DARBPROC>,
    pub glCompressedTexSubImage2D:                     Option<PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC>,
    pub glCompressedTexSubImage2DARB:                  Option<PFNGLCOMPRESSEDTEXSUBIMAGE2DARBPROC>,
    pub glCompressedTexSubImage3D:                     Option<PFNGLCOMPRESSEDTEXSUBIMAGE3DPROC>,
    pub glCompressedTexSubImage3DARB:                  Option<PFNGLCOMPRESSEDTEXSUBIMAGE3DARBPROC>,
    pub glCompressedTextureSubImage1D:                 Option<PFNGLCOMPRESSEDTEXTURESUBIMAGE1DPROC>,
    pub glCompressedTextureSubImage2D:                 Option<PFNGLCOMPRESSEDTEXTURESUBIMAGE2DPROC>,
    pub glCompressedTextureSubImage3D:                 Option<PFNGLCOMPRESSEDTEXTURESUBIMAGE3DPROC>,
    pub glConvolutionParameterxOES:                    Option<PFNGLCONVOLUTIONPARAMETERXOESPROC>,
    pub glConvolutionParameterxvOES:                   Option<PFNGLCONVOLUTIONPARAMETERXVOESPROC>,
    pub glCopyBufferSubData:                           Option<PFNGLCOPYBUFFERSUBDATAPROC>,
    pub glCopyImageSubData:                            Option<PFNGLCOPYIMAGESUBDATAPROC>,
    pub glCopyNamedBufferSubData:                      Option<PFNGLCOPYNAMEDBUFFERSUBDATAPROC>,
    pub glCopyTexImage1D:                              Option<PFNGLCOPYTEXIMAGE1DPROC>,
    pub glCopyTexImage2D:                              Option<PFNGLCOPYTEXIMAGE2DPROC>,
    pub glCopyTexSubImage1D:                           Option<PFNGLCOPYTEXSUBIMAGE1DPROC>,
    pub glCopyTexSubImage2D:                           Option<PFNGLCOPYTEXSUBIMAGE2DPROC>,
    pub glCopyTexSubImage3D:                           Option<PFNGLCOPYTEXSUBIMAGE3DPROC>,
    pub glCopyTextureSubImage1D:                       Option<PFNGLCOPYTEXTURESUBIMAGE1DPROC>,
    pub glCopyTextureSubImage2D:                       Option<PFNGLCOPYTEXTURESUBIMAGE2DPROC>,
    pub glCopyTextureSubImage3D:                       Option<PFNGLCOPYTEXTURESUBIMAGE3DPROC>,
    pub glCreateBuffers:                               Option<PFNGLCREATEBUFFERSPROC>,
    pub glCreateFramebuffers:                          Option<PFNGLCREATEFRAMEBUFFERSPROC>,
    pub glCreateProgram:                               Option<PFNGLCREATEPROGRAMPROC>,
    pub glCreateProgramObjectARB:                      Option<PFNGLCREATEPROGRAMOBJECTARBPROC>,
    pub glCreateProgramPipelines:                      Option<PFNGLCREATEPROGRAMPIPELINESPROC>,
    pub glCreateQueries:                               Option<PFNGLCREATEQUERIESPROC>,
    pub glCreateRenderbuffers:                         Option<PFNGLCREATERENDERBUFFERSPROC>,
    pub glCreateSamplers:                              Option<PFNGLCREATESAMPLERSPROC>,
    pub glCreateShader:                                Option<PFNGLCREATESHADERPROC>,
    pub glCreateShaderObjectARB:                       Option<PFNGLCREATESHADEROBJECTARBPROC>,
    pub glCreateShaderProgramv:                        Option<PFNGLCREATESHADERPROGRAMVPROC>,
    pub glCreateTextures:                              Option<PFNGLCREATETEXTURESPROC>,
    pub glCreateTransformFeedbacks:                    Option<PFNGLCREATETRANSFORMFEEDBACKSPROC>,
    pub glCreateVertexArrays:                          Option<PFNGLCREATEVERTEXARRAYSPROC>,
    pub glCullFace:                                    Option<PFNGLCULLFACEPROC>,
    pub glDebugMessageCallback:                        Option<PFNGLDEBUGMESSAGECALLBACKPROC>,
    pub glDebugMessageCallbackARB:                     Option<PFNGLDEBUGMESSAGECALLBACKARBPROC>,
    pub glDebugMessageControl:                         Option<PFNGLDEBUGMESSAGECONTROLPROC>,
    pub glDebugMessageControlARB:                      Option<PFNGLDEBUGMESSAGECONTROLARBPROC>,
    pub glDebugMessageInsert:                          Option<PFNGLDEBUGMESSAGEINSERTPROC>,
    pub glDebugMessageInsertARB:                       Option<PFNGLDEBUGMESSAGEINSERTARBPROC>,
    pub glDeleteBuffers:                               Option<PFNGLDELETEBUFFERSPROC>,
    pub glDeleteBuffersARB:                            Option<PFNGLDELETEBUFFERSARBPROC>,
    pub glDeleteFramebuffers:                          Option<PFNGLDELETEFRAMEBUFFERSPROC>,
    pub glDeleteFramebuffersEXT:                       Option<PFNGLDELETEFRAMEBUFFERSEXTPROC>,
    pub glDeleteNamedStringARB:                        Option<PFNGLDELETENAMEDSTRINGARBPROC>,
    pub glDeleteObjectARB:                             Option<PFNGLDELETEOBJECTARBPROC>,
    pub glDeleteProgram:                               Option<PFNGLDELETEPROGRAMPROC>,
    pub glDeleteProgramPipelines:                      Option<PFNGLDELETEPROGRAMPIPELINESPROC>,
    pub glDeleteProgramsARB:                           Option<PFNGLDELETEPROGRAMSARBPROC>,
    pub glDeleteQueries:                               Option<PFNGLDELETEQUERIESPROC>,
    pub glDeleteQueriesARB:                            Option<PFNGLDELETEQUERIESARBPROC>,
    pub glDeleteRenderbuffers:                         Option<PFNGLDELETERENDERBUFFERSPROC>,
    pub glDeleteRenderbuffersEXT:                      Option<PFNGLDELETERENDERBUFFERSEXTPROC>,
    pub glDeleteSamplers:                              Option<PFNGLDELETESAMPLERSPROC>,
    pub glDeleteShader:                                Option<PFNGLDELETESHADERPROC>,
    pub glDeleteSync:                                  Option<PFNGLDELETESYNCPROC>,
    pub glDeleteTextures:                              Option<PFNGLDELETETEXTURESPROC>,
    pub glDeleteTransformFeedbacks:                    Option<PFNGLDELETETRANSFORMFEEDBACKSPROC>,
    pub glDeleteVertexArrays:                          Option<PFNGLDELETEVERTEXARRAYSPROC>,
    pub glDepthFunc:                                   Option<PFNGLDEPTHFUNCPROC>,
    pub glDepthMask:                                   Option<PFNGLDEPTHMASKPROC>,
    pub glDepthRange:                                  Option<PFNGLDEPTHRANGEPROC>,
    pub glDepthRangeArrayv:                            Option<PFNGLDEPTHRANGEARRAYVPROC>,
    pub glDepthRangeIndexed:                           Option<PFNGLDEPTHRANGEINDEXEDPROC>,
    pub glDepthRangef:                                 Option<PFNGLDEPTHRANGEFPROC>,
    pub glDepthRangexOES:                              Option<PFNGLDEPTHRANGEXOESPROC>,
    pub glDetachObjectARB:                             Option<PFNGLDETACHOBJECTARBPROC>,
    pub glDetachShader:                                Option<PFNGLDETACHSHADERPROC>,
    pub glDisable:                                     Option<PFNGLDISABLEPROC>,
    pub glDisableVertexArrayAttrib:                    Option<PFNGLDISABLEVERTEXARRAYATTRIBPROC>,
    pub glDisableVertexAttribArray:                    Option<PFNGLDISABLEVERTEXATTRIBARRAYPROC>,
    pub glDisableVertexAttribArrayARB:                 Option<PFNGLDISABLEVERTEXATTRIBARRAYARBPROC>,
    pub glDisablei:                                    Option<PFNGLDISABLEIPROC>,
    pub glDispatchCompute:                             Option<PFNGLDISPATCHCOMPUTEPROC>,
    pub glDispatchComputeGroupSizeARB:                 Option<PFNGLDISPATCHCOMPUTEGROUPSIZEARBPROC>,
    pub glDispatchComputeIndirect:                     Option<PFNGLDISPATCHCOMPUTEINDIRECTPROC>,
    pub glDrawArrays:                                  Option<PFNGLDRAWARRAYSPROC>,
    pub glDrawArraysIndirect:                          Option<PFNGLDRAWARRAYSINDIRECTPROC>,
    pub glDrawArraysInstanced:                         Option<PFNGLDRAWARRAYSINSTANCEDPROC>,
    pub glDrawArraysInstancedARB:                      Option<PFNGLDRAWARRAYSINSTANCEDARBPROC>,
    pub glDrawArraysInstancedBaseInstance:             Option<PFNGLDRAWARRAYSINSTANCEDBASEINSTANCEPROC>,
    pub glDrawArraysInstancedEXT:                      Option<PFNGLDRAWARRAYSINSTANCEDEXTPROC>,
    pub glDrawBuffer:                                  Option<PFNGLDRAWBUFFERPROC>,
    pub glDrawBuffers:                                 Option<PFNGLDRAWBUFFERSPROC>,
    pub glDrawBuffersARB:                              Option<PFNGLDRAWBUFFERSARBPROC>,
    pub glDrawElements:                                Option<PFNGLDRAWELEMENTSPROC>,
    pub glDrawElementsBaseVertex:                      Option<PFNGLDRAWELEMENTSBASEVERTEXPROC>,
    pub glDrawElementsIndirect:                        Option<PFNGLDRAWELEMENTSINDIRECTPROC>,
    pub glDrawElementsInstanced:                       Option<PFNGLDRAWELEMENTSINSTANCEDPROC>,
    pub glDrawElementsInstancedARB:                    Option<PFNGLDRAWELEMENTSINSTANCEDARBPROC>,
    pub glDrawElementsInstancedBaseInstance:           Option<PFNGLDRAWELEMENTSINSTANCEDBASEINSTANCEPROC>,
    pub glDrawElementsInstancedBaseVertex:             Option<PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC>,
    pub glDrawElementsInstancedBaseVertexBaseInstance: Option<PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXBASEINSTANCEPROC>,
    pub glDrawElementsInstancedEXT:                    Option<PFNGLDRAWELEMENTSINSTANCEDEXTPROC>,
    pub glDrawRangeElements:                           Option<PFNGLDRAWRANGEELEMENTSPROC>,
    pub glDrawRangeElementsBaseVertex:                 Option<PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC>,
    pub glDrawTransformFeedback:                       Option<PFNGLDRAWTRANSFORMFEEDBACKPROC>,
    pub glDrawTransformFeedbackInstanced:              Option<PFNGLDRAWTRANSFORMFEEDBACKINSTANCEDPROC>,
    pub glDrawTransformFeedbackStream:                 Option<PFNGLDRAWTRANSFORMFEEDBACKSTREAMPROC>,
    pub glDrawTransformFeedbackStreamInstanced:        Option<PFNGLDRAWTRANSFORMFEEDBACKSTREAMINSTANCEDPROC>,
    pub glEnable:                                      Option<PFNGLENABLEPROC>,
    pub glEnableVertexArrayAttrib:                     Option<PFNGLENABLEVERTEXARRAYATTRIBPROC>,
    pub glEnableVertexAttribArray:                     Option<PFNGLENABLEVERTEXATTRIBARRAYPROC>,
    pub glEnableVertexAttribArrayARB:                  Option<PFNGLENABLEVERTEXATTRIBARRAYARBPROC>,
    pub glEnablei:                                     Option<PFNGLENABLEIPROC>,
    pub glEndConditionalRender:                        Option<PFNGLENDCONDITIONALRENDERPROC>,
    pub glEndQuery:                                    Option<PFNGLENDQUERYPROC>,
    pub glEndQueryARB:                                 Option<PFNGLENDQUERYARBPROC>,
    pub glEndQueryIndexed:                             Option<PFNGLENDQUERYINDEXEDPROC>,
    pub glEndTransformFeedback:                        Option<PFNGLENDTRANSFORMFEEDBACKPROC>,
    pub glEvalCoord1xOES:                              Option<PFNGLEVALCOORD1XOESPROC>,
    pub glEvalCoord1xvOES:                             Option<PFNGLEVALCOORD1XVOESPROC>,
    pub glEvalCoord2xOES:                              Option<PFNGLEVALCOORD2XOESPROC>,
    pub glEvalCoord2xvOES:                             Option<PFNGLEVALCOORD2XVOESPROC>,
    pub glEvaluateDepthValuesARB:                      Option<PFNGLEVALUATEDEPTHVALUESARBPROC>,
    pub glFeedbackBufferxOES:                          Option<PFNGLFEEDBACKBUFFERXOESPROC>,
    pub glFenceSync:                                   Option<PFNGLFENCESYNCPROC>,
    pub glFinish:                                      Option<PFNGLFINISHPROC>,
    pub glFlush:                                       Option<PFNGLFLUSHPROC>,
    pub glFlushMappedBufferRange:                      Option<PFNGLFLUSHMAPPEDBUFFERRANGEPROC>,
    pub glFlushMappedNamedBufferRange:                 Option<PFNGLFLUSHMAPPEDNAMEDBUFFERRANGEPROC>,
    pub glFogCoordPointerEXT:                          Option<PFNGLFOGCOORDPOINTEREXTPROC>,
    pub glFogCoorddEXT:                                Option<PFNGLFOGCOORDDEXTPROC>,
    pub glFogCoorddvEXT:                               Option<PFNGLFOGCOORDDVEXTPROC>,
    pub glFogCoordfEXT:                                Option<PFNGLFOGCOORDFEXTPROC>,
    pub glFogCoordfvEXT:                               Option<PFNGLFOGCOORDFVEXTPROC>,
    pub glFogxOES:                                     Option<PFNGLFOGXOESPROC>,
    pub glFogxvOES:                                    Option<PFNGLFOGXVOESPROC>,
    pub glFramebufferParameteri:                       Option<PFNGLFRAMEBUFFERPARAMETERIPROC>,
    pub glFramebufferRenderbuffer:                     Option<PFNGLFRAMEBUFFERRENDERBUFFERPROC>,
    pub glFramebufferRenderbufferEXT:                  Option<PFNGLFRAMEBUFFERRENDERBUFFEREXTPROC>,
    pub glFramebufferSampleLocationsfvARB:             Option<PFNGLFRAMEBUFFERSAMPLELOCATIONSFVARBPROC>,
    pub glFramebufferTexture:                          Option<PFNGLFRAMEBUFFERTEXTUREPROC>,
    pub glFramebufferTexture1D:                        Option<PFNGLFRAMEBUFFERTEXTURE1DPROC>,
    pub glFramebufferTexture1DEXT:                     Option<PFNGLFRAMEBUFFERTEXTURE1DEXTPROC>,
    pub glFramebufferTexture2D:                        Option<PFNGLFRAMEBUFFERTEXTURE2DPROC>,
    pub glFramebufferTexture2DEXT:                     Option<PFNGLFRAMEBUFFERTEXTURE2DEXTPROC>,
    pub glFramebufferTexture3D:                        Option<PFNGLFRAMEBUFFERTEXTURE3DPROC>,
    pub glFramebufferTexture3DEXT:                     Option<PFNGLFRAMEBUFFERTEXTURE3DEXTPROC>,
    pub glFramebufferTextureARB:                       Option<PFNGLFRAMEBUFFERTEXTUREARBPROC>,
    pub glFramebufferTextureFaceARB:                   Option<PFNGLFRAMEBUFFERTEXTUREFACEARBPROC>,
    pub glFramebufferTextureLayer:                     Option<PFNGLFRAMEBUFFERTEXTURELAYERPROC>,
    pub glFramebufferTextureLayerARB:                  Option<PFNGLFRAMEBUFFERTEXTURELAYERARBPROC>,
    pub glFrontFace:                                   Option<PFNGLFRONTFACEPROC>,
    pub glFrustumxOES:                                 Option<PFNGLFRUSTUMXOESPROC>,
    pub glGenBuffers:                                  Option<PFNGLGENBUFFERSPROC>,
    pub glGenBuffersARB:                               Option<PFNGLGENBUFFERSARBPROC>,
    pub glGenFramebuffers:                             Option<PFNGLGENFRAMEBUFFERSPROC>,
    pub glGenFramebuffersEXT:                          Option<PFNGLGENFRAMEBUFFERSEXTPROC>,
    pub glGenProgramPipelines:                         Option<PFNGLGENPROGRAMPIPELINESPROC>,
    pub glGenProgramsARB:                              Option<PFNGLGENPROGRAMSARBPROC>,
    pub glGenQueries:                                  Option<PFNGLGENQUERIESPROC>,
    pub glGenQueriesARB:                               Option<PFNGLGENQUERIESARBPROC>,
    pub glGenRenderbuffers:                            Option<PFNGLGENRENDERBUFFERSPROC>,
    pub glGenRenderbuffersEXT:                         Option<PFNGLGENRENDERBUFFERSEXTPROC>,
    pub glGenSamplers:                                 Option<PFNGLGENSAMPLERSPROC>,
    pub glGenTextures:                                 Option<PFNGLGENTEXTURESPROC>,
    pub glGenTransformFeedbacks:                       Option<PFNGLGENTRANSFORMFEEDBACKSPROC>,
    pub glGenVertexArrays:                             Option<PFNGLGENVERTEXARRAYSPROC>,
    pub glGenerateMipmap:                              Option<PFNGLGENERATEMIPMAPPROC>,
    pub glGenerateMipmapEXT:                           Option<PFNGLGENERATEMIPMAPEXTPROC>,
    pub glGenerateTextureMipmap:                       Option<PFNGLGENERATETEXTUREMIPMAPPROC>,
    pub glGetActiveAtomicCounterBufferiv:              Option<PFNGLGETACTIVEATOMICCOUNTERBUFFERIVPROC>,
    pub glGetActiveAttrib:                             Option<PFNGLGETACTIVEATTRIBPROC>,
    pub glGetActiveAttribARB:                          Option<PFNGLGETACTIVEATTRIBARBPROC>,
    pub glGetActiveSubroutineName:                     Option<PFNGLGETACTIVESUBROUTINENAMEPROC>,
    pub glGetActiveSubroutineUniformName:              Option<PFNGLGETACTIVESUBROUTINEUNIFORMNAMEPROC>,
    pub glGetActiveSubroutineUniformiv:                Option<PFNGLGETACTIVESUBROUTINEUNIFORMIVPROC>,
    pub glGetActiveUniform:                            Option<PFNGLGETACTIVEUNIFORMPROC>,
    pub glGetActiveUniformARB:                         Option<PFNGLGETACTIVEUNIFORMARBPROC>,
    pub glGetActiveUniformBlockName:                   Option<PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC>,
    pub glGetActiveUniformBlockiv:                     Option<PFNGLGETACTIVEUNIFORMBLOCKIVPROC>,
    pub glGetActiveUniformName:                        Option<PFNGLGETACTIVEUNIFORMNAMEPROC>,
    pub glGetActiveUniformsiv:                         Option<PFNGLGETACTIVEUNIFORMSIVPROC>,
    pub glGetAttachedObjectsARB:                       Option<PFNGLGETATTACHEDOBJECTSARBPROC>,
    pub glGetAttachedShaders:                          Option<PFNGLGETATTACHEDSHADERSPROC>,
    pub glGetAttribLocation:                           Option<PFNGLGETATTRIBLOCATIONPROC>,
    pub glGetAttribLocationARB:                        Option<PFNGLGETATTRIBLOCATIONARBPROC>,
    pub glGetBooleani_v:                               Option<PFNGLGETBOOLEANI_VPROC>,
    pub glGetBooleanv:                                 Option<PFNGLGETBOOLEANVPROC>,
    pub glGetBufferParameteri64v:                      Option<PFNGLGETBUFFERPARAMETERI64VPROC>,
    pub glGetBufferParameteriv:                        Option<PFNGLGETBUFFERPARAMETERIVPROC>,
    pub glGetBufferParameterivARB:                     Option<PFNGLGETBUFFERPARAMETERIVARBPROC>,
    pub glGetBufferPointerv:                           Option<PFNGLGETBUFFERPOINTERVPROC>,
    pub glGetBufferPointervARB:                        Option<PFNGLGETBUFFERPOINTERVARBPROC>,
    pub glGetBufferSubData:                            Option<PFNGLGETBUFFERSUBDATAPROC>,
    pub glGetBufferSubDataARB:                         Option<PFNGLGETBUFFERSUBDATAARBPROC>,
    pub glGetClipPlanexOES:                            Option<PFNGLGETCLIPPLANEXOESPROC>,
    pub glGetCompressedTexImage:                       Option<PFNGLGETCOMPRESSEDTEXIMAGEPROC>,
    pub glGetCompressedTexImageARB:                    Option<PFNGLGETCOMPRESSEDTEXIMAGEARBPROC>,
    pub glGetCompressedTextureImage:                   Option<PFNGLGETCOMPRESSEDTEXTUREIMAGEPROC>,
    pub glGetCompressedTextureSubImage:                Option<PFNGLGETCOMPRESSEDTEXTURESUBIMAGEPROC>,
    pub glGetConvolutionParameterxvOES:                Option<PFNGLGETCONVOLUTIONPARAMETERXVOESPROC>,
    pub glGetDebugMessageLog:                          Option<PFNGLGETDEBUGMESSAGELOGPROC>,
    pub glGetDebugMessageLogARB:                       Option<PFNGLGETDEBUGMESSAGELOGARBPROC>,
    pub glGetDoublei_v:                                Option<PFNGLGETDOUBLEI_VPROC>,
    pub glGetDoublev:                                  Option<PFNGLGETDOUBLEVPROC>,
    pub glGetError:                                    Option<PFNGLGETERRORPROC>,
    pub glGetFixedvOES:                                Option<PFNGLGETFIXEDVOESPROC>,
    pub glGetFloati_v:                                 Option<PFNGLGETFLOATI_VPROC>,
    pub glGetFloatv:                                   Option<PFNGLGETFLOATVPROC>,
    pub glGetFragDataIndex:                            Option<PFNGLGETFRAGDATAINDEXPROC>,
    pub glGetFragDataLocation:                         Option<PFNGLGETFRAGDATALOCATIONPROC>,
    pub glGetFramebufferAttachmentParameteriv:         Option<PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC>,
    pub glGetFramebufferAttachmentParameterivEXT:      Option<PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVEXTPROC>,
    pub glGetFramebufferParameteriv:                   Option<PFNGLGETFRAMEBUFFERPARAMETERIVPROC>,
    pub glGetHandleARB:                                Option<PFNGLGETHANDLEARBPROC>,
    pub glGetHistogramParameterxvOES:                  Option<PFNGLGETHISTOGRAMPARAMETERXVOESPROC>,
    pub glGetInfoLogARB:                               Option<PFNGLGETINFOLOGARBPROC>,
    pub glGetInteger64i_v:                             Option<PFNGLGETINTEGER64I_VPROC>,
    pub glGetInteger64v:                               Option<PFNGLGETINTEGER64VPROC>,
    pub glGetIntegeri_v:                               Option<PFNGLGETINTEGERI_VPROC>,
    pub glGetIntegerv:                                 Option<PFNGLGETINTEGERVPROC>,
    pub glGetInternalformati64v:                       Option<PFNGLGETINTERNALFORMATI64VPROC>,
    pub glGetInternalformativ:                         Option<PFNGLGETINTERNALFORMATIVPROC>,
    pub glGetLightxOES:                                Option<PFNGLGETLIGHTXOESPROC>,
    pub glGetMapxvOES:                                 Option<PFNGLGETMAPXVOESPROC>,
    pub glGetMaterialxOES:                             Option<PFNGLGETMATERIALXOESPROC>,
    pub glGetMultisamplefv:                            Option<PFNGLGETMULTISAMPLEFVPROC>,
    pub glGetNamedBufferParameteri64v:                 Option<PFNGLGETNAMEDBUFFERPARAMETERI64VPROC>,
    pub glGetNamedBufferParameteriv:                   Option<PFNGLGETNAMEDBUFFERPARAMETERIVPROC>,
    pub glGetNamedBufferPointerv:                      Option<PFNGLGETNAMEDBUFFERPOINTERVPROC>,
    pub glGetNamedBufferSubData:                       Option<PFNGLGETNAMEDBUFFERSUBDATAPROC>,
    pub glGetNamedFramebufferAttachmentParameteriv:    Option<PFNGLGETNAMEDFRAMEBUFFERATTACHMENTPARAMETERIVPROC>,
    pub glGetNamedFramebufferParameteriv:              Option<PFNGLGETNAMEDFRAMEBUFFERPARAMETERIVPROC>,
    pub glGetNamedRenderbufferParameteriv:             Option<PFNGLGETNAMEDRENDERBUFFERPARAMETERIVPROC>,
    pub glGetNamedStringARB:                           Option<PFNGLGETNAMEDSTRINGARBPROC>,
    pub glGetNamedStringivARB:                         Option<PFNGLGETNAMEDSTRINGIVARBPROC>,
    pub glGetObjectLabel:                              Option<PFNGLGETOBJECTLABELPROC>,
    pub glGetObjectParameterfvARB:                     Option<PFNGLGETOBJECTPARAMETERFVARBPROC>,
    pub glGetObjectParameterivARB:                     Option<PFNGLGETOBJECTPARAMETERIVARBPROC>,
    pub glGetObjectPtrLabel:                           Option<PFNGLGETOBJECTPTRLABELPROC>,
    pub glGetPixelMapxv:                               Option<PFNGLGETPIXELMAPXVPROC>,
    pub glGetPointerv:                                 Option<PFNGLGETPOINTERVPROC>,
    pub glGetProgramBinary:                            Option<PFNGLGETPROGRAMBINARYPROC>,
    pub glGetProgramEnvParameterdvARB:                 Option<PFNGLGETPROGRAMENVPARAMETERDVARBPROC>,
    pub glGetProgramEnvParameterfvARB:                 Option<PFNGLGETPROGRAMENVPARAMETERFVARBPROC>,
    pub glGetProgramInfoLog:                           Option<PFNGLGETPROGRAMINFOLOGPROC>,
    pub glGetProgramInterfaceiv:                       Option<PFNGLGETPROGRAMINTERFACEIVPROC>,
    pub glGetProgramLocalParameterdvARB:               Option<PFNGLGETPROGRAMLOCALPARAMETERDVARBPROC>,
    pub glGetProgramLocalParameterfvARB:               Option<PFNGLGETPROGRAMLOCALPARAMETERFVARBPROC>,
    pub glGetProgramPipelineInfoLog:                   Option<PFNGLGETPROGRAMPIPELINEINFOLOGPROC>,
    pub glGetProgramPipelineiv:                        Option<PFNGLGETPROGRAMPIPELINEIVPROC>,
    pub glGetProgramResourceIndex:                     Option<PFNGLGETPROGRAMRESOURCEINDEXPROC>,
    pub glGetProgramResourceLocation:                  Option<PFNGLGETPROGRAMRESOURCELOCATIONPROC>,
    pub glGetProgramResourceLocationIndex:             Option<PFNGLGETPROGRAMRESOURCELOCATIONINDEXPROC>,
    pub glGetProgramResourceName:                      Option<PFNGLGETPROGRAMRESOURCENAMEPROC>,
    pub glGetProgramResourceiv:                        Option<PFNGLGETPROGRAMRESOURCEIVPROC>,
    pub glGetProgramStageiv:                           Option<PFNGLGETPROGRAMSTAGEIVPROC>,
    pub glGetProgramStringARB:                         Option<PFNGLGETPROGRAMSTRINGARBPROC>,
    pub glGetProgramiv:                                Option<PFNGLGETPROGRAMIVPROC>,
    pub glGetProgramivARB:                             Option<PFNGLGETPROGRAMIVARBPROC>,
    pub glGetQueryBufferObjecti64v:                    Option<PFNGLGETQUERYBUFFEROBJECTI64VPROC>,
    pub glGetQueryBufferObjectiv:                      Option<PFNGLGETQUERYBUFFEROBJECTIVPROC>,
    pub glGetQueryBufferObjectui64v:                   Option<PFNGLGETQUERYBUFFEROBJECTUI64VPROC>,
    pub glGetQueryBufferObjectuiv:                     Option<PFNGLGETQUERYBUFFEROBJECTUIVPROC>,
    pub glGetQueryIndexediv:                           Option<PFNGLGETQUERYINDEXEDIVPROC>,
    pub glGetQueryObjecti64v:                          Option<PFNGLGETQUERYOBJECTI64VPROC>,
    pub glGetQueryObjectiv:                            Option<PFNGLGETQUERYOBJECTIVPROC>,
    pub glGetQueryObjectivARB:                         Option<PFNGLGETQUERYOBJECTIVARBPROC>,
    pub glGetQueryObjectui64v:                         Option<PFNGLGETQUERYOBJECTUI64VPROC>,
    pub glGetQueryObjectuiv:                           Option<PFNGLGETQUERYOBJECTUIVPROC>,
    pub glGetQueryObjectuivARB:                        Option<PFNGLGETQUERYOBJECTUIVARBPROC>,
    pub glGetQueryiv:                                  Option<PFNGLGETQUERYIVPROC>,
    pub glGetQueryivARB:                               Option<PFNGLGETQUERYIVARBPROC>,
    pub glGetRenderbufferParameteriv:                  Option<PFNGLGETRENDERBUFFERPARAMETERIVPROC>,
    pub glGetRenderbufferParameterivEXT:               Option<PFNGLGETRENDERBUFFERPARAMETERIVEXTPROC>,
    pub glGetSamplerParameterIiv:                      Option<PFNGLGETSAMPLERPARAMETERIIVPROC>,
    pub glGetSamplerParameterIuiv:                     Option<PFNGLGETSAMPLERPARAMETERIUIVPROC>,
    pub glGetSamplerParameterfv:                       Option<PFNGLGETSAMPLERPARAMETERFVPROC>,
    pub glGetSamplerParameteriv:                       Option<PFNGLGETSAMPLERPARAMETERIVPROC>,
    pub glGetShaderInfoLog:                            Option<PFNGLGETSHADERINFOLOGPROC>,
    pub glGetShaderPrecisionFormat:                    Option<PFNGLGETSHADERPRECISIONFORMATPROC>,
    pub glGetShaderSource:                             Option<PFNGLGETSHADERSOURCEPROC>,
    pub glGetShaderSourceARB:                          Option<PFNGLGETSHADERSOURCEARBPROC>,
    pub glGetShaderiv:                                 Option<PFNGLGETSHADERIVPROC>,
    pub glGetString:                                   Option<PFNGLGETSTRINGPROC>,
    pub glGetStringi:                                  Option<PFNGLGETSTRINGIPROC>,
    pub glGetSubroutineIndex:                          Option<PFNGLGETSUBROUTINEINDEXPROC>,
    pub glGetSubroutineUniformLocation:                Option<PFNGLGETSUBROUTINEUNIFORMLOCATIONPROC>,
    pub glGetSynciv:                                   Option<PFNGLGETSYNCIVPROC>,
    pub glGetTexEnvxvOES:                              Option<PFNGLGETTEXENVXVOESPROC>,
    pub glGetTexGenxvOES:                              Option<PFNGLGETTEXGENXVOESPROC>,
    pub glGetTexImage:                                 Option<PFNGLGETTEXIMAGEPROC>,
    pub glGetTexLevelParameterfv:                      Option<PFNGLGETTEXLEVELPARAMETERFVPROC>,
    pub glGetTexLevelParameteriv:                      Option<PFNGLGETTEXLEVELPARAMETERIVPROC>,
    pub glGetTexLevelParameterxvOES:                   Option<PFNGLGETTEXLEVELPARAMETERXVOESPROC>,
    pub glGetTexParameterIiv:                          Option<PFNGLGETTEXPARAMETERIIVPROC>,
    pub glGetTexParameterIuiv:                         Option<PFNGLGETTEXPARAMETERIUIVPROC>,
    pub glGetTexParameterfv:                           Option<PFNGLGETTEXPARAMETERFVPROC>,
    pub glGetTexParameteriv:                           Option<PFNGLGETTEXPARAMETERIVPROC>,
    pub glGetTexParameterxvOES:                        Option<PFNGLGETTEXPARAMETERXVOESPROC>,
    pub glGetTextureImage:                             Option<PFNGLGETTEXTUREIMAGEPROC>,
    pub glGetTextureLevelParameterfv:                  Option<PFNGLGETTEXTURELEVELPARAMETERFVPROC>,
    pub glGetTextureLevelParameteriv:                  Option<PFNGLGETTEXTURELEVELPARAMETERIVPROC>,
    pub glGetTextureParameterIiv:                      Option<PFNGLGETTEXTUREPARAMETERIIVPROC>,
    pub glGetTextureParameterIuiv:                     Option<PFNGLGETTEXTUREPARAMETERIUIVPROC>,
    pub glGetTextureParameterfv:                       Option<PFNGLGETTEXTUREPARAMETERFVPROC>,
    pub glGetTextureParameteriv:                       Option<PFNGLGETTEXTUREPARAMETERIVPROC>,
    pub glGetTextureSubImage:                          Option<PFNGLGETTEXTURESUBIMAGEPROC>,
    pub glGetTransformFeedbackVarying:                 Option<PFNGLGETTRANSFORMFEEDBACKVARYINGPROC>,
    pub glGetTransformFeedbacki64_v:                   Option<PFNGLGETTRANSFORMFEEDBACKI64_VPROC>,
    pub glGetTransformFeedbacki_v:                     Option<PFNGLGETTRANSFORMFEEDBACKI_VPROC>,
    pub glGetTransformFeedbackiv:                      Option<PFNGLGETTRANSFORMFEEDBACKIVPROC>,
    pub glGetUniformBlockIndex:                        Option<PFNGLGETUNIFORMBLOCKINDEXPROC>,
    pub glGetUniformIndices:                           Option<PFNGLGETUNIFORMINDICESPROC>,
    pub glGetUniformLocation:                          Option<PFNGLGETUNIFORMLOCATIONPROC>,
    pub glGetUniformLocationARB:                       Option<PFNGLGETUNIFORMLOCATIONARBPROC>,
    pub glGetUniformSubroutineuiv:                     Option<PFNGLGETUNIFORMSUBROUTINEUIVPROC>,
    pub glGetUniformdv:                                Option<PFNGLGETUNIFORMDVPROC>,
    pub glGetUniformfv:                                Option<PFNGLGETUNIFORMFVPROC>,
    pub glGetUniformfvARB:                             Option<PFNGLGETUNIFORMFVARBPROC>,
    pub glGetUniformi64vARB:                           Option<PFNGLGETUNIFORMI64VARBPROC>,
    pub glGetUniformiv:                                Option<PFNGLGETUNIFORMIVPROC>,
    pub glGetUniformivARB:                             Option<PFNGLGETUNIFORMIVARBPROC>,
    pub glGetUniformui64vARB:                          Option<PFNGLGETUNIFORMUI64VARBPROC>,
    pub glGetUniformuiv:                               Option<PFNGLGETUNIFORMUIVPROC>,
    pub glGetVertexArrayIndexed64iv:                   Option<PFNGLGETVERTEXARRAYINDEXED64IVPROC>,
    pub glGetVertexArrayIndexediv:                     Option<PFNGLGETVERTEXARRAYINDEXEDIVPROC>,
    pub glGetVertexArrayiv:                            Option<PFNGLGETVERTEXARRAYIVPROC>,
    pub glGetVertexAttribIiv:                          Option<PFNGLGETVERTEXATTRIBIIVPROC>,
    pub glGetVertexAttribIuiv:                         Option<PFNGLGETVERTEXATTRIBIUIVPROC>,
    pub glGetVertexAttribLdv:                          Option<PFNGLGETVERTEXATTRIBLDVPROC>,
    pub glGetVertexAttribPointerv:                     Option<PFNGLGETVERTEXATTRIBPOINTERVPROC>,
    pub glGetVertexAttribPointervARB:                  Option<PFNGLGETVERTEXATTRIBPOINTERVARBPROC>,
    pub glGetVertexAttribdv:                           Option<PFNGLGETVERTEXATTRIBDVPROC>,
    pub glGetVertexAttribdvARB:                        Option<PFNGLGETVERTEXATTRIBDVARBPROC>,
    pub glGetVertexAttribfv:                           Option<PFNGLGETVERTEXATTRIBFVPROC>,
    pub glGetVertexAttribfvARB:                        Option<PFNGLGETVERTEXATTRIBFVARBPROC>,
    pub glGetVertexAttribiv:                           Option<PFNGLGETVERTEXATTRIBIVPROC>,
    pub glGetVertexAttribivARB:                        Option<PFNGLGETVERTEXATTRIBIVARBPROC>,
    pub glGetnUniformi64vARB:                          Option<PFNGLGETNUNIFORMI64VARBPROC>,
    pub glGetnUniformui64vARB:                         Option<PFNGLGETNUNIFORMUI64VARBPROC>,
    pub glHint:                                        Option<PFNGLHINTPROC>,
    pub glIndexxOES:                                   Option<PFNGLINDEXXOESPROC>,
    pub glIndexxvOES:                                  Option<PFNGLINDEXXVOESPROC>,
    pub glInvalidateBufferData:                        Option<PFNGLINVALIDATEBUFFERDATAPROC>,
    pub glInvalidateBufferSubData:                     Option<PFNGLINVALIDATEBUFFERSUBDATAPROC>,
    pub glInvalidateFramebuffer:                       Option<PFNGLINVALIDATEFRAMEBUFFERPROC>,
    pub glInvalidateNamedFramebufferData:              Option<PFNGLINVALIDATENAMEDFRAMEBUFFERDATAPROC>,
    pub glInvalidateNamedFramebufferSubData:           Option<PFNGLINVALIDATENAMEDFRAMEBUFFERSUBDATAPROC>,
    pub glInvalidateSubFramebuffer:                    Option<PFNGLINVALIDATESUBFRAMEBUFFERPROC>,
    pub glInvalidateTexImage:                          Option<PFNGLINVALIDATETEXIMAGEPROC>,
    pub glInvalidateTexSubImage:                       Option<PFNGLINVALIDATETEXSUBIMAGEPROC>,
    pub glIsBuffer:                                    Option<PFNGLISBUFFERPROC>,
    pub glIsBufferARB:                                 Option<PFNGLISBUFFERARBPROC>,
    pub glIsEnabled:                                   Option<PFNGLISENABLEDPROC>,
    pub glIsEnabledi:                                  Option<PFNGLISENABLEDIPROC>,
    pub glIsFramebuffer:                               Option<PFNGLISFRAMEBUFFERPROC>,
    pub glIsFramebufferEXT:                            Option<PFNGLISFRAMEBUFFEREXTPROC>,
    pub glIsNamedStringARB:                            Option<PFNGLISNAMEDSTRINGARBPROC>,
    pub glIsProgram:                                   Option<PFNGLISPROGRAMPROC>,
    pub glIsProgramARB:                                Option<PFNGLISPROGRAMARBPROC>,
    pub glIsProgramPipeline:                           Option<PFNGLISPROGRAMPIPELINEPROC>,
    pub glIsQuery:                                     Option<PFNGLISQUERYPROC>,
    pub glIsQueryARB:                                  Option<PFNGLISQUERYARBPROC>,
    pub glIsRenderbuffer:                              Option<PFNGLISRENDERBUFFERPROC>,
    pub glIsRenderbufferEXT:                           Option<PFNGLISRENDERBUFFEREXTPROC>,
    pub glIsSampler:                                   Option<PFNGLISSAMPLERPROC>,
    pub glIsShader:                                    Option<PFNGLISSHADERPROC>,
    pub glIsSync:                                      Option<PFNGLISSYNCPROC>,
    pub glIsTexture:                                   Option<PFNGLISTEXTUREPROC>,
    pub glIsTransformFeedback:                         Option<PFNGLISTRANSFORMFEEDBACKPROC>,
    pub glIsVertexArray:                               Option<PFNGLISVERTEXARRAYPROC>,
    pub glLightModelxOES:                              Option<PFNGLLIGHTMODELXOESPROC>,
    pub glLightModelxvOES:                             Option<PFNGLLIGHTMODELXVOESPROC>,
    pub glLightxOES:                                   Option<PFNGLLIGHTXOESPROC>,
    pub glLightxvOES:                                  Option<PFNGLLIGHTXVOESPROC>,
    pub glLineWidth:                                   Option<PFNGLLINEWIDTHPROC>,
    pub glLineWidthxOES:                               Option<PFNGLLINEWIDTHXOESPROC>,
    pub glLinkProgram:                                 Option<PFNGLLINKPROGRAMPROC>,
    pub glLinkProgramARB:                              Option<PFNGLLINKPROGRAMARBPROC>,
    pub glLoadMatrixxOES:                              Option<PFNGLLOADMATRIXXOESPROC>,
    pub glLoadTransposeMatrixdARB:                     Option<PFNGLLOADTRANSPOSEMATRIXDARBPROC>,
    pub glLoadTransposeMatrixfARB:                     Option<PFNGLLOADTRANSPOSEMATRIXFARBPROC>,
    pub glLoadTransposeMatrixxOES:                     Option<PFNGLLOADTRANSPOSEMATRIXXOESPROC>,
    pub glLogicOp:                                     Option<PFNGLLOGICOPPROC>,
    pub glMap1xOES:                                    Option<PFNGLMAP1XOESPROC>,
    pub glMap2xOES:                                    Option<PFNGLMAP2XOESPROC>,
    pub glMapBuffer:                                   Option<PFNGLMAPBUFFERPROC>,
    pub glMapBufferARB:                                Option<PFNGLMAPBUFFERARBPROC>,
    pub glMapBufferRange:                              Option<PFNGLMAPBUFFERRANGEPROC>,
    pub glMapGrid1xOES:                                Option<PFNGLMAPGRID1XOESPROC>,
    pub glMapGrid2xOES:                                Option<PFNGLMAPGRID2XOESPROC>,
    pub glMapNamedBuffer:                              Option<PFNGLMAPNAMEDBUFFERPROC>,
    pub glMapNamedBufferRange:                         Option<PFNGLMAPNAMEDBUFFERRANGEPROC>,
    pub glMaterialxOES:                                Option<PFNGLMATERIALXOESPROC>,
    pub glMaterialxvOES:                               Option<PFNGLMATERIALXVOESPROC>,
    pub glMemoryBarrier:                               Option<PFNGLMEMORYBARRIERPROC>,
    pub glMemoryBarrierByRegion:                       Option<PFNGLMEMORYBARRIERBYREGIONPROC>,
    pub glMinSampleShading:                            Option<PFNGLMINSAMPLESHADINGPROC>,
    pub glMinSampleShadingARB:                         Option<PFNGLMINSAMPLESHADINGARBPROC>,
    pub glMultMatrixxOES:                              Option<PFNGLMULTMATRIXXOESPROC>,
    pub glMultTransposeMatrixdARB:                     Option<PFNGLMULTTRANSPOSEMATRIXDARBPROC>,
    pub glMultTransposeMatrixfARB:                     Option<PFNGLMULTTRANSPOSEMATRIXFARBPROC>,
    pub glMultTransposeMatrixxOES:                     Option<PFNGLMULTTRANSPOSEMATRIXXOESPROC>,
    pub glMultiDrawArrays:                             Option<PFNGLMULTIDRAWARRAYSPROC>,
    pub glMultiDrawArraysIndirect:                     Option<PFNGLMULTIDRAWARRAYSINDIRECTPROC>,
    pub glMultiDrawElements:                           Option<PFNGLMULTIDRAWELEMENTSPROC>,
    pub glMultiDrawElementsBaseVertex:                 Option<PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC>,
    pub glMultiDrawElementsIndirect:                   Option<PFNGLMULTIDRAWELEMENTSINDIRECTPROC>,
    pub glMultiTexCoord1dARB:                          Option<PFNGLMULTITEXCOORD1DARBPROC>,
    pub glMultiTexCoord1dvARB:                         Option<PFNGLMULTITEXCOORD1DVARBPROC>,
    pub glMultiTexCoord1fARB:                          Option<PFNGLMULTITEXCOORD1FARBPROC>,
    pub glMultiTexCoord1fvARB:                         Option<PFNGLMULTITEXCOORD1FVARBPROC>,
    pub glMultiTexCoord1iARB:                          Option<PFNGLMULTITEXCOORD1IARBPROC>,
    pub glMultiTexCoord1ivARB:                         Option<PFNGLMULTITEXCOORD1IVARBPROC>,
    pub glMultiTexCoord1sARB:                          Option<PFNGLMULTITEXCOORD1SARBPROC>,
    pub glMultiTexCoord1svARB:                         Option<PFNGLMULTITEXCOORD1SVARBPROC>,
    pub glMultiTexCoord1xOES:                          Option<PFNGLMULTITEXCOORD1XOESPROC>,
    pub glMultiTexCoord1xvOES:                         Option<PFNGLMULTITEXCOORD1XVOESPROC>,
    pub glMultiTexCoord2dARB:                          Option<PFNGLMULTITEXCOORD2DARBPROC>,
    pub glMultiTexCoord2dvARB:                         Option<PFNGLMULTITEXCOORD2DVARBPROC>,
    pub glMultiTexCoord2fARB:                          Option<PFNGLMULTITEXCOORD2FARBPROC>,
    pub glMultiTexCoord2fvARB:                         Option<PFNGLMULTITEXCOORD2FVARBPROC>,
    pub glMultiTexCoord2iARB:                          Option<PFNGLMULTITEXCOORD2IARBPROC>,
    pub glMultiTexCoord2ivARB:                         Option<PFNGLMULTITEXCOORD2IVARBPROC>,
    pub glMultiTexCoord2sARB:                          Option<PFNGLMULTITEXCOORD2SARBPROC>,
    pub glMultiTexCoord2svARB:                         Option<PFNGLMULTITEXCOORD2SVARBPROC>,
    pub glMultiTexCoord2xOES:                          Option<PFNGLMULTITEXCOORD2XOESPROC>,
    pub glMultiTexCoord2xvOES:                         Option<PFNGLMULTITEXCOORD2XVOESPROC>,
    pub glMultiTexCoord3dARB:                          Option<PFNGLMULTITEXCOORD3DARBPROC>,
    pub glMultiTexCoord3dvARB:                         Option<PFNGLMULTITEXCOORD3DVARBPROC>,
    pub glMultiTexCoord3fARB:                          Option<PFNGLMULTITEXCOORD3FARBPROC>,
    pub glMultiTexCoord3fvARB:                         Option<PFNGLMULTITEXCOORD3FVARBPROC>,
    pub glMultiTexCoord3iARB:                          Option<PFNGLMULTITEXCOORD3IARBPROC>,
    pub glMultiTexCoord3ivARB:                         Option<PFNGLMULTITEXCOORD3IVARBPROC>,
    pub glMultiTexCoord3sARB:                          Option<PFNGLMULTITEXCOORD3SARBPROC>,
    pub glMultiTexCoord3svARB:                         Option<PFNGLMULTITEXCOORD3SVARBPROC>,
    pub glMultiTexCoord3xOES:                          Option<PFNGLMULTITEXCOORD3XOESPROC>,
    pub glMultiTexCoord3xvOES:                         Option<PFNGLMULTITEXCOORD3XVOESPROC>,
    pub glMultiTexCoord4dARB:                          Option<PFNGLMULTITEXCOORD4DARBPROC>,
    pub glMultiTexCoord4dvARB:                         Option<PFNGLMULTITEXCOORD4DVARBPROC>,
    pub glMultiTexCoord4fARB:                          Option<PFNGLMULTITEXCOORD4FARBPROC>,
    pub glMultiTexCoord4fvARB:                         Option<PFNGLMULTITEXCOORD4FVARBPROC>,
    pub glMultiTexCoord4iARB:                          Option<PFNGLMULTITEXCOORD4IARBPROC>,
    pub glMultiTexCoord4ivARB:                         Option<PFNGLMULTITEXCOORD4IVARBPROC>,
    pub glMultiTexCoord4sARB:                          Option<PFNGLMULTITEXCOORD4SARBPROC>,
    pub glMultiTexCoord4svARB:                         Option<PFNGLMULTITEXCOORD4SVARBPROC>,
    pub glMultiTexCoord4xOES:                          Option<PFNGLMULTITEXCOORD4XOESPROC>,
    pub glMultiTexCoord4xvOES:                         Option<PFNGLMULTITEXCOORD4XVOESPROC>,
    pub glNamedBufferData:                             Option<PFNGLNAMEDBUFFERDATAPROC>,
    pub glNamedBufferStorage:                          Option<PFNGLNAMEDBUFFERSTORAGEPROC>,
    pub glNamedBufferSubData:                          Option<PFNGLNAMEDBUFFERSUBDATAPROC>,
    pub glNamedFramebufferDrawBuffer:                  Option<PFNGLNAMEDFRAMEBUFFERDRAWBUFFERPROC>,
    pub glNamedFramebufferDrawBuffers:                 Option<PFNGLNAMEDFRAMEBUFFERDRAWBUFFERSPROC>,
    pub glNamedFramebufferParameteri:                  Option<PFNGLNAMEDFRAMEBUFFERPARAMETERIPROC>,
    pub glNamedFramebufferReadBuffer:                  Option<PFNGLNAMEDFRAMEBUFFERREADBUFFERPROC>,
    pub glNamedFramebufferRenderbuffer:                Option<PFNGLNAMEDFRAMEBUFFERRENDERBUFFERPROC>,
    pub glNamedFramebufferSampleLocationsfvARB:        Option<PFNGLNAMEDFRAMEBUFFERSAMPLELOCATIONSFVARBPROC>,
    pub glNamedFramebufferTexture:                     Option<PFNGLNAMEDFRAMEBUFFERTEXTUREPROC>,
    pub glNamedFramebufferTextureLayer:                Option<PFNGLNAMEDFRAMEBUFFERTEXTURELAYERPROC>,
    pub glNamedRenderbufferStorage:                    Option<PFNGLNAMEDRENDERBUFFERSTORAGEPROC>,
    pub glNamedRenderbufferStorageMultisample:         Option<PFNGLNAMEDRENDERBUFFERSTORAGEMULTISAMPLEPROC>,
    pub glNamedStringARB:                              Option<PFNGLNAMEDSTRINGARBPROC>,
    pub glNormal3xOES:                                 Option<PFNGLNORMAL3XOESPROC>,
    pub glNormal3xvOES:                                Option<PFNGLNORMAL3XVOESPROC>,
    pub glObjectLabel:                                 Option<PFNGLOBJECTLABELPROC>,
    pub glObjectPtrLabel:                              Option<PFNGLOBJECTPTRLABELPROC>,
    pub glOrthoxOES:                                   Option<PFNGLORTHOXOESPROC>,
    pub glPassThroughxOES:                             Option<PFNGLPASSTHROUGHXOESPROC>,
    pub glPatchParameterfv:                            Option<PFNGLPATCHPARAMETERFVPROC>,
    pub glPatchParameteri:                             Option<PFNGLPATCHPARAMETERIPROC>,
    pub glPauseTransformFeedback:                      Option<PFNGLPAUSETRANSFORMFEEDBACKPROC>,
    pub glPixelMapx:                                   Option<PFNGLPIXELMAPXPROC>,
    pub glPixelStoref:                                 Option<PFNGLPIXELSTOREFPROC>,
    pub glPixelStorei:                                 Option<PFNGLPIXELSTOREIPROC>,
    pub glPixelStorex:                                 Option<PFNGLPIXELSTOREXPROC>,
    pub glPixelTransferxOES:                           Option<PFNGLPIXELTRANSFERXOESPROC>,
    pub glPixelZoomxOES:                               Option<PFNGLPIXELZOOMXOESPROC>,
    pub glPointParameterf:                             Option<PFNGLPOINTPARAMETERFPROC>,
    pub glPointParameterfv:                            Option<PFNGLPOINTPARAMETERFVPROC>,
    pub glPointParameteri:                             Option<PFNGLPOINTPARAMETERIPROC>,
    pub glPointParameteriv:                            Option<PFNGLPOINTPARAMETERIVPROC>,
    pub glPointParameterxvOES:                         Option<PFNGLPOINTPARAMETERXVOESPROC>,
    pub glPointSize:                                   Option<PFNGLPOINTSIZEPROC>,
    pub glPointSizexOES:                               Option<PFNGLPOINTSIZEXOESPROC>,
    pub glPolygonMode:                                 Option<PFNGLPOLYGONMODEPROC>,
    pub glPolygonOffset:                               Option<PFNGLPOLYGONOFFSETPROC>,
    pub glPolygonOffsetxOES:                           Option<PFNGLPOLYGONOFFSETXOESPROC>,
    pub glPopDebugGroup:                               Option<PFNGLPOPDEBUGGROUPPROC>,
    pub glPrimitiveBoundingBoxARB:                     Option<PFNGLPRIMITIVEBOUNDINGBOXARBPROC>,
    pub glPrimitiveRestartIndex:                       Option<PFNGLPRIMITIVERESTARTINDEXPROC>,
    pub glPrioritizeTexturesxOES:                      Option<PFNGLPRIORITIZETEXTURESXOESPROC>,
    pub glProgramBinary:                               Option<PFNGLPROGRAMBINARYPROC>,
    pub glProgramEnvParameter4dARB:                    Option<PFNGLPROGRAMENVPARAMETER4DARBPROC>,
    pub glProgramEnvParameter4dvARB:                   Option<PFNGLPROGRAMENVPARAMETER4DVARBPROC>,
    pub glProgramEnvParameter4fARB:                    Option<PFNGLPROGRAMENVPARAMETER4FARBPROC>,
    pub glProgramEnvParameter4fvARB:                   Option<PFNGLPROGRAMENVPARAMETER4FVARBPROC>,
    pub glProgramLocalParameter4dARB:                  Option<PFNGLPROGRAMLOCALPARAMETER4DARBPROC>,
    pub glProgramLocalParameter4dvARB:                 Option<PFNGLPROGRAMLOCALPARAMETER4DVARBPROC>,
    pub glProgramLocalParameter4fARB:                  Option<PFNGLPROGRAMLOCALPARAMETER4FARBPROC>,
    pub glProgramLocalParameter4fvARB:                 Option<PFNGLPROGRAMLOCALPARAMETER4FVARBPROC>,
    pub glProgramParameteri:                           Option<PFNGLPROGRAMPARAMETERIPROC>,
    pub glProgramParameteriARB:                        Option<PFNGLPROGRAMPARAMETERIARBPROC>,
    pub glProgramStringARB:                            Option<PFNGLPROGRAMSTRINGARBPROC>,
    pub glProgramUniform1d:                            Option<PFNGLPROGRAMUNIFORM1DPROC>,
    pub glProgramUniform1dv:                           Option<PFNGLPROGRAMUNIFORM1DVPROC>,
    pub glProgramUniform1f:                            Option<PFNGLPROGRAMUNIFORM1FPROC>,
    pub glProgramUniform1fv:                           Option<PFNGLPROGRAMUNIFORM1FVPROC>,
    pub glProgramUniform1i:                            Option<PFNGLPROGRAMUNIFORM1IPROC>,
    pub glProgramUniform1i64ARB:                       Option<PFNGLPROGRAMUNIFORM1I64ARBPROC>,
    pub glProgramUniform1i64vARB:                      Option<PFNGLPROGRAMUNIFORM1I64VARBPROC>,
    pub glProgramUniform1iv:                           Option<PFNGLPROGRAMUNIFORM1IVPROC>,
    pub glProgramUniform1ui:                           Option<PFNGLPROGRAMUNIFORM1UIPROC>,
    pub glProgramUniform1ui64ARB:                      Option<PFNGLPROGRAMUNIFORM1UI64ARBPROC>,
    pub glProgramUniform1ui64vARB:                     Option<PFNGLPROGRAMUNIFORM1UI64VARBPROC>,
    pub glProgramUniform1uiv:                          Option<PFNGLPROGRAMUNIFORM1UIVPROC>,
    pub glProgramUniform2d:                            Option<PFNGLPROGRAMUNIFORM2DPROC>,
    pub glProgramUniform2dv:                           Option<PFNGLPROGRAMUNIFORM2DVPROC>,
    pub glProgramUniform2f:                            Option<PFNGLPROGRAMUNIFORM2FPROC>,
    pub glProgramUniform2fv:                           Option<PFNGLPROGRAMUNIFORM2FVPROC>,
    pub glProgramUniform2i:                            Option<PFNGLPROGRAMUNIFORM2IPROC>,
    pub glProgramUniform2i64ARB:                       Option<PFNGLPROGRAMUNIFORM2I64ARBPROC>,
    pub glProgramUniform2i64vARB:                      Option<PFNGLPROGRAMUNIFORM2I64VARBPROC>,
    pub glProgramUniform2iv:                           Option<PFNGLPROGRAMUNIFORM2IVPROC>,
    pub glProgramUniform2ui:                           Option<PFNGLPROGRAMUNIFORM2UIPROC>,
    pub glProgramUniform2ui64ARB:                      Option<PFNGLPROGRAMUNIFORM2UI64ARBPROC>,
    pub glProgramUniform2ui64vARB:                     Option<PFNGLPROGRAMUNIFORM2UI64VARBPROC>,
    pub glProgramUniform2uiv:                          Option<PFNGLPROGRAMUNIFORM2UIVPROC>,
    pub glProgramUniform3d:                            Option<PFNGLPROGRAMUNIFORM3DPROC>,
    pub glProgramUniform3dv:                           Option<PFNGLPROGRAMUNIFORM3DVPROC>,
    pub glProgramUniform3f:                            Option<PFNGLPROGRAMUNIFORM3FPROC>,
    pub glProgramUniform3fv:                           Option<PFNGLPROGRAMUNIFORM3FVPROC>,
    pub glProgramUniform3i:                            Option<PFNGLPROGRAMUNIFORM3IPROC>,
    pub glProgramUniform3i64ARB:                       Option<PFNGLPROGRAMUNIFORM3I64ARBPROC>,
    pub glProgramUniform3i64vARB:                      Option<PFNGLPROGRAMUNIFORM3I64VARBPROC>,
    pub glProgramUniform3iv:                           Option<PFNGLPROGRAMUNIFORM3IVPROC>,
    pub glProgramUniform3ui:                           Option<PFNGLPROGRAMUNIFORM3UIPROC>,
    pub glProgramUniform3ui64ARB:                      Option<PFNGLPROGRAMUNIFORM3UI64ARBPROC>,
    pub glProgramUniform3ui64vARB:                     Option<PFNGLPROGRAMUNIFORM3UI64VARBPROC>,
    pub glProgramUniform3uiv:                          Option<PFNGLPROGRAMUNIFORM3UIVPROC>,
    pub glProgramUniform4d:                            Option<PFNGLPROGRAMUNIFORM4DPROC>,
    pub glProgramUniform4dv:                           Option<PFNGLPROGRAMUNIFORM4DVPROC>,
    pub glProgramUniform4f:                            Option<PFNGLPROGRAMUNIFORM4FPROC>,
    pub glProgramUniform4fv:                           Option<PFNGLPROGRAMUNIFORM4FVPROC>,
    pub glProgramUniform4i:                            Option<PFNGLPROGRAMUNIFORM4IPROC>,
    pub glProgramUniform4i64ARB:                       Option<PFNGLPROGRAMUNIFORM4I64ARBPROC>,
    pub glProgramUniform4i64vARB:                      Option<PFNGLPROGRAMUNIFORM4I64VARBPROC>,
    pub glProgramUniform4iv:                           Option<PFNGLPROGRAMUNIFORM4IVPROC>,
    pub glProgramUniform4ui:                           Option<PFNGLPROGRAMUNIFORM4UIPROC>,
    pub glProgramUniform4ui64ARB:                      Option<PFNGLPROGRAMUNIFORM4UI64ARBPROC>,
    pub glProgramUniform4ui64vARB:                     Option<PFNGLPROGRAMUNIFORM4UI64VARBPROC>,
    pub glProgramUniform4uiv:                          Option<PFNGLPROGRAMUNIFORM4UIVPROC>,
    pub glProgramUniformMatrix2dv:                     Option<PFNGLPROGRAMUNIFORMMATRIX2DVPROC>,
    pub glProgramUniformMatrix2fv:                     Option<PFNGLPROGRAMUNIFORMMATRIX2FVPROC>,
    pub glProgramUniformMatrix2x3dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX2X3DVPROC>,
    pub glProgramUniformMatrix2x3fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX2X3FVPROC>,
    pub glProgramUniformMatrix2x4dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX2X4DVPROC>,
    pub glProgramUniformMatrix2x4fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX2X4FVPROC>,
    pub glProgramUniformMatrix3dv:                     Option<PFNGLPROGRAMUNIFORMMATRIX3DVPROC>,
    pub glProgramUniformMatrix3fv:                     Option<PFNGLPROGRAMUNIFORMMATRIX3FVPROC>,
    pub glProgramUniformMatrix3x2dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX3X2DVPROC>,
    pub glProgramUniformMatrix3x2fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX3X2FVPROC>,
    pub glProgramUniformMatrix3x4dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX3X4DVPROC>,
    pub glProgramUniformMatrix3x4fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX3X4FVPROC>,
    pub glProgramUniformMatrix4dv:                     Option<PFNGLPROGRAMUNIFORMMATRIX4DVPROC>,
    pub glProgramUniformMatrix4fv:                     Option<PFNGLPROGRAMUNIFORMMATRIX4FVPROC>,
    pub glProgramUniformMatrix4x2dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX4X2DVPROC>,
    pub glProgramUniformMatrix4x2fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX4X2FVPROC>,
    pub glProgramUniformMatrix4x3dv:                   Option<PFNGLPROGRAMUNIFORMMATRIX4X3DVPROC>,
    pub glProgramUniformMatrix4x3fv:                   Option<PFNGLPROGRAMUNIFORMMATRIX4X3FVPROC>,
    pub glProvokingVertex:                             Option<PFNGLPROVOKINGVERTEXPROC>,
    pub glPushDebugGroup:                              Option<PFNGLPUSHDEBUGGROUPPROC>,
    pub glQueryCounter:                                Option<PFNGLQUERYCOUNTERPROC>,
    pub glRasterPos2xOES:                              Option<PFNGLRASTERPOS2XOESPROC>,
    pub glRasterPos2xvOES:                             Option<PFNGLRASTERPOS2XVOESPROC>,
    pub glRasterPos3xOES:                              Option<PFNGLRASTERPOS3XOESPROC>,
    pub glRasterPos3xvOES:                             Option<PFNGLRASTERPOS3XVOESPROC>,
    pub glRasterPos4xOES:                              Option<PFNGLRASTERPOS4XOESPROC>,
    pub glRasterPos4xvOES:                             Option<PFNGLRASTERPOS4XVOESPROC>,
    pub glReadBuffer:                                  Option<PFNGLREADBUFFERPROC>,
    pub glReadPixels:                                  Option<PFNGLREADPIXELSPROC>,
    pub glRectxOES:                                    Option<PFNGLRECTXOESPROC>,
    pub glRectxvOES:                                   Option<PFNGLRECTXVOESPROC>,
    pub glReleaseShaderCompiler:                       Option<PFNGLRELEASESHADERCOMPILERPROC>,
    pub glRenderbufferStorage:                         Option<PFNGLRENDERBUFFERSTORAGEPROC>,
    pub glRenderbufferStorageEXT:                      Option<PFNGLRENDERBUFFERSTORAGEEXTPROC>,
    pub glRenderbufferStorageMultisample:              Option<PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC>,
    pub glRenderbufferStorageMultisampleEXT:           Option<PFNGLRENDERBUFFERSTORAGEMULTISAMPLEEXTPROC>,
    pub glResumeTransformFeedback:                     Option<PFNGLRESUMETRANSFORMFEEDBACKPROC>,
    pub glRotatexOES:                                  Option<PFNGLROTATEXOESPROC>,
    pub glSampleCoverage:                              Option<PFNGLSAMPLECOVERAGEPROC>,
    pub glSampleCoverageARB:                           Option<PFNGLSAMPLECOVERAGEARBPROC>,
    pub glSampleMaski:                                 Option<PFNGLSAMPLEMASKIPROC>,
    pub glSamplerParameterIiv:                         Option<PFNGLSAMPLERPARAMETERIIVPROC>,
    pub glSamplerParameterIuiv:                        Option<PFNGLSAMPLERPARAMETERIUIVPROC>,
    pub glSamplerParameterf:                           Option<PFNGLSAMPLERPARAMETERFPROC>,
    pub glSamplerParameterfv:                          Option<PFNGLSAMPLERPARAMETERFVPROC>,
    pub glSamplerParameteri:                           Option<PFNGLSAMPLERPARAMETERIPROC>,
    pub glSamplerParameteriv:                          Option<PFNGLSAMPLERPARAMETERIVPROC>,
    pub glScalexOES:                                   Option<PFNGLSCALEXOESPROC>,
    pub glScissor:                                     Option<PFNGLSCISSORPROC>,
    pub glScissorArrayv:                               Option<PFNGLSCISSORARRAYVPROC>,
    pub glScissorIndexed:                              Option<PFNGLSCISSORINDEXEDPROC>,
    pub glScissorIndexedv:                             Option<PFNGLSCISSORINDEXEDVPROC>,
    pub glShaderBinary:                                Option<PFNGLSHADERBINARYPROC>,
    pub glShaderSource:                                Option<PFNGLSHADERSOURCEPROC>,
    pub glShaderSourceARB:                             Option<PFNGLSHADERSOURCEARBPROC>,
    pub glShaderStorageBlockBinding:                   Option<PFNGLSHADERSTORAGEBLOCKBINDINGPROC>,
    pub glSpecializeShaderARB:                         Option<PFNGLSPECIALIZESHADERARBPROC>,
    pub glStencilFunc:                                 Option<PFNGLSTENCILFUNCPROC>,
    pub glStencilFuncSeparate:                         Option<PFNGLSTENCILFUNCSEPARATEPROC>,
    pub glStencilMask:                                 Option<PFNGLSTENCILMASKPROC>,
    pub glStencilMaskSeparate:                         Option<PFNGLSTENCILMASKSEPARATEPROC>,
    pub glStencilOp:                                   Option<PFNGLSTENCILOPPROC>,
    pub glStencilOpSeparate:                           Option<PFNGLSTENCILOPSEPARATEPROC>,
    pub glTexBuffer:                                   Option<PFNGLTEXBUFFERPROC>,
    pub glTexBufferRange:                              Option<PFNGLTEXBUFFERRANGEPROC>,
    pub glTexCoord1xOES:                               Option<PFNGLTEXCOORD1XOESPROC>,
    pub glTexCoord1xvOES:                              Option<PFNGLTEXCOORD1XVOESPROC>,
    pub glTexCoord2xOES:                               Option<PFNGLTEXCOORD2XOESPROC>,
    pub glTexCoord2xvOES:                              Option<PFNGLTEXCOORD2XVOESPROC>,
    pub glTexCoord3xOES:                               Option<PFNGLTEXCOORD3XOESPROC>,
    pub glTexCoord3xvOES:                              Option<PFNGLTEXCOORD3XVOESPROC>,
    pub glTexCoord4xOES:                               Option<PFNGLTEXCOORD4XOESPROC>,
    pub glTexCoord4xvOES:                              Option<PFNGLTEXCOORD4XVOESPROC>,
    pub glTexEnvxOES:                                  Option<PFNGLTEXENVXOESPROC>,
    pub glTexEnvxvOES:                                 Option<PFNGLTEXENVXVOESPROC>,
    pub glTexGenxOES:                                  Option<PFNGLTEXGENXOESPROC>,
    pub glTexGenxvOES:                                 Option<PFNGLTEXGENXVOESPROC>,
    pub glTexImage1D:                                  Option<PFNGLTEXIMAGE1DPROC>,
    pub glTexImage2D:                                  Option<PFNGLTEXIMAGE2DPROC>,
    pub glTexImage2DMultisample:                       Option<PFNGLTEXIMAGE2DMULTISAMPLEPROC>,
    pub glTexImage3D:                                  Option<PFNGLTEXIMAGE3DPROC>,
    pub glTexImage3DMultisample:                       Option<PFNGLTEXIMAGE3DMULTISAMPLEPROC>,
    pub glTexParameterIiv:                             Option<PFNGLTEXPARAMETERIIVPROC>,
    pub glTexParameterIuiv:                            Option<PFNGLTEXPARAMETERIUIVPROC>,
    pub glTexParameterf:                               Option<PFNGLTEXPARAMETERFPROC>,
    pub glTexParameterfv:                              Option<PFNGLTEXPARAMETERFVPROC>,
    pub glTexParameteri:                               Option<PFNGLTEXPARAMETERIPROC>,
    pub glTexParameteriv:                              Option<PFNGLTEXPARAMETERIVPROC>,
    pub glTexParameterxOES:                            Option<PFNGLTEXPARAMETERXOESPROC>,
    pub glTexParameterxvOES:                           Option<PFNGLTEXPARAMETERXVOESPROC>,
    pub glTexStorage1D:                                Option<PFNGLTEXSTORAGE1DPROC>,
    pub glTexStorage2D:                                Option<PFNGLTEXSTORAGE2DPROC>,
    pub glTexStorage2DMultisample:                     Option<PFNGLTEXSTORAGE2DMULTISAMPLEPROC>,
    pub glTexStorage3D:                                Option<PFNGLTEXSTORAGE3DPROC>,
    pub glTexStorage3DMultisample:                     Option<PFNGLTEXSTORAGE3DMULTISAMPLEPROC>,
    pub glTexSubImage1D:                               Option<PFNGLTEXSUBIMAGE1DPROC>,
    pub glTexSubImage2D:                               Option<PFNGLTEXSUBIMAGE2DPROC>,
    pub glTexSubImage3D:                               Option<PFNGLTEXSUBIMAGE3DPROC>,
    pub glTextureBuffer:                               Option<PFNGLTEXTUREBUFFERPROC>,
    pub glTextureBufferRange:                          Option<PFNGLTEXTUREBUFFERRANGEPROC>,
    pub glTextureParameterIiv:                         Option<PFNGLTEXTUREPARAMETERIIVPROC>,
    pub glTextureParameterIuiv:                        Option<PFNGLTEXTUREPARAMETERIUIVPROC>,
    pub glTextureParameterf:                           Option<PFNGLTEXTUREPARAMETERFPROC>,
    pub glTextureParameterfv:                          Option<PFNGLTEXTUREPARAMETERFVPROC>,
    pub glTextureParameteri:                           Option<PFNGLTEXTUREPARAMETERIPROC>,
    pub glTextureParameteriv:                          Option<PFNGLTEXTUREPARAMETERIVPROC>,
    pub glTextureStorage1D:                            Option<PFNGLTEXTURESTORAGE1DPROC>,
    pub glTextureStorage2D:                            Option<PFNGLTEXTURESTORAGE2DPROC>,
    pub glTextureStorage2DMultisample:                 Option<PFNGLTEXTURESTORAGE2DMULTISAMPLEPROC>,
    pub glTextureStorage3D:                            Option<PFNGLTEXTURESTORAGE3DPROC>,
    pub glTextureStorage3DMultisample:                 Option<PFNGLTEXTURESTORAGE3DMULTISAMPLEPROC>,
    pub glTextureSubImage1D:                           Option<PFNGLTEXTURESUBIMAGE1DPROC>,
    pub glTextureSubImage2D:                           Option<PFNGLTEXTURESUBIMAGE2DPROC>,
    pub glTextureSubImage3D:                           Option<PFNGLTEXTURESUBIMAGE3DPROC>,
    pub glTextureView:                                 Option<PFNGLTEXTUREVIEWPROC>,
    pub glTransformFeedbackBufferBase:                 Option<PFNGLTRANSFORMFEEDBACKBUFFERBASEPROC>,
    pub glTransformFeedbackBufferRange:                Option<PFNGLTRANSFORMFEEDBACKBUFFERRANGEPROC>,
    pub glTransformFeedbackVaryings:                   Option<PFNGLTRANSFORMFEEDBACKVARYINGSPROC>,
    pub glTranslatexOES:                               Option<PFNGLTRANSLATEXOESPROC>,
    pub glUniform1d:                                   Option<PFNGLUNIFORM1DPROC>,
    pub glUniform1dv:                                  Option<PFNGLUNIFORM1DVPROC>,
    pub glUniform1f:                                   Option<PFNGLUNIFORM1FPROC>,
    pub glUniform1fARB:                                Option<PFNGLUNIFORM1FARBPROC>,
    pub glUniform1fv:                                  Option<PFNGLUNIFORM1FVPROC>,
    pub glUniform1fvARB:                               Option<PFNGLUNIFORM1FVARBPROC>,
    pub glUniform1i:                                   Option<PFNGLUNIFORM1IPROC>,
    pub glUniform1i64ARB:                              Option<PFNGLUNIFORM1I64ARBPROC>,
    pub glUniform1i64vARB:                             Option<PFNGLUNIFORM1I64VARBPROC>,
    pub glUniform1iARB:                                Option<PFNGLUNIFORM1IARBPROC>,
    pub glUniform1iv:                                  Option<PFNGLUNIFORM1IVPROC>,
    pub glUniform1ivARB:                               Option<PFNGLUNIFORM1IVARBPROC>,
    pub glUniform1ui:                                  Option<PFNGLUNIFORM1UIPROC>,
    pub glUniform1ui64ARB:                             Option<PFNGLUNIFORM1UI64ARBPROC>,
    pub glUniform1ui64vARB:                            Option<PFNGLUNIFORM1UI64VARBPROC>,
    pub glUniform1uiv:                                 Option<PFNGLUNIFORM1UIVPROC>,
    pub glUniform2d:                                   Option<PFNGLUNIFORM2DPROC>,
    pub glUniform2dv:                                  Option<PFNGLUNIFORM2DVPROC>,
    pub glUniform2f:                                   Option<PFNGLUNIFORM2FPROC>,
    pub glUniform2fARB:                                Option<PFNGLUNIFORM2FARBPROC>,
    pub glUniform2fv:                                  Option<PFNGLUNIFORM2FVPROC>,
    pub glUniform2fvARB:                               Option<PFNGLUNIFORM2FVARBPROC>,
    pub glUniform2i:                                   Option<PFNGLUNIFORM2IPROC>,
    pub glUniform2i64ARB:                              Option<PFNGLUNIFORM2I64ARBPROC>,
    pub glUniform2i64vARB:                             Option<PFNGLUNIFORM2I64VARBPROC>,
    pub glUniform2iARB:                                Option<PFNGLUNIFORM2IARBPROC>,
    pub glUniform2iv:                                  Option<PFNGLUNIFORM2IVPROC>,
    pub glUniform2ivARB:                               Option<PFNGLUNIFORM2IVARBPROC>,
    pub glUniform2ui:                                  Option<PFNGLUNIFORM2UIPROC>,
    pub glUniform2ui64ARB:                             Option<PFNGLUNIFORM2UI64ARBPROC>,
    pub glUniform2ui64vARB:                            Option<PFNGLUNIFORM2UI64VARBPROC>,
    pub glUniform2uiv:                                 Option<PFNGLUNIFORM2UIVPROC>,
    pub glUniform3d:                                   Option<PFNGLUNIFORM3DPROC>,
    pub glUniform3dv:                                  Option<PFNGLUNIFORM3DVPROC>,
    pub glUniform3f:                                   Option<PFNGLUNIFORM3FPROC>,
    pub glUniform3fARB:                                Option<PFNGLUNIFORM3FARBPROC>,
    pub glUniform3fv:                                  Option<PFNGLUNIFORM3FVPROC>,
    pub glUniform3fvARB:                               Option<PFNGLUNIFORM3FVARBPROC>,
    pub glUniform3i:                                   Option<PFNGLUNIFORM3IPROC>,
    pub glUniform3i64ARB:                              Option<PFNGLUNIFORM3I64ARBPROC>,
    pub glUniform3i64vARB:                             Option<PFNGLUNIFORM3I64VARBPROC>,
    pub glUniform3iARB:                                Option<PFNGLUNIFORM3IARBPROC>,
    pub glUniform3iv:                                  Option<PFNGLUNIFORM3IVPROC>,
    pub glUniform3ivARB:                               Option<PFNGLUNIFORM3IVARBPROC>,
    pub glUniform3ui:                                  Option<PFNGLUNIFORM3UIPROC>,
    pub glUniform3ui64ARB:                             Option<PFNGLUNIFORM3UI64ARBPROC>,
    pub glUniform3ui64vARB:                            Option<PFNGLUNIFORM3UI64VARBPROC>,
    pub glUniform3uiv:                                 Option<PFNGLUNIFORM3UIVPROC>,
    pub glUniform4d:                                   Option<PFNGLUNIFORM4DPROC>,
    pub glUniform4dv:                                  Option<PFNGLUNIFORM4DVPROC>,
    pub glUniform4f:                                   Option<PFNGLUNIFORM4FPROC>,
    pub glUniform4fARB:                                Option<PFNGLUNIFORM4FARBPROC>,
    pub glUniform4fv:                                  Option<PFNGLUNIFORM4FVPROC>,
    pub glUniform4fvARB:                               Option<PFNGLUNIFORM4FVARBPROC>,
    pub glUniform4i:                                   Option<PFNGLUNIFORM4IPROC>,
    pub glUniform4i64ARB:                              Option<PFNGLUNIFORM4I64ARBPROC>,
    pub glUniform4i64vARB:                             Option<PFNGLUNIFORM4I64VARBPROC>,
    pub glUniform4iARB:                                Option<PFNGLUNIFORM4IARBPROC>,
    pub glUniform4iv:                                  Option<PFNGLUNIFORM4IVPROC>,
    pub glUniform4ivARB:                               Option<PFNGLUNIFORM4IVARBPROC>,
    pub glUniform4ui:                                  Option<PFNGLUNIFORM4UIPROC>,
    pub glUniform4ui64ARB:                             Option<PFNGLUNIFORM4UI64ARBPROC>,
    pub glUniform4ui64vARB:                            Option<PFNGLUNIFORM4UI64VARBPROC>,
    pub glUniform4uiv:                                 Option<PFNGLUNIFORM4UIVPROC>,
    pub glUniformBlockBinding:                         Option<PFNGLUNIFORMBLOCKBINDINGPROC>,
    pub glUniformMatrix2dv:                            Option<PFNGLUNIFORMMATRIX2DVPROC>,
    pub glUniformMatrix2fv:                            Option<PFNGLUNIFORMMATRIX2FVPROC>,
    pub glUniformMatrix2fvARB:                         Option<PFNGLUNIFORMMATRIX2FVARBPROC>,
    pub glUniformMatrix2x3dv:                          Option<PFNGLUNIFORMMATRIX2X3DVPROC>,
    pub glUniformMatrix2x3fv:                          Option<PFNGLUNIFORMMATRIX2X3FVPROC>,
    pub glUniformMatrix2x4dv:                          Option<PFNGLUNIFORMMATRIX2X4DVPROC>,
    pub glUniformMatrix2x4fv:                          Option<PFNGLUNIFORMMATRIX2X4FVPROC>,
    pub glUniformMatrix3dv:                            Option<PFNGLUNIFORMMATRIX3DVPROC>,
    pub glUniformMatrix3fv:                            Option<PFNGLUNIFORMMATRIX3FVPROC>,
    pub glUniformMatrix3fvARB:                         Option<PFNGLUNIFORMMATRIX3FVARBPROC>,
    pub glUniformMatrix3x2dv:                          Option<PFNGLUNIFORMMATRIX3X2DVPROC>,
    pub glUniformMatrix3x2fv:                          Option<PFNGLUNIFORMMATRIX3X2FVPROC>,
    pub glUniformMatrix3x4dv:                          Option<PFNGLUNIFORMMATRIX3X4DVPROC>,
    pub glUniformMatrix3x4fv:                          Option<PFNGLUNIFORMMATRIX3X4FVPROC>,
    pub glUniformMatrix4dv:                            Option<PFNGLUNIFORMMATRIX4DVPROC>,
    pub glUniformMatrix4fv:                            Option<PFNGLUNIFORMMATRIX4FVPROC>,
    pub glUniformMatrix4fvARB:                         Option<PFNGLUNIFORMMATRIX4FVARBPROC>,
    pub glUniformMatrix4x2dv:                          Option<PFNGLUNIFORMMATRIX4X2DVPROC>,
    pub glUniformMatrix4x2fv:                          Option<PFNGLUNIFORMMATRIX4X2FVPROC>,
    pub glUniformMatrix4x3dv:                          Option<PFNGLUNIFORMMATRIX4X3DVPROC>,
    pub glUniformMatrix4x3fv:                          Option<PFNGLUNIFORMMATRIX4X3FVPROC>,
    pub glUniformSubroutinesuiv:                       Option<PFNGLUNIFORMSUBROUTINESUIVPROC>,
    pub glUnmapBuffer:                                 Option<PFNGLUNMAPBUFFERPROC>,
    pub glUnmapBufferARB:                              Option<PFNGLUNMAPBUFFERARBPROC>,
    pub glUnmapNamedBuffer:                            Option<PFNGLUNMAPNAMEDBUFFERPROC>,
    pub glUseProgram:                                  Option<PFNGLUSEPROGRAMPROC>,
    pub glUseProgramObjectARB:                         Option<PFNGLUSEPROGRAMOBJECTARBPROC>,
    pub glUseProgramStages:                            Option<PFNGLUSEPROGRAMSTAGESPROC>,
    pub glValidateProgram:                             Option<PFNGLVALIDATEPROGRAMPROC>,
    pub glValidateProgramARB:                          Option<PFNGLVALIDATEPROGRAMARBPROC>,
    pub glValidateProgramPipeline:                     Option<PFNGLVALIDATEPROGRAMPIPELINEPROC>,
    pub glVertex2xOES:                                 Option<PFNGLVERTEX2XOESPROC>,
    pub glVertex2xvOES:                                Option<PFNGLVERTEX2XVOESPROC>,
    pub glVertex3xOES:                                 Option<PFNGLVERTEX3XOESPROC>,
    pub glVertex3xvOES:                                Option<PFNGLVERTEX3XVOESPROC>,
    pub glVertex4xOES:                                 Option<PFNGLVERTEX4XOESPROC>,
    pub glVertex4xvOES:                                Option<PFNGLVERTEX4XVOESPROC>,
    pub glVertexArrayAttribBinding:                    Option<PFNGLVERTEXARRAYATTRIBBINDINGPROC>,
    pub glVertexArrayAttribFormat:                     Option<PFNGLVERTEXARRAYATTRIBFORMATPROC>,
    pub glVertexArrayAttribIFormat:                    Option<PFNGLVERTEXARRAYATTRIBIFORMATPROC>,
    pub glVertexArrayAttribLFormat:                    Option<PFNGLVERTEXARRAYATTRIBLFORMATPROC>,
    pub glVertexArrayBindingDivisor:                   Option<PFNGLVERTEXARRAYBINDINGDIVISORPROC>,
    pub glVertexArrayElementBuffer:                    Option<PFNGLVERTEXARRAYELEMENTBUFFERPROC>,
    pub glVertexArrayVertexBuffer:                     Option<PFNGLVERTEXARRAYVERTEXBUFFERPROC>,
    pub glVertexArrayVertexBuffers:                    Option<PFNGLVERTEXARRAYVERTEXBUFFERSPROC>,
    pub glVertexAttrib1d:                              Option<PFNGLVERTEXATTRIB1DPROC>,
    pub glVertexAttrib1dARB:                           Option<PFNGLVERTEXATTRIB1DARBPROC>,
    pub glVertexAttrib1dv:                             Option<PFNGLVERTEXATTRIB1DVPROC>,
    pub glVertexAttrib1dvARB:                          Option<PFNGLVERTEXATTRIB1DVARBPROC>,
    pub glVertexAttrib1f:                              Option<PFNGLVERTEXATTRIB1FPROC>,
    pub glVertexAttrib1fARB:                           Option<PFNGLVERTEXATTRIB1FARBPROC>,
    pub glVertexAttrib1fv:                             Option<PFNGLVERTEXATTRIB1FVPROC>,
    pub glVertexAttrib1fvARB:                          Option<PFNGLVERTEXATTRIB1FVARBPROC>,
    pub glVertexAttrib1s:                              Option<PFNGLVERTEXATTRIB1SPROC>,
    pub glVertexAttrib1sARB:                           Option<PFNGLVERTEXATTRIB1SARBPROC>,
    pub glVertexAttrib1sv:                             Option<PFNGLVERTEXATTRIB1SVPROC>,
    pub glVertexAttrib1svARB:                          Option<PFNGLVERTEXATTRIB1SVARBPROC>,
    pub glVertexAttrib2d:                              Option<PFNGLVERTEXATTRIB2DPROC>,
    pub glVertexAttrib2dARB:                           Option<PFNGLVERTEXATTRIB2DARBPROC>,
    pub glVertexAttrib2dv:                             Option<PFNGLVERTEXATTRIB2DVPROC>,
    pub glVertexAttrib2dvARB:                          Option<PFNGLVERTEXATTRIB2DVARBPROC>,
    pub glVertexAttrib2f:                              Option<PFNGLVERTEXATTRIB2FPROC>,
    pub glVertexAttrib2fARB:                           Option<PFNGLVERTEXATTRIB2FARBPROC>,
    pub glVertexAttrib2fv:                             Option<PFNGLVERTEXATTRIB2FVPROC>,
    pub glVertexAttrib2fvARB:                          Option<PFNGLVERTEXATTRIB2FVARBPROC>,
    pub glVertexAttrib2s:                              Option<PFNGLVERTEXATTRIB2SPROC>,
    pub glVertexAttrib2sARB:                           Option<PFNGLVERTEXATTRIB2SARBPROC>,
    pub glVertexAttrib2sv:                             Option<PFNGLVERTEXATTRIB2SVPROC>,
    pub glVertexAttrib2svARB:                          Option<PFNGLVERTEXATTRIB2SVARBPROC>,
    pub glVertexAttrib3d:                              Option<PFNGLVERTEXATTRIB3DPROC>,
    pub glVertexAttrib3dARB:                           Option<PFNGLVERTEXATTRIB3DARBPROC>,
    pub glVertexAttrib3dv:                             Option<PFNGLVERTEXATTRIB3DVPROC>,
    pub glVertexAttrib3dvARB:                          Option<PFNGLVERTEXATTRIB3DVARBPROC>,
    pub glVertexAttrib3f:                              Option<PFNGLVERTEXATTRIB3FPROC>,
    pub glVertexAttrib3fARB:                           Option<PFNGLVERTEXATTRIB3FARBPROC>,
    pub glVertexAttrib3fv:                             Option<PFNGLVERTEXATTRIB3FVPROC>,
    pub glVertexAttrib3fvARB:                          Option<PFNGLVERTEXATTRIB3FVARBPROC>,
    pub glVertexAttrib3s:                              Option<PFNGLVERTEXATTRIB3SPROC>,
    pub glVertexAttrib3sARB:                           Option<PFNGLVERTEXATTRIB3SARBPROC>,
    pub glVertexAttrib3sv:                             Option<PFNGLVERTEXATTRIB3SVPROC>,
    pub glVertexAttrib3svARB:                          Option<PFNGLVERTEXATTRIB3SVARBPROC>,
    pub glVertexAttrib4Nbv:                            Option<PFNGLVERTEXATTRIB4NBVPROC>,
    pub glVertexAttrib4NbvARB:                         Option<PFNGLVERTEXATTRIB4NBVARBPROC>,
    pub glVertexAttrib4Niv:                            Option<PFNGLVERTEXATTRIB4NIVPROC>,
    pub glVertexAttrib4NivARB:                         Option<PFNGLVERTEXATTRIB4NIVARBPROC>,
    pub glVertexAttrib4Nsv:                            Option<PFNGLVERTEXATTRIB4NSVPROC>,
    pub glVertexAttrib4NsvARB:                         Option<PFNGLVERTEXATTRIB4NSVARBPROC>,
    pub glVertexAttrib4Nub:                            Option<PFNGLVERTEXATTRIB4NUBPROC>,
    pub glVertexAttrib4NubARB:                         Option<PFNGLVERTEXATTRIB4NUBARBPROC>,
    pub glVertexAttrib4Nubv:                           Option<PFNGLVERTEXATTRIB4NUBVPROC>,
    pub glVertexAttrib4NubvARB:                        Option<PFNGLVERTEXATTRIB4NUBVARBPROC>,
    pub glVertexAttrib4Nuiv:                           Option<PFNGLVERTEXATTRIB4NUIVPROC>,
    pub glVertexAttrib4NuivARB:                        Option<PFNGLVERTEXATTRIB4NUIVARBPROC>,
    pub glVertexAttrib4Nusv:                           Option<PFNGLVERTEXATTRIB4NUSVPROC>,
    pub glVertexAttrib4NusvARB:                        Option<PFNGLVERTEXATTRIB4NUSVARBPROC>,
    pub glVertexAttrib4bv:                             Option<PFNGLVERTEXATTRIB4BVPROC>,
    pub glVertexAttrib4bvARB:                          Option<PFNGLVERTEXATTRIB4BVARBPROC>,
    pub glVertexAttrib4d:                              Option<PFNGLVERTEXATTRIB4DPROC>,
    pub glVertexAttrib4dARB:                           Option<PFNGLVERTEXATTRIB4DARBPROC>,
    pub glVertexAttrib4dv:                             Option<PFNGLVERTEXATTRIB4DVPROC>,
    pub glVertexAttrib4dvARB:                          Option<PFNGLVERTEXATTRIB4DVARBPROC>,
    pub glVertexAttrib4f:                              Option<PFNGLVERTEXATTRIB4FPROC>,
    pub glVertexAttrib4fARB:                           Option<PFNGLVERTEXATTRIB4FARBPROC>,
    pub glVertexAttrib4fv:                             Option<PFNGLVERTEXATTRIB4FVPROC>,
    pub glVertexAttrib4fvARB:                          Option<PFNGLVERTEXATTRIB4FVARBPROC>,
    pub glVertexAttrib4iv:                             Option<PFNGLVERTEXATTRIB4IVPROC>,
    pub glVertexAttrib4ivARB:                          Option<PFNGLVERTEXATTRIB4IVARBPROC>,
    pub glVertexAttrib4s:                              Option<PFNGLVERTEXATTRIB4SPROC>,
    pub glVertexAttrib4sARB:                           Option<PFNGLVERTEXATTRIB4SARBPROC>,
    pub glVertexAttrib4sv:                             Option<PFNGLVERTEXATTRIB4SVPROC>,
    pub glVertexAttrib4svARB:                          Option<PFNGLVERTEXATTRIB4SVARBPROC>,
    pub glVertexAttrib4ubv:                            Option<PFNGLVERTEXATTRIB4UBVPROC>,
    pub glVertexAttrib4ubvARB:                         Option<PFNGLVERTEXATTRIB4UBVARBPROC>,
    pub glVertexAttrib4uiv:                            Option<PFNGLVERTEXATTRIB4UIVPROC>,
    pub glVertexAttrib4uivARB:                         Option<PFNGLVERTEXATTRIB4UIVARBPROC>,
    pub glVertexAttrib4usv:                            Option<PFNGLVERTEXATTRIB4USVPROC>,
    pub glVertexAttrib4usvARB:                         Option<PFNGLVERTEXATTRIB4USVARBPROC>,
    pub glVertexAttribBinding:                         Option<PFNGLVERTEXATTRIBBINDINGPROC>,
    pub glVertexAttribDivisor:                         Option<PFNGLVERTEXATTRIBDIVISORPROC>,
    pub glVertexAttribDivisorARB:                      Option<PFNGLVERTEXATTRIBDIVISORARBPROC>,
    pub glVertexAttribFormat:                          Option<PFNGLVERTEXATTRIBFORMATPROC>,
    pub glVertexAttribI1i:                             Option<PFNGLVERTEXATTRIBI1IPROC>,
    pub glVertexAttribI1iv:                            Option<PFNGLVERTEXATTRIBI1IVPROC>,
    pub glVertexAttribI1ui:                            Option<PFNGLVERTEXATTRIBI1UIPROC>,
    pub glVertexAttribI1uiv:                           Option<PFNGLVERTEXATTRIBI1UIVPROC>,
    pub glVertexAttribI2i:                             Option<PFNGLVERTEXATTRIBI2IPROC>,
    pub glVertexAttribI2iv:                            Option<PFNGLVERTEXATTRIBI2IVPROC>,
    pub glVertexAttribI2ui:                            Option<PFNGLVERTEXATTRIBI2UIPROC>,
    pub glVertexAttribI2uiv:                           Option<PFNGLVERTEXATTRIBI2UIVPROC>,
    pub glVertexAttribI3i:                             Option<PFNGLVERTEXATTRIBI3IPROC>,
    pub glVertexAttribI3iv:                            Option<PFNGLVERTEXATTRIBI3IVPROC>,
    pub glVertexAttribI3ui:                            Option<PFNGLVERTEXATTRIBI3UIPROC>,
    pub glVertexAttribI3uiv:                           Option<PFNGLVERTEXATTRIBI3UIVPROC>,
    pub glVertexAttribI4bv:                            Option<PFNGLVERTEXATTRIBI4BVPROC>,
    pub glVertexAttribI4i:                             Option<PFNGLVERTEXATTRIBI4IPROC>,
    pub glVertexAttribI4iv:                            Option<PFNGLVERTEXATTRIBI4IVPROC>,
    pub glVertexAttribI4sv:                            Option<PFNGLVERTEXATTRIBI4SVPROC>,
    pub glVertexAttribI4ubv:                           Option<PFNGLVERTEXATTRIBI4UBVPROC>,
    pub glVertexAttribI4ui:                            Option<PFNGLVERTEXATTRIBI4UIPROC>,
    pub glVertexAttribI4uiv:                           Option<PFNGLVERTEXATTRIBI4UIVPROC>,
    pub glVertexAttribI4usv:                           Option<PFNGLVERTEXATTRIBI4USVPROC>,
    pub glVertexAttribIFormat:                         Option<PFNGLVERTEXATTRIBIFORMATPROC>,
    pub glVertexAttribIPointer:                        Option<PFNGLVERTEXATTRIBIPOINTERPROC>,
    pub glVertexAttribL1d:                             Option<PFNGLVERTEXATTRIBL1DPROC>,
    pub glVertexAttribL1dv:                            Option<PFNGLVERTEXATTRIBL1DVPROC>,
    pub glVertexAttribL2d:                             Option<PFNGLVERTEXATTRIBL2DPROC>,
    pub glVertexAttribL2dv:                            Option<PFNGLVERTEXATTRIBL2DVPROC>,
    pub glVertexAttribL3d:                             Option<PFNGLVERTEXATTRIBL3DPROC>,
    pub glVertexAttribL3dv:                            Option<PFNGLVERTEXATTRIBL3DVPROC>,
    pub glVertexAttribL4d:                             Option<PFNGLVERTEXATTRIBL4DPROC>,
    pub glVertexAttribL4dv:                            Option<PFNGLVERTEXATTRIBL4DVPROC>,
    pub glVertexAttribLFormat:                         Option<PFNGLVERTEXATTRIBLFORMATPROC>,
    pub glVertexAttribLPointer:                        Option<PFNGLVERTEXATTRIBLPOINTERPROC>,
    pub glVertexAttribP1ui:                            Option<PFNGLVERTEXATTRIBP1UIPROC>,
    pub glVertexAttribP1uiv:                           Option<PFNGLVERTEXATTRIBP1UIVPROC>,
    pub glVertexAttribP2ui:                            Option<PFNGLVERTEXATTRIBP2UIPROC>,
    pub glVertexAttribP2uiv:                           Option<PFNGLVERTEXATTRIBP2UIVPROC>,
    pub glVertexAttribP3ui:                            Option<PFNGLVERTEXATTRIBP3UIPROC>,
    pub glVertexAttribP3uiv:                           Option<PFNGLVERTEXATTRIBP3UIVPROC>,
    pub glVertexAttribP4ui:                            Option<PFNGLVERTEXATTRIBP4UIPROC>,
    pub glVertexAttribP4uiv:                           Option<PFNGLVERTEXATTRIBP4UIVPROC>,
    pub glVertexAttribPointer:                         Option<PFNGLVERTEXATTRIBPOINTERPROC>,
    pub glVertexAttribPointerARB:                      Option<PFNGLVERTEXATTRIBPOINTERARBPROC>,
    pub glVertexBindingDivisor:                        Option<PFNGLVERTEXBINDINGDIVISORPROC>,
    pub glViewport:                                    Option<PFNGLVIEWPORTPROC>,
    pub glViewportArrayv:                              Option<PFNGLVIEWPORTARRAYVPROC>,
    pub glViewportIndexedf:                            Option<PFNGLVIEWPORTINDEXEDFPROC>,
    pub glViewportIndexedfv:                           Option<PFNGLVIEWPORTINDEXEDFVPROC>,
    pub glWaitSync:                                    Option<PFNGLWAITSYNCPROC>,
}

macro_rules! load {
    (($loader:expr)($userptr:expr, $name:expr) as $FNTYPE:ty) => {
        $loader($userptr, $name).map(|f| std::mem::transmute::<unsafe extern "C" fn(), $FNTYPE>(f))
    };
}

impl Glad {
    pub(super) unsafe fn gl_load_GL_VERSION_1_0(&mut self, compat: &GladVersionCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_1_0 { return; }
            self.glBlendFunc                                   = load!((load)(userptr, c"glBlendFunc"                                    .as_ptr()) as PFNGLBLENDFUNCPROC                                  );
            self.glClear                                       = load!((load)(userptr, c"glClear"                                        .as_ptr()) as PFNGLCLEARPROC                                      );
            self.glClearColor                                  = load!((load)(userptr, c"glClearColor"                                   .as_ptr()) as PFNGLCLEARCOLORPROC                                 );
            self.glClearDepth                                  = load!((load)(userptr, c"glClearDepth"                                   .as_ptr()) as PFNGLCLEARDEPTHPROC                                 );
            self.glClearStencil                                = load!((load)(userptr, c"glClearStencil"                                 .as_ptr()) as PFNGLCLEARSTENCILPROC                               );
            self.glColorMask                                   = load!((load)(userptr, c"glColorMask"                                    .as_ptr()) as PFNGLCOLORMASKPROC                                  );
            self.glCullFace                                    = load!((load)(userptr, c"glCullFace"                                     .as_ptr()) as PFNGLCULLFACEPROC                                   );
            self.glDepthFunc                                   = load!((load)(userptr, c"glDepthFunc"                                    .as_ptr()) as PFNGLDEPTHFUNCPROC                                  );
            self.glDepthMask                                   = load!((load)(userptr, c"glDepthMask"                                    .as_ptr()) as PFNGLDEPTHMASKPROC                                  );
            self.glDepthRange                                  = load!((load)(userptr, c"glDepthRange"                                   .as_ptr()) as PFNGLDEPTHRANGEPROC                                 );
            self.glDisable                                     = load!((load)(userptr, c"glDisable"                                      .as_ptr()) as PFNGLDISABLEPROC                                    );
            self.glDrawBuffer                                  = load!((load)(userptr, c"glDrawBuffer"                                   .as_ptr()) as PFNGLDRAWBUFFERPROC                                 );
            self.glEnable                                      = load!((load)(userptr, c"glEnable"                                       .as_ptr()) as PFNGLENABLEPROC                                     );
            self.glFinish                                      = load!((load)(userptr, c"glFinish"                                       .as_ptr()) as PFNGLFINISHPROC                                     );
            self.glFlush                                       = load!((load)(userptr, c"glFlush"                                        .as_ptr()) as PFNGLFLUSHPROC                                      );
            self.glFrontFace                                   = load!((load)(userptr, c"glFrontFace"                                    .as_ptr()) as PFNGLFRONTFACEPROC                                  );
            self.glGetBooleanv                                 = load!((load)(userptr, c"glGetBooleanv"                                  .as_ptr()) as PFNGLGETBOOLEANVPROC                                );
            self.glGetDoublev                                  = load!((load)(userptr, c"glGetDoublev"                                   .as_ptr()) as PFNGLGETDOUBLEVPROC                                 );
            self.glGetError                                    = load!((load)(userptr, c"glGetError"                                     .as_ptr()) as PFNGLGETERRORPROC                                   );
            self.glGetFloatv                                   = load!((load)(userptr, c"glGetFloatv"                                    .as_ptr()) as PFNGLGETFLOATVPROC                                  );
            self.glGetIntegerv                                 = load!((load)(userptr, c"glGetIntegerv"                                  .as_ptr()) as PFNGLGETINTEGERVPROC                                );
            self.glGetString                                   = load!((load)(userptr, c"glGetString"                                    .as_ptr()) as PFNGLGETSTRINGPROC                                  );
            self.glGetTexImage                                 = load!((load)(userptr, c"glGetTexImage"                                  .as_ptr()) as PFNGLGETTEXIMAGEPROC                                );
            self.glGetTexLevelParameterfv                      = load!((load)(userptr, c"glGetTexLevelParameterfv"                       .as_ptr()) as PFNGLGETTEXLEVELPARAMETERFVPROC                     );
            self.glGetTexLevelParameteriv                      = load!((load)(userptr, c"glGetTexLevelParameteriv"                       .as_ptr()) as PFNGLGETTEXLEVELPARAMETERIVPROC                     );
            self.glGetTexParameterfv                           = load!((load)(userptr, c"glGetTexParameterfv"                            .as_ptr()) as PFNGLGETTEXPARAMETERFVPROC                          );
            self.glGetTexParameteriv                           = load!((load)(userptr, c"glGetTexParameteriv"                            .as_ptr()) as PFNGLGETTEXPARAMETERIVPROC                          );
            self.glHint                                        = load!((load)(userptr, c"glHint"                                         .as_ptr()) as PFNGLHINTPROC                                       );
            self.glIsEnabled                                   = load!((load)(userptr, c"glIsEnabled"                                    .as_ptr()) as PFNGLISENABLEDPROC                                  );
            self.glLineWidth                                   = load!((load)(userptr, c"glLineWidth"                                    .as_ptr()) as PFNGLLINEWIDTHPROC                                  );
            self.glLogicOp                                     = load!((load)(userptr, c"glLogicOp"                                      .as_ptr()) as PFNGLLOGICOPPROC                                    );
            self.glPixelStoref                                 = load!((load)(userptr, c"glPixelStoref"                                  .as_ptr()) as PFNGLPIXELSTOREFPROC                                );
            self.glPixelStorei                                 = load!((load)(userptr, c"glPixelStorei"                                  .as_ptr()) as PFNGLPIXELSTOREIPROC                                );
            self.glPointSize                                   = load!((load)(userptr, c"glPointSize"                                    .as_ptr()) as PFNGLPOINTSIZEPROC                                  );
            self.glPolygonMode                                 = load!((load)(userptr, c"glPolygonMode"                                  .as_ptr()) as PFNGLPOLYGONMODEPROC                                );
            self.glReadBuffer                                  = load!((load)(userptr, c"glReadBuffer"                                   .as_ptr()) as PFNGLREADBUFFERPROC                                 );
            self.glReadPixels                                  = load!((load)(userptr, c"glReadPixels"                                   .as_ptr()) as PFNGLREADPIXELSPROC                                 );
            self.glScissor                                     = load!((load)(userptr, c"glScissor"                                      .as_ptr()) as PFNGLSCISSORPROC                                    );
            self.glStencilFunc                                 = load!((load)(userptr, c"glStencilFunc"                                  .as_ptr()) as PFNGLSTENCILFUNCPROC                                );
            self.glStencilMask                                 = load!((load)(userptr, c"glStencilMask"                                  .as_ptr()) as PFNGLSTENCILMASKPROC                                );
            self.glStencilOp                                   = load!((load)(userptr, c"glStencilOp"                                    .as_ptr()) as PFNGLSTENCILOPPROC                                  );
            self.glTexImage1D                                  = load!((load)(userptr, c"glTexImage1D"                                   .as_ptr()) as PFNGLTEXIMAGE1DPROC                                 );
            self.glTexImage2D                                  = load!((load)(userptr, c"glTexImage2D"                                   .as_ptr()) as PFNGLTEXIMAGE2DPROC                                 );
            self.glTexParameterf                               = load!((load)(userptr, c"glTexParameterf"                                .as_ptr()) as PFNGLTEXPARAMETERFPROC                              );
            self.glTexParameterfv                              = load!((load)(userptr, c"glTexParameterfv"                               .as_ptr()) as PFNGLTEXPARAMETERFVPROC                             );
            self.glTexParameteri                               = load!((load)(userptr, c"glTexParameteri"                                .as_ptr()) as PFNGLTEXPARAMETERIPROC                              );
            self.glTexParameteriv                              = load!((load)(userptr, c"glTexParameteriv"                               .as_ptr()) as PFNGLTEXPARAMETERIVPROC                             );
            self.glViewport                                    = load!((load)(userptr, c"glViewport"                                     .as_ptr()) as PFNGLVIEWPORTPROC                                   );
        }
    }
    pub(super) unsafe fn gl_load_GL_VERSION_1_1(&mut self, compat: &GladVersionCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_1_1 { return; }
            self.glBindTexture                                 = load!((load)(userptr, c"glBindTexture"                                  .as_ptr()) as PFNGLBINDTEXTUREPROC                                );
            self.glCopyTexImage1D                              = load!((load)(userptr, c"glCopyTexImage1D"                               .as_ptr()) as PFNGLCOPYTEXIMAGE1DPROC                             );
            self.glCopyTexImage2D                              = load!((load)(userptr, c"glCopyTexImage2D"                               .as_ptr()) as PFNGLCOPYTEXIMAGE2DPROC                             );
            self.glCopyTexSubImage1D                           = load!((load)(userptr, c"glCopyTexSubImage1D"                            .as_ptr()) as PFNGLCOPYTEXSUBIMAGE1DPROC                          );
            self.glCopyTexSubImage2D                           = load!((load)(userptr, c"glCopyTexSubImage2D"                            .as_ptr()) as PFNGLCOPYTEXSUBIMAGE2DPROC                          );
            self.glDeleteTextures                              = load!((load)(userptr, c"glDeleteTextures"                               .as_ptr()) as PFNGLDELETETEXTURESPROC                             );
            self.glDrawArrays                                  = load!((load)(userptr, c"glDrawArrays"                                   .as_ptr()) as PFNGLDRAWARRAYSPROC                                 );
            self.glDrawElements                                = load!((load)(userptr, c"glDrawElements"                                 .as_ptr()) as PFNGLDRAWELEMENTSPROC                               );
            self.glGenTextures                                 = load!((load)(userptr, c"glGenTextures"                                  .as_ptr()) as PFNGLGENTEXTURESPROC                                );
            self.glGetPointerv                                 = load!((load)(userptr, c"glGetPointerv"                                  .as_ptr()) as PFNGLGETPOINTERVPROC                                );
            self.glIsTexture                                   = load!((load)(userptr, c"glIsTexture"                                    .as_ptr()) as PFNGLISTEXTUREPROC                                  );
            self.glPolygonOffset                               = load!((load)(userptr, c"glPolygonOffset"                                .as_ptr()) as PFNGLPOLYGONOFFSETPROC                              );
            self.glTexSubImage1D                               = load!((load)(userptr, c"glTexSubImage1D"                                .as_ptr()) as PFNGLTEXSUBIMAGE1DPROC                              );
            self.glTexSubImage2D                               = load!((load)(userptr, c"glTexSubImage2D"                                .as_ptr()) as PFNGLTEXSUBIMAGE2DPROC                              );
        }
    }
    pub(super) unsafe fn gl_load_GL_VERSION_1_2(&mut self, compat: &GladVersionCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_1_2 { return; }
            self.glCopyTexSubImage3D                           = load!((load)(userptr, c"glCopyTexSubImage3D"                            .as_ptr()) as PFNGLCOPYTEXSUBIMAGE3DPROC                          );
            self.glDrawRangeElements                           = load!((load)(userptr, c"glDrawRangeElements"                            .as_ptr()) as PFNGLDRAWRANGEELEMENTSPROC                          );
            self.glTexImage3D                                  = load!((load)(userptr, c"glTexImage3D"                                   .as_ptr()) as PFNGLTEXIMAGE3DPROC                                 );
            self.glTexSubImage3D                               = load!((load)(userptr, c"glTexSubImage3D"                                .as_ptr()) as PFNGLTEXSUBIMAGE3DPROC                              );
        }
    }
    pub(super) unsafe fn gl_load_GL_VERSION_1_3(&mut self, compat: &GladVersionCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_1_3 { return; }
            self.glActiveTexture                               = load!((load)(userptr, c"glActiveTexture"                                .as_ptr()) as PFNGLACTIVETEXTUREPROC                              );
            self.glCompressedTexImage1D                        = load!((load)(userptr, c"glCompressedTexImage1D"                         .as_ptr()) as PFNGLCOMPRESSEDTEXIMAGE1DPROC                       );
            self.glCompressedTexImage2D                        = load!((load)(userptr, c"glCompressedTexImage2D"                         .as_ptr()) as PFNGLCOMPRESSEDTEXIMAGE2DPROC                       );
            self.glCompressedTexImage3D                        = load!((load)(userptr, c"glCompressedTexImage3D"                         .as_ptr()) as PFNGLCOMPRESSEDTEXIMAGE3DPROC                       );
            self.glCompressedTexSubImage1D                     = load!((load)(userptr, c"glCompressedTexSubImage1D"                      .as_ptr()) as PFNGLCOMPRESSEDTEXSUBIMAGE1DPROC                    );
            self.glCompressedTexSubImage2D                     = load!((load)(userptr, c"glCompressedTexSubImage2D"                      .as_ptr()) as PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC                    );
            self.glCompressedTexSubImage3D                     = load!((load)(userptr, c"glCompressedTexSubImage3D"                      .as_ptr()) as PFNGLCOMPRESSEDTEXSUBIMAGE3DPROC                    );
            self.glGetCompressedTexImage                       = load!((load)(userptr, c"glGetCompressedTexImage"                        .as_ptr()) as PFNGLGETCOMPRESSEDTEXIMAGEPROC                      );
            self.glSampleCoverage                              = load!((load)(userptr, c"glSampleCoverage"                               .as_ptr()) as PFNGLSAMPLECOVERAGEPROC                             );
        }
    }
    pub(super) unsafe fn gl_load_GL_VERSION_1_4(&mut self, compat: &GladVersionCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_1_4 { return; }
            self.glBlendColor                                  = load!((load)(userptr, c"glBlendColor"                                   .as_ptr()) as PFNGLBLENDCOLORPROC                                 );
            self.glBlendEquation                               = load!((load)(userptr, c"glBlendEquation"                                .as_ptr()) as PFNGLBLENDEQUATIONPROC                              );
            self.glBlendFuncSeparate                           = load!((load)(userptr, c"glBlendFuncSeparate"                            .as_ptr()) as PFNGLBLENDFUNCSEPARATEPROC                          );
            self.glMultiDrawArrays                             = load!((load)(userptr, c"glMultiDrawArrays"                              .as_ptr()) as PFNGLMULTIDRAWARRAYSPROC                            );
            self.glMultiDrawElements                           = load!((load)(userptr, c"glMultiDrawElements"                            .as_ptr()) as PFNGLMULTIDRAWELEMENTSPROC                          );
            self.glPointParameterf                             = load!((load)(userptr, c"glPointParameterf"                              .as_ptr()) as PFNGLPOINTPARAMETERFPROC                            );
            self.glPointParameterfv                            = load!((load)(userptr, c"glPointParameterfv"                             .as_ptr()) as PFNGLPOINTPARAMETERFVPROC                           );
            self.glPointParameteri                             = load!((load)(userptr, c"glPointParameteri"                              .as_ptr()) as PFNGLPOINTPARAMETERIPROC                            );
            self.glPointParameteriv                            = load!((load)(userptr, c"glPointParameteriv"                             .as_ptr()) as PFNGLPOINTPARAMETERIVPROC                           );
        }
    }
    pub(super) unsafe fn gl_load_GL_VERSION_1_5(&mut self, compat: &GladVersionCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_1_5 { return; }
            self.glBeginQuery                                  = load!((load)(userptr, c"glBeginQuery"                                   .as_ptr()) as PFNGLBEGINQUERYPROC                                 );
            self.glBindBuffer                                  = load!((load)(userptr, c"glBindBuffer"                                   .as_ptr()) as PFNGLBINDBUFFERPROC                                 );
            self.glBufferData                                  = load!((load)(userptr, c"glBufferData"                                   .as_ptr()) as PFNGLBUFFERDATAPROC                                 );
            self.glBufferSubData                               = load!((load)(userptr, c"glBufferSubData"                                .as_ptr()) as PFNGLBUFFERSUBDATAPROC                              );
            self.glDeleteBuffers                               = load!((load)(userptr, c"glDeleteBuffers"                                .as_ptr()) as PFNGLDELETEBUFFERSPROC                              );
            self.glDeleteQueries                               = load!((load)(userptr, c"glDeleteQueries"                                .as_ptr()) as PFNGLDELETEQUERIESPROC                              );
            self.glEndQuery                                    = load!((load)(userptr, c"glEndQuery"                                     .as_ptr()) as PFNGLENDQUERYPROC                                   );
            self.glGenBuffers                                  = load!((load)(userptr, c"glGenBuffers"                                   .as_ptr()) as PFNGLGENBUFFERSPROC                                 );
            self.glGenQueries                                  = load!((load)(userptr, c"glGenQueries"                                   .as_ptr()) as PFNGLGENQUERIESPROC                                 );
            self.glGetBufferParameteriv                        = load!((load)(userptr, c"glGetBufferParameteriv"                         .as_ptr()) as PFNGLGETBUFFERPARAMETERIVPROC                       );
            self.glGetBufferPointerv                           = load!((load)(userptr, c"glGetBufferPointerv"                            .as_ptr()) as PFNGLGETBUFFERPOINTERVPROC                          );
            self.glGetBufferSubData                            = load!((load)(userptr, c"glGetBufferSubData"                             .as_ptr()) as PFNGLGETBUFFERSUBDATAPROC                           );
            self.glGetQueryObjectiv                            = load!((load)(userptr, c"glGetQueryObjectiv"                             .as_ptr()) as PFNGLGETQUERYOBJECTIVPROC                           );
            self.glGetQueryObjectuiv                           = load!((load)(userptr, c"glGetQueryObjectuiv"                            .as_ptr()) as PFNGLGETQUERYOBJECTUIVPROC                          );
            self.glGetQueryiv                                  = load!((load)(userptr, c"glGetQueryiv"                                   .as_ptr()) as PFNGLGETQUERYIVPROC                                 );
            self.glIsBuffer                                    = load!((load)(userptr, c"glIsBuffer"                                     .as_ptr()) as PFNGLISBUFFERPROC                                   );
            self.glIsQuery                                     = load!((load)(userptr, c"glIsQuery"                                      .as_ptr()) as PFNGLISQUERYPROC                                    );
            self.glMapBuffer                                   = load!((load)(userptr, c"glMapBuffer"                                    .as_ptr()) as PFNGLMAPBUFFERPROC                                  );
            self.glUnmapBuffer                                 = load!((load)(userptr, c"glUnmapBuffer"                                  .as_ptr()) as PFNGLUNMAPBUFFERPROC                                );
        }
    }
    pub(super) unsafe fn gl_load_GL_VERSION_2_0(&mut self, compat: &GladVersionCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_2_0 { return; }
            self.glAttachShader                                = load!((load)(userptr, c"glAttachShader"                                 .as_ptr()) as PFNGLATTACHSHADERPROC                               );
            self.glBindAttribLocation                          = load!((load)(userptr, c"glBindAttribLocation"                           .as_ptr()) as PFNGLBINDATTRIBLOCATIONPROC                         );
            self.glBlendEquationSeparate                       = load!((load)(userptr, c"glBlendEquationSeparate"                        .as_ptr()) as PFNGLBLENDEQUATIONSEPARATEPROC                      );
            self.glCompileShader                               = load!((load)(userptr, c"glCompileShader"                                .as_ptr()) as PFNGLCOMPILESHADERPROC                              );
            self.glCreateProgram                               = load!((load)(userptr, c"glCreateProgram"                                .as_ptr()) as PFNGLCREATEPROGRAMPROC                              );
            self.glCreateShader                                = load!((load)(userptr, c"glCreateShader"                                 .as_ptr()) as PFNGLCREATESHADERPROC                               );
            self.glDeleteProgram                               = load!((load)(userptr, c"glDeleteProgram"                                .as_ptr()) as PFNGLDELETEPROGRAMPROC                              );
            self.glDeleteShader                                = load!((load)(userptr, c"glDeleteShader"                                 .as_ptr()) as PFNGLDELETESHADERPROC                               );
            self.glDetachShader                                = load!((load)(userptr, c"glDetachShader"                                 .as_ptr()) as PFNGLDETACHSHADERPROC                               );
            self.glDisableVertexAttribArray                    = load!((load)(userptr, c"glDisableVertexAttribArray"                     .as_ptr()) as PFNGLDISABLEVERTEXATTRIBARRAYPROC                   );
            self.glDrawBuffers                                 = load!((load)(userptr, c"glDrawBuffers"                                  .as_ptr()) as PFNGLDRAWBUFFERSPROC                                );
            self.glEnableVertexAttribArray                     = load!((load)(userptr, c"glEnableVertexAttribArray"                      .as_ptr()) as PFNGLENABLEVERTEXATTRIBARRAYPROC                    );
            self.glGetActiveAttrib                             = load!((load)(userptr, c"glGetActiveAttrib"                              .as_ptr()) as PFNGLGETACTIVEATTRIBPROC                            );
            self.glGetActiveUniform                            = load!((load)(userptr, c"glGetActiveUniform"                             .as_ptr()) as PFNGLGETACTIVEUNIFORMPROC                           );
            self.glGetAttachedShaders                          = load!((load)(userptr, c"glGetAttachedShaders"                           .as_ptr()) as PFNGLGETATTACHEDSHADERSPROC                         );
            self.glGetAttribLocation                           = load!((load)(userptr, c"glGetAttribLocation"                            .as_ptr()) as PFNGLGETATTRIBLOCATIONPROC                          );
            self.glGetProgramInfoLog                           = load!((load)(userptr, c"glGetProgramInfoLog"                            .as_ptr()) as PFNGLGETPROGRAMINFOLOGPROC                          );
            self.glGetProgramiv                                = load!((load)(userptr, c"glGetProgramiv"                                 .as_ptr()) as PFNGLGETPROGRAMIVPROC                               );
            self.glGetShaderInfoLog                            = load!((load)(userptr, c"glGetShaderInfoLog"                             .as_ptr()) as PFNGLGETSHADERINFOLOGPROC                           );
            self.glGetShaderSource                             = load!((load)(userptr, c"glGetShaderSource"                              .as_ptr()) as PFNGLGETSHADERSOURCEPROC                            );
            self.glGetShaderiv                                 = load!((load)(userptr, c"glGetShaderiv"                                  .as_ptr()) as PFNGLGETSHADERIVPROC                                );
            self.glGetUniformLocation                          = load!((load)(userptr, c"glGetUniformLocation"                           .as_ptr()) as PFNGLGETUNIFORMLOCATIONPROC                         );
            self.glGetUniformfv                                = load!((load)(userptr, c"glGetUniformfv"                                 .as_ptr()) as PFNGLGETUNIFORMFVPROC                               );
            self.glGetUniformiv                                = load!((load)(userptr, c"glGetUniformiv"                                 .as_ptr()) as PFNGLGETUNIFORMIVPROC                               );
            self.glGetVertexAttribPointerv                     = load!((load)(userptr, c"glGetVertexAttribPointerv"                      .as_ptr()) as PFNGLGETVERTEXATTRIBPOINTERVPROC                    );
            self.glGetVertexAttribdv                           = load!((load)(userptr, c"glGetVertexAttribdv"                            .as_ptr()) as PFNGLGETVERTEXATTRIBDVPROC                          );
            self.glGetVertexAttribfv                           = load!((load)(userptr, c"glGetVertexAttribfv"                            .as_ptr()) as PFNGLGETVERTEXATTRIBFVPROC                          );
            self.glGetVertexAttribiv                           = load!((load)(userptr, c"glGetVertexAttribiv"                            .as_ptr()) as PFNGLGETVERTEXATTRIBIVPROC                          );
            self.glIsProgram                                   = load!((load)(userptr, c"glIsProgram"                                    .as_ptr()) as PFNGLISPROGRAMPROC                                  );
            self.glIsShader                                    = load!((load)(userptr, c"glIsShader"                                     .as_ptr()) as PFNGLISSHADERPROC                                   );
            self.glLinkProgram                                 = load!((load)(userptr, c"glLinkProgram"                                  .as_ptr()) as PFNGLLINKPROGRAMPROC                                );
            self.glShaderSource                                = load!((load)(userptr, c"glShaderSource"                                 .as_ptr()) as PFNGLSHADERSOURCEPROC                               );
            self.glStencilFuncSeparate                         = load!((load)(userptr, c"glStencilFuncSeparate"                          .as_ptr()) as PFNGLSTENCILFUNCSEPARATEPROC                        );
            self.glStencilMaskSeparate                         = load!((load)(userptr, c"glStencilMaskSeparate"                          .as_ptr()) as PFNGLSTENCILMASKSEPARATEPROC                        );
            self.glStencilOpSeparate                           = load!((load)(userptr, c"glStencilOpSeparate"                            .as_ptr()) as PFNGLSTENCILOPSEPARATEPROC                          );
            self.glUniform1f                                   = load!((load)(userptr, c"glUniform1f"                                    .as_ptr()) as PFNGLUNIFORM1FPROC                                  );
            self.glUniform1fv                                  = load!((load)(userptr, c"glUniform1fv"                                   .as_ptr()) as PFNGLUNIFORM1FVPROC                                 );
            self.glUniform1i                                   = load!((load)(userptr, c"glUniform1i"                                    .as_ptr()) as PFNGLUNIFORM1IPROC                                  );
            self.glUniform1iv                                  = load!((load)(userptr, c"glUniform1iv"                                   .as_ptr()) as PFNGLUNIFORM1IVPROC                                 );
            self.glUniform2f                                   = load!((load)(userptr, c"glUniform2f"                                    .as_ptr()) as PFNGLUNIFORM2FPROC                                  );
            self.glUniform2fv                                  = load!((load)(userptr, c"glUniform2fv"                                   .as_ptr()) as PFNGLUNIFORM2FVPROC                                 );
            self.glUniform2i                                   = load!((load)(userptr, c"glUniform2i"                                    .as_ptr()) as PFNGLUNIFORM2IPROC                                  );
            self.glUniform2iv                                  = load!((load)(userptr, c"glUniform2iv"                                   .as_ptr()) as PFNGLUNIFORM2IVPROC                                 );
            self.glUniform3f                                   = load!((load)(userptr, c"glUniform3f"                                    .as_ptr()) as PFNGLUNIFORM3FPROC                                  );
            self.glUniform3fv                                  = load!((load)(userptr, c"glUniform3fv"                                   .as_ptr()) as PFNGLUNIFORM3FVPROC                                 );
            self.glUniform3i                                   = load!((load)(userptr, c"glUniform3i"                                    .as_ptr()) as PFNGLUNIFORM3IPROC                                  );
            self.glUniform3iv                                  = load!((load)(userptr, c"glUniform3iv"                                   .as_ptr()) as PFNGLUNIFORM3IVPROC                                 );
            self.glUniform4f                                   = load!((load)(userptr, c"glUniform4f"                                    .as_ptr()) as PFNGLUNIFORM4FPROC                                  );
            self.glUniform4fv                                  = load!((load)(userptr, c"glUniform4fv"                                   .as_ptr()) as PFNGLUNIFORM4FVPROC                                 );
            self.glUniform4i                                   = load!((load)(userptr, c"glUniform4i"                                    .as_ptr()) as PFNGLUNIFORM4IPROC                                  );
            self.glUniform4iv                                  = load!((load)(userptr, c"glUniform4iv"                                   .as_ptr()) as PFNGLUNIFORM4IVPROC                                 );
            self.glUniformMatrix2fv                            = load!((load)(userptr, c"glUniformMatrix2fv"                             .as_ptr()) as PFNGLUNIFORMMATRIX2FVPROC                           );
            self.glUniformMatrix3fv                            = load!((load)(userptr, c"glUniformMatrix3fv"                             .as_ptr()) as PFNGLUNIFORMMATRIX3FVPROC                           );
            self.glUniformMatrix4fv                            = load!((load)(userptr, c"glUniformMatrix4fv"                             .as_ptr()) as PFNGLUNIFORMMATRIX4FVPROC                           );
            self.glUseProgram                                  = load!((load)(userptr, c"glUseProgram"                                   .as_ptr()) as PFNGLUSEPROGRAMPROC                                 );
            self.glValidateProgram                             = load!((load)(userptr, c"glValidateProgram"                              .as_ptr()) as PFNGLVALIDATEPROGRAMPROC                            );
            self.glVertexAttrib1d                              = load!((load)(userptr, c"glVertexAttrib1d"                               .as_ptr()) as PFNGLVERTEXATTRIB1DPROC                             );
            self.glVertexAttrib1dv                             = load!((load)(userptr, c"glVertexAttrib1dv"                              .as_ptr()) as PFNGLVERTEXATTRIB1DVPROC                            );
            self.glVertexAttrib1f                              = load!((load)(userptr, c"glVertexAttrib1f"                               .as_ptr()) as PFNGLVERTEXATTRIB1FPROC                             );
            self.glVertexAttrib1fv                             = load!((load)(userptr, c"glVertexAttrib1fv"                              .as_ptr()) as PFNGLVERTEXATTRIB1FVPROC                            );
            self.glVertexAttrib1s                              = load!((load)(userptr, c"glVertexAttrib1s"                               .as_ptr()) as PFNGLVERTEXATTRIB1SPROC                             );
            self.glVertexAttrib1sv                             = load!((load)(userptr, c"glVertexAttrib1sv"                              .as_ptr()) as PFNGLVERTEXATTRIB1SVPROC                            );
            self.glVertexAttrib2d                              = load!((load)(userptr, c"glVertexAttrib2d"                               .as_ptr()) as PFNGLVERTEXATTRIB2DPROC                             );
            self.glVertexAttrib2dv                             = load!((load)(userptr, c"glVertexAttrib2dv"                              .as_ptr()) as PFNGLVERTEXATTRIB2DVPROC                            );
            self.glVertexAttrib2f                              = load!((load)(userptr, c"glVertexAttrib2f"                               .as_ptr()) as PFNGLVERTEXATTRIB2FPROC                             );
            self.glVertexAttrib2fv                             = load!((load)(userptr, c"glVertexAttrib2fv"                              .as_ptr()) as PFNGLVERTEXATTRIB2FVPROC                            );
            self.glVertexAttrib2s                              = load!((load)(userptr, c"glVertexAttrib2s"                               .as_ptr()) as PFNGLVERTEXATTRIB2SPROC                             );
            self.glVertexAttrib2sv                             = load!((load)(userptr, c"glVertexAttrib2sv"                              .as_ptr()) as PFNGLVERTEXATTRIB2SVPROC                            );
            self.glVertexAttrib3d                              = load!((load)(userptr, c"glVertexAttrib3d"                               .as_ptr()) as PFNGLVERTEXATTRIB3DPROC                             );
            self.glVertexAttrib3dv                             = load!((load)(userptr, c"glVertexAttrib3dv"                              .as_ptr()) as PFNGLVERTEXATTRIB3DVPROC                            );
            self.glVertexAttrib3f                              = load!((load)(userptr, c"glVertexAttrib3f"                               .as_ptr()) as PFNGLVERTEXATTRIB3FPROC                             );
            self.glVertexAttrib3fv                             = load!((load)(userptr, c"glVertexAttrib3fv"                              .as_ptr()) as PFNGLVERTEXATTRIB3FVPROC                            );
            self.glVertexAttrib3s                              = load!((load)(userptr, c"glVertexAttrib3s"                               .as_ptr()) as PFNGLVERTEXATTRIB3SPROC                             );
            self.glVertexAttrib3sv                             = load!((load)(userptr, c"glVertexAttrib3sv"                              .as_ptr()) as PFNGLVERTEXATTRIB3SVPROC                            );
            self.glVertexAttrib4Nbv                            = load!((load)(userptr, c"glVertexAttrib4Nbv"                             .as_ptr()) as PFNGLVERTEXATTRIB4NBVPROC                           );
            self.glVertexAttrib4Niv                            = load!((load)(userptr, c"glVertexAttrib4Niv"                             .as_ptr()) as PFNGLVERTEXATTRIB4NIVPROC                           );
            self.glVertexAttrib4Nsv                            = load!((load)(userptr, c"glVertexAttrib4Nsv"                             .as_ptr()) as PFNGLVERTEXATTRIB4NSVPROC                           );
            self.glVertexAttrib4Nub                            = load!((load)(userptr, c"glVertexAttrib4Nub"                             .as_ptr()) as PFNGLVERTEXATTRIB4NUBPROC                           );
            self.glVertexAttrib4Nubv                           = load!((load)(userptr, c"glVertexAttrib4Nubv"                            .as_ptr()) as PFNGLVERTEXATTRIB4NUBVPROC                          );
            self.glVertexAttrib4Nuiv                           = load!((load)(userptr, c"glVertexAttrib4Nuiv"                            .as_ptr()) as PFNGLVERTEXATTRIB4NUIVPROC                          );
            self.glVertexAttrib4Nusv                           = load!((load)(userptr, c"glVertexAttrib4Nusv"                            .as_ptr()) as PFNGLVERTEXATTRIB4NUSVPROC                          );
            self.glVertexAttrib4bv                             = load!((load)(userptr, c"glVertexAttrib4bv"                              .as_ptr()) as PFNGLVERTEXATTRIB4BVPROC                            );
            self.glVertexAttrib4d                              = load!((load)(userptr, c"glVertexAttrib4d"                               .as_ptr()) as PFNGLVERTEXATTRIB4DPROC                             );
            self.glVertexAttrib4dv                             = load!((load)(userptr, c"glVertexAttrib4dv"                              .as_ptr()) as PFNGLVERTEXATTRIB4DVPROC                            );
            self.glVertexAttrib4f                              = load!((load)(userptr, c"glVertexAttrib4f"                               .as_ptr()) as PFNGLVERTEXATTRIB4FPROC                             );
            self.glVertexAttrib4fv                             = load!((load)(userptr, c"glVertexAttrib4fv"                              .as_ptr()) as PFNGLVERTEXATTRIB4FVPROC                            );
            self.glVertexAttrib4iv                             = load!((load)(userptr, c"glVertexAttrib4iv"                              .as_ptr()) as PFNGLVERTEXATTRIB4IVPROC                            );
            self.glVertexAttrib4s                              = load!((load)(userptr, c"glVertexAttrib4s"                               .as_ptr()) as PFNGLVERTEXATTRIB4SPROC                             );
            self.glVertexAttrib4sv                             = load!((load)(userptr, c"glVertexAttrib4sv"                              .as_ptr()) as PFNGLVERTEXATTRIB4SVPROC                            );
            self.glVertexAttrib4ubv                            = load!((load)(userptr, c"glVertexAttrib4ubv"                             .as_ptr()) as PFNGLVERTEXATTRIB4UBVPROC                           );
            self.glVertexAttrib4uiv                            = load!((load)(userptr, c"glVertexAttrib4uiv"                             .as_ptr()) as PFNGLVERTEXATTRIB4UIVPROC                           );
            self.glVertexAttrib4usv                            = load!((load)(userptr, c"glVertexAttrib4usv"                             .as_ptr()) as PFNGLVERTEXATTRIB4USVPROC                           );
            self.glVertexAttribPointer                         = load!((load)(userptr, c"glVertexAttribPointer"                          .as_ptr()) as PFNGLVERTEXATTRIBPOINTERPROC                        );
        }
    }
    pub(super) unsafe fn gl_load_GL_VERSION_2_1(&mut self, compat: &GladVersionCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_2_1 { return; }
            self.glUniformMatrix2x3fv                          = load!((load)(userptr, c"glUniformMatrix2x3fv"                           .as_ptr()) as PFNGLUNIFORMMATRIX2X3FVPROC                         );
            self.glUniformMatrix2x4fv                          = load!((load)(userptr, c"glUniformMatrix2x4fv"                           .as_ptr()) as PFNGLUNIFORMMATRIX2X4FVPROC                         );
            self.glUniformMatrix3x2fv                          = load!((load)(userptr, c"glUniformMatrix3x2fv"                           .as_ptr()) as PFNGLUNIFORMMATRIX3X2FVPROC                         );
            self.glUniformMatrix3x4fv                          = load!((load)(userptr, c"glUniformMatrix3x4fv"                           .as_ptr()) as PFNGLUNIFORMMATRIX3X4FVPROC                         );
            self.glUniformMatrix4x2fv                          = load!((load)(userptr, c"glUniformMatrix4x2fv"                           .as_ptr()) as PFNGLUNIFORMMATRIX4X2FVPROC                         );
            self.glUniformMatrix4x3fv                          = load!((load)(userptr, c"glUniformMatrix4x3fv"                           .as_ptr()) as PFNGLUNIFORMMATRIX4X3FVPROC                         );
        }
    }
    pub(super) unsafe fn gl_load_GL_VERSION_3_0(&mut self, compat: &GladVersionCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_3_0 { return; }
            self.glBeginConditionalRender                      = load!((load)(userptr, c"glBeginConditionalRender"                       .as_ptr()) as PFNGLBEGINCONDITIONALRENDERPROC                     );
            self.glBeginTransformFeedback                      = load!((load)(userptr, c"glBeginTransformFeedback"                       .as_ptr()) as PFNGLBEGINTRANSFORMFEEDBACKPROC                     );
            self.glBindBufferBase                              = load!((load)(userptr, c"glBindBufferBase"                               .as_ptr()) as PFNGLBINDBUFFERBASEPROC                             );
            self.glBindBufferRange                             = load!((load)(userptr, c"glBindBufferRange"                              .as_ptr()) as PFNGLBINDBUFFERRANGEPROC                            );
            self.glBindFragDataLocation                        = load!((load)(userptr, c"glBindFragDataLocation"                         .as_ptr()) as PFNGLBINDFRAGDATALOCATIONPROC                       );
            self.glBindFramebuffer                             = load!((load)(userptr, c"glBindFramebuffer"                              .as_ptr()) as PFNGLBINDFRAMEBUFFERPROC                            );
            self.glBindRenderbuffer                            = load!((load)(userptr, c"glBindRenderbuffer"                             .as_ptr()) as PFNGLBINDRENDERBUFFERPROC                           );
            self.glBindVertexArray                             = load!((load)(userptr, c"glBindVertexArray"                              .as_ptr()) as PFNGLBINDVERTEXARRAYPROC                            );
            self.glBlitFramebuffer                             = load!((load)(userptr, c"glBlitFramebuffer"                              .as_ptr()) as PFNGLBLITFRAMEBUFFERPROC                            );
            self.glCheckFramebufferStatus                      = load!((load)(userptr, c"glCheckFramebufferStatus"                       .as_ptr()) as PFNGLCHECKFRAMEBUFFERSTATUSPROC                     );
            self.glClampColor                                  = load!((load)(userptr, c"glClampColor"                                   .as_ptr()) as PFNGLCLAMPCOLORPROC                                 );
            self.glClearBufferfi                               = load!((load)(userptr, c"glClearBufferfi"                                .as_ptr()) as PFNGLCLEARBUFFERFIPROC                              );
            self.glClearBufferfv                               = load!((load)(userptr, c"glClearBufferfv"                                .as_ptr()) as PFNGLCLEARBUFFERFVPROC                              );
            self.glClearBufferiv                               = load!((load)(userptr, c"glClearBufferiv"                                .as_ptr()) as PFNGLCLEARBUFFERIVPROC                              );
            self.glClearBufferuiv                              = load!((load)(userptr, c"glClearBufferuiv"                               .as_ptr()) as PFNGLCLEARBUFFERUIVPROC                             );
            self.glColorMaski                                  = load!((load)(userptr, c"glColorMaski"                                   .as_ptr()) as PFNGLCOLORMASKIPROC                                 );
            self.glDeleteFramebuffers                          = load!((load)(userptr, c"glDeleteFramebuffers"                           .as_ptr()) as PFNGLDELETEFRAMEBUFFERSPROC                         );
            self.glDeleteRenderbuffers                         = load!((load)(userptr, c"glDeleteRenderbuffers"                          .as_ptr()) as PFNGLDELETERENDERBUFFERSPROC                        );
            self.glDeleteVertexArrays                          = load!((load)(userptr, c"glDeleteVertexArrays"                           .as_ptr()) as PFNGLDELETEVERTEXARRAYSPROC                         );
            self.glDisablei                                    = load!((load)(userptr, c"glDisablei"                                     .as_ptr()) as PFNGLDISABLEIPROC                                   );
            self.glEnablei                                     = load!((load)(userptr, c"glEnablei"                                      .as_ptr()) as PFNGLENABLEIPROC                                    );
            self.glEndConditionalRender                        = load!((load)(userptr, c"glEndConditionalRender"                         .as_ptr()) as PFNGLENDCONDITIONALRENDERPROC                       );
            self.glEndTransformFeedback                        = load!((load)(userptr, c"glEndTransformFeedback"                         .as_ptr()) as PFNGLENDTRANSFORMFEEDBACKPROC                       );
            self.glFlushMappedBufferRange                      = load!((load)(userptr, c"glFlushMappedBufferRange"                       .as_ptr()) as PFNGLFLUSHMAPPEDBUFFERRANGEPROC                     );
            self.glFramebufferRenderbuffer                     = load!((load)(userptr, c"glFramebufferRenderbuffer"                      .as_ptr()) as PFNGLFRAMEBUFFERRENDERBUFFERPROC                    );
            self.glFramebufferTexture1D                        = load!((load)(userptr, c"glFramebufferTexture1D"                         .as_ptr()) as PFNGLFRAMEBUFFERTEXTURE1DPROC                       );
            self.glFramebufferTexture2D                        = load!((load)(userptr, c"glFramebufferTexture2D"                         .as_ptr()) as PFNGLFRAMEBUFFERTEXTURE2DPROC                       );
            self.glFramebufferTexture3D                        = load!((load)(userptr, c"glFramebufferTexture3D"                         .as_ptr()) as PFNGLFRAMEBUFFERTEXTURE3DPROC                       );
            self.glFramebufferTextureLayer                     = load!((load)(userptr, c"glFramebufferTextureLayer"                      .as_ptr()) as PFNGLFRAMEBUFFERTEXTURELAYERPROC                    );
            self.glGenFramebuffers                             = load!((load)(userptr, c"glGenFramebuffers"                              .as_ptr()) as PFNGLGENFRAMEBUFFERSPROC                            );
            self.glGenRenderbuffers                            = load!((load)(userptr, c"glGenRenderbuffers"                             .as_ptr()) as PFNGLGENRENDERBUFFERSPROC                           );
            self.glGenVertexArrays                             = load!((load)(userptr, c"glGenVertexArrays"                              .as_ptr()) as PFNGLGENVERTEXARRAYSPROC                            );
            self.glGenerateMipmap                              = load!((load)(userptr, c"glGenerateMipmap"                               .as_ptr()) as PFNGLGENERATEMIPMAPPROC                             );
            self.glGetBooleani_v                               = load!((load)(userptr, c"glGetBooleani_v"                                .as_ptr()) as PFNGLGETBOOLEANI_VPROC                              );
            self.glGetFragDataLocation                         = load!((load)(userptr, c"glGetFragDataLocation"                          .as_ptr()) as PFNGLGETFRAGDATALOCATIONPROC                        );
            self.glGetFramebufferAttachmentParameteriv         = load!((load)(userptr, c"glGetFramebufferAttachmentParameteriv"          .as_ptr()) as PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC        );
            self.glGetIntegeri_v                               = load!((load)(userptr, c"glGetIntegeri_v"                                .as_ptr()) as PFNGLGETINTEGERI_VPROC                              );
            self.glGetRenderbufferParameteriv                  = load!((load)(userptr, c"glGetRenderbufferParameteriv"                   .as_ptr()) as PFNGLGETRENDERBUFFERPARAMETERIVPROC                 );
            self.glGetStringi                                  = load!((load)(userptr, c"glGetStringi"                                   .as_ptr()) as PFNGLGETSTRINGIPROC                                 );
            self.glGetTexParameterIiv                          = load!((load)(userptr, c"glGetTexParameterIiv"                           .as_ptr()) as PFNGLGETTEXPARAMETERIIVPROC                         );
            self.glGetTexParameterIuiv                         = load!((load)(userptr, c"glGetTexParameterIuiv"                          .as_ptr()) as PFNGLGETTEXPARAMETERIUIVPROC                        );
            self.glGetTransformFeedbackVarying                 = load!((load)(userptr, c"glGetTransformFeedbackVarying"                  .as_ptr()) as PFNGLGETTRANSFORMFEEDBACKVARYINGPROC                );
            self.glGetUniformuiv                               = load!((load)(userptr, c"glGetUniformuiv"                                .as_ptr()) as PFNGLGETUNIFORMUIVPROC                              );
            self.glGetVertexAttribIiv                          = load!((load)(userptr, c"glGetVertexAttribIiv"                           .as_ptr()) as PFNGLGETVERTEXATTRIBIIVPROC                         );
            self.glGetVertexAttribIuiv                         = load!((load)(userptr, c"glGetVertexAttribIuiv"                          .as_ptr()) as PFNGLGETVERTEXATTRIBIUIVPROC                        );
            self.glIsEnabledi                                  = load!((load)(userptr, c"glIsEnabledi"                                   .as_ptr()) as PFNGLISENABLEDIPROC                                 );
            self.glIsFramebuffer                               = load!((load)(userptr, c"glIsFramebuffer"                                .as_ptr()) as PFNGLISFRAMEBUFFERPROC                              );
            self.glIsRenderbuffer                              = load!((load)(userptr, c"glIsRenderbuffer"                               .as_ptr()) as PFNGLISRENDERBUFFERPROC                             );
            self.glIsVertexArray                               = load!((load)(userptr, c"glIsVertexArray"                                .as_ptr()) as PFNGLISVERTEXARRAYPROC                              );
            self.glMapBufferRange                              = load!((load)(userptr, c"glMapBufferRange"                               .as_ptr()) as PFNGLMAPBUFFERRANGEPROC                             );
            self.glRenderbufferStorage                         = load!((load)(userptr, c"glRenderbufferStorage"                          .as_ptr()) as PFNGLRENDERBUFFERSTORAGEPROC                        );
            self.glRenderbufferStorageMultisample              = load!((load)(userptr, c"glRenderbufferStorageMultisample"               .as_ptr()) as PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC             );
            self.glTexParameterIiv                             = load!((load)(userptr, c"glTexParameterIiv"                              .as_ptr()) as PFNGLTEXPARAMETERIIVPROC                            );
            self.glTexParameterIuiv                            = load!((load)(userptr, c"glTexParameterIuiv"                             .as_ptr()) as PFNGLTEXPARAMETERIUIVPROC                           );
            self.glTransformFeedbackVaryings                   = load!((load)(userptr, c"glTransformFeedbackVaryings"                    .as_ptr()) as PFNGLTRANSFORMFEEDBACKVARYINGSPROC                  );
            self.glUniform1ui                                  = load!((load)(userptr, c"glUniform1ui"                                   .as_ptr()) as PFNGLUNIFORM1UIPROC                                 );
            self.glUniform1uiv                                 = load!((load)(userptr, c"glUniform1uiv"                                  .as_ptr()) as PFNGLUNIFORM1UIVPROC                                );
            self.glUniform2ui                                  = load!((load)(userptr, c"glUniform2ui"                                   .as_ptr()) as PFNGLUNIFORM2UIPROC                                 );
            self.glUniform2uiv                                 = load!((load)(userptr, c"glUniform2uiv"                                  .as_ptr()) as PFNGLUNIFORM2UIVPROC                                );
            self.glUniform3ui                                  = load!((load)(userptr, c"glUniform3ui"                                   .as_ptr()) as PFNGLUNIFORM3UIPROC                                 );
            self.glUniform3uiv                                 = load!((load)(userptr, c"glUniform3uiv"                                  .as_ptr()) as PFNGLUNIFORM3UIVPROC                                );
            self.glUniform4ui                                  = load!((load)(userptr, c"glUniform4ui"                                   .as_ptr()) as PFNGLUNIFORM4UIPROC                                 );
            self.glUniform4uiv                                 = load!((load)(userptr, c"glUniform4uiv"                                  .as_ptr()) as PFNGLUNIFORM4UIVPROC                                );
            self.glVertexAttribI1i                             = load!((load)(userptr, c"glVertexAttribI1i"                              .as_ptr()) as PFNGLVERTEXATTRIBI1IPROC                            );
            self.glVertexAttribI1iv                            = load!((load)(userptr, c"glVertexAttribI1iv"                             .as_ptr()) as PFNGLVERTEXATTRIBI1IVPROC                           );
            self.glVertexAttribI1ui                            = load!((load)(userptr, c"glVertexAttribI1ui"                             .as_ptr()) as PFNGLVERTEXATTRIBI1UIPROC                           );
            self.glVertexAttribI1uiv                           = load!((load)(userptr, c"glVertexAttribI1uiv"                            .as_ptr()) as PFNGLVERTEXATTRIBI1UIVPROC                          );
            self.glVertexAttribI2i                             = load!((load)(userptr, c"glVertexAttribI2i"                              .as_ptr()) as PFNGLVERTEXATTRIBI2IPROC                            );
            self.glVertexAttribI2iv                            = load!((load)(userptr, c"glVertexAttribI2iv"                             .as_ptr()) as PFNGLVERTEXATTRIBI2IVPROC                           );
            self.glVertexAttribI2ui                            = load!((load)(userptr, c"glVertexAttribI2ui"                             .as_ptr()) as PFNGLVERTEXATTRIBI2UIPROC                           );
            self.glVertexAttribI2uiv                           = load!((load)(userptr, c"glVertexAttribI2uiv"                            .as_ptr()) as PFNGLVERTEXATTRIBI2UIVPROC                          );
            self.glVertexAttribI3i                             = load!((load)(userptr, c"glVertexAttribI3i"                              .as_ptr()) as PFNGLVERTEXATTRIBI3IPROC                            );
            self.glVertexAttribI3iv                            = load!((load)(userptr, c"glVertexAttribI3iv"                             .as_ptr()) as PFNGLVERTEXATTRIBI3IVPROC                           );
            self.glVertexAttribI3ui                            = load!((load)(userptr, c"glVertexAttribI3ui"                             .as_ptr()) as PFNGLVERTEXATTRIBI3UIPROC                           );
            self.glVertexAttribI3uiv                           = load!((load)(userptr, c"glVertexAttribI3uiv"                            .as_ptr()) as PFNGLVERTEXATTRIBI3UIVPROC                          );
            self.glVertexAttribI4bv                            = load!((load)(userptr, c"glVertexAttribI4bv"                             .as_ptr()) as PFNGLVERTEXATTRIBI4BVPROC                           );
            self.glVertexAttribI4i                             = load!((load)(userptr, c"glVertexAttribI4i"                              .as_ptr()) as PFNGLVERTEXATTRIBI4IPROC                            );
            self.glVertexAttribI4iv                            = load!((load)(userptr, c"glVertexAttribI4iv"                             .as_ptr()) as PFNGLVERTEXATTRIBI4IVPROC                           );
            self.glVertexAttribI4sv                            = load!((load)(userptr, c"glVertexAttribI4sv"                             .as_ptr()) as PFNGLVERTEXATTRIBI4SVPROC                           );
            self.glVertexAttribI4ubv                           = load!((load)(userptr, c"glVertexAttribI4ubv"                            .as_ptr()) as PFNGLVERTEXATTRIBI4UBVPROC                          );
            self.glVertexAttribI4ui                            = load!((load)(userptr, c"glVertexAttribI4ui"                             .as_ptr()) as PFNGLVERTEXATTRIBI4UIPROC                           );
            self.glVertexAttribI4uiv                           = load!((load)(userptr, c"glVertexAttribI4uiv"                            .as_ptr()) as PFNGLVERTEXATTRIBI4UIVPROC                          );
            self.glVertexAttribI4usv                           = load!((load)(userptr, c"glVertexAttribI4usv"                            .as_ptr()) as PFNGLVERTEXATTRIBI4USVPROC                          );
            self.glVertexAttribIPointer                        = load!((load)(userptr, c"glVertexAttribIPointer"                         .as_ptr()) as PFNGLVERTEXATTRIBIPOINTERPROC                       );
        }
    }
    pub(super) unsafe fn gl_load_GL_VERSION_3_1(&mut self, compat: &GladVersionCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_3_1 { return; }
            self.glBindBufferBase                              = load!((load)(userptr, c"glBindBufferBase"                               .as_ptr()) as PFNGLBINDBUFFERBASEPROC                             );
            self.glBindBufferRange                             = load!((load)(userptr, c"glBindBufferRange"                              .as_ptr()) as PFNGLBINDBUFFERRANGEPROC                            );
            self.glCopyBufferSubData                           = load!((load)(userptr, c"glCopyBufferSubData"                            .as_ptr()) as PFNGLCOPYBUFFERSUBDATAPROC                          );
            self.glDrawArraysInstanced                         = load!((load)(userptr, c"glDrawArraysInstanced"                          .as_ptr()) as PFNGLDRAWARRAYSINSTANCEDPROC                        );
            self.glDrawElementsInstanced                       = load!((load)(userptr, c"glDrawElementsInstanced"                        .as_ptr()) as PFNGLDRAWELEMENTSINSTANCEDPROC                      );
            self.glGetActiveUniformBlockName                   = load!((load)(userptr, c"glGetActiveUniformBlockName"                    .as_ptr()) as PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC                  );
            self.glGetActiveUniformBlockiv                     = load!((load)(userptr, c"glGetActiveUniformBlockiv"                      .as_ptr()) as PFNGLGETACTIVEUNIFORMBLOCKIVPROC                    );
            self.glGetActiveUniformName                        = load!((load)(userptr, c"glGetActiveUniformName"                         .as_ptr()) as PFNGLGETACTIVEUNIFORMNAMEPROC                       );
            self.glGetActiveUniformsiv                         = load!((load)(userptr, c"glGetActiveUniformsiv"                          .as_ptr()) as PFNGLGETACTIVEUNIFORMSIVPROC                        );
            self.glGetIntegeri_v                               = load!((load)(userptr, c"glGetIntegeri_v"                                .as_ptr()) as PFNGLGETINTEGERI_VPROC                              );
            self.glGetUniformBlockIndex                        = load!((load)(userptr, c"glGetUniformBlockIndex"                         .as_ptr()) as PFNGLGETUNIFORMBLOCKINDEXPROC                       );
            self.glGetUniformIndices                           = load!((load)(userptr, c"glGetUniformIndices"                            .as_ptr()) as PFNGLGETUNIFORMINDICESPROC                          );
            self.glPrimitiveRestartIndex                       = load!((load)(userptr, c"glPrimitiveRestartIndex"                        .as_ptr()) as PFNGLPRIMITIVERESTARTINDEXPROC                      );
            self.glTexBuffer                                   = load!((load)(userptr, c"glTexBuffer"                                    .as_ptr()) as PFNGLTEXBUFFERPROC                                  );
            self.glUniformBlockBinding                         = load!((load)(userptr, c"glUniformBlockBinding"                          .as_ptr()) as PFNGLUNIFORMBLOCKBINDINGPROC                        );
        }
    }
    pub(super) unsafe fn gl_load_GL_VERSION_3_2(&mut self, compat: &GladVersionCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_3_2 { return; }
            self.glClientWaitSync                              = load!((load)(userptr, c"glClientWaitSync"                               .as_ptr()) as PFNGLCLIENTWAITSYNCPROC                             );
            self.glDeleteSync                                  = load!((load)(userptr, c"glDeleteSync"                                   .as_ptr()) as PFNGLDELETESYNCPROC                                 );
            self.glDrawElementsBaseVertex                      = load!((load)(userptr, c"glDrawElementsBaseVertex"                       .as_ptr()) as PFNGLDRAWELEMENTSBASEVERTEXPROC                     );
            self.glDrawElementsInstancedBaseVertex             = load!((load)(userptr, c"glDrawElementsInstancedBaseVertex"              .as_ptr()) as PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC            );
            self.glDrawRangeElementsBaseVertex                 = load!((load)(userptr, c"glDrawRangeElementsBaseVertex"                  .as_ptr()) as PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC                );
            self.glFenceSync                                   = load!((load)(userptr, c"glFenceSync"                                    .as_ptr()) as PFNGLFENCESYNCPROC                                  );
            self.glFramebufferTexture                          = load!((load)(userptr, c"glFramebufferTexture"                           .as_ptr()) as PFNGLFRAMEBUFFERTEXTUREPROC                         );
            self.glGetBufferParameteri64v                      = load!((load)(userptr, c"glGetBufferParameteri64v"                       .as_ptr()) as PFNGLGETBUFFERPARAMETERI64VPROC                     );
            self.glGetInteger64i_v                             = load!((load)(userptr, c"glGetInteger64i_v"                              .as_ptr()) as PFNGLGETINTEGER64I_VPROC                            );
            self.glGetInteger64v                               = load!((load)(userptr, c"glGetInteger64v"                                .as_ptr()) as PFNGLGETINTEGER64VPROC                              );
            self.glGetMultisamplefv                            = load!((load)(userptr, c"glGetMultisamplefv"                             .as_ptr()) as PFNGLGETMULTISAMPLEFVPROC                           );
            self.glGetSynciv                                   = load!((load)(userptr, c"glGetSynciv"                                    .as_ptr()) as PFNGLGETSYNCIVPROC                                  );
            self.glIsSync                                      = load!((load)(userptr, c"glIsSync"                                       .as_ptr()) as PFNGLISSYNCPROC                                     );
            self.glMultiDrawElementsBaseVertex                 = load!((load)(userptr, c"glMultiDrawElementsBaseVertex"                  .as_ptr()) as PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC                );
            self.glProvokingVertex                             = load!((load)(userptr, c"glProvokingVertex"                              .as_ptr()) as PFNGLPROVOKINGVERTEXPROC                            );
            self.glSampleMaski                                 = load!((load)(userptr, c"glSampleMaski"                                  .as_ptr()) as PFNGLSAMPLEMASKIPROC                                );
            self.glTexImage2DMultisample                       = load!((load)(userptr, c"glTexImage2DMultisample"                        .as_ptr()) as PFNGLTEXIMAGE2DMULTISAMPLEPROC                      );
            self.glTexImage3DMultisample                       = load!((load)(userptr, c"glTexImage3DMultisample"                        .as_ptr()) as PFNGLTEXIMAGE3DMULTISAMPLEPROC                      );
            self.glWaitSync                                    = load!((load)(userptr, c"glWaitSync"                                     .as_ptr()) as PFNGLWAITSYNCPROC                                   );
        }
    }
    pub(super) unsafe fn gl_load_GL_VERSION_3_3(&mut self, compat: &GladVersionCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_3_3 { return; }
            self.glBindFragDataLocationIndexed                 = load!((load)(userptr, c"glBindFragDataLocationIndexed"                  .as_ptr()) as PFNGLBINDFRAGDATALOCATIONINDEXEDPROC                );
            self.glBindSampler                                 = load!((load)(userptr, c"glBindSampler"                                  .as_ptr()) as PFNGLBINDSAMPLERPROC                                );
            self.glDeleteSamplers                              = load!((load)(userptr, c"glDeleteSamplers"                               .as_ptr()) as PFNGLDELETESAMPLERSPROC                             );
            self.glGenSamplers                                 = load!((load)(userptr, c"glGenSamplers"                                  .as_ptr()) as PFNGLGENSAMPLERSPROC                                );
            self.glGetFragDataIndex                            = load!((load)(userptr, c"glGetFragDataIndex"                             .as_ptr()) as PFNGLGETFRAGDATAINDEXPROC                           );
            self.glGetQueryObjecti64v                          = load!((load)(userptr, c"glGetQueryObjecti64v"                           .as_ptr()) as PFNGLGETQUERYOBJECTI64VPROC                         );
            self.glGetQueryObjectui64v                         = load!((load)(userptr, c"glGetQueryObjectui64v"                          .as_ptr()) as PFNGLGETQUERYOBJECTUI64VPROC                        );
            self.glGetSamplerParameterIiv                      = load!((load)(userptr, c"glGetSamplerParameterIiv"                       .as_ptr()) as PFNGLGETSAMPLERPARAMETERIIVPROC                     );
            self.glGetSamplerParameterIuiv                     = load!((load)(userptr, c"glGetSamplerParameterIuiv"                      .as_ptr()) as PFNGLGETSAMPLERPARAMETERIUIVPROC                    );
            self.glGetSamplerParameterfv                       = load!((load)(userptr, c"glGetSamplerParameterfv"                        .as_ptr()) as PFNGLGETSAMPLERPARAMETERFVPROC                      );
            self.glGetSamplerParameteriv                       = load!((load)(userptr, c"glGetSamplerParameteriv"                        .as_ptr()) as PFNGLGETSAMPLERPARAMETERIVPROC                      );
            self.glIsSampler                                   = load!((load)(userptr, c"glIsSampler"                                    .as_ptr()) as PFNGLISSAMPLERPROC                                  );
            self.glQueryCounter                                = load!((load)(userptr, c"glQueryCounter"                                 .as_ptr()) as PFNGLQUERYCOUNTERPROC                               );
            self.glSamplerParameterIiv                         = load!((load)(userptr, c"glSamplerParameterIiv"                          .as_ptr()) as PFNGLSAMPLERPARAMETERIIVPROC                        );
            self.glSamplerParameterIuiv                        = load!((load)(userptr, c"glSamplerParameterIuiv"                         .as_ptr()) as PFNGLSAMPLERPARAMETERIUIVPROC                       );
            self.glSamplerParameterf                           = load!((load)(userptr, c"glSamplerParameterf"                            .as_ptr()) as PFNGLSAMPLERPARAMETERFPROC                          );
            self.glSamplerParameterfv                          = load!((load)(userptr, c"glSamplerParameterfv"                           .as_ptr()) as PFNGLSAMPLERPARAMETERFVPROC                         );
            self.glSamplerParameteri                           = load!((load)(userptr, c"glSamplerParameteri"                            .as_ptr()) as PFNGLSAMPLERPARAMETERIPROC                          );
            self.glSamplerParameteriv                          = load!((load)(userptr, c"glSamplerParameteriv"                           .as_ptr()) as PFNGLSAMPLERPARAMETERIVPROC                         );
            self.glVertexAttribDivisor                         = load!((load)(userptr, c"glVertexAttribDivisor"                          .as_ptr()) as PFNGLVERTEXATTRIBDIVISORPROC                        );
            self.glVertexAttribP1ui                            = load!((load)(userptr, c"glVertexAttribP1ui"                             .as_ptr()) as PFNGLVERTEXATTRIBP1UIPROC                           );
            self.glVertexAttribP1uiv                           = load!((load)(userptr, c"glVertexAttribP1uiv"                            .as_ptr()) as PFNGLVERTEXATTRIBP1UIVPROC                          );
            self.glVertexAttribP2ui                            = load!((load)(userptr, c"glVertexAttribP2ui"                             .as_ptr()) as PFNGLVERTEXATTRIBP2UIPROC                           );
            self.glVertexAttribP2uiv                           = load!((load)(userptr, c"glVertexAttribP2uiv"                            .as_ptr()) as PFNGLVERTEXATTRIBP2UIVPROC                          );
            self.glVertexAttribP3ui                            = load!((load)(userptr, c"glVertexAttribP3ui"                             .as_ptr()) as PFNGLVERTEXATTRIBP3UIPROC                           );
            self.glVertexAttribP3uiv                           = load!((load)(userptr, c"glVertexAttribP3uiv"                            .as_ptr()) as PFNGLVERTEXATTRIBP3UIVPROC                          );
            self.glVertexAttribP4ui                            = load!((load)(userptr, c"glVertexAttribP4ui"                             .as_ptr()) as PFNGLVERTEXATTRIBP4UIPROC                           );
            self.glVertexAttribP4uiv                           = load!((load)(userptr, c"glVertexAttribP4uiv"                            .as_ptr()) as PFNGLVERTEXATTRIBP4UIVPROC                          );
        }
    }
    pub(super) unsafe fn gl_load_GL_VERSION_4_0(&mut self, compat: &GladVersionCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_4_0 { return; }
            self.glBeginQueryIndexed                           = load!((load)(userptr, c"glBeginQueryIndexed"                            .as_ptr()) as PFNGLBEGINQUERYINDEXEDPROC                          );
            self.glBindTransformFeedback                       = load!((load)(userptr, c"glBindTransformFeedback"                        .as_ptr()) as PFNGLBINDTRANSFORMFEEDBACKPROC                      );
            self.glBlendEquationSeparatei                      = load!((load)(userptr, c"glBlendEquationSeparatei"                       .as_ptr()) as PFNGLBLENDEQUATIONSEPARATEIPROC                     );
            self.glBlendEquationi                              = load!((load)(userptr, c"glBlendEquationi"                               .as_ptr()) as PFNGLBLENDEQUATIONIPROC                             );
            self.glBlendFuncSeparatei                          = load!((load)(userptr, c"glBlendFuncSeparatei"                           .as_ptr()) as PFNGLBLENDFUNCSEPARATEIPROC                         );
            self.glBlendFunci                                  = load!((load)(userptr, c"glBlendFunci"                                   .as_ptr()) as PFNGLBLENDFUNCIPROC                                 );
            self.glDeleteTransformFeedbacks                    = load!((load)(userptr, c"glDeleteTransformFeedbacks"                     .as_ptr()) as PFNGLDELETETRANSFORMFEEDBACKSPROC                   );
            self.glDrawArraysIndirect                          = load!((load)(userptr, c"glDrawArraysIndirect"                           .as_ptr()) as PFNGLDRAWARRAYSINDIRECTPROC                         );
            self.glDrawElementsIndirect                        = load!((load)(userptr, c"glDrawElementsIndirect"                         .as_ptr()) as PFNGLDRAWELEMENTSINDIRECTPROC                       );
            self.glDrawTransformFeedback                       = load!((load)(userptr, c"glDrawTransformFeedback"                        .as_ptr()) as PFNGLDRAWTRANSFORMFEEDBACKPROC                      );
            self.glDrawTransformFeedbackStream                 = load!((load)(userptr, c"glDrawTransformFeedbackStream"                  .as_ptr()) as PFNGLDRAWTRANSFORMFEEDBACKSTREAMPROC                );
            self.glEndQueryIndexed                             = load!((load)(userptr, c"glEndQueryIndexed"                              .as_ptr()) as PFNGLENDQUERYINDEXEDPROC                            );
            self.glGenTransformFeedbacks                       = load!((load)(userptr, c"glGenTransformFeedbacks"                        .as_ptr()) as PFNGLGENTRANSFORMFEEDBACKSPROC                      );
            self.glGetActiveSubroutineName                     = load!((load)(userptr, c"glGetActiveSubroutineName"                      .as_ptr()) as PFNGLGETACTIVESUBROUTINENAMEPROC                    );
            self.glGetActiveSubroutineUniformName              = load!((load)(userptr, c"glGetActiveSubroutineUniformName"               .as_ptr()) as PFNGLGETACTIVESUBROUTINEUNIFORMNAMEPROC             );
            self.glGetActiveSubroutineUniformiv                = load!((load)(userptr, c"glGetActiveSubroutineUniformiv"                 .as_ptr()) as PFNGLGETACTIVESUBROUTINEUNIFORMIVPROC               );
            self.glGetProgramStageiv                           = load!((load)(userptr, c"glGetProgramStageiv"                            .as_ptr()) as PFNGLGETPROGRAMSTAGEIVPROC                          );
            self.glGetQueryIndexediv                           = load!((load)(userptr, c"glGetQueryIndexediv"                            .as_ptr()) as PFNGLGETQUERYINDEXEDIVPROC                          );
            self.glGetSubroutineIndex                          = load!((load)(userptr, c"glGetSubroutineIndex"                           .as_ptr()) as PFNGLGETSUBROUTINEINDEXPROC                         );
            self.glGetSubroutineUniformLocation                = load!((load)(userptr, c"glGetSubroutineUniformLocation"                 .as_ptr()) as PFNGLGETSUBROUTINEUNIFORMLOCATIONPROC               );
            self.glGetUniformSubroutineuiv                     = load!((load)(userptr, c"glGetUniformSubroutineuiv"                      .as_ptr()) as PFNGLGETUNIFORMSUBROUTINEUIVPROC                    );
            self.glGetUniformdv                                = load!((load)(userptr, c"glGetUniformdv"                                 .as_ptr()) as PFNGLGETUNIFORMDVPROC                               );
            self.glIsTransformFeedback                         = load!((load)(userptr, c"glIsTransformFeedback"                          .as_ptr()) as PFNGLISTRANSFORMFEEDBACKPROC                        );
            self.glMinSampleShading                            = load!((load)(userptr, c"glMinSampleShading"                             .as_ptr()) as PFNGLMINSAMPLESHADINGPROC                           );
            self.glPatchParameterfv                            = load!((load)(userptr, c"glPatchParameterfv"                             .as_ptr()) as PFNGLPATCHPARAMETERFVPROC                           );
            self.glPatchParameteri                             = load!((load)(userptr, c"glPatchParameteri"                              .as_ptr()) as PFNGLPATCHPARAMETERIPROC                            );
            self.glPauseTransformFeedback                      = load!((load)(userptr, c"glPauseTransformFeedback"                       .as_ptr()) as PFNGLPAUSETRANSFORMFEEDBACKPROC                     );
            self.glResumeTransformFeedback                     = load!((load)(userptr, c"glResumeTransformFeedback"                      .as_ptr()) as PFNGLRESUMETRANSFORMFEEDBACKPROC                    );
            self.glUniform1d                                   = load!((load)(userptr, c"glUniform1d"                                    .as_ptr()) as PFNGLUNIFORM1DPROC                                  );
            self.glUniform1dv                                  = load!((load)(userptr, c"glUniform1dv"                                   .as_ptr()) as PFNGLUNIFORM1DVPROC                                 );
            self.glUniform2d                                   = load!((load)(userptr, c"glUniform2d"                                    .as_ptr()) as PFNGLUNIFORM2DPROC                                  );
            self.glUniform2dv                                  = load!((load)(userptr, c"glUniform2dv"                                   .as_ptr()) as PFNGLUNIFORM2DVPROC                                 );
            self.glUniform3d                                   = load!((load)(userptr, c"glUniform3d"                                    .as_ptr()) as PFNGLUNIFORM3DPROC                                  );
            self.glUniform3dv                                  = load!((load)(userptr, c"glUniform3dv"                                   .as_ptr()) as PFNGLUNIFORM3DVPROC                                 );
            self.glUniform4d                                   = load!((load)(userptr, c"glUniform4d"                                    .as_ptr()) as PFNGLUNIFORM4DPROC                                  );
            self.glUniform4dv                                  = load!((load)(userptr, c"glUniform4dv"                                   .as_ptr()) as PFNGLUNIFORM4DVPROC                                 );
            self.glUniformMatrix2dv                            = load!((load)(userptr, c"glUniformMatrix2dv"                             .as_ptr()) as PFNGLUNIFORMMATRIX2DVPROC                           );
            self.glUniformMatrix2x3dv                          = load!((load)(userptr, c"glUniformMatrix2x3dv"                           .as_ptr()) as PFNGLUNIFORMMATRIX2X3DVPROC                         );
            self.glUniformMatrix2x4dv                          = load!((load)(userptr, c"glUniformMatrix2x4dv"                           .as_ptr()) as PFNGLUNIFORMMATRIX2X4DVPROC                         );
            self.glUniformMatrix3dv                            = load!((load)(userptr, c"glUniformMatrix3dv"                             .as_ptr()) as PFNGLUNIFORMMATRIX3DVPROC                           );
            self.glUniformMatrix3x2dv                          = load!((load)(userptr, c"glUniformMatrix3x2dv"                           .as_ptr()) as PFNGLUNIFORMMATRIX3X2DVPROC                         );
            self.glUniformMatrix3x4dv                          = load!((load)(userptr, c"glUniformMatrix3x4dv"                           .as_ptr()) as PFNGLUNIFORMMATRIX3X4DVPROC                         );
            self.glUniformMatrix4dv                            = load!((load)(userptr, c"glUniformMatrix4dv"                             .as_ptr()) as PFNGLUNIFORMMATRIX4DVPROC                           );
            self.glUniformMatrix4x2dv                          = load!((load)(userptr, c"glUniformMatrix4x2dv"                           .as_ptr()) as PFNGLUNIFORMMATRIX4X2DVPROC                         );
            self.glUniformMatrix4x3dv                          = load!((load)(userptr, c"glUniformMatrix4x3dv"                           .as_ptr()) as PFNGLUNIFORMMATRIX4X3DVPROC                         );
            self.glUniformSubroutinesuiv                       = load!((load)(userptr, c"glUniformSubroutinesuiv"                        .as_ptr()) as PFNGLUNIFORMSUBROUTINESUIVPROC                      );
        }
    }
    pub(super) unsafe fn gl_load_GL_VERSION_4_1(&mut self, compat: &GladVersionCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_4_1 { return; }
            self.glActiveShaderProgram                         = load!((load)(userptr, c"glActiveShaderProgram"                          .as_ptr()) as PFNGLACTIVESHADERPROGRAMPROC                        );
            self.glBindProgramPipeline                         = load!((load)(userptr, c"glBindProgramPipeline"                          .as_ptr()) as PFNGLBINDPROGRAMPIPELINEPROC                        );
            self.glClearDepthf                                 = load!((load)(userptr, c"glClearDepthf"                                  .as_ptr()) as PFNGLCLEARDEPTHFPROC                                );
            self.glCreateShaderProgramv                        = load!((load)(userptr, c"glCreateShaderProgramv"                         .as_ptr()) as PFNGLCREATESHADERPROGRAMVPROC                       );
            self.glDeleteProgramPipelines                      = load!((load)(userptr, c"glDeleteProgramPipelines"                       .as_ptr()) as PFNGLDELETEPROGRAMPIPELINESPROC                     );
            self.glDepthRangeArrayv                            = load!((load)(userptr, c"glDepthRangeArrayv"                             .as_ptr()) as PFNGLDEPTHRANGEARRAYVPROC                           );
            self.glDepthRangeIndexed                           = load!((load)(userptr, c"glDepthRangeIndexed"                            .as_ptr()) as PFNGLDEPTHRANGEINDEXEDPROC                          );
            self.glDepthRangef                                 = load!((load)(userptr, c"glDepthRangef"                                  .as_ptr()) as PFNGLDEPTHRANGEFPROC                                );
            self.glGenProgramPipelines                         = load!((load)(userptr, c"glGenProgramPipelines"                          .as_ptr()) as PFNGLGENPROGRAMPIPELINESPROC                        );
            self.glGetDoublei_v                                = load!((load)(userptr, c"glGetDoublei_v"                                 .as_ptr()) as PFNGLGETDOUBLEI_VPROC                               );
            self.glGetFloati_v                                 = load!((load)(userptr, c"glGetFloati_v"                                  .as_ptr()) as PFNGLGETFLOATI_VPROC                                );
            self.glGetProgramBinary                            = load!((load)(userptr, c"glGetProgramBinary"                             .as_ptr()) as PFNGLGETPROGRAMBINARYPROC                           );
            self.glGetProgramPipelineInfoLog                   = load!((load)(userptr, c"glGetProgramPipelineInfoLog"                    .as_ptr()) as PFNGLGETPROGRAMPIPELINEINFOLOGPROC                  );
            self.glGetProgramPipelineiv                        = load!((load)(userptr, c"glGetProgramPipelineiv"                         .as_ptr()) as PFNGLGETPROGRAMPIPELINEIVPROC                       );
            self.glGetShaderPrecisionFormat                    = load!((load)(userptr, c"glGetShaderPrecisionFormat"                     .as_ptr()) as PFNGLGETSHADERPRECISIONFORMATPROC                   );
            self.glGetVertexAttribLdv                          = load!((load)(userptr, c"glGetVertexAttribLdv"                           .as_ptr()) as PFNGLGETVERTEXATTRIBLDVPROC                         );
            self.glIsProgramPipeline                           = load!((load)(userptr, c"glIsProgramPipeline"                            .as_ptr()) as PFNGLISPROGRAMPIPELINEPROC                          );
            self.glProgramBinary                               = load!((load)(userptr, c"glProgramBinary"                                .as_ptr()) as PFNGLPROGRAMBINARYPROC                              );
            self.glProgramParameteri                           = load!((load)(userptr, c"glProgramParameteri"                            .as_ptr()) as PFNGLPROGRAMPARAMETERIPROC                          );
            self.glProgramUniform1d                            = load!((load)(userptr, c"glProgramUniform1d"                             .as_ptr()) as PFNGLPROGRAMUNIFORM1DPROC                           );
            self.glProgramUniform1dv                           = load!((load)(userptr, c"glProgramUniform1dv"                            .as_ptr()) as PFNGLPROGRAMUNIFORM1DVPROC                          );
            self.glProgramUniform1f                            = load!((load)(userptr, c"glProgramUniform1f"                             .as_ptr()) as PFNGLPROGRAMUNIFORM1FPROC                           );
            self.glProgramUniform1fv                           = load!((load)(userptr, c"glProgramUniform1fv"                            .as_ptr()) as PFNGLPROGRAMUNIFORM1FVPROC                          );
            self.glProgramUniform1i                            = load!((load)(userptr, c"glProgramUniform1i"                             .as_ptr()) as PFNGLPROGRAMUNIFORM1IPROC                           );
            self.glProgramUniform1iv                           = load!((load)(userptr, c"glProgramUniform1iv"                            .as_ptr()) as PFNGLPROGRAMUNIFORM1IVPROC                          );
            self.glProgramUniform1ui                           = load!((load)(userptr, c"glProgramUniform1ui"                            .as_ptr()) as PFNGLPROGRAMUNIFORM1UIPROC                          );
            self.glProgramUniform1uiv                          = load!((load)(userptr, c"glProgramUniform1uiv"                           .as_ptr()) as PFNGLPROGRAMUNIFORM1UIVPROC                         );
            self.glProgramUniform2d                            = load!((load)(userptr, c"glProgramUniform2d"                             .as_ptr()) as PFNGLPROGRAMUNIFORM2DPROC                           );
            self.glProgramUniform2dv                           = load!((load)(userptr, c"glProgramUniform2dv"                            .as_ptr()) as PFNGLPROGRAMUNIFORM2DVPROC                          );
            self.glProgramUniform2f                            = load!((load)(userptr, c"glProgramUniform2f"                             .as_ptr()) as PFNGLPROGRAMUNIFORM2FPROC                           );
            self.glProgramUniform2fv                           = load!((load)(userptr, c"glProgramUniform2fv"                            .as_ptr()) as PFNGLPROGRAMUNIFORM2FVPROC                          );
            self.glProgramUniform2i                            = load!((load)(userptr, c"glProgramUniform2i"                             .as_ptr()) as PFNGLPROGRAMUNIFORM2IPROC                           );
            self.glProgramUniform2iv                           = load!((load)(userptr, c"glProgramUniform2iv"                            .as_ptr()) as PFNGLPROGRAMUNIFORM2IVPROC                          );
            self.glProgramUniform2ui                           = load!((load)(userptr, c"glProgramUniform2ui"                            .as_ptr()) as PFNGLPROGRAMUNIFORM2UIPROC                          );
            self.glProgramUniform2uiv                          = load!((load)(userptr, c"glProgramUniform2uiv"                           .as_ptr()) as PFNGLPROGRAMUNIFORM2UIVPROC                         );
            self.glProgramUniform3d                            = load!((load)(userptr, c"glProgramUniform3d"                             .as_ptr()) as PFNGLPROGRAMUNIFORM3DPROC                           );
            self.glProgramUniform3dv                           = load!((load)(userptr, c"glProgramUniform3dv"                            .as_ptr()) as PFNGLPROGRAMUNIFORM3DVPROC                          );
            self.glProgramUniform3f                            = load!((load)(userptr, c"glProgramUniform3f"                             .as_ptr()) as PFNGLPROGRAMUNIFORM3FPROC                           );
            self.glProgramUniform3fv                           = load!((load)(userptr, c"glProgramUniform3fv"                            .as_ptr()) as PFNGLPROGRAMUNIFORM3FVPROC                          );
            self.glProgramUniform3i                            = load!((load)(userptr, c"glProgramUniform3i"                             .as_ptr()) as PFNGLPROGRAMUNIFORM3IPROC                           );
            self.glProgramUniform3iv                           = load!((load)(userptr, c"glProgramUniform3iv"                            .as_ptr()) as PFNGLPROGRAMUNIFORM3IVPROC                          );
            self.glProgramUniform3ui                           = load!((load)(userptr, c"glProgramUniform3ui"                            .as_ptr()) as PFNGLPROGRAMUNIFORM3UIPROC                          );
            self.glProgramUniform3uiv                          = load!((load)(userptr, c"glProgramUniform3uiv"                           .as_ptr()) as PFNGLPROGRAMUNIFORM3UIVPROC                         );
            self.glProgramUniform4d                            = load!((load)(userptr, c"glProgramUniform4d"                             .as_ptr()) as PFNGLPROGRAMUNIFORM4DPROC                           );
            self.glProgramUniform4dv                           = load!((load)(userptr, c"glProgramUniform4dv"                            .as_ptr()) as PFNGLPROGRAMUNIFORM4DVPROC                          );
            self.glProgramUniform4f                            = load!((load)(userptr, c"glProgramUniform4f"                             .as_ptr()) as PFNGLPROGRAMUNIFORM4FPROC                           );
            self.glProgramUniform4fv                           = load!((load)(userptr, c"glProgramUniform4fv"                            .as_ptr()) as PFNGLPROGRAMUNIFORM4FVPROC                          );
            self.glProgramUniform4i                            = load!((load)(userptr, c"glProgramUniform4i"                             .as_ptr()) as PFNGLPROGRAMUNIFORM4IPROC                           );
            self.glProgramUniform4iv                           = load!((load)(userptr, c"glProgramUniform4iv"                            .as_ptr()) as PFNGLPROGRAMUNIFORM4IVPROC                          );
            self.glProgramUniform4ui                           = load!((load)(userptr, c"glProgramUniform4ui"                            .as_ptr()) as PFNGLPROGRAMUNIFORM4UIPROC                          );
            self.glProgramUniform4uiv                          = load!((load)(userptr, c"glProgramUniform4uiv"                           .as_ptr()) as PFNGLPROGRAMUNIFORM4UIVPROC                         );
            self.glProgramUniformMatrix2dv                     = load!((load)(userptr, c"glProgramUniformMatrix2dv"                      .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX2DVPROC                    );
            self.glProgramUniformMatrix2fv                     = load!((load)(userptr, c"glProgramUniformMatrix2fv"                      .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX2FVPROC                    );
            self.glProgramUniformMatrix2x3dv                   = load!((load)(userptr, c"glProgramUniformMatrix2x3dv"                    .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX2X3DVPROC                  );
            self.glProgramUniformMatrix2x3fv                   = load!((load)(userptr, c"glProgramUniformMatrix2x3fv"                    .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX2X3FVPROC                  );
            self.glProgramUniformMatrix2x4dv                   = load!((load)(userptr, c"glProgramUniformMatrix2x4dv"                    .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX2X4DVPROC                  );
            self.glProgramUniformMatrix2x4fv                   = load!((load)(userptr, c"glProgramUniformMatrix2x4fv"                    .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX2X4FVPROC                  );
            self.glProgramUniformMatrix3dv                     = load!((load)(userptr, c"glProgramUniformMatrix3dv"                      .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX3DVPROC                    );
            self.glProgramUniformMatrix3fv                     = load!((load)(userptr, c"glProgramUniformMatrix3fv"                      .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX3FVPROC                    );
            self.glProgramUniformMatrix3x2dv                   = load!((load)(userptr, c"glProgramUniformMatrix3x2dv"                    .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX3X2DVPROC                  );
            self.glProgramUniformMatrix3x2fv                   = load!((load)(userptr, c"glProgramUniformMatrix3x2fv"                    .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX3X2FVPROC                  );
            self.glProgramUniformMatrix3x4dv                   = load!((load)(userptr, c"glProgramUniformMatrix3x4dv"                    .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX3X4DVPROC                  );
            self.glProgramUniformMatrix3x4fv                   = load!((load)(userptr, c"glProgramUniformMatrix3x4fv"                    .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX3X4FVPROC                  );
            self.glProgramUniformMatrix4dv                     = load!((load)(userptr, c"glProgramUniformMatrix4dv"                      .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX4DVPROC                    );
            self.glProgramUniformMatrix4fv                     = load!((load)(userptr, c"glProgramUniformMatrix4fv"                      .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX4FVPROC                    );
            self.glProgramUniformMatrix4x2dv                   = load!((load)(userptr, c"glProgramUniformMatrix4x2dv"                    .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX4X2DVPROC                  );
            self.glProgramUniformMatrix4x2fv                   = load!((load)(userptr, c"glProgramUniformMatrix4x2fv"                    .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX4X2FVPROC                  );
            self.glProgramUniformMatrix4x3dv                   = load!((load)(userptr, c"glProgramUniformMatrix4x3dv"                    .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX4X3DVPROC                  );
            self.glProgramUniformMatrix4x3fv                   = load!((load)(userptr, c"glProgramUniformMatrix4x3fv"                    .as_ptr()) as PFNGLPROGRAMUNIFORMMATRIX4X3FVPROC                  );
            self.glReleaseShaderCompiler                       = load!((load)(userptr, c"glReleaseShaderCompiler"                        .as_ptr()) as PFNGLRELEASESHADERCOMPILERPROC                      );
            self.glScissorArrayv                               = load!((load)(userptr, c"glScissorArrayv"                                .as_ptr()) as PFNGLSCISSORARRAYVPROC                              );
            self.glScissorIndexed                              = load!((load)(userptr, c"glScissorIndexed"                               .as_ptr()) as PFNGLSCISSORINDEXEDPROC                             );
            self.glScissorIndexedv                             = load!((load)(userptr, c"glScissorIndexedv"                              .as_ptr()) as PFNGLSCISSORINDEXEDVPROC                            );
            self.glShaderBinary                                = load!((load)(userptr, c"glShaderBinary"                                 .as_ptr()) as PFNGLSHADERBINARYPROC                               );
            self.glUseProgramStages                            = load!((load)(userptr, c"glUseProgramStages"                             .as_ptr()) as PFNGLUSEPROGRAMSTAGESPROC                           );
            self.glValidateProgramPipeline                     = load!((load)(userptr, c"glValidateProgramPipeline"                      .as_ptr()) as PFNGLVALIDATEPROGRAMPIPELINEPROC                    );
            self.glVertexAttribL1d                             = load!((load)(userptr, c"glVertexAttribL1d"                              .as_ptr()) as PFNGLVERTEXATTRIBL1DPROC                            );
            self.glVertexAttribL1dv                            = load!((load)(userptr, c"glVertexAttribL1dv"                             .as_ptr()) as PFNGLVERTEXATTRIBL1DVPROC                           );
            self.glVertexAttribL2d                             = load!((load)(userptr, c"glVertexAttribL2d"                              .as_ptr()) as PFNGLVERTEXATTRIBL2DPROC                            );
            self.glVertexAttribL2dv                            = load!((load)(userptr, c"glVertexAttribL2dv"                             .as_ptr()) as PFNGLVERTEXATTRIBL2DVPROC                           );
            self.glVertexAttribL3d                             = load!((load)(userptr, c"glVertexAttribL3d"                              .as_ptr()) as PFNGLVERTEXATTRIBL3DPROC                            );
            self.glVertexAttribL3dv                            = load!((load)(userptr, c"glVertexAttribL3dv"                             .as_ptr()) as PFNGLVERTEXATTRIBL3DVPROC                           );
            self.glVertexAttribL4d                             = load!((load)(userptr, c"glVertexAttribL4d"                              .as_ptr()) as PFNGLVERTEXATTRIBL4DPROC                            );
            self.glVertexAttribL4dv                            = load!((load)(userptr, c"glVertexAttribL4dv"                             .as_ptr()) as PFNGLVERTEXATTRIBL4DVPROC                           );
            self.glVertexAttribLPointer                        = load!((load)(userptr, c"glVertexAttribLPointer"                         .as_ptr()) as PFNGLVERTEXATTRIBLPOINTERPROC                       );
            self.glViewportArrayv                              = load!((load)(userptr, c"glViewportArrayv"                               .as_ptr()) as PFNGLVIEWPORTARRAYVPROC                             );
            self.glViewportIndexedf                            = load!((load)(userptr, c"glViewportIndexedf"                             .as_ptr()) as PFNGLVIEWPORTINDEXEDFPROC                           );
            self.glViewportIndexedfv                           = load!((load)(userptr, c"glViewportIndexedfv"                            .as_ptr()) as PFNGLVIEWPORTINDEXEDFVPROC                          );
        }
    }
    pub(super) unsafe fn gl_load_GL_VERSION_4_2(&mut self, compat: &GladVersionCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_4_2 { return; }
            self.glBindImageTexture                            = load!((load)(userptr, c"glBindImageTexture"                             .as_ptr()) as PFNGLBINDIMAGETEXTUREPROC                           );
            self.glDrawArraysInstancedBaseInstance             = load!((load)(userptr, c"glDrawArraysInstancedBaseInstance"              .as_ptr()) as PFNGLDRAWARRAYSINSTANCEDBASEINSTANCEPROC            );
            self.glDrawElementsInstancedBaseInstance           = load!((load)(userptr, c"glDrawElementsInstancedBaseInstance"            .as_ptr()) as PFNGLDRAWELEMENTSINSTANCEDBASEINSTANCEPROC          );
            self.glDrawElementsInstancedBaseVertexBaseInstance = load!((load)(userptr, c"glDrawElementsInstancedBaseVertexBaseInstance"  .as_ptr()) as PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXBASEINSTANCEPROC);
            self.glDrawTransformFeedbackInstanced              = load!((load)(userptr, c"glDrawTransformFeedbackInstanced"               .as_ptr()) as PFNGLDRAWTRANSFORMFEEDBACKINSTANCEDPROC             );
            self.glDrawTransformFeedbackStreamInstanced        = load!((load)(userptr, c"glDrawTransformFeedbackStreamInstanced"         .as_ptr()) as PFNGLDRAWTRANSFORMFEEDBACKSTREAMINSTANCEDPROC       );
            self.glGetActiveAtomicCounterBufferiv              = load!((load)(userptr, c"glGetActiveAtomicCounterBufferiv"               .as_ptr()) as PFNGLGETACTIVEATOMICCOUNTERBUFFERIVPROC             );
            self.glGetInternalformativ                         = load!((load)(userptr, c"glGetInternalformativ"                          .as_ptr()) as PFNGLGETINTERNALFORMATIVPROC                        );
            self.glMemoryBarrier                               = load!((load)(userptr, c"glMemoryBarrier"                                .as_ptr()) as PFNGLMEMORYBARRIERPROC                              );
            self.glTexStorage1D                                = load!((load)(userptr, c"glTexStorage1D"                                 .as_ptr()) as PFNGLTEXSTORAGE1DPROC                               );
            self.glTexStorage2D                                = load!((load)(userptr, c"glTexStorage2D"                                 .as_ptr()) as PFNGLTEXSTORAGE2DPROC                               );
            self.glTexStorage3D                                = load!((load)(userptr, c"glTexStorage3D"                                 .as_ptr()) as PFNGLTEXSTORAGE3DPROC                               );
        }
    }
    pub(super) unsafe fn gl_load_GL_VERSION_4_3(&mut self, compat: &GladVersionCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_VERSION_4_3 { return; }
            self.glBindVertexBuffer                            = load!((load)(userptr, c"glBindVertexBuffer"                             .as_ptr()) as PFNGLBINDVERTEXBUFFERPROC                           );
            self.glClearBufferData                             = load!((load)(userptr, c"glClearBufferData"                              .as_ptr()) as PFNGLCLEARBUFFERDATAPROC                            );
            self.glClearBufferSubData                          = load!((load)(userptr, c"glClearBufferSubData"                           .as_ptr()) as PFNGLCLEARBUFFERSUBDATAPROC                         );
            self.glCopyImageSubData                            = load!((load)(userptr, c"glCopyImageSubData"                             .as_ptr()) as PFNGLCOPYIMAGESUBDATAPROC                           );
            self.glDebugMessageCallback                        = load!((load)(userptr, c"glDebugMessageCallback"                         .as_ptr()) as PFNGLDEBUGMESSAGECALLBACKPROC                       );
            self.glDebugMessageControl                         = load!((load)(userptr, c"glDebugMessageControl"                          .as_ptr()) as PFNGLDEBUGMESSAGECONTROLPROC                        );
            self.glDebugMessageInsert                          = load!((load)(userptr, c"glDebugMessageInsert"                           .as_ptr()) as PFNGLDEBUGMESSAGEINSERTPROC                         );
            self.glDispatchCompute                             = load!((load)(userptr, c"glDispatchCompute"                              .as_ptr()) as PFNGLDISPATCHCOMPUTEPROC                            );
            self.glDispatchComputeIndirect                     = load!((load)(userptr, c"glDispatchComputeIndirect"                      .as_ptr()) as PFNGLDISPATCHCOMPUTEINDIRECTPROC                    );
            self.glFramebufferParameteri                       = load!((load)(userptr, c"glFramebufferParameteri"                        .as_ptr()) as PFNGLFRAMEBUFFERPARAMETERIPROC                      );
            self.glGetDebugMessageLog                          = load!((load)(userptr, c"glGetDebugMessageLog"                           .as_ptr()) as PFNGLGETDEBUGMESSAGELOGPROC                         );
            self.glGetFramebufferParameteriv                   = load!((load)(userptr, c"glGetFramebufferParameteriv"                    .as_ptr()) as PFNGLGETFRAMEBUFFERPARAMETERIVPROC                  );
            self.glGetInternalformati64v                       = load!((load)(userptr, c"glGetInternalformati64v"                        .as_ptr()) as PFNGLGETINTERNALFORMATI64VPROC                      );
            self.glGetObjectLabel                              = load!((load)(userptr, c"glGetObjectLabel"                               .as_ptr()) as PFNGLGETOBJECTLABELPROC                             );
            self.glGetObjectPtrLabel                           = load!((load)(userptr, c"glGetObjectPtrLabel"                            .as_ptr()) as PFNGLGETOBJECTPTRLABELPROC                          );
            self.glGetPointerv                                 = load!((load)(userptr, c"glGetPointerv"                                  .as_ptr()) as PFNGLGETPOINTERVPROC                                );
            self.glGetProgramInterfaceiv                       = load!((load)(userptr, c"glGetProgramInterfaceiv"                        .as_ptr()) as PFNGLGETPROGRAMINTERFACEIVPROC                      );
            self.glGetProgramResourceIndex                     = load!((load)(userptr, c"glGetProgramResourceIndex"                      .as_ptr()) as PFNGLGETPROGRAMRESOURCEINDEXPROC                    );
            self.glGetProgramResourceLocation                  = load!((load)(userptr, c"glGetProgramResourceLocation"                   .as_ptr()) as PFNGLGETPROGRAMRESOURCELOCATIONPROC                 );
            self.glGetProgramResourceLocationIndex             = load!((load)(userptr, c"glGetProgramResourceLocationIndex"              .as_ptr()) as PFNGLGETPROGRAMRESOURCELOCATIONINDEXPROC            );
            self.glGetProgramResourceName                      = load!((load)(userptr, c"glGetProgramResourceName"                       .as_ptr()) as PFNGLGETPROGRAMRESOURCENAMEPROC                     );
            self.glGetProgramResourceiv                        = load!((load)(userptr, c"glGetProgramResourceiv"                         .as_ptr()) as PFNGLGETPROGRAMRESOURCEIVPROC                       );
            self.glInvalidateBufferData                        = load!((load)(userptr, c"glInvalidateBufferData"                         .as_ptr()) as PFNGLINVALIDATEBUFFERDATAPROC                       );
            self.glInvalidateBufferSubData                     = load!((load)(userptr, c"glInvalidateBufferSubData"                      .as_ptr()) as PFNGLINVALIDATEBUFFERSUBDATAPROC                    );
            self.glInvalidateFramebuffer                       = load!((load)(userptr, c"glInvalidateFramebuffer"                        .as_ptr()) as PFNGLINVALIDATEFRAMEBUFFERPROC                      );
            self.glInvalidateSubFramebuffer                    = load!((load)(userptr, c"glInvalidateSubFramebuffer"                     .as_ptr()) as PFNGLINVALIDATESUBFRAMEBUFFERPROC                   );
            self.glInvalidateTexImage                          = load!((load)(userptr, c"glInvalidateTexImage"                           .as_ptr()) as PFNGLINVALIDATETEXIMAGEPROC                         );
            self.glInvalidateTexSubImage                       = load!((load)(userptr, c"glInvalidateTexSubImage"                        .as_ptr()) as PFNGLINVALIDATETEXSUBIMAGEPROC                      );
            self.glMultiDrawArraysIndirect                     = load!((load)(userptr, c"glMultiDrawArraysIndirect"                      .as_ptr()) as PFNGLMULTIDRAWARRAYSINDIRECTPROC                    );
            self.glMultiDrawElementsIndirect                   = load!((load)(userptr, c"glMultiDrawElementsIndirect"                    .as_ptr()) as PFNGLMULTIDRAWELEMENTSINDIRECTPROC                  );
            self.glObjectLabel                                 = load!((load)(userptr, c"glObjectLabel"                                  .as_ptr()) as PFNGLOBJECTLABELPROC                                );
            self.glObjectPtrLabel                              = load!((load)(userptr, c"glObjectPtrLabel"                               .as_ptr()) as PFNGLOBJECTPTRLABELPROC                             );
            self.glPopDebugGroup                               = load!((load)(userptr, c"glPopDebugGroup"                                .as_ptr()) as PFNGLPOPDEBUGGROUPPROC                              );
            self.glPushDebugGroup                              = load!((load)(userptr, c"glPushDebugGroup"                               .as_ptr()) as PFNGLPUSHDEBUGGROUPPROC                             );
            self.glShaderStorageBlockBinding                   = load!((load)(userptr, c"glShaderStorageBlockBinding"                    .as_ptr()) as PFNGLSHADERSTORAGEBLOCKBINDINGPROC                  );
            self.glTexBufferRange                              = load!((load)(userptr, c"glTexBufferRange"                               .as_ptr()) as PFNGLTEXBUFFERRANGEPROC                             );
            self.glTexStorage2DMultisample                     = load!((load)(userptr, c"glTexStorage2DMultisample"                      .as_ptr()) as PFNGLTEXSTORAGE2DMULTISAMPLEPROC                    );
            self.glTexStorage3DMultisample                     = load!((load)(userptr, c"glTexStorage3DMultisample"                      .as_ptr()) as PFNGLTEXSTORAGE3DMULTISAMPLEPROC                    );
            self.glTextureView                                 = load!((load)(userptr, c"glTextureView"                                  .as_ptr()) as PFNGLTEXTUREVIEWPROC                                );
            self.glVertexAttribBinding                         = load!((load)(userptr, c"glVertexAttribBinding"                          .as_ptr()) as PFNGLVERTEXATTRIBBINDINGPROC                        );
            self.glVertexAttribFormat                          = load!((load)(userptr, c"glVertexAttribFormat"                           .as_ptr()) as PFNGLVERTEXATTRIBFORMATPROC                         );
            self.glVertexAttribIFormat                         = load!((load)(userptr, c"glVertexAttribIFormat"                          .as_ptr()) as PFNGLVERTEXATTRIBIFORMATPROC                        );
            self.glVertexAttribLFormat                         = load!((load)(userptr, c"glVertexAttribLFormat"                          .as_ptr()) as PFNGLVERTEXATTRIBLFORMATPROC                        );
            self.glVertexBindingDivisor                        = load!((load)(userptr, c"glVertexBindingDivisor"                         .as_ptr()) as PFNGLVERTEXBINDINGDIVISORPROC                       );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_ES2_compatibility(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_ES2_compatibility { return; }
            self.glClearDepthf                                 = load!((load)(userptr, c"glClearDepthf"                                  .as_ptr()) as PFNGLCLEARDEPTHFPROC                                );
            self.glDepthRangef                                 = load!((load)(userptr, c"glDepthRangef"                                  .as_ptr()) as PFNGLDEPTHRANGEFPROC                                );
            self.glGetShaderPrecisionFormat                    = load!((load)(userptr, c"glGetShaderPrecisionFormat"                     .as_ptr()) as PFNGLGETSHADERPRECISIONFORMATPROC                   );
            self.glReleaseShaderCompiler                       = load!((load)(userptr, c"glReleaseShaderCompiler"                        .as_ptr()) as PFNGLRELEASESHADERCOMPILERPROC                      );
            self.glShaderBinary                                = load!((load)(userptr, c"glShaderBinary"                                 .as_ptr()) as PFNGLSHADERBINARYPROC                               );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_ES3_1_compatibility(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_ES3_1_compatibility { return; }
            self.glMemoryBarrierByRegion                       = load!((load)(userptr, c"glMemoryBarrierByRegion"                        .as_ptr()) as PFNGLMEMORYBARRIERBYREGIONPROC                      );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_ES3_2_compatibility(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_ES3_2_compatibility { return; }
            self.glPrimitiveBoundingBoxARB                     = load!((load)(userptr, c"glPrimitiveBoundingBoxARB"                      .as_ptr()) as PFNGLPRIMITIVEBOUNDINGBOXARBPROC                    );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_blend_func_extended(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_blend_func_extended { return; }
            self.glBindFragDataLocationIndexed                 = load!((load)(userptr, c"glBindFragDataLocationIndexed"                  .as_ptr()) as PFNGLBINDFRAGDATALOCATIONINDEXEDPROC                );
            self.glGetFragDataIndex                            = load!((load)(userptr, c"glGetFragDataIndex"                             .as_ptr()) as PFNGLGETFRAGDATAINDEXPROC                           );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_buffer_storage(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_buffer_storage { return; }
            self.glBufferStorage                               = load!((load)(userptr, c"glBufferStorage"                                .as_ptr()) as PFNGLBUFFERSTORAGEPROC                              );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_clear_buffer_object(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_clear_buffer_object { return; }
            self.glClearBufferData                             = load!((load)(userptr, c"glClearBufferData"                              .as_ptr()) as PFNGLCLEARBUFFERDATAPROC                            );
            self.glClearBufferSubData                          = load!((load)(userptr, c"glClearBufferSubData"                           .as_ptr()) as PFNGLCLEARBUFFERSUBDATAPROC                         );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_clear_texture(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_clear_texture { return; }
            self.glClearTexImage                               = load!((load)(userptr, c"glClearTexImage"                                .as_ptr()) as PFNGLCLEARTEXIMAGEPROC                              );
            self.glClearTexSubImage                            = load!((load)(userptr, c"glClearTexSubImage"                             .as_ptr()) as PFNGLCLEARTEXSUBIMAGEPROC                           );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_color_buffer_float(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_color_buffer_float { return; }
            self.glClampColorARB                               = load!((load)(userptr, c"glClampColorARB"                                .as_ptr()) as PFNGLCLAMPCOLORARBPROC                              );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_compute_shader(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_compute_shader { return; }
            self.glDispatchCompute                             = load!((load)(userptr, c"glDispatchCompute"                              .as_ptr()) as PFNGLDISPATCHCOMPUTEPROC                            );
            self.glDispatchComputeIndirect                     = load!((load)(userptr, c"glDispatchComputeIndirect"                      .as_ptr()) as PFNGLDISPATCHCOMPUTEINDIRECTPROC                    );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_compute_variable_group_size(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_compute_variable_group_size { return; }
            self.glDispatchComputeGroupSizeARB                 = load!((load)(userptr, c"glDispatchComputeGroupSizeARB"                  .as_ptr()) as PFNGLDISPATCHCOMPUTEGROUPSIZEARBPROC                );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_copy_buffer(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_copy_buffer { return; }
            self.glCopyBufferSubData                           = load!((load)(userptr, c"glCopyBufferSubData"                            .as_ptr()) as PFNGLCOPYBUFFERSUBDATAPROC                          );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_copy_image(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_copy_image { return; }
            self.glCopyImageSubData                            = load!((load)(userptr, c"glCopyImageSubData"                             .as_ptr()) as PFNGLCOPYIMAGESUBDATAPROC                           );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_debug_output(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_debug_output { return; }
            self.glDebugMessageCallbackARB                     = load!((load)(userptr, c"glDebugMessageCallbackARB"                      .as_ptr()) as PFNGLDEBUGMESSAGECALLBACKARBPROC                    );
            self.glDebugMessageControlARB                      = load!((load)(userptr, c"glDebugMessageControlARB"                       .as_ptr()) as PFNGLDEBUGMESSAGECONTROLARBPROC                     );
            self.glDebugMessageInsertARB                       = load!((load)(userptr, c"glDebugMessageInsertARB"                        .as_ptr()) as PFNGLDEBUGMESSAGEINSERTARBPROC                      );
            self.glGetDebugMessageLogARB                       = load!((load)(userptr, c"glGetDebugMessageLogARB"                        .as_ptr()) as PFNGLGETDEBUGMESSAGELOGARBPROC                      );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_direct_state_access(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_direct_state_access { return; }
            self.glBindTextureUnit                             = load!((load)(userptr, c"glBindTextureUnit"                              .as_ptr()) as PFNGLBINDTEXTUREUNITPROC                            );
            self.glBlitNamedFramebuffer                        = load!((load)(userptr, c"glBlitNamedFramebuffer"                         .as_ptr()) as PFNGLBLITNAMEDFRAMEBUFFERPROC                       );
            self.glCheckNamedFramebufferStatus                 = load!((load)(userptr, c"glCheckNamedFramebufferStatus"                  .as_ptr()) as PFNGLCHECKNAMEDFRAMEBUFFERSTATUSPROC                );
            self.glClearNamedBufferData                        = load!((load)(userptr, c"glClearNamedBufferData"                         .as_ptr()) as PFNGLCLEARNAMEDBUFFERDATAPROC                       );
            self.glClearNamedBufferSubData                     = load!((load)(userptr, c"glClearNamedBufferSubData"                      .as_ptr()) as PFNGLCLEARNAMEDBUFFERSUBDATAPROC                    );
            self.glClearNamedFramebufferfi                     = load!((load)(userptr, c"glClearNamedFramebufferfi"                      .as_ptr()) as PFNGLCLEARNAMEDFRAMEBUFFERFIPROC                    );
            self.glClearNamedFramebufferfv                     = load!((load)(userptr, c"glClearNamedFramebufferfv"                      .as_ptr()) as PFNGLCLEARNAMEDFRAMEBUFFERFVPROC                    );
            self.glClearNamedFramebufferiv                     = load!((load)(userptr, c"glClearNamedFramebufferiv"                      .as_ptr()) as PFNGLCLEARNAMEDFRAMEBUFFERIVPROC                    );
            self.glClearNamedFramebufferuiv                    = load!((load)(userptr, c"glClearNamedFramebufferuiv"                     .as_ptr()) as PFNGLCLEARNAMEDFRAMEBUFFERUIVPROC                   );
            self.glCompressedTextureSubImage1D                 = load!((load)(userptr, c"glCompressedTextureSubImage1D"                  .as_ptr()) as PFNGLCOMPRESSEDTEXTURESUBIMAGE1DPROC                );
            self.glCompressedTextureSubImage2D                 = load!((load)(userptr, c"glCompressedTextureSubImage2D"                  .as_ptr()) as PFNGLCOMPRESSEDTEXTURESUBIMAGE2DPROC                );
            self.glCompressedTextureSubImage3D                 = load!((load)(userptr, c"glCompressedTextureSubImage3D"                  .as_ptr()) as PFNGLCOMPRESSEDTEXTURESUBIMAGE3DPROC                );
            self.glCopyNamedBufferSubData                      = load!((load)(userptr, c"glCopyNamedBufferSubData"                       .as_ptr()) as PFNGLCOPYNAMEDBUFFERSUBDATAPROC                     );
            self.glCopyTextureSubImage1D                       = load!((load)(userptr, c"glCopyTextureSubImage1D"                        .as_ptr()) as PFNGLCOPYTEXTURESUBIMAGE1DPROC                      );
            self.glCopyTextureSubImage2D                       = load!((load)(userptr, c"glCopyTextureSubImage2D"                        .as_ptr()) as PFNGLCOPYTEXTURESUBIMAGE2DPROC                      );
            self.glCopyTextureSubImage3D                       = load!((load)(userptr, c"glCopyTextureSubImage3D"                        .as_ptr()) as PFNGLCOPYTEXTURESUBIMAGE3DPROC                      );
            self.glCreateBuffers                               = load!((load)(userptr, c"glCreateBuffers"                                .as_ptr()) as PFNGLCREATEBUFFERSPROC                              );
            self.glCreateFramebuffers                          = load!((load)(userptr, c"glCreateFramebuffers"                           .as_ptr()) as PFNGLCREATEFRAMEBUFFERSPROC                         );
            self.glCreateProgramPipelines                      = load!((load)(userptr, c"glCreateProgramPipelines"                       .as_ptr()) as PFNGLCREATEPROGRAMPIPELINESPROC                     );
            self.glCreateQueries                               = load!((load)(userptr, c"glCreateQueries"                                .as_ptr()) as PFNGLCREATEQUERIESPROC                              );
            self.glCreateRenderbuffers                         = load!((load)(userptr, c"glCreateRenderbuffers"                          .as_ptr()) as PFNGLCREATERENDERBUFFERSPROC                        );
            self.glCreateSamplers                              = load!((load)(userptr, c"glCreateSamplers"                               .as_ptr()) as PFNGLCREATESAMPLERSPROC                             );
            self.glCreateTextures                              = load!((load)(userptr, c"glCreateTextures"                               .as_ptr()) as PFNGLCREATETEXTURESPROC                             );
            self.glCreateTransformFeedbacks                    = load!((load)(userptr, c"glCreateTransformFeedbacks"                     .as_ptr()) as PFNGLCREATETRANSFORMFEEDBACKSPROC                   );
            self.glCreateVertexArrays                          = load!((load)(userptr, c"glCreateVertexArrays"                           .as_ptr()) as PFNGLCREATEVERTEXARRAYSPROC                         );
            self.glDisableVertexArrayAttrib                    = load!((load)(userptr, c"glDisableVertexArrayAttrib"                     .as_ptr()) as PFNGLDISABLEVERTEXARRAYATTRIBPROC                   );
            self.glEnableVertexArrayAttrib                     = load!((load)(userptr, c"glEnableVertexArrayAttrib"                      .as_ptr()) as PFNGLENABLEVERTEXARRAYATTRIBPROC                    );
            self.glFlushMappedNamedBufferRange                 = load!((load)(userptr, c"glFlushMappedNamedBufferRange"                  .as_ptr()) as PFNGLFLUSHMAPPEDNAMEDBUFFERRANGEPROC                );
            self.glGenerateTextureMipmap                       = load!((load)(userptr, c"glGenerateTextureMipmap"                        .as_ptr()) as PFNGLGENERATETEXTUREMIPMAPPROC                      );
            self.glGetCompressedTextureImage                   = load!((load)(userptr, c"glGetCompressedTextureImage"                    .as_ptr()) as PFNGLGETCOMPRESSEDTEXTUREIMAGEPROC                  );
            self.glGetNamedBufferParameteri64v                 = load!((load)(userptr, c"glGetNamedBufferParameteri64v"                  .as_ptr()) as PFNGLGETNAMEDBUFFERPARAMETERI64VPROC                );
            self.glGetNamedBufferParameteriv                   = load!((load)(userptr, c"glGetNamedBufferParameteriv"                    .as_ptr()) as PFNGLGETNAMEDBUFFERPARAMETERIVPROC                  );
            self.glGetNamedBufferPointerv                      = load!((load)(userptr, c"glGetNamedBufferPointerv"                       .as_ptr()) as PFNGLGETNAMEDBUFFERPOINTERVPROC                     );
            self.glGetNamedBufferSubData                       = load!((load)(userptr, c"glGetNamedBufferSubData"                        .as_ptr()) as PFNGLGETNAMEDBUFFERSUBDATAPROC                      );
            self.glGetNamedFramebufferAttachmentParameteriv    = load!((load)(userptr, c"glGetNamedFramebufferAttachmentParameteriv"     .as_ptr()) as PFNGLGETNAMEDFRAMEBUFFERATTACHMENTPARAMETERIVPROC   );
            self.glGetNamedFramebufferParameteriv              = load!((load)(userptr, c"glGetNamedFramebufferParameteriv"               .as_ptr()) as PFNGLGETNAMEDFRAMEBUFFERPARAMETERIVPROC             );
            self.glGetNamedRenderbufferParameteriv             = load!((load)(userptr, c"glGetNamedRenderbufferParameteriv"              .as_ptr()) as PFNGLGETNAMEDRENDERBUFFERPARAMETERIVPROC            );
            self.glGetQueryBufferObjecti64v                    = load!((load)(userptr, c"glGetQueryBufferObjecti64v"                     .as_ptr()) as PFNGLGETQUERYBUFFEROBJECTI64VPROC                   );
            self.glGetQueryBufferObjectiv                      = load!((load)(userptr, c"glGetQueryBufferObjectiv"                       .as_ptr()) as PFNGLGETQUERYBUFFEROBJECTIVPROC                     );
            self.glGetQueryBufferObjectui64v                   = load!((load)(userptr, c"glGetQueryBufferObjectui64v"                    .as_ptr()) as PFNGLGETQUERYBUFFEROBJECTUI64VPROC                  );
            self.glGetQueryBufferObjectuiv                     = load!((load)(userptr, c"glGetQueryBufferObjectuiv"                      .as_ptr()) as PFNGLGETQUERYBUFFEROBJECTUIVPROC                    );
            self.glGetTextureImage                             = load!((load)(userptr, c"glGetTextureImage"                              .as_ptr()) as PFNGLGETTEXTUREIMAGEPROC                            );
            self.glGetTextureLevelParameterfv                  = load!((load)(userptr, c"glGetTextureLevelParameterfv"                   .as_ptr()) as PFNGLGETTEXTURELEVELPARAMETERFVPROC                 );
            self.glGetTextureLevelParameteriv                  = load!((load)(userptr, c"glGetTextureLevelParameteriv"                   .as_ptr()) as PFNGLGETTEXTURELEVELPARAMETERIVPROC                 );
            self.glGetTextureParameterIiv                      = load!((load)(userptr, c"glGetTextureParameterIiv"                       .as_ptr()) as PFNGLGETTEXTUREPARAMETERIIVPROC                     );
            self.glGetTextureParameterIuiv                     = load!((load)(userptr, c"glGetTextureParameterIuiv"                      .as_ptr()) as PFNGLGETTEXTUREPARAMETERIUIVPROC                    );
            self.glGetTextureParameterfv                       = load!((load)(userptr, c"glGetTextureParameterfv"                        .as_ptr()) as PFNGLGETTEXTUREPARAMETERFVPROC                      );
            self.glGetTextureParameteriv                       = load!((load)(userptr, c"glGetTextureParameteriv"                        .as_ptr()) as PFNGLGETTEXTUREPARAMETERIVPROC                      );
            self.glGetTransformFeedbacki64_v                   = load!((load)(userptr, c"glGetTransformFeedbacki64_v"                    .as_ptr()) as PFNGLGETTRANSFORMFEEDBACKI64_VPROC                  );
            self.glGetTransformFeedbacki_v                     = load!((load)(userptr, c"glGetTransformFeedbacki_v"                      .as_ptr()) as PFNGLGETTRANSFORMFEEDBACKI_VPROC                    );
            self.glGetTransformFeedbackiv                      = load!((load)(userptr, c"glGetTransformFeedbackiv"                       .as_ptr()) as PFNGLGETTRANSFORMFEEDBACKIVPROC                     );
            self.glGetVertexArrayIndexed64iv                   = load!((load)(userptr, c"glGetVertexArrayIndexed64iv"                    .as_ptr()) as PFNGLGETVERTEXARRAYINDEXED64IVPROC                  );
            self.glGetVertexArrayIndexediv                     = load!((load)(userptr, c"glGetVertexArrayIndexediv"                      .as_ptr()) as PFNGLGETVERTEXARRAYINDEXEDIVPROC                    );
            self.glGetVertexArrayiv                            = load!((load)(userptr, c"glGetVertexArrayiv"                             .as_ptr()) as PFNGLGETVERTEXARRAYIVPROC                           );
            self.glInvalidateNamedFramebufferData              = load!((load)(userptr, c"glInvalidateNamedFramebufferData"               .as_ptr()) as PFNGLINVALIDATENAMEDFRAMEBUFFERDATAPROC             );
            self.glInvalidateNamedFramebufferSubData           = load!((load)(userptr, c"glInvalidateNamedFramebufferSubData"            .as_ptr()) as PFNGLINVALIDATENAMEDFRAMEBUFFERSUBDATAPROC          );
            self.glMapNamedBuffer                              = load!((load)(userptr, c"glMapNamedBuffer"                               .as_ptr()) as PFNGLMAPNAMEDBUFFERPROC                             );
            self.glMapNamedBufferRange                         = load!((load)(userptr, c"glMapNamedBufferRange"                          .as_ptr()) as PFNGLMAPNAMEDBUFFERRANGEPROC                        );
            self.glNamedBufferData                             = load!((load)(userptr, c"glNamedBufferData"                              .as_ptr()) as PFNGLNAMEDBUFFERDATAPROC                            );
            self.glNamedBufferStorage                          = load!((load)(userptr, c"glNamedBufferStorage"                           .as_ptr()) as PFNGLNAMEDBUFFERSTORAGEPROC                         );
            self.glNamedBufferSubData                          = load!((load)(userptr, c"glNamedBufferSubData"                           .as_ptr()) as PFNGLNAMEDBUFFERSUBDATAPROC                         );
            self.glNamedFramebufferDrawBuffer                  = load!((load)(userptr, c"glNamedFramebufferDrawBuffer"                   .as_ptr()) as PFNGLNAMEDFRAMEBUFFERDRAWBUFFERPROC                 );
            self.glNamedFramebufferDrawBuffers                 = load!((load)(userptr, c"glNamedFramebufferDrawBuffers"                  .as_ptr()) as PFNGLNAMEDFRAMEBUFFERDRAWBUFFERSPROC                );
            self.glNamedFramebufferParameteri                  = load!((load)(userptr, c"glNamedFramebufferParameteri"                   .as_ptr()) as PFNGLNAMEDFRAMEBUFFERPARAMETERIPROC                 );
            self.glNamedFramebufferReadBuffer                  = load!((load)(userptr, c"glNamedFramebufferReadBuffer"                   .as_ptr()) as PFNGLNAMEDFRAMEBUFFERREADBUFFERPROC                 );
            self.glNamedFramebufferRenderbuffer                = load!((load)(userptr, c"glNamedFramebufferRenderbuffer"                 .as_ptr()) as PFNGLNAMEDFRAMEBUFFERRENDERBUFFERPROC               );
            self.glNamedFramebufferTexture                     = load!((load)(userptr, c"glNamedFramebufferTexture"                      .as_ptr()) as PFNGLNAMEDFRAMEBUFFERTEXTUREPROC                    );
            self.glNamedFramebufferTextureLayer                = load!((load)(userptr, c"glNamedFramebufferTextureLayer"                 .as_ptr()) as PFNGLNAMEDFRAMEBUFFERTEXTURELAYERPROC               );
            self.glNamedRenderbufferStorage                    = load!((load)(userptr, c"glNamedRenderbufferStorage"                     .as_ptr()) as PFNGLNAMEDRENDERBUFFERSTORAGEPROC                   );
            self.glNamedRenderbufferStorageMultisample         = load!((load)(userptr, c"glNamedRenderbufferStorageMultisample"          .as_ptr()) as PFNGLNAMEDRENDERBUFFERSTORAGEMULTISAMPLEPROC        );
            self.glTextureBuffer                               = load!((load)(userptr, c"glTextureBuffer"                                .as_ptr()) as PFNGLTEXTUREBUFFERPROC                              );
            self.glTextureBufferRange                          = load!((load)(userptr, c"glTextureBufferRange"                           .as_ptr()) as PFNGLTEXTUREBUFFERRANGEPROC                         );
            self.glTextureParameterIiv                         = load!((load)(userptr, c"glTextureParameterIiv"                          .as_ptr()) as PFNGLTEXTUREPARAMETERIIVPROC                        );
            self.glTextureParameterIuiv                        = load!((load)(userptr, c"glTextureParameterIuiv"                         .as_ptr()) as PFNGLTEXTUREPARAMETERIUIVPROC                       );
            self.glTextureParameterf                           = load!((load)(userptr, c"glTextureParameterf"                            .as_ptr()) as PFNGLTEXTUREPARAMETERFPROC                          );
            self.glTextureParameterfv                          = load!((load)(userptr, c"glTextureParameterfv"                           .as_ptr()) as PFNGLTEXTUREPARAMETERFVPROC                         );
            self.glTextureParameteri                           = load!((load)(userptr, c"glTextureParameteri"                            .as_ptr()) as PFNGLTEXTUREPARAMETERIPROC                          );
            self.glTextureParameteriv                          = load!((load)(userptr, c"glTextureParameteriv"                           .as_ptr()) as PFNGLTEXTUREPARAMETERIVPROC                         );
            self.glTextureStorage1D                            = load!((load)(userptr, c"glTextureStorage1D"                             .as_ptr()) as PFNGLTEXTURESTORAGE1DPROC                           );
            self.glTextureStorage2D                            = load!((load)(userptr, c"glTextureStorage2D"                             .as_ptr()) as PFNGLTEXTURESTORAGE2DPROC                           );
            self.glTextureStorage2DMultisample                 = load!((load)(userptr, c"glTextureStorage2DMultisample"                  .as_ptr()) as PFNGLTEXTURESTORAGE2DMULTISAMPLEPROC                );
            self.glTextureStorage3D                            = load!((load)(userptr, c"glTextureStorage3D"                             .as_ptr()) as PFNGLTEXTURESTORAGE3DPROC                           );
            self.glTextureStorage3DMultisample                 = load!((load)(userptr, c"glTextureStorage3DMultisample"                  .as_ptr()) as PFNGLTEXTURESTORAGE3DMULTISAMPLEPROC                );
            self.glTextureSubImage1D                           = load!((load)(userptr, c"glTextureSubImage1D"                            .as_ptr()) as PFNGLTEXTURESUBIMAGE1DPROC                          );
            self.glTextureSubImage2D                           = load!((load)(userptr, c"glTextureSubImage2D"                            .as_ptr()) as PFNGLTEXTURESUBIMAGE2DPROC                          );
            self.glTextureSubImage3D                           = load!((load)(userptr, c"glTextureSubImage3D"                            .as_ptr()) as PFNGLTEXTURESUBIMAGE3DPROC                          );
            self.glTransformFeedbackBufferBase                 = load!((load)(userptr, c"glTransformFeedbackBufferBase"                  .as_ptr()) as PFNGLTRANSFORMFEEDBACKBUFFERBASEPROC                );
            self.glTransformFeedbackBufferRange                = load!((load)(userptr, c"glTransformFeedbackBufferRange"                 .as_ptr()) as PFNGLTRANSFORMFEEDBACKBUFFERRANGEPROC               );
            self.glUnmapNamedBuffer                            = load!((load)(userptr, c"glUnmapNamedBuffer"                             .as_ptr()) as PFNGLUNMAPNAMEDBUFFERPROC                           );
            self.glVertexArrayAttribBinding                    = load!((load)(userptr, c"glVertexArrayAttribBinding"                     .as_ptr()) as PFNGLVERTEXARRAYATTRIBBINDINGPROC                   );
            self.glVertexArrayAttribFormat                     = load!((load)(userptr, c"glVertexArrayAttribFormat"                      .as_ptr()) as PFNGLVERTEXARRAYATTRIBFORMATPROC                    );
            self.glVertexArrayAttribIFormat                    = load!((load)(userptr, c"glVertexArrayAttribIFormat"                     .as_ptr()) as PFNGLVERTEXARRAYATTRIBIFORMATPROC                   );
            self.glVertexArrayAttribLFormat                    = load!((load)(userptr, c"glVertexArrayAttribLFormat"                     .as_ptr()) as PFNGLVERTEXARRAYATTRIBLFORMATPROC                   );
            self.glVertexArrayBindingDivisor                   = load!((load)(userptr, c"glVertexArrayBindingDivisor"                    .as_ptr()) as PFNGLVERTEXARRAYBINDINGDIVISORPROC                  );
            self.glVertexArrayElementBuffer                    = load!((load)(userptr, c"glVertexArrayElementBuffer"                     .as_ptr()) as PFNGLVERTEXARRAYELEMENTBUFFERPROC                   );
            self.glVertexArrayVertexBuffer                     = load!((load)(userptr, c"glVertexArrayVertexBuffer"                      .as_ptr()) as PFNGLVERTEXARRAYVERTEXBUFFERPROC                    );
            self.glVertexArrayVertexBuffers                    = load!((load)(userptr, c"glVertexArrayVertexBuffers"                     .as_ptr()) as PFNGLVERTEXARRAYVERTEXBUFFERSPROC                   );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_draw_buffers(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_draw_buffers { return; }
            self.glDrawBuffersARB                              = load!((load)(userptr, c"glDrawBuffersARB"                               .as_ptr()) as PFNGLDRAWBUFFERSARBPROC                             );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_draw_buffers_blend(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_draw_buffers_blend { return; }
            self.glBlendEquationSeparateiARB                   = load!((load)(userptr, c"glBlendEquationSeparateiARB"                    .as_ptr()) as PFNGLBLENDEQUATIONSEPARATEIARBPROC                  );
            self.glBlendEquationiARB                           = load!((load)(userptr, c"glBlendEquationiARB"                            .as_ptr()) as PFNGLBLENDEQUATIONIARBPROC                          );
            self.glBlendFuncSeparateiARB                       = load!((load)(userptr, c"glBlendFuncSeparateiARB"                        .as_ptr()) as PFNGLBLENDFUNCSEPARATEIARBPROC                      );
            self.glBlendFunciARB                               = load!((load)(userptr, c"glBlendFunciARB"                                .as_ptr()) as PFNGLBLENDFUNCIARBPROC                              );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_draw_elements_base_vertex(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_draw_elements_base_vertex { return; }
            self.glDrawElementsBaseVertex                      = load!((load)(userptr, c"glDrawElementsBaseVertex"                       .as_ptr()) as PFNGLDRAWELEMENTSBASEVERTEXPROC                     );
            self.glDrawElementsInstancedBaseVertex             = load!((load)(userptr, c"glDrawElementsInstancedBaseVertex"              .as_ptr()) as PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC            );
            self.glDrawRangeElementsBaseVertex                 = load!((load)(userptr, c"glDrawRangeElementsBaseVertex"                  .as_ptr()) as PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC                );
            self.glMultiDrawElementsBaseVertex                 = load!((load)(userptr, c"glMultiDrawElementsBaseVertex"                  .as_ptr()) as PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC                );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_draw_indirect(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_draw_indirect { return; }
            self.glDrawArraysIndirect                          = load!((load)(userptr, c"glDrawArraysIndirect"                           .as_ptr()) as PFNGLDRAWARRAYSINDIRECTPROC                         );
            self.glDrawElementsIndirect                        = load!((load)(userptr, c"glDrawElementsIndirect"                         .as_ptr()) as PFNGLDRAWELEMENTSINDIRECTPROC                       );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_draw_instanced(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_draw_instanced { return; }
            self.glDrawArraysInstancedARB                      = load!((load)(userptr, c"glDrawArraysInstancedARB"                       .as_ptr()) as PFNGLDRAWARRAYSINSTANCEDARBPROC                     );
            self.glDrawElementsInstancedARB                    = load!((load)(userptr, c"glDrawElementsInstancedARB"                     .as_ptr()) as PFNGLDRAWELEMENTSINSTANCEDARBPROC                   );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_fragment_program(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_fragment_program { return; }
            self.glBindProgramARB                              = load!((load)(userptr, c"glBindProgramARB"                               .as_ptr()) as PFNGLBINDPROGRAMARBPROC                             );
            self.glDeleteProgramsARB                           = load!((load)(userptr, c"glDeleteProgramsARB"                            .as_ptr()) as PFNGLDELETEPROGRAMSARBPROC                          );
            self.glGenProgramsARB                              = load!((load)(userptr, c"glGenProgramsARB"                               .as_ptr()) as PFNGLGENPROGRAMSARBPROC                             );
            self.glGetProgramEnvParameterdvARB                 = load!((load)(userptr, c"glGetProgramEnvParameterdvARB"                  .as_ptr()) as PFNGLGETPROGRAMENVPARAMETERDVARBPROC                );
            self.glGetProgramEnvParameterfvARB                 = load!((load)(userptr, c"glGetProgramEnvParameterfvARB"                  .as_ptr()) as PFNGLGETPROGRAMENVPARAMETERFVARBPROC                );
            self.glGetProgramLocalParameterdvARB               = load!((load)(userptr, c"glGetProgramLocalParameterdvARB"                .as_ptr()) as PFNGLGETPROGRAMLOCALPARAMETERDVARBPROC              );
            self.glGetProgramLocalParameterfvARB               = load!((load)(userptr, c"glGetProgramLocalParameterfvARB"                .as_ptr()) as PFNGLGETPROGRAMLOCALPARAMETERFVARBPROC              );
            self.glGetProgramStringARB                         = load!((load)(userptr, c"glGetProgramStringARB"                          .as_ptr()) as PFNGLGETPROGRAMSTRINGARBPROC                        );
            self.glGetProgramivARB                             = load!((load)(userptr, c"glGetProgramivARB"                              .as_ptr()) as PFNGLGETPROGRAMIVARBPROC                            );
            self.glIsProgramARB                                = load!((load)(userptr, c"glIsProgramARB"                                 .as_ptr()) as PFNGLISPROGRAMARBPROC                               );
            self.glProgramEnvParameter4dARB                    = load!((load)(userptr, c"glProgramEnvParameter4dARB"                     .as_ptr()) as PFNGLPROGRAMENVPARAMETER4DARBPROC                   );
            self.glProgramEnvParameter4dvARB                   = load!((load)(userptr, c"glProgramEnvParameter4dvARB"                    .as_ptr()) as PFNGLPROGRAMENVPARAMETER4DVARBPROC                  );
            self.glProgramEnvParameter4fARB                    = load!((load)(userptr, c"glProgramEnvParameter4fARB"                     .as_ptr()) as PFNGLPROGRAMENVPARAMETER4FARBPROC                   );
            self.glProgramEnvParameter4fvARB                   = load!((load)(userptr, c"glProgramEnvParameter4fvARB"                    .as_ptr()) as PFNGLPROGRAMENVPARAMETER4FVARBPROC                  );
            self.glProgramLocalParameter4dARB                  = load!((load)(userptr, c"glProgramLocalParameter4dARB"                   .as_ptr()) as PFNGLPROGRAMLOCALPARAMETER4DARBPROC                 );
            self.glProgramLocalParameter4dvARB                 = load!((load)(userptr, c"glProgramLocalParameter4dvARB"                  .as_ptr()) as PFNGLPROGRAMLOCALPARAMETER4DVARBPROC                );
            self.glProgramLocalParameter4fARB                  = load!((load)(userptr, c"glProgramLocalParameter4fARB"                   .as_ptr()) as PFNGLPROGRAMLOCALPARAMETER4FARBPROC                 );
            self.glProgramLocalParameter4fvARB                 = load!((load)(userptr, c"glProgramLocalParameter4fvARB"                  .as_ptr()) as PFNGLPROGRAMLOCALPARAMETER4FVARBPROC                );
            self.glProgramStringARB                            = load!((load)(userptr, c"glProgramStringARB"                             .as_ptr()) as PFNGLPROGRAMSTRINGARBPROC                           );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_framebuffer_no_attachments(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_framebuffer_no_attachments { return; }
            self.glFramebufferParameteri                       = load!((load)(userptr, c"glFramebufferParameteri"                        .as_ptr()) as PFNGLFRAMEBUFFERPARAMETERIPROC                      );
            self.glGetFramebufferParameteriv                   = load!((load)(userptr, c"glGetFramebufferParameteriv"                    .as_ptr()) as PFNGLGETFRAMEBUFFERPARAMETERIVPROC                  );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_framebuffer_object(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_framebuffer_object { return; }
            self.glBindFramebuffer                             = load!((load)(userptr, c"glBindFramebuffer"                              .as_ptr()) as PFNGLBINDFRAMEBUFFERPROC                            );
            self.glBindRenderbuffer                            = load!((load)(userptr, c"glBindRenderbuffer"                             .as_ptr()) as PFNGLBINDRENDERBUFFERPROC                           );
            self.glBlitFramebuffer                             = load!((load)(userptr, c"glBlitFramebuffer"                              .as_ptr()) as PFNGLBLITFRAMEBUFFERPROC                            );
            self.glCheckFramebufferStatus                      = load!((load)(userptr, c"glCheckFramebufferStatus"                       .as_ptr()) as PFNGLCHECKFRAMEBUFFERSTATUSPROC                     );
            self.glDeleteFramebuffers                          = load!((load)(userptr, c"glDeleteFramebuffers"                           .as_ptr()) as PFNGLDELETEFRAMEBUFFERSPROC                         );
            self.glDeleteRenderbuffers                         = load!((load)(userptr, c"glDeleteRenderbuffers"                          .as_ptr()) as PFNGLDELETERENDERBUFFERSPROC                        );
            self.glFramebufferRenderbuffer                     = load!((load)(userptr, c"glFramebufferRenderbuffer"                      .as_ptr()) as PFNGLFRAMEBUFFERRENDERBUFFERPROC                    );
            self.glFramebufferTexture1D                        = load!((load)(userptr, c"glFramebufferTexture1D"                         .as_ptr()) as PFNGLFRAMEBUFFERTEXTURE1DPROC                       );
            self.glFramebufferTexture2D                        = load!((load)(userptr, c"glFramebufferTexture2D"                         .as_ptr()) as PFNGLFRAMEBUFFERTEXTURE2DPROC                       );
            self.glFramebufferTexture3D                        = load!((load)(userptr, c"glFramebufferTexture3D"                         .as_ptr()) as PFNGLFRAMEBUFFERTEXTURE3DPROC                       );
            self.glFramebufferTextureLayer                     = load!((load)(userptr, c"glFramebufferTextureLayer"                      .as_ptr()) as PFNGLFRAMEBUFFERTEXTURELAYERPROC                    );
            self.glGenFramebuffers                             = load!((load)(userptr, c"glGenFramebuffers"                              .as_ptr()) as PFNGLGENFRAMEBUFFERSPROC                            );
            self.glGenRenderbuffers                            = load!((load)(userptr, c"glGenRenderbuffers"                             .as_ptr()) as PFNGLGENRENDERBUFFERSPROC                           );
            self.glGenerateMipmap                              = load!((load)(userptr, c"glGenerateMipmap"                               .as_ptr()) as PFNGLGENERATEMIPMAPPROC                             );
            self.glGetFramebufferAttachmentParameteriv         = load!((load)(userptr, c"glGetFramebufferAttachmentParameteriv"          .as_ptr()) as PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC        );
            self.glGetRenderbufferParameteriv                  = load!((load)(userptr, c"glGetRenderbufferParameteriv"                   .as_ptr()) as PFNGLGETRENDERBUFFERPARAMETERIVPROC                 );
            self.glIsFramebuffer                               = load!((load)(userptr, c"glIsFramebuffer"                                .as_ptr()) as PFNGLISFRAMEBUFFERPROC                              );
            self.glIsRenderbuffer                              = load!((load)(userptr, c"glIsRenderbuffer"                               .as_ptr()) as PFNGLISRENDERBUFFERPROC                             );
            self.glRenderbufferStorage                         = load!((load)(userptr, c"glRenderbufferStorage"                          .as_ptr()) as PFNGLRENDERBUFFERSTORAGEPROC                        );
            self.glRenderbufferStorageMultisample              = load!((load)(userptr, c"glRenderbufferStorageMultisample"               .as_ptr()) as PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC             );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_geometry_shader4(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_geometry_shader4 { return; }
            self.glFramebufferTextureARB                       = load!((load)(userptr, c"glFramebufferTextureARB"                        .as_ptr()) as PFNGLFRAMEBUFFERTEXTUREARBPROC                      );
            self.glFramebufferTextureFaceARB                   = load!((load)(userptr, c"glFramebufferTextureFaceARB"                    .as_ptr()) as PFNGLFRAMEBUFFERTEXTUREFACEARBPROC                  );
            self.glFramebufferTextureLayerARB                  = load!((load)(userptr, c"glFramebufferTextureLayerARB"                   .as_ptr()) as PFNGLFRAMEBUFFERTEXTURELAYERARBPROC                 );
            self.glProgramParameteriARB                        = load!((load)(userptr, c"glProgramParameteriARB"                         .as_ptr()) as PFNGLPROGRAMPARAMETERIARBPROC                       );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_get_program_binary(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_get_program_binary { return; }
            self.glGetProgramBinary                            = load!((load)(userptr, c"glGetProgramBinary"                             .as_ptr()) as PFNGLGETPROGRAMBINARYPROC                           );
            self.glProgramBinary                               = load!((load)(userptr, c"glProgramBinary"                                .as_ptr()) as PFNGLPROGRAMBINARYPROC                              );
            self.glProgramParameteri                           = load!((load)(userptr, c"glProgramParameteri"                            .as_ptr()) as PFNGLPROGRAMPARAMETERIPROC                          );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_get_texture_sub_image(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_get_texture_sub_image { return; }
            self.glGetCompressedTextureSubImage                = load!((load)(userptr, c"glGetCompressedTextureSubImage"                 .as_ptr()) as PFNGLGETCOMPRESSEDTEXTURESUBIMAGEPROC               );
            self.glGetTextureSubImage                          = load!((load)(userptr, c"glGetTextureSubImage"                           .as_ptr()) as PFNGLGETTEXTURESUBIMAGEPROC                         );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_gl_spirv(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_gl_spirv { return; }
            self.glSpecializeShaderARB                         = load!((load)(userptr, c"glSpecializeShaderARB"                          .as_ptr()) as PFNGLSPECIALIZESHADERARBPROC                        );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_gpu_shader_fp64(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_gpu_shader_fp64 { return; }
            self.glGetUniformdv                                = load!((load)(userptr, c"glGetUniformdv"                                 .as_ptr()) as PFNGLGETUNIFORMDVPROC                               );
            self.glUniform1d                                   = load!((load)(userptr, c"glUniform1d"                                    .as_ptr()) as PFNGLUNIFORM1DPROC                                  );
            self.glUniform1dv                                  = load!((load)(userptr, c"glUniform1dv"                                   .as_ptr()) as PFNGLUNIFORM1DVPROC                                 );
            self.glUniform2d                                   = load!((load)(userptr, c"glUniform2d"                                    .as_ptr()) as PFNGLUNIFORM2DPROC                                  );
            self.glUniform2dv                                  = load!((load)(userptr, c"glUniform2dv"                                   .as_ptr()) as PFNGLUNIFORM2DVPROC                                 );
            self.glUniform3d                                   = load!((load)(userptr, c"glUniform3d"                                    .as_ptr()) as PFNGLUNIFORM3DPROC                                  );
            self.glUniform3dv                                  = load!((load)(userptr, c"glUniform3dv"                                   .as_ptr()) as PFNGLUNIFORM3DVPROC                                 );
            self.glUniform4d                                   = load!((load)(userptr, c"glUniform4d"                                    .as_ptr()) as PFNGLUNIFORM4DPROC                                  );
            self.glUniform4dv                                  = load!((load)(userptr, c"glUniform4dv"                                   .as_ptr()) as PFNGLUNIFORM4DVPROC                                 );
            self.glUniformMatrix2dv                            = load!((load)(userptr, c"glUniformMatrix2dv"                             .as_ptr()) as PFNGLUNIFORMMATRIX2DVPROC                           );
            self.glUniformMatrix2x3dv                          = load!((load)(userptr, c"glUniformMatrix2x3dv"                           .as_ptr()) as PFNGLUNIFORMMATRIX2X3DVPROC                         );
            self.glUniformMatrix2x4dv                          = load!((load)(userptr, c"glUniformMatrix2x4dv"                           .as_ptr()) as PFNGLUNIFORMMATRIX2X4DVPROC                         );
            self.glUniformMatrix3dv                            = load!((load)(userptr, c"glUniformMatrix3dv"                             .as_ptr()) as PFNGLUNIFORMMATRIX3DVPROC                           );
            self.glUniformMatrix3x2dv                          = load!((load)(userptr, c"glUniformMatrix3x2dv"                           .as_ptr()) as PFNGLUNIFORMMATRIX3X2DVPROC                         );
            self.glUniformMatrix3x4dv                          = load!((load)(userptr, c"glUniformMatrix3x4dv"                           .as_ptr()) as PFNGLUNIFORMMATRIX3X4DVPROC                         );
            self.glUniformMatrix4dv                            = load!((load)(userptr, c"glUniformMatrix4dv"                             .as_ptr()) as PFNGLUNIFORMMATRIX4DVPROC                           );
            self.glUniformMatrix4x2dv                          = load!((load)(userptr, c"glUniformMatrix4x2dv"                           .as_ptr()) as PFNGLUNIFORMMATRIX4X2DVPROC                         );
            self.glUniformMatrix4x3dv                          = load!((load)(userptr, c"glUniformMatrix4x3dv"                           .as_ptr()) as PFNGLUNIFORMMATRIX4X3DVPROC                         );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_gpu_shader_int64(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_gpu_shader_int64 { return; }
            self.glGetUniformi64vARB                           = load!((load)(userptr, c"glGetUniformi64vARB"                            .as_ptr()) as PFNGLGETUNIFORMI64VARBPROC                          );
            self.glGetUniformui64vARB                          = load!((load)(userptr, c"glGetUniformui64vARB"                           .as_ptr()) as PFNGLGETUNIFORMUI64VARBPROC                         );
            self.glGetnUniformi64vARB                          = load!((load)(userptr, c"glGetnUniformi64vARB"                           .as_ptr()) as PFNGLGETNUNIFORMI64VARBPROC                         );
            self.glGetnUniformui64vARB                         = load!((load)(userptr, c"glGetnUniformui64vARB"                          .as_ptr()) as PFNGLGETNUNIFORMUI64VARBPROC                        );
            self.glProgramUniform1i64ARB                       = load!((load)(userptr, c"glProgramUniform1i64ARB"                        .as_ptr()) as PFNGLPROGRAMUNIFORM1I64ARBPROC                      );
            self.glProgramUniform1i64vARB                      = load!((load)(userptr, c"glProgramUniform1i64vARB"                       .as_ptr()) as PFNGLPROGRAMUNIFORM1I64VARBPROC                     );
            self.glProgramUniform1ui64ARB                      = load!((load)(userptr, c"glProgramUniform1ui64ARB"                       .as_ptr()) as PFNGLPROGRAMUNIFORM1UI64ARBPROC                     );
            self.glProgramUniform1ui64vARB                     = load!((load)(userptr, c"glProgramUniform1ui64vARB"                      .as_ptr()) as PFNGLPROGRAMUNIFORM1UI64VARBPROC                    );
            self.glProgramUniform2i64ARB                       = load!((load)(userptr, c"glProgramUniform2i64ARB"                        .as_ptr()) as PFNGLPROGRAMUNIFORM2I64ARBPROC                      );
            self.glProgramUniform2i64vARB                      = load!((load)(userptr, c"glProgramUniform2i64vARB"                       .as_ptr()) as PFNGLPROGRAMUNIFORM2I64VARBPROC                     );
            self.glProgramUniform2ui64ARB                      = load!((load)(userptr, c"glProgramUniform2ui64ARB"                       .as_ptr()) as PFNGLPROGRAMUNIFORM2UI64ARBPROC                     );
            self.glProgramUniform2ui64vARB                     = load!((load)(userptr, c"glProgramUniform2ui64vARB"                      .as_ptr()) as PFNGLPROGRAMUNIFORM2UI64VARBPROC                    );
            self.glProgramUniform3i64ARB                       = load!((load)(userptr, c"glProgramUniform3i64ARB"                        .as_ptr()) as PFNGLPROGRAMUNIFORM3I64ARBPROC                      );
            self.glProgramUniform3i64vARB                      = load!((load)(userptr, c"glProgramUniform3i64vARB"                       .as_ptr()) as PFNGLPROGRAMUNIFORM3I64VARBPROC                     );
            self.glProgramUniform3ui64ARB                      = load!((load)(userptr, c"glProgramUniform3ui64ARB"                       .as_ptr()) as PFNGLPROGRAMUNIFORM3UI64ARBPROC                     );
            self.glProgramUniform3ui64vARB                     = load!((load)(userptr, c"glProgramUniform3ui64vARB"                      .as_ptr()) as PFNGLPROGRAMUNIFORM3UI64VARBPROC                    );
            self.glProgramUniform4i64ARB                       = load!((load)(userptr, c"glProgramUniform4i64ARB"                        .as_ptr()) as PFNGLPROGRAMUNIFORM4I64ARBPROC                      );
            self.glProgramUniform4i64vARB                      = load!((load)(userptr, c"glProgramUniform4i64vARB"                       .as_ptr()) as PFNGLPROGRAMUNIFORM4I64VARBPROC                     );
            self.glProgramUniform4ui64ARB                      = load!((load)(userptr, c"glProgramUniform4ui64ARB"                       .as_ptr()) as PFNGLPROGRAMUNIFORM4UI64ARBPROC                     );
            self.glProgramUniform4ui64vARB                     = load!((load)(userptr, c"glProgramUniform4ui64vARB"                      .as_ptr()) as PFNGLPROGRAMUNIFORM4UI64VARBPROC                    );
            self.glUniform1i64ARB                              = load!((load)(userptr, c"glUniform1i64ARB"                               .as_ptr()) as PFNGLUNIFORM1I64ARBPROC                             );
            self.glUniform1i64vARB                             = load!((load)(userptr, c"glUniform1i64vARB"                              .as_ptr()) as PFNGLUNIFORM1I64VARBPROC                            );
            self.glUniform1ui64ARB                             = load!((load)(userptr, c"glUniform1ui64ARB"                              .as_ptr()) as PFNGLUNIFORM1UI64ARBPROC                            );
            self.glUniform1ui64vARB                            = load!((load)(userptr, c"glUniform1ui64vARB"                             .as_ptr()) as PFNGLUNIFORM1UI64VARBPROC                           );
            self.glUniform2i64ARB                              = load!((load)(userptr, c"glUniform2i64ARB"                               .as_ptr()) as PFNGLUNIFORM2I64ARBPROC                             );
            self.glUniform2i64vARB                             = load!((load)(userptr, c"glUniform2i64vARB"                              .as_ptr()) as PFNGLUNIFORM2I64VARBPROC                            );
            self.glUniform2ui64ARB                             = load!((load)(userptr, c"glUniform2ui64ARB"                              .as_ptr()) as PFNGLUNIFORM2UI64ARBPROC                            );
            self.glUniform2ui64vARB                            = load!((load)(userptr, c"glUniform2ui64vARB"                             .as_ptr()) as PFNGLUNIFORM2UI64VARBPROC                           );
            self.glUniform3i64ARB                              = load!((load)(userptr, c"glUniform3i64ARB"                               .as_ptr()) as PFNGLUNIFORM3I64ARBPROC                             );
            self.glUniform3i64vARB                             = load!((load)(userptr, c"glUniform3i64vARB"                              .as_ptr()) as PFNGLUNIFORM3I64VARBPROC                            );
            self.glUniform3ui64ARB                             = load!((load)(userptr, c"glUniform3ui64ARB"                              .as_ptr()) as PFNGLUNIFORM3UI64ARBPROC                            );
            self.glUniform3ui64vARB                            = load!((load)(userptr, c"glUniform3ui64vARB"                             .as_ptr()) as PFNGLUNIFORM3UI64VARBPROC                           );
            self.glUniform4i64ARB                              = load!((load)(userptr, c"glUniform4i64ARB"                               .as_ptr()) as PFNGLUNIFORM4I64ARBPROC                             );
            self.glUniform4i64vARB                             = load!((load)(userptr, c"glUniform4i64vARB"                              .as_ptr()) as PFNGLUNIFORM4I64VARBPROC                            );
            self.glUniform4ui64ARB                             = load!((load)(userptr, c"glUniform4ui64ARB"                              .as_ptr()) as PFNGLUNIFORM4UI64ARBPROC                            );
            self.glUniform4ui64vARB                            = load!((load)(userptr, c"glUniform4ui64vARB"                             .as_ptr()) as PFNGLUNIFORM4UI64VARBPROC                           );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_instanced_arrays(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_instanced_arrays { return; }
            self.glVertexAttribDivisorARB                      = load!((load)(userptr, c"glVertexAttribDivisorARB"                       .as_ptr()) as PFNGLVERTEXATTRIBDIVISORARBPROC                     );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_internalformat_query(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_internalformat_query { return; }
            self.glGetInternalformativ                         = load!((load)(userptr, c"glGetInternalformativ"                          .as_ptr()) as PFNGLGETINTERNALFORMATIVPROC                        );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_internalformat_query2(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_internalformat_query2 { return; }
            self.glGetInternalformati64v                       = load!((load)(userptr, c"glGetInternalformati64v"                        .as_ptr()) as PFNGLGETINTERNALFORMATI64VPROC                      );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_map_buffer_range(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_map_buffer_range { return; }
            self.glFlushMappedBufferRange                      = load!((load)(userptr, c"glFlushMappedBufferRange"                       .as_ptr()) as PFNGLFLUSHMAPPEDBUFFERRANGEPROC                     );
            self.glMapBufferRange                              = load!((load)(userptr, c"glMapBufferRange"                               .as_ptr()) as PFNGLMAPBUFFERRANGEPROC                             );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_multi_bind(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_multi_bind { return; }
            self.glBindBuffersBase                             = load!((load)(userptr, c"glBindBuffersBase"                              .as_ptr()) as PFNGLBINDBUFFERSBASEPROC                            );
            self.glBindBuffersRange                            = load!((load)(userptr, c"glBindBuffersRange"                             .as_ptr()) as PFNGLBINDBUFFERSRANGEPROC                           );
            self.glBindImageTextures                           = load!((load)(userptr, c"glBindImageTextures"                            .as_ptr()) as PFNGLBINDIMAGETEXTURESPROC                          );
            self.glBindSamplers                                = load!((load)(userptr, c"glBindSamplers"                                 .as_ptr()) as PFNGLBINDSAMPLERSPROC                               );
            self.glBindTextures                                = load!((load)(userptr, c"glBindTextures"                                 .as_ptr()) as PFNGLBINDTEXTURESPROC                               );
            self.glBindVertexBuffers                           = load!((load)(userptr, c"glBindVertexBuffers"                            .as_ptr()) as PFNGLBINDVERTEXBUFFERSPROC                          );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_multi_draw_indirect(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_multi_draw_indirect { return; }
            self.glMultiDrawArraysIndirect                     = load!((load)(userptr, c"glMultiDrawArraysIndirect"                      .as_ptr()) as PFNGLMULTIDRAWARRAYSINDIRECTPROC                    );
            self.glMultiDrawElementsIndirect                   = load!((load)(userptr, c"glMultiDrawElementsIndirect"                    .as_ptr()) as PFNGLMULTIDRAWELEMENTSINDIRECTPROC                  );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_multisample(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_multisample { return; }
            self.glSampleCoverageARB                           = load!((load)(userptr, c"glSampleCoverageARB"                            .as_ptr()) as PFNGLSAMPLECOVERAGEARBPROC                          );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_multitexture(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_multitexture { return; }
            self.glActiveTextureARB                            = load!((load)(userptr, c"glActiveTextureARB"                             .as_ptr()) as PFNGLACTIVETEXTUREARBPROC                           );
            self.glClientActiveTextureARB                      = load!((load)(userptr, c"glClientActiveTextureARB"                       .as_ptr()) as PFNGLCLIENTACTIVETEXTUREARBPROC                     );
            self.glMultiTexCoord1dARB                          = load!((load)(userptr, c"glMultiTexCoord1dARB"                           .as_ptr()) as PFNGLMULTITEXCOORD1DARBPROC                         );
            self.glMultiTexCoord1dvARB                         = load!((load)(userptr, c"glMultiTexCoord1dvARB"                          .as_ptr()) as PFNGLMULTITEXCOORD1DVARBPROC                        );
            self.glMultiTexCoord1fARB                          = load!((load)(userptr, c"glMultiTexCoord1fARB"                           .as_ptr()) as PFNGLMULTITEXCOORD1FARBPROC                         );
            self.glMultiTexCoord1fvARB                         = load!((load)(userptr, c"glMultiTexCoord1fvARB"                          .as_ptr()) as PFNGLMULTITEXCOORD1FVARBPROC                        );
            self.glMultiTexCoord1iARB                          = load!((load)(userptr, c"glMultiTexCoord1iARB"                           .as_ptr()) as PFNGLMULTITEXCOORD1IARBPROC                         );
            self.glMultiTexCoord1ivARB                         = load!((load)(userptr, c"glMultiTexCoord1ivARB"                          .as_ptr()) as PFNGLMULTITEXCOORD1IVARBPROC                        );
            self.glMultiTexCoord1sARB                          = load!((load)(userptr, c"glMultiTexCoord1sARB"                           .as_ptr()) as PFNGLMULTITEXCOORD1SARBPROC                         );
            self.glMultiTexCoord1svARB                         = load!((load)(userptr, c"glMultiTexCoord1svARB"                          .as_ptr()) as PFNGLMULTITEXCOORD1SVARBPROC                        );
            self.glMultiTexCoord2dARB                          = load!((load)(userptr, c"glMultiTexCoord2dARB"                           .as_ptr()) as PFNGLMULTITEXCOORD2DARBPROC                         );
            self.glMultiTexCoord2dvARB                         = load!((load)(userptr, c"glMultiTexCoord2dvARB"                          .as_ptr()) as PFNGLMULTITEXCOORD2DVARBPROC                        );
            self.glMultiTexCoord2fARB                          = load!((load)(userptr, c"glMultiTexCoord2fARB"                           .as_ptr()) as PFNGLMULTITEXCOORD2FARBPROC                         );
            self.glMultiTexCoord2fvARB                         = load!((load)(userptr, c"glMultiTexCoord2fvARB"                          .as_ptr()) as PFNGLMULTITEXCOORD2FVARBPROC                        );
            self.glMultiTexCoord2iARB                          = load!((load)(userptr, c"glMultiTexCoord2iARB"                           .as_ptr()) as PFNGLMULTITEXCOORD2IARBPROC                         );
            self.glMultiTexCoord2ivARB                         = load!((load)(userptr, c"glMultiTexCoord2ivARB"                          .as_ptr()) as PFNGLMULTITEXCOORD2IVARBPROC                        );
            self.glMultiTexCoord2sARB                          = load!((load)(userptr, c"glMultiTexCoord2sARB"                           .as_ptr()) as PFNGLMULTITEXCOORD2SARBPROC                         );
            self.glMultiTexCoord2svARB                         = load!((load)(userptr, c"glMultiTexCoord2svARB"                          .as_ptr()) as PFNGLMULTITEXCOORD2SVARBPROC                        );
            self.glMultiTexCoord3dARB                          = load!((load)(userptr, c"glMultiTexCoord3dARB"                           .as_ptr()) as PFNGLMULTITEXCOORD3DARBPROC                         );
            self.glMultiTexCoord3dvARB                         = load!((load)(userptr, c"glMultiTexCoord3dvARB"                          .as_ptr()) as PFNGLMULTITEXCOORD3DVARBPROC                        );
            self.glMultiTexCoord3fARB                          = load!((load)(userptr, c"glMultiTexCoord3fARB"                           .as_ptr()) as PFNGLMULTITEXCOORD3FARBPROC                         );
            self.glMultiTexCoord3fvARB                         = load!((load)(userptr, c"glMultiTexCoord3fvARB"                          .as_ptr()) as PFNGLMULTITEXCOORD3FVARBPROC                        );
            self.glMultiTexCoord3iARB                          = load!((load)(userptr, c"glMultiTexCoord3iARB"                           .as_ptr()) as PFNGLMULTITEXCOORD3IARBPROC                         );
            self.glMultiTexCoord3ivARB                         = load!((load)(userptr, c"glMultiTexCoord3ivARB"                          .as_ptr()) as PFNGLMULTITEXCOORD3IVARBPROC                        );
            self.glMultiTexCoord3sARB                          = load!((load)(userptr, c"glMultiTexCoord3sARB"                           .as_ptr()) as PFNGLMULTITEXCOORD3SARBPROC                         );
            self.glMultiTexCoord3svARB                         = load!((load)(userptr, c"glMultiTexCoord3svARB"                          .as_ptr()) as PFNGLMULTITEXCOORD3SVARBPROC                        );
            self.glMultiTexCoord4dARB                          = load!((load)(userptr, c"glMultiTexCoord4dARB"                           .as_ptr()) as PFNGLMULTITEXCOORD4DARBPROC                         );
            self.glMultiTexCoord4dvARB                         = load!((load)(userptr, c"glMultiTexCoord4dvARB"                          .as_ptr()) as PFNGLMULTITEXCOORD4DVARBPROC                        );
            self.glMultiTexCoord4fARB                          = load!((load)(userptr, c"glMultiTexCoord4fARB"                           .as_ptr()) as PFNGLMULTITEXCOORD4FARBPROC                         );
            self.glMultiTexCoord4fvARB                         = load!((load)(userptr, c"glMultiTexCoord4fvARB"                          .as_ptr()) as PFNGLMULTITEXCOORD4FVARBPROC                        );
            self.glMultiTexCoord4iARB                          = load!((load)(userptr, c"glMultiTexCoord4iARB"                           .as_ptr()) as PFNGLMULTITEXCOORD4IARBPROC                         );
            self.glMultiTexCoord4ivARB                         = load!((load)(userptr, c"glMultiTexCoord4ivARB"                          .as_ptr()) as PFNGLMULTITEXCOORD4IVARBPROC                        );
            self.glMultiTexCoord4sARB                          = load!((load)(userptr, c"glMultiTexCoord4sARB"                           .as_ptr()) as PFNGLMULTITEXCOORD4SARBPROC                         );
            self.glMultiTexCoord4svARB                         = load!((load)(userptr, c"glMultiTexCoord4svARB"                          .as_ptr()) as PFNGLMULTITEXCOORD4SVARBPROC                        );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_occlusion_query(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_occlusion_query { return; }
            self.glBeginQueryARB                               = load!((load)(userptr, c"glBeginQueryARB"                                .as_ptr()) as PFNGLBEGINQUERYARBPROC                              );
            self.glDeleteQueriesARB                            = load!((load)(userptr, c"glDeleteQueriesARB"                             .as_ptr()) as PFNGLDELETEQUERIESARBPROC                           );
            self.glEndQueryARB                                 = load!((load)(userptr, c"glEndQueryARB"                                  .as_ptr()) as PFNGLENDQUERYARBPROC                                );
            self.glGenQueriesARB                               = load!((load)(userptr, c"glGenQueriesARB"                                .as_ptr()) as PFNGLGENQUERIESARBPROC                              );
            self.glGetQueryObjectivARB                         = load!((load)(userptr, c"glGetQueryObjectivARB"                          .as_ptr()) as PFNGLGETQUERYOBJECTIVARBPROC                        );
            self.glGetQueryObjectuivARB                        = load!((load)(userptr, c"glGetQueryObjectuivARB"                         .as_ptr()) as PFNGLGETQUERYOBJECTUIVARBPROC                       );
            self.glGetQueryivARB                               = load!((load)(userptr, c"glGetQueryivARB"                                .as_ptr()) as PFNGLGETQUERYIVARBPROC                              );
            self.glIsQueryARB                                  = load!((load)(userptr, c"glIsQueryARB"                                   .as_ptr()) as PFNGLISQUERYARBPROC                                 );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_sample_locations(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_sample_locations { return; }
            self.glEvaluateDepthValuesARB                      = load!((load)(userptr, c"glEvaluateDepthValuesARB"                       .as_ptr()) as PFNGLEVALUATEDEPTHVALUESARBPROC                     );
            self.glFramebufferSampleLocationsfvARB             = load!((load)(userptr, c"glFramebufferSampleLocationsfvARB"              .as_ptr()) as PFNGLFRAMEBUFFERSAMPLELOCATIONSFVARBPROC            );
            self.glNamedFramebufferSampleLocationsfvARB        = load!((load)(userptr, c"glNamedFramebufferSampleLocationsfvARB"         .as_ptr()) as PFNGLNAMEDFRAMEBUFFERSAMPLELOCATIONSFVARBPROC       );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_sample_shading(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_sample_shading { return; }
            self.glMinSampleShadingARB                         = load!((load)(userptr, c"glMinSampleShadingARB"                          .as_ptr()) as PFNGLMINSAMPLESHADINGARBPROC                        );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_shader_atomic_counters(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_shader_atomic_counters { return; }
            self.glGetActiveAtomicCounterBufferiv              = load!((load)(userptr, c"glGetActiveAtomicCounterBufferiv"               .as_ptr()) as PFNGLGETACTIVEATOMICCOUNTERBUFFERIVPROC             );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_shader_image_load_store(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_shader_image_load_store { return; }
            self.glBindImageTexture                            = load!((load)(userptr, c"glBindImageTexture"                             .as_ptr()) as PFNGLBINDIMAGETEXTUREPROC                           );
            self.glMemoryBarrier                               = load!((load)(userptr, c"glMemoryBarrier"                                .as_ptr()) as PFNGLMEMORYBARRIERPROC                              );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_shader_objects(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_shader_objects { return; }
            self.glAttachObjectARB                             = load!((load)(userptr, c"glAttachObjectARB"                              .as_ptr()) as PFNGLATTACHOBJECTARBPROC                            );
            self.glCompileShaderARB                            = load!((load)(userptr, c"glCompileShaderARB"                             .as_ptr()) as PFNGLCOMPILESHADERARBPROC                           );
            self.glCreateProgramObjectARB                      = load!((load)(userptr, c"glCreateProgramObjectARB"                       .as_ptr()) as PFNGLCREATEPROGRAMOBJECTARBPROC                     );
            self.glCreateShaderObjectARB                       = load!((load)(userptr, c"glCreateShaderObjectARB"                        .as_ptr()) as PFNGLCREATESHADEROBJECTARBPROC                      );
            self.glDeleteObjectARB                             = load!((load)(userptr, c"glDeleteObjectARB"                              .as_ptr()) as PFNGLDELETEOBJECTARBPROC                            );
            self.glDetachObjectARB                             = load!((load)(userptr, c"glDetachObjectARB"                              .as_ptr()) as PFNGLDETACHOBJECTARBPROC                            );
            self.glGetActiveUniformARB                         = load!((load)(userptr, c"glGetActiveUniformARB"                          .as_ptr()) as PFNGLGETACTIVEUNIFORMARBPROC                        );
            self.glGetAttachedObjectsARB                       = load!((load)(userptr, c"glGetAttachedObjectsARB"                        .as_ptr()) as PFNGLGETATTACHEDOBJECTSARBPROC                      );
            self.glGetHandleARB                                = load!((load)(userptr, c"glGetHandleARB"                                 .as_ptr()) as PFNGLGETHANDLEARBPROC                               );
            self.glGetInfoLogARB                               = load!((load)(userptr, c"glGetInfoLogARB"                                .as_ptr()) as PFNGLGETINFOLOGARBPROC                              );
            self.glGetObjectParameterfvARB                     = load!((load)(userptr, c"glGetObjectParameterfvARB"                      .as_ptr()) as PFNGLGETOBJECTPARAMETERFVARBPROC                    );
            self.glGetObjectParameterivARB                     = load!((load)(userptr, c"glGetObjectParameterivARB"                      .as_ptr()) as PFNGLGETOBJECTPARAMETERIVARBPROC                    );
            self.glGetShaderSourceARB                          = load!((load)(userptr, c"glGetShaderSourceARB"                           .as_ptr()) as PFNGLGETSHADERSOURCEARBPROC                         );
            self.glGetUniformLocationARB                       = load!((load)(userptr, c"glGetUniformLocationARB"                        .as_ptr()) as PFNGLGETUNIFORMLOCATIONARBPROC                      );
            self.glGetUniformfvARB                             = load!((load)(userptr, c"glGetUniformfvARB"                              .as_ptr()) as PFNGLGETUNIFORMFVARBPROC                            );
            self.glGetUniformivARB                             = load!((load)(userptr, c"glGetUniformivARB"                              .as_ptr()) as PFNGLGETUNIFORMIVARBPROC                            );
            self.glLinkProgramARB                              = load!((load)(userptr, c"glLinkProgramARB"                               .as_ptr()) as PFNGLLINKPROGRAMARBPROC                             );
            self.glShaderSourceARB                             = load!((load)(userptr, c"glShaderSourceARB"                              .as_ptr()) as PFNGLSHADERSOURCEARBPROC                            );
            self.glUniform1fARB                                = load!((load)(userptr, c"glUniform1fARB"                                 .as_ptr()) as PFNGLUNIFORM1FARBPROC                               );
            self.glUniform1fvARB                               = load!((load)(userptr, c"glUniform1fvARB"                                .as_ptr()) as PFNGLUNIFORM1FVARBPROC                              );
            self.glUniform1iARB                                = load!((load)(userptr, c"glUniform1iARB"                                 .as_ptr()) as PFNGLUNIFORM1IARBPROC                               );
            self.glUniform1ivARB                               = load!((load)(userptr, c"glUniform1ivARB"                                .as_ptr()) as PFNGLUNIFORM1IVARBPROC                              );
            self.glUniform2fARB                                = load!((load)(userptr, c"glUniform2fARB"                                 .as_ptr()) as PFNGLUNIFORM2FARBPROC                               );
            self.glUniform2fvARB                               = load!((load)(userptr, c"glUniform2fvARB"                                .as_ptr()) as PFNGLUNIFORM2FVARBPROC                              );
            self.glUniform2iARB                                = load!((load)(userptr, c"glUniform2iARB"                                 .as_ptr()) as PFNGLUNIFORM2IARBPROC                               );
            self.glUniform2ivARB                               = load!((load)(userptr, c"glUniform2ivARB"                                .as_ptr()) as PFNGLUNIFORM2IVARBPROC                              );
            self.glUniform3fARB                                = load!((load)(userptr, c"glUniform3fARB"                                 .as_ptr()) as PFNGLUNIFORM3FARBPROC                               );
            self.glUniform3fvARB                               = load!((load)(userptr, c"glUniform3fvARB"                                .as_ptr()) as PFNGLUNIFORM3FVARBPROC                              );
            self.glUniform3iARB                                = load!((load)(userptr, c"glUniform3iARB"                                 .as_ptr()) as PFNGLUNIFORM3IARBPROC                               );
            self.glUniform3ivARB                               = load!((load)(userptr, c"glUniform3ivARB"                                .as_ptr()) as PFNGLUNIFORM3IVARBPROC                              );
            self.glUniform4fARB                                = load!((load)(userptr, c"glUniform4fARB"                                 .as_ptr()) as PFNGLUNIFORM4FARBPROC                               );
            self.glUniform4fvARB                               = load!((load)(userptr, c"glUniform4fvARB"                                .as_ptr()) as PFNGLUNIFORM4FVARBPROC                              );
            self.glUniform4iARB                                = load!((load)(userptr, c"glUniform4iARB"                                 .as_ptr()) as PFNGLUNIFORM4IARBPROC                               );
            self.glUniform4ivARB                               = load!((load)(userptr, c"glUniform4ivARB"                                .as_ptr()) as PFNGLUNIFORM4IVARBPROC                              );
            self.glUniformMatrix2fvARB                         = load!((load)(userptr, c"glUniformMatrix2fvARB"                          .as_ptr()) as PFNGLUNIFORMMATRIX2FVARBPROC                        );
            self.glUniformMatrix3fvARB                         = load!((load)(userptr, c"glUniformMatrix3fvARB"                          .as_ptr()) as PFNGLUNIFORMMATRIX3FVARBPROC                        );
            self.glUniformMatrix4fvARB                         = load!((load)(userptr, c"glUniformMatrix4fvARB"                          .as_ptr()) as PFNGLUNIFORMMATRIX4FVARBPROC                        );
            self.glUseProgramObjectARB                         = load!((load)(userptr, c"glUseProgramObjectARB"                          .as_ptr()) as PFNGLUSEPROGRAMOBJECTARBPROC                        );
            self.glValidateProgramARB                          = load!((load)(userptr, c"glValidateProgramARB"                           .as_ptr()) as PFNGLVALIDATEPROGRAMARBPROC                         );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_shader_storage_buffer_object(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_shader_storage_buffer_object { return; }
            self.glShaderStorageBlockBinding                   = load!((load)(userptr, c"glShaderStorageBlockBinding"                    .as_ptr()) as PFNGLSHADERSTORAGEBLOCKBINDINGPROC                  );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_shading_language_include(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_shading_language_include { return; }
            self.glCompileShaderIncludeARB                     = load!((load)(userptr, c"glCompileShaderIncludeARB"                      .as_ptr()) as PFNGLCOMPILESHADERINCLUDEARBPROC                    );
            self.glDeleteNamedStringARB                        = load!((load)(userptr, c"glDeleteNamedStringARB"                         .as_ptr()) as PFNGLDELETENAMEDSTRINGARBPROC                       );
            self.glGetNamedStringARB                           = load!((load)(userptr, c"glGetNamedStringARB"                            .as_ptr()) as PFNGLGETNAMEDSTRINGARBPROC                          );
            self.glGetNamedStringivARB                         = load!((load)(userptr, c"glGetNamedStringivARB"                          .as_ptr()) as PFNGLGETNAMEDSTRINGIVARBPROC                        );
            self.glIsNamedStringARB                            = load!((load)(userptr, c"glIsNamedStringARB"                             .as_ptr()) as PFNGLISNAMEDSTRINGARBPROC                           );
            self.glNamedStringARB                              = load!((load)(userptr, c"glNamedStringARB"                               .as_ptr()) as PFNGLNAMEDSTRINGARBPROC                             );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_tessellation_shader(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_tessellation_shader { return; }
            self.glPatchParameterfv                            = load!((load)(userptr, c"glPatchParameterfv"                             .as_ptr()) as PFNGLPATCHPARAMETERFVPROC                           );
            self.glPatchParameteri                             = load!((load)(userptr, c"glPatchParameteri"                              .as_ptr()) as PFNGLPATCHPARAMETERIPROC                            );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_texture_compression(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_texture_compression { return; }
            self.glCompressedTexImage1DARB                     = load!((load)(userptr, c"glCompressedTexImage1DARB"                      .as_ptr()) as PFNGLCOMPRESSEDTEXIMAGE1DARBPROC                    );
            self.glCompressedTexImage2DARB                     = load!((load)(userptr, c"glCompressedTexImage2DARB"                      .as_ptr()) as PFNGLCOMPRESSEDTEXIMAGE2DARBPROC                    );
            self.glCompressedTexImage3DARB                     = load!((load)(userptr, c"glCompressedTexImage3DARB"                      .as_ptr()) as PFNGLCOMPRESSEDTEXIMAGE3DARBPROC                    );
            self.glCompressedTexSubImage1DARB                  = load!((load)(userptr, c"glCompressedTexSubImage1DARB"                   .as_ptr()) as PFNGLCOMPRESSEDTEXSUBIMAGE1DARBPROC                 );
            self.glCompressedTexSubImage2DARB                  = load!((load)(userptr, c"glCompressedTexSubImage2DARB"                   .as_ptr()) as PFNGLCOMPRESSEDTEXSUBIMAGE2DARBPROC                 );
            self.glCompressedTexSubImage3DARB                  = load!((load)(userptr, c"glCompressedTexSubImage3DARB"                   .as_ptr()) as PFNGLCOMPRESSEDTEXSUBIMAGE3DARBPROC                 );
            self.glGetCompressedTexImageARB                    = load!((load)(userptr, c"glGetCompressedTexImageARB"                     .as_ptr()) as PFNGLGETCOMPRESSEDTEXIMAGEARBPROC                   );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_texture_multisample(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_texture_multisample { return; }
            self.glGetMultisamplefv                            = load!((load)(userptr, c"glGetMultisamplefv"                             .as_ptr()) as PFNGLGETMULTISAMPLEFVPROC                           );
            self.glSampleMaski                                 = load!((load)(userptr, c"glSampleMaski"                                  .as_ptr()) as PFNGLSAMPLEMASKIPROC                                );
            self.glTexImage2DMultisample                       = load!((load)(userptr, c"glTexImage2DMultisample"                        .as_ptr()) as PFNGLTEXIMAGE2DMULTISAMPLEPROC                      );
            self.glTexImage3DMultisample                       = load!((load)(userptr, c"glTexImage3DMultisample"                        .as_ptr()) as PFNGLTEXIMAGE3DMULTISAMPLEPROC                      );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_texture_storage(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_texture_storage { return; }
            self.glTexStorage1D                                = load!((load)(userptr, c"glTexStorage1D"                                 .as_ptr()) as PFNGLTEXSTORAGE1DPROC                               );
            self.glTexStorage2D                                = load!((load)(userptr, c"glTexStorage2D"                                 .as_ptr()) as PFNGLTEXSTORAGE2DPROC                               );
            self.glTexStorage3D                                = load!((load)(userptr, c"glTexStorage3D"                                 .as_ptr()) as PFNGLTEXSTORAGE3DPROC                               );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_texture_view(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_texture_view { return; }
            self.glTextureView                                 = load!((load)(userptr, c"glTextureView"                                  .as_ptr()) as PFNGLTEXTUREVIEWPROC                                );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_timer_query(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_timer_query { return; }
            self.glGetQueryObjecti64v                          = load!((load)(userptr, c"glGetQueryObjecti64v"                           .as_ptr()) as PFNGLGETQUERYOBJECTI64VPROC                         );
            self.glGetQueryObjectui64v                         = load!((load)(userptr, c"glGetQueryObjectui64v"                          .as_ptr()) as PFNGLGETQUERYOBJECTUI64VPROC                        );
            self.glQueryCounter                                = load!((load)(userptr, c"glQueryCounter"                                 .as_ptr()) as PFNGLQUERYCOUNTERPROC                               );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_transpose_matrix(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_transpose_matrix { return; }
            self.glLoadTransposeMatrixdARB                     = load!((load)(userptr, c"glLoadTransposeMatrixdARB"                      .as_ptr()) as PFNGLLOADTRANSPOSEMATRIXDARBPROC                    );
            self.glLoadTransposeMatrixfARB                     = load!((load)(userptr, c"glLoadTransposeMatrixfARB"                      .as_ptr()) as PFNGLLOADTRANSPOSEMATRIXFARBPROC                    );
            self.glMultTransposeMatrixdARB                     = load!((load)(userptr, c"glMultTransposeMatrixdARB"                      .as_ptr()) as PFNGLMULTTRANSPOSEMATRIXDARBPROC                    );
            self.glMultTransposeMatrixfARB                     = load!((load)(userptr, c"glMultTransposeMatrixfARB"                      .as_ptr()) as PFNGLMULTTRANSPOSEMATRIXFARBPROC                    );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_uniform_buffer_object(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_uniform_buffer_object { return; }
            self.glBindBufferBase                              = load!((load)(userptr, c"glBindBufferBase"                               .as_ptr()) as PFNGLBINDBUFFERBASEPROC                             );
            self.glBindBufferRange                             = load!((load)(userptr, c"glBindBufferRange"                              .as_ptr()) as PFNGLBINDBUFFERRANGEPROC                            );
            self.glGetActiveUniformBlockName                   = load!((load)(userptr, c"glGetActiveUniformBlockName"                    .as_ptr()) as PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC                  );
            self.glGetActiveUniformBlockiv                     = load!((load)(userptr, c"glGetActiveUniformBlockiv"                      .as_ptr()) as PFNGLGETACTIVEUNIFORMBLOCKIVPROC                    );
            self.glGetActiveUniformName                        = load!((load)(userptr, c"glGetActiveUniformName"                         .as_ptr()) as PFNGLGETACTIVEUNIFORMNAMEPROC                       );
            self.glGetActiveUniformsiv                         = load!((load)(userptr, c"glGetActiveUniformsiv"                          .as_ptr()) as PFNGLGETACTIVEUNIFORMSIVPROC                        );
            self.glGetIntegeri_v                               = load!((load)(userptr, c"glGetIntegeri_v"                                .as_ptr()) as PFNGLGETINTEGERI_VPROC                              );
            self.glGetUniformBlockIndex                        = load!((load)(userptr, c"glGetUniformBlockIndex"                         .as_ptr()) as PFNGLGETUNIFORMBLOCKINDEXPROC                       );
            self.glGetUniformIndices                           = load!((load)(userptr, c"glGetUniformIndices"                            .as_ptr()) as PFNGLGETUNIFORMINDICESPROC                          );
            self.glUniformBlockBinding                         = load!((load)(userptr, c"glUniformBlockBinding"                          .as_ptr()) as PFNGLUNIFORMBLOCKBINDINGPROC                        );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_vertex_array_object(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_vertex_array_object { return; }
            self.glBindVertexArray                             = load!((load)(userptr, c"glBindVertexArray"                              .as_ptr()) as PFNGLBINDVERTEXARRAYPROC                            );
            self.glDeleteVertexArrays                          = load!((load)(userptr, c"glDeleteVertexArrays"                           .as_ptr()) as PFNGLDELETEVERTEXARRAYSPROC                         );
            self.glGenVertexArrays                             = load!((load)(userptr, c"glGenVertexArrays"                              .as_ptr()) as PFNGLGENVERTEXARRAYSPROC                            );
            self.glIsVertexArray                               = load!((load)(userptr, c"glIsVertexArray"                                .as_ptr()) as PFNGLISVERTEXARRAYPROC                              );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_vertex_attrib_binding(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_vertex_attrib_binding { return; }
            self.glBindVertexBuffer                            = load!((load)(userptr, c"glBindVertexBuffer"                             .as_ptr()) as PFNGLBINDVERTEXBUFFERPROC                           );
            self.glVertexAttribBinding                         = load!((load)(userptr, c"glVertexAttribBinding"                          .as_ptr()) as PFNGLVERTEXATTRIBBINDINGPROC                        );
            self.glVertexAttribFormat                          = load!((load)(userptr, c"glVertexAttribFormat"                           .as_ptr()) as PFNGLVERTEXATTRIBFORMATPROC                         );
            self.glVertexAttribIFormat                         = load!((load)(userptr, c"glVertexAttribIFormat"                          .as_ptr()) as PFNGLVERTEXATTRIBIFORMATPROC                        );
            self.glVertexAttribLFormat                         = load!((load)(userptr, c"glVertexAttribLFormat"                          .as_ptr()) as PFNGLVERTEXATTRIBLFORMATPROC                        );
            self.glVertexBindingDivisor                        = load!((load)(userptr, c"glVertexBindingDivisor"                         .as_ptr()) as PFNGLVERTEXBINDINGDIVISORPROC                       );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_vertex_buffer_object(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_vertex_buffer_object { return; }
            self.glBindBufferARB                               = load!((load)(userptr, c"glBindBufferARB"                                .as_ptr()) as PFNGLBINDBUFFERARBPROC                              );
            self.glBufferDataARB                               = load!((load)(userptr, c"glBufferDataARB"                                .as_ptr()) as PFNGLBUFFERDATAARBPROC                              );
            self.glBufferSubDataARB                            = load!((load)(userptr, c"glBufferSubDataARB"                             .as_ptr()) as PFNGLBUFFERSUBDATAARBPROC                           );
            self.glDeleteBuffersARB                            = load!((load)(userptr, c"glDeleteBuffersARB"                             .as_ptr()) as PFNGLDELETEBUFFERSARBPROC                           );
            self.glGenBuffersARB                               = load!((load)(userptr, c"glGenBuffersARB"                                .as_ptr()) as PFNGLGENBUFFERSARBPROC                              );
            self.glGetBufferParameterivARB                     = load!((load)(userptr, c"glGetBufferParameterivARB"                      .as_ptr()) as PFNGLGETBUFFERPARAMETERIVARBPROC                    );
            self.glGetBufferPointervARB                        = load!((load)(userptr, c"glGetBufferPointervARB"                         .as_ptr()) as PFNGLGETBUFFERPOINTERVARBPROC                       );
            self.glGetBufferSubDataARB                         = load!((load)(userptr, c"glGetBufferSubDataARB"                          .as_ptr()) as PFNGLGETBUFFERSUBDATAARBPROC                        );
            self.glIsBufferARB                                 = load!((load)(userptr, c"glIsBufferARB"                                  .as_ptr()) as PFNGLISBUFFERARBPROC                                );
            self.glMapBufferARB                                = load!((load)(userptr, c"glMapBufferARB"                                 .as_ptr()) as PFNGLMAPBUFFERARBPROC                               );
            self.glUnmapBufferARB                              = load!((load)(userptr, c"glUnmapBufferARB"                               .as_ptr()) as PFNGLUNMAPBUFFERARBPROC                             );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_vertex_program(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_vertex_program { return; }
            self.glBindProgramARB                              = load!((load)(userptr, c"glBindProgramARB"                               .as_ptr()) as PFNGLBINDPROGRAMARBPROC                             );
            self.glDeleteProgramsARB                           = load!((load)(userptr, c"glDeleteProgramsARB"                            .as_ptr()) as PFNGLDELETEPROGRAMSARBPROC                          );
            self.glDisableVertexAttribArrayARB                 = load!((load)(userptr, c"glDisableVertexAttribArrayARB"                  .as_ptr()) as PFNGLDISABLEVERTEXATTRIBARRAYARBPROC                );
            self.glEnableVertexAttribArrayARB                  = load!((load)(userptr, c"glEnableVertexAttribArrayARB"                   .as_ptr()) as PFNGLENABLEVERTEXATTRIBARRAYARBPROC                 );
            self.glGenProgramsARB                              = load!((load)(userptr, c"glGenProgramsARB"                               .as_ptr()) as PFNGLGENPROGRAMSARBPROC                             );
            self.glGetProgramEnvParameterdvARB                 = load!((load)(userptr, c"glGetProgramEnvParameterdvARB"                  .as_ptr()) as PFNGLGETPROGRAMENVPARAMETERDVARBPROC                );
            self.glGetProgramEnvParameterfvARB                 = load!((load)(userptr, c"glGetProgramEnvParameterfvARB"                  .as_ptr()) as PFNGLGETPROGRAMENVPARAMETERFVARBPROC                );
            self.glGetProgramLocalParameterdvARB               = load!((load)(userptr, c"glGetProgramLocalParameterdvARB"                .as_ptr()) as PFNGLGETPROGRAMLOCALPARAMETERDVARBPROC              );
            self.glGetProgramLocalParameterfvARB               = load!((load)(userptr, c"glGetProgramLocalParameterfvARB"                .as_ptr()) as PFNGLGETPROGRAMLOCALPARAMETERFVARBPROC              );
            self.glGetProgramStringARB                         = load!((load)(userptr, c"glGetProgramStringARB"                          .as_ptr()) as PFNGLGETPROGRAMSTRINGARBPROC                        );
            self.glGetProgramivARB                             = load!((load)(userptr, c"glGetProgramivARB"                              .as_ptr()) as PFNGLGETPROGRAMIVARBPROC                            );
            self.glGetVertexAttribPointervARB                  = load!((load)(userptr, c"glGetVertexAttribPointervARB"                   .as_ptr()) as PFNGLGETVERTEXATTRIBPOINTERVARBPROC                 );
            self.glGetVertexAttribdvARB                        = load!((load)(userptr, c"glGetVertexAttribdvARB"                         .as_ptr()) as PFNGLGETVERTEXATTRIBDVARBPROC                       );
            self.glGetVertexAttribfvARB                        = load!((load)(userptr, c"glGetVertexAttribfvARB"                         .as_ptr()) as PFNGLGETVERTEXATTRIBFVARBPROC                       );
            self.glGetVertexAttribivARB                        = load!((load)(userptr, c"glGetVertexAttribivARB"                         .as_ptr()) as PFNGLGETVERTEXATTRIBIVARBPROC                       );
            self.glIsProgramARB                                = load!((load)(userptr, c"glIsProgramARB"                                 .as_ptr()) as PFNGLISPROGRAMARBPROC                               );
            self.glProgramEnvParameter4dARB                    = load!((load)(userptr, c"glProgramEnvParameter4dARB"                     .as_ptr()) as PFNGLPROGRAMENVPARAMETER4DARBPROC                   );
            self.glProgramEnvParameter4dvARB                   = load!((load)(userptr, c"glProgramEnvParameter4dvARB"                    .as_ptr()) as PFNGLPROGRAMENVPARAMETER4DVARBPROC                  );
            self.glProgramEnvParameter4fARB                    = load!((load)(userptr, c"glProgramEnvParameter4fARB"                     .as_ptr()) as PFNGLPROGRAMENVPARAMETER4FARBPROC                   );
            self.glProgramEnvParameter4fvARB                   = load!((load)(userptr, c"glProgramEnvParameter4fvARB"                    .as_ptr()) as PFNGLPROGRAMENVPARAMETER4FVARBPROC                  );
            self.glProgramLocalParameter4dARB                  = load!((load)(userptr, c"glProgramLocalParameter4dARB"                   .as_ptr()) as PFNGLPROGRAMLOCALPARAMETER4DARBPROC                 );
            self.glProgramLocalParameter4dvARB                 = load!((load)(userptr, c"glProgramLocalParameter4dvARB"                  .as_ptr()) as PFNGLPROGRAMLOCALPARAMETER4DVARBPROC                );
            self.glProgramLocalParameter4fARB                  = load!((load)(userptr, c"glProgramLocalParameter4fARB"                   .as_ptr()) as PFNGLPROGRAMLOCALPARAMETER4FARBPROC                 );
            self.glProgramLocalParameter4fvARB                 = load!((load)(userptr, c"glProgramLocalParameter4fvARB"                  .as_ptr()) as PFNGLPROGRAMLOCALPARAMETER4FVARBPROC                );
            self.glProgramStringARB                            = load!((load)(userptr, c"glProgramStringARB"                             .as_ptr()) as PFNGLPROGRAMSTRINGARBPROC                           );
            self.glVertexAttrib1dARB                           = load!((load)(userptr, c"glVertexAttrib1dARB"                            .as_ptr()) as PFNGLVERTEXATTRIB1DARBPROC                          );
            self.glVertexAttrib1dvARB                          = load!((load)(userptr, c"glVertexAttrib1dvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB1DVARBPROC                         );
            self.glVertexAttrib1fARB                           = load!((load)(userptr, c"glVertexAttrib1fARB"                            .as_ptr()) as PFNGLVERTEXATTRIB1FARBPROC                          );
            self.glVertexAttrib1fvARB                          = load!((load)(userptr, c"glVertexAttrib1fvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB1FVARBPROC                         );
            self.glVertexAttrib1sARB                           = load!((load)(userptr, c"glVertexAttrib1sARB"                            .as_ptr()) as PFNGLVERTEXATTRIB1SARBPROC                          );
            self.glVertexAttrib1svARB                          = load!((load)(userptr, c"glVertexAttrib1svARB"                           .as_ptr()) as PFNGLVERTEXATTRIB1SVARBPROC                         );
            self.glVertexAttrib2dARB                           = load!((load)(userptr, c"glVertexAttrib2dARB"                            .as_ptr()) as PFNGLVERTEXATTRIB2DARBPROC                          );
            self.glVertexAttrib2dvARB                          = load!((load)(userptr, c"glVertexAttrib2dvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB2DVARBPROC                         );
            self.glVertexAttrib2fARB                           = load!((load)(userptr, c"glVertexAttrib2fARB"                            .as_ptr()) as PFNGLVERTEXATTRIB2FARBPROC                          );
            self.glVertexAttrib2fvARB                          = load!((load)(userptr, c"glVertexAttrib2fvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB2FVARBPROC                         );
            self.glVertexAttrib2sARB                           = load!((load)(userptr, c"glVertexAttrib2sARB"                            .as_ptr()) as PFNGLVERTEXATTRIB2SARBPROC                          );
            self.glVertexAttrib2svARB                          = load!((load)(userptr, c"glVertexAttrib2svARB"                           .as_ptr()) as PFNGLVERTEXATTRIB2SVARBPROC                         );
            self.glVertexAttrib3dARB                           = load!((load)(userptr, c"glVertexAttrib3dARB"                            .as_ptr()) as PFNGLVERTEXATTRIB3DARBPROC                          );
            self.glVertexAttrib3dvARB                          = load!((load)(userptr, c"glVertexAttrib3dvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB3DVARBPROC                         );
            self.glVertexAttrib3fARB                           = load!((load)(userptr, c"glVertexAttrib3fARB"                            .as_ptr()) as PFNGLVERTEXATTRIB3FARBPROC                          );
            self.glVertexAttrib3fvARB                          = load!((load)(userptr, c"glVertexAttrib3fvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB3FVARBPROC                         );
            self.glVertexAttrib3sARB                           = load!((load)(userptr, c"glVertexAttrib3sARB"                            .as_ptr()) as PFNGLVERTEXATTRIB3SARBPROC                          );
            self.glVertexAttrib3svARB                          = load!((load)(userptr, c"glVertexAttrib3svARB"                           .as_ptr()) as PFNGLVERTEXATTRIB3SVARBPROC                         );
            self.glVertexAttrib4NbvARB                         = load!((load)(userptr, c"glVertexAttrib4NbvARB"                          .as_ptr()) as PFNGLVERTEXATTRIB4NBVARBPROC                        );
            self.glVertexAttrib4NivARB                         = load!((load)(userptr, c"glVertexAttrib4NivARB"                          .as_ptr()) as PFNGLVERTEXATTRIB4NIVARBPROC                        );
            self.glVertexAttrib4NsvARB                         = load!((load)(userptr, c"glVertexAttrib4NsvARB"                          .as_ptr()) as PFNGLVERTEXATTRIB4NSVARBPROC                        );
            self.glVertexAttrib4NubARB                         = load!((load)(userptr, c"glVertexAttrib4NubARB"                          .as_ptr()) as PFNGLVERTEXATTRIB4NUBARBPROC                        );
            self.glVertexAttrib4NubvARB                        = load!((load)(userptr, c"glVertexAttrib4NubvARB"                         .as_ptr()) as PFNGLVERTEXATTRIB4NUBVARBPROC                       );
            self.glVertexAttrib4NuivARB                        = load!((load)(userptr, c"glVertexAttrib4NuivARB"                         .as_ptr()) as PFNGLVERTEXATTRIB4NUIVARBPROC                       );
            self.glVertexAttrib4NusvARB                        = load!((load)(userptr, c"glVertexAttrib4NusvARB"                         .as_ptr()) as PFNGLVERTEXATTRIB4NUSVARBPROC                       );
            self.glVertexAttrib4bvARB                          = load!((load)(userptr, c"glVertexAttrib4bvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB4BVARBPROC                         );
            self.glVertexAttrib4dARB                           = load!((load)(userptr, c"glVertexAttrib4dARB"                            .as_ptr()) as PFNGLVERTEXATTRIB4DARBPROC                          );
            self.glVertexAttrib4dvARB                          = load!((load)(userptr, c"glVertexAttrib4dvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB4DVARBPROC                         );
            self.glVertexAttrib4fARB                           = load!((load)(userptr, c"glVertexAttrib4fARB"                            .as_ptr()) as PFNGLVERTEXATTRIB4FARBPROC                          );
            self.glVertexAttrib4fvARB                          = load!((load)(userptr, c"glVertexAttrib4fvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB4FVARBPROC                         );
            self.glVertexAttrib4ivARB                          = load!((load)(userptr, c"glVertexAttrib4ivARB"                           .as_ptr()) as PFNGLVERTEXATTRIB4IVARBPROC                         );
            self.glVertexAttrib4sARB                           = load!((load)(userptr, c"glVertexAttrib4sARB"                            .as_ptr()) as PFNGLVERTEXATTRIB4SARBPROC                          );
            self.glVertexAttrib4svARB                          = load!((load)(userptr, c"glVertexAttrib4svARB"                           .as_ptr()) as PFNGLVERTEXATTRIB4SVARBPROC                         );
            self.glVertexAttrib4ubvARB                         = load!((load)(userptr, c"glVertexAttrib4ubvARB"                          .as_ptr()) as PFNGLVERTEXATTRIB4UBVARBPROC                        );
            self.glVertexAttrib4uivARB                         = load!((load)(userptr, c"glVertexAttrib4uivARB"                          .as_ptr()) as PFNGLVERTEXATTRIB4UIVARBPROC                        );
            self.glVertexAttrib4usvARB                         = load!((load)(userptr, c"glVertexAttrib4usvARB"                          .as_ptr()) as PFNGLVERTEXATTRIB4USVARBPROC                        );
            self.glVertexAttribPointerARB                      = load!((load)(userptr, c"glVertexAttribPointerARB"                       .as_ptr()) as PFNGLVERTEXATTRIBPOINTERARBPROC                     );
        }
    }
    pub(super) unsafe fn gl_load_GL_ARB_vertex_shader(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_ARB_vertex_shader { return; }
            self.glBindAttribLocationARB                       = load!((load)(userptr, c"glBindAttribLocationARB"                        .as_ptr()) as PFNGLBINDATTRIBLOCATIONARBPROC                      );
            self.glDisableVertexAttribArrayARB                 = load!((load)(userptr, c"glDisableVertexAttribArrayARB"                  .as_ptr()) as PFNGLDISABLEVERTEXATTRIBARRAYARBPROC                );
            self.glEnableVertexAttribArrayARB                  = load!((load)(userptr, c"glEnableVertexAttribArrayARB"                   .as_ptr()) as PFNGLENABLEVERTEXATTRIBARRAYARBPROC                 );
            self.glGetActiveAttribARB                          = load!((load)(userptr, c"glGetActiveAttribARB"                           .as_ptr()) as PFNGLGETACTIVEATTRIBARBPROC                         );
            self.glGetAttribLocationARB                        = load!((load)(userptr, c"glGetAttribLocationARB"                         .as_ptr()) as PFNGLGETATTRIBLOCATIONARBPROC                       );
            self.glGetVertexAttribPointervARB                  = load!((load)(userptr, c"glGetVertexAttribPointervARB"                   .as_ptr()) as PFNGLGETVERTEXATTRIBPOINTERVARBPROC                 );
            self.glGetVertexAttribdvARB                        = load!((load)(userptr, c"glGetVertexAttribdvARB"                         .as_ptr()) as PFNGLGETVERTEXATTRIBDVARBPROC                       );
            self.glGetVertexAttribfvARB                        = load!((load)(userptr, c"glGetVertexAttribfvARB"                         .as_ptr()) as PFNGLGETVERTEXATTRIBFVARBPROC                       );
            self.glGetVertexAttribivARB                        = load!((load)(userptr, c"glGetVertexAttribivARB"                         .as_ptr()) as PFNGLGETVERTEXATTRIBIVARBPROC                       );
            self.glVertexAttrib1dARB                           = load!((load)(userptr, c"glVertexAttrib1dARB"                            .as_ptr()) as PFNGLVERTEXATTRIB1DARBPROC                          );
            self.glVertexAttrib1dvARB                          = load!((load)(userptr, c"glVertexAttrib1dvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB1DVARBPROC                         );
            self.glVertexAttrib1fARB                           = load!((load)(userptr, c"glVertexAttrib1fARB"                            .as_ptr()) as PFNGLVERTEXATTRIB1FARBPROC                          );
            self.glVertexAttrib1fvARB                          = load!((load)(userptr, c"glVertexAttrib1fvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB1FVARBPROC                         );
            self.glVertexAttrib1sARB                           = load!((load)(userptr, c"glVertexAttrib1sARB"                            .as_ptr()) as PFNGLVERTEXATTRIB1SARBPROC                          );
            self.glVertexAttrib1svARB                          = load!((load)(userptr, c"glVertexAttrib1svARB"                           .as_ptr()) as PFNGLVERTEXATTRIB1SVARBPROC                         );
            self.glVertexAttrib2dARB                           = load!((load)(userptr, c"glVertexAttrib2dARB"                            .as_ptr()) as PFNGLVERTEXATTRIB2DARBPROC                          );
            self.glVertexAttrib2dvARB                          = load!((load)(userptr, c"glVertexAttrib2dvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB2DVARBPROC                         );
            self.glVertexAttrib2fARB                           = load!((load)(userptr, c"glVertexAttrib2fARB"                            .as_ptr()) as PFNGLVERTEXATTRIB2FARBPROC                          );
            self.glVertexAttrib2fvARB                          = load!((load)(userptr, c"glVertexAttrib2fvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB2FVARBPROC                         );
            self.glVertexAttrib2sARB                           = load!((load)(userptr, c"glVertexAttrib2sARB"                            .as_ptr()) as PFNGLVERTEXATTRIB2SARBPROC                          );
            self.glVertexAttrib2svARB                          = load!((load)(userptr, c"glVertexAttrib2svARB"                           .as_ptr()) as PFNGLVERTEXATTRIB2SVARBPROC                         );
            self.glVertexAttrib3dARB                           = load!((load)(userptr, c"glVertexAttrib3dARB"                            .as_ptr()) as PFNGLVERTEXATTRIB3DARBPROC                          );
            self.glVertexAttrib3dvARB                          = load!((load)(userptr, c"glVertexAttrib3dvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB3DVARBPROC                         );
            self.glVertexAttrib3fARB                           = load!((load)(userptr, c"glVertexAttrib3fARB"                            .as_ptr()) as PFNGLVERTEXATTRIB3FARBPROC                          );
            self.glVertexAttrib3fvARB                          = load!((load)(userptr, c"glVertexAttrib3fvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB3FVARBPROC                         );
            self.glVertexAttrib3sARB                           = load!((load)(userptr, c"glVertexAttrib3sARB"                            .as_ptr()) as PFNGLVERTEXATTRIB3SARBPROC                          );
            self.glVertexAttrib3svARB                          = load!((load)(userptr, c"glVertexAttrib3svARB"                           .as_ptr()) as PFNGLVERTEXATTRIB3SVARBPROC                         );
            self.glVertexAttrib4NbvARB                         = load!((load)(userptr, c"glVertexAttrib4NbvARB"                          .as_ptr()) as PFNGLVERTEXATTRIB4NBVARBPROC                        );
            self.glVertexAttrib4NivARB                         = load!((load)(userptr, c"glVertexAttrib4NivARB"                          .as_ptr()) as PFNGLVERTEXATTRIB4NIVARBPROC                        );
            self.glVertexAttrib4NsvARB                         = load!((load)(userptr, c"glVertexAttrib4NsvARB"                          .as_ptr()) as PFNGLVERTEXATTRIB4NSVARBPROC                        );
            self.glVertexAttrib4NubARB                         = load!((load)(userptr, c"glVertexAttrib4NubARB"                          .as_ptr()) as PFNGLVERTEXATTRIB4NUBARBPROC                        );
            self.glVertexAttrib4NubvARB                        = load!((load)(userptr, c"glVertexAttrib4NubvARB"                         .as_ptr()) as PFNGLVERTEXATTRIB4NUBVARBPROC                       );
            self.glVertexAttrib4NuivARB                        = load!((load)(userptr, c"glVertexAttrib4NuivARB"                         .as_ptr()) as PFNGLVERTEXATTRIB4NUIVARBPROC                       );
            self.glVertexAttrib4NusvARB                        = load!((load)(userptr, c"glVertexAttrib4NusvARB"                         .as_ptr()) as PFNGLVERTEXATTRIB4NUSVARBPROC                       );
            self.glVertexAttrib4bvARB                          = load!((load)(userptr, c"glVertexAttrib4bvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB4BVARBPROC                         );
            self.glVertexAttrib4dARB                           = load!((load)(userptr, c"glVertexAttrib4dARB"                            .as_ptr()) as PFNGLVERTEXATTRIB4DARBPROC                          );
            self.glVertexAttrib4dvARB                          = load!((load)(userptr, c"glVertexAttrib4dvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB4DVARBPROC                         );
            self.glVertexAttrib4fARB                           = load!((load)(userptr, c"glVertexAttrib4fARB"                            .as_ptr()) as PFNGLVERTEXATTRIB4FARBPROC                          );
            self.glVertexAttrib4fvARB                          = load!((load)(userptr, c"glVertexAttrib4fvARB"                           .as_ptr()) as PFNGLVERTEXATTRIB4FVARBPROC                         );
            self.glVertexAttrib4ivARB                          = load!((load)(userptr, c"glVertexAttrib4ivARB"                           .as_ptr()) as PFNGLVERTEXATTRIB4IVARBPROC                         );
            self.glVertexAttrib4sARB                           = load!((load)(userptr, c"glVertexAttrib4sARB"                            .as_ptr()) as PFNGLVERTEXATTRIB4SARBPROC                          );
            self.glVertexAttrib4svARB                          = load!((load)(userptr, c"glVertexAttrib4svARB"                           .as_ptr()) as PFNGLVERTEXATTRIB4SVARBPROC                         );
            self.glVertexAttrib4ubvARB                         = load!((load)(userptr, c"glVertexAttrib4ubvARB"                          .as_ptr()) as PFNGLVERTEXATTRIB4UBVARBPROC                        );
            self.glVertexAttrib4uivARB                         = load!((load)(userptr, c"glVertexAttrib4uivARB"                          .as_ptr()) as PFNGLVERTEXATTRIB4UIVARBPROC                        );
            self.glVertexAttrib4usvARB                         = load!((load)(userptr, c"glVertexAttrib4usvARB"                          .as_ptr()) as PFNGLVERTEXATTRIB4USVARBPROC                        );
            self.glVertexAttribPointerARB                      = load!((load)(userptr, c"glVertexAttribPointerARB"                       .as_ptr()) as PFNGLVERTEXATTRIBPOINTERARBPROC                     );
        }
    }
    pub(super) unsafe fn gl_load_GL_EXT_draw_instanced(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_EXT_draw_instanced { return; }
            self.glDrawArraysInstancedEXT                      = load!((load)(userptr, c"glDrawArraysInstancedEXT"                       .as_ptr()) as PFNGLDRAWARRAYSINSTANCEDEXTPROC                     );
            self.glDrawElementsInstancedEXT                    = load!((load)(userptr, c"glDrawElementsInstancedEXT"                     .as_ptr()) as PFNGLDRAWELEMENTSINSTANCEDEXTPROC                   );
        }
    }
    pub(super) unsafe fn gl_load_GL_EXT_fog_coord(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_EXT_fog_coord { return; }
            self.glFogCoordPointerEXT                          = load!((load)(userptr, c"glFogCoordPointerEXT"                           .as_ptr()) as PFNGLFOGCOORDPOINTEREXTPROC                         );
            self.glFogCoorddEXT                                = load!((load)(userptr, c"glFogCoorddEXT"                                 .as_ptr()) as PFNGLFOGCOORDDEXTPROC                               );
            self.glFogCoorddvEXT                               = load!((load)(userptr, c"glFogCoorddvEXT"                                .as_ptr()) as PFNGLFOGCOORDDVEXTPROC                              );
            self.glFogCoordfEXT                                = load!((load)(userptr, c"glFogCoordfEXT"                                 .as_ptr()) as PFNGLFOGCOORDFEXTPROC                               );
            self.glFogCoordfvEXT                               = load!((load)(userptr, c"glFogCoordfvEXT"                                .as_ptr()) as PFNGLFOGCOORDFVEXTPROC                              );
        }
    }
    pub(super) unsafe fn gl_load_GL_EXT_framebuffer_blit(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_EXT_framebuffer_blit { return; }
            self.glBlitFramebufferEXT                          = load!((load)(userptr, c"glBlitFramebufferEXT"                           .as_ptr()) as PFNGLBLITFRAMEBUFFEREXTPROC                         );
        }
    }
    pub(super) unsafe fn gl_load_GL_EXT_framebuffer_multisample(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_EXT_framebuffer_multisample { return; }
            self.glRenderbufferStorageMultisampleEXT           = load!((load)(userptr, c"glRenderbufferStorageMultisampleEXT"            .as_ptr()) as PFNGLRENDERBUFFERSTORAGEMULTISAMPLEEXTPROC          );
        }
    }
    pub(super) unsafe fn gl_load_GL_EXT_framebuffer_object(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_EXT_framebuffer_object { return; }
            self.glBindFramebufferEXT                          = load!((load)(userptr, c"glBindFramebufferEXT"                           .as_ptr()) as PFNGLBINDFRAMEBUFFEREXTPROC                         );
            self.glBindRenderbufferEXT                         = load!((load)(userptr, c"glBindRenderbufferEXT"                          .as_ptr()) as PFNGLBINDRENDERBUFFEREXTPROC                        );
            self.glCheckFramebufferStatusEXT                   = load!((load)(userptr, c"glCheckFramebufferStatusEXT"                    .as_ptr()) as PFNGLCHECKFRAMEBUFFERSTATUSEXTPROC                  );
            self.glDeleteFramebuffersEXT                       = load!((load)(userptr, c"glDeleteFramebuffersEXT"                        .as_ptr()) as PFNGLDELETEFRAMEBUFFERSEXTPROC                      );
            self.glDeleteRenderbuffersEXT                      = load!((load)(userptr, c"glDeleteRenderbuffersEXT"                       .as_ptr()) as PFNGLDELETERENDERBUFFERSEXTPROC                     );
            self.glFramebufferRenderbufferEXT                  = load!((load)(userptr, c"glFramebufferRenderbufferEXT"                   .as_ptr()) as PFNGLFRAMEBUFFERRENDERBUFFEREXTPROC                 );
            self.glFramebufferTexture1DEXT                     = load!((load)(userptr, c"glFramebufferTexture1DEXT"                      .as_ptr()) as PFNGLFRAMEBUFFERTEXTURE1DEXTPROC                    );
            self.glFramebufferTexture2DEXT                     = load!((load)(userptr, c"glFramebufferTexture2DEXT"                      .as_ptr()) as PFNGLFRAMEBUFFERTEXTURE2DEXTPROC                    );
            self.glFramebufferTexture3DEXT                     = load!((load)(userptr, c"glFramebufferTexture3DEXT"                      .as_ptr()) as PFNGLFRAMEBUFFERTEXTURE3DEXTPROC                    );
            self.glGenFramebuffersEXT                          = load!((load)(userptr, c"glGenFramebuffersEXT"                           .as_ptr()) as PFNGLGENFRAMEBUFFERSEXTPROC                         );
            self.glGenRenderbuffersEXT                         = load!((load)(userptr, c"glGenRenderbuffersEXT"                          .as_ptr()) as PFNGLGENRENDERBUFFERSEXTPROC                        );
            self.glGenerateMipmapEXT                           = load!((load)(userptr, c"glGenerateMipmapEXT"                            .as_ptr()) as PFNGLGENERATEMIPMAPEXTPROC                          );
            self.glGetFramebufferAttachmentParameterivEXT      = load!((load)(userptr, c"glGetFramebufferAttachmentParameterivEXT"       .as_ptr()) as PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVEXTPROC     );
            self.glGetRenderbufferParameterivEXT               = load!((load)(userptr, c"glGetRenderbufferParameterivEXT"                .as_ptr()) as PFNGLGETRENDERBUFFERPARAMETERIVEXTPROC              );
            self.glIsFramebufferEXT                            = load!((load)(userptr, c"glIsFramebufferEXT"                             .as_ptr()) as PFNGLISFRAMEBUFFEREXTPROC                           );
            self.glIsRenderbufferEXT                           = load!((load)(userptr, c"glIsRenderbufferEXT"                            .as_ptr()) as PFNGLISRENDERBUFFEREXTPROC                          );
            self.glRenderbufferStorageEXT                      = load!((load)(userptr, c"glRenderbufferStorageEXT"                       .as_ptr()) as PFNGLRENDERBUFFERSTORAGEEXTPROC                     );
        }
    }
    pub(super) unsafe fn gl_load_GL_OES_fixed_point(&mut self, compat: &GladExtCompat, load: GLADuserptrloadfunc, userptr: *mut c_void) {
        unsafe {
            if !compat.GL_OES_fixed_point { return; }
            self.glAccumxOES                                   = load!((load)(userptr, c"glAccumxOES"                                    .as_ptr()) as PFNGLACCUMXOESPROC                                  );
            self.glAlphaFuncxOES                               = load!((load)(userptr, c"glAlphaFuncxOES"                                .as_ptr()) as PFNGLALPHAFUNCXOESPROC                              );
            self.glBitmapxOES                                  = load!((load)(userptr, c"glBitmapxOES"                                   .as_ptr()) as PFNGLBITMAPXOESPROC                                 );
            self.glBlendColorxOES                              = load!((load)(userptr, c"glBlendColorxOES"                               .as_ptr()) as PFNGLBLENDCOLORXOESPROC                             );
            self.glClearAccumxOES                              = load!((load)(userptr, c"glClearAccumxOES"                               .as_ptr()) as PFNGLCLEARACCUMXOESPROC                             );
            self.glClearColorxOES                              = load!((load)(userptr, c"glClearColorxOES"                               .as_ptr()) as PFNGLCLEARCOLORXOESPROC                             );
            self.glClearDepthxOES                              = load!((load)(userptr, c"glClearDepthxOES"                               .as_ptr()) as PFNGLCLEARDEPTHXOESPROC                             );
            self.glClipPlanexOES                               = load!((load)(userptr, c"glClipPlanexOES"                                .as_ptr()) as PFNGLCLIPPLANEXOESPROC                              );
            self.glColor3xOES                                  = load!((load)(userptr, c"glColor3xOES"                                   .as_ptr()) as PFNGLCOLOR3XOESPROC                                 );
            self.glColor3xvOES                                 = load!((load)(userptr, c"glColor3xvOES"                                  .as_ptr()) as PFNGLCOLOR3XVOESPROC                                );
            self.glColor4xOES                                  = load!((load)(userptr, c"glColor4xOES"                                   .as_ptr()) as PFNGLCOLOR4XOESPROC                                 );
            self.glColor4xvOES                                 = load!((load)(userptr, c"glColor4xvOES"                                  .as_ptr()) as PFNGLCOLOR4XVOESPROC                                );
            self.glConvolutionParameterxOES                    = load!((load)(userptr, c"glConvolutionParameterxOES"                     .as_ptr()) as PFNGLCONVOLUTIONPARAMETERXOESPROC                   );
            self.glConvolutionParameterxvOES                   = load!((load)(userptr, c"glConvolutionParameterxvOES"                    .as_ptr()) as PFNGLCONVOLUTIONPARAMETERXVOESPROC                  );
            self.glDepthRangexOES                              = load!((load)(userptr, c"glDepthRangexOES"                               .as_ptr()) as PFNGLDEPTHRANGEXOESPROC                             );
            self.glEvalCoord1xOES                              = load!((load)(userptr, c"glEvalCoord1xOES"                               .as_ptr()) as PFNGLEVALCOORD1XOESPROC                             );
            self.glEvalCoord1xvOES                             = load!((load)(userptr, c"glEvalCoord1xvOES"                              .as_ptr()) as PFNGLEVALCOORD1XVOESPROC                            );
            self.glEvalCoord2xOES                              = load!((load)(userptr, c"glEvalCoord2xOES"                               .as_ptr()) as PFNGLEVALCOORD2XOESPROC                             );
            self.glEvalCoord2xvOES                             = load!((load)(userptr, c"glEvalCoord2xvOES"                              .as_ptr()) as PFNGLEVALCOORD2XVOESPROC                            );
            self.glFeedbackBufferxOES                          = load!((load)(userptr, c"glFeedbackBufferxOES"                           .as_ptr()) as PFNGLFEEDBACKBUFFERXOESPROC                         );
            self.glFogxOES                                     = load!((load)(userptr, c"glFogxOES"                                      .as_ptr()) as PFNGLFOGXOESPROC                                    );
            self.glFogxvOES                                    = load!((load)(userptr, c"glFogxvOES"                                     .as_ptr()) as PFNGLFOGXVOESPROC                                   );
            self.glFrustumxOES                                 = load!((load)(userptr, c"glFrustumxOES"                                  .as_ptr()) as PFNGLFRUSTUMXOESPROC                                );
            self.glGetClipPlanexOES                            = load!((load)(userptr, c"glGetClipPlanexOES"                             .as_ptr()) as PFNGLGETCLIPPLANEXOESPROC                           );
            self.glGetConvolutionParameterxvOES                = load!((load)(userptr, c"glGetConvolutionParameterxvOES"                 .as_ptr()) as PFNGLGETCONVOLUTIONPARAMETERXVOESPROC               );
            self.glGetFixedvOES                                = load!((load)(userptr, c"glGetFixedvOES"                                 .as_ptr()) as PFNGLGETFIXEDVOESPROC                               );
            self.glGetHistogramParameterxvOES                  = load!((load)(userptr, c"glGetHistogramParameterxvOES"                   .as_ptr()) as PFNGLGETHISTOGRAMPARAMETERXVOESPROC                 );
            self.glGetLightxOES                                = load!((load)(userptr, c"glGetLightxOES"                                 .as_ptr()) as PFNGLGETLIGHTXOESPROC                               );
            self.glGetMapxvOES                                 = load!((load)(userptr, c"glGetMapxvOES"                                  .as_ptr()) as PFNGLGETMAPXVOESPROC                                );
            self.glGetMaterialxOES                             = load!((load)(userptr, c"glGetMaterialxOES"                              .as_ptr()) as PFNGLGETMATERIALXOESPROC                            );
            self.glGetPixelMapxv                               = load!((load)(userptr, c"glGetPixelMapxv"                                .as_ptr()) as PFNGLGETPIXELMAPXVPROC                              );
            self.glGetTexEnvxvOES                              = load!((load)(userptr, c"glGetTexEnvxvOES"                               .as_ptr()) as PFNGLGETTEXENVXVOESPROC                             );
            self.glGetTexGenxvOES                              = load!((load)(userptr, c"glGetTexGenxvOES"                               .as_ptr()) as PFNGLGETTEXGENXVOESPROC                             );
            self.glGetTexLevelParameterxvOES                   = load!((load)(userptr, c"glGetTexLevelParameterxvOES"                    .as_ptr()) as PFNGLGETTEXLEVELPARAMETERXVOESPROC                  );
            self.glGetTexParameterxvOES                        = load!((load)(userptr, c"glGetTexParameterxvOES"                         .as_ptr()) as PFNGLGETTEXPARAMETERXVOESPROC                       );
            self.glIndexxOES                                   = load!((load)(userptr, c"glIndexxOES"                                    .as_ptr()) as PFNGLINDEXXOESPROC                                  );
            self.glIndexxvOES                                  = load!((load)(userptr, c"glIndexxvOES"                                   .as_ptr()) as PFNGLINDEXXVOESPROC                                 );
            self.glLightModelxOES                              = load!((load)(userptr, c"glLightModelxOES"                               .as_ptr()) as PFNGLLIGHTMODELXOESPROC                             );
            self.glLightModelxvOES                             = load!((load)(userptr, c"glLightModelxvOES"                              .as_ptr()) as PFNGLLIGHTMODELXVOESPROC                            );
            self.glLightxOES                                   = load!((load)(userptr, c"glLightxOES"                                    .as_ptr()) as PFNGLLIGHTXOESPROC                                  );
            self.glLightxvOES                                  = load!((load)(userptr, c"glLightxvOES"                                   .as_ptr()) as PFNGLLIGHTXVOESPROC                                 );
            self.glLineWidthxOES                               = load!((load)(userptr, c"glLineWidthxOES"                                .as_ptr()) as PFNGLLINEWIDTHXOESPROC                              );
            self.glLoadMatrixxOES                              = load!((load)(userptr, c"glLoadMatrixxOES"                               .as_ptr()) as PFNGLLOADMATRIXXOESPROC                             );
            self.glLoadTransposeMatrixxOES                     = load!((load)(userptr, c"glLoadTransposeMatrixxOES"                      .as_ptr()) as PFNGLLOADTRANSPOSEMATRIXXOESPROC                    );
            self.glMap1xOES                                    = load!((load)(userptr, c"glMap1xOES"                                     .as_ptr()) as PFNGLMAP1XOESPROC                                   );
            self.glMap2xOES                                    = load!((load)(userptr, c"glMap2xOES"                                     .as_ptr()) as PFNGLMAP2XOESPROC                                   );
            self.glMapGrid1xOES                                = load!((load)(userptr, c"glMapGrid1xOES"                                 .as_ptr()) as PFNGLMAPGRID1XOESPROC                               );
            self.glMapGrid2xOES                                = load!((load)(userptr, c"glMapGrid2xOES"                                 .as_ptr()) as PFNGLMAPGRID2XOESPROC                               );
            self.glMaterialxOES                                = load!((load)(userptr, c"glMaterialxOES"                                 .as_ptr()) as PFNGLMATERIALXOESPROC                               );
            self.glMaterialxvOES                               = load!((load)(userptr, c"glMaterialxvOES"                                .as_ptr()) as PFNGLMATERIALXVOESPROC                              );
            self.glMultMatrixxOES                              = load!((load)(userptr, c"glMultMatrixxOES"                               .as_ptr()) as PFNGLMULTMATRIXXOESPROC                             );
            self.glMultTransposeMatrixxOES                     = load!((load)(userptr, c"glMultTransposeMatrixxOES"                      .as_ptr()) as PFNGLMULTTRANSPOSEMATRIXXOESPROC                    );
            self.glMultiTexCoord1xOES                          = load!((load)(userptr, c"glMultiTexCoord1xOES"                           .as_ptr()) as PFNGLMULTITEXCOORD1XOESPROC                         );
            self.glMultiTexCoord1xvOES                         = load!((load)(userptr, c"glMultiTexCoord1xvOES"                          .as_ptr()) as PFNGLMULTITEXCOORD1XVOESPROC                        );
            self.glMultiTexCoord2xOES                          = load!((load)(userptr, c"glMultiTexCoord2xOES"                           .as_ptr()) as PFNGLMULTITEXCOORD2XOESPROC                         );
            self.glMultiTexCoord2xvOES                         = load!((load)(userptr, c"glMultiTexCoord2xvOES"                          .as_ptr()) as PFNGLMULTITEXCOORD2XVOESPROC                        );
            self.glMultiTexCoord3xOES                          = load!((load)(userptr, c"glMultiTexCoord3xOES"                           .as_ptr()) as PFNGLMULTITEXCOORD3XOESPROC                         );
            self.glMultiTexCoord3xvOES                         = load!((load)(userptr, c"glMultiTexCoord3xvOES"                          .as_ptr()) as PFNGLMULTITEXCOORD3XVOESPROC                        );
            self.glMultiTexCoord4xOES                          = load!((load)(userptr, c"glMultiTexCoord4xOES"                           .as_ptr()) as PFNGLMULTITEXCOORD4XOESPROC                         );
            self.glMultiTexCoord4xvOES                         = load!((load)(userptr, c"glMultiTexCoord4xvOES"                          .as_ptr()) as PFNGLMULTITEXCOORD4XVOESPROC                        );
            self.glNormal3xOES                                 = load!((load)(userptr, c"glNormal3xOES"                                  .as_ptr()) as PFNGLNORMAL3XOESPROC                                );
            self.glNormal3xvOES                                = load!((load)(userptr, c"glNormal3xvOES"                                 .as_ptr()) as PFNGLNORMAL3XVOESPROC                               );
            self.glOrthoxOES                                   = load!((load)(userptr, c"glOrthoxOES"                                    .as_ptr()) as PFNGLORTHOXOESPROC                                  );
            self.glPassThroughxOES                             = load!((load)(userptr, c"glPassThroughxOES"                              .as_ptr()) as PFNGLPASSTHROUGHXOESPROC                            );
            self.glPixelMapx                                   = load!((load)(userptr, c"glPixelMapx"                                    .as_ptr()) as PFNGLPIXELMAPXPROC                                  );
            self.glPixelStorex                                 = load!((load)(userptr, c"glPixelStorex"                                  .as_ptr()) as PFNGLPIXELSTOREXPROC                                );
            self.glPixelTransferxOES                           = load!((load)(userptr, c"glPixelTransferxOES"                            .as_ptr()) as PFNGLPIXELTRANSFERXOESPROC                          );
            self.glPixelZoomxOES                               = load!((load)(userptr, c"glPixelZoomxOES"                                .as_ptr()) as PFNGLPIXELZOOMXOESPROC                              );
            self.glPointParameterxvOES                         = load!((load)(userptr, c"glPointParameterxvOES"                          .as_ptr()) as PFNGLPOINTPARAMETERXVOESPROC                        );
            self.glPointSizexOES                               = load!((load)(userptr, c"glPointSizexOES"                                .as_ptr()) as PFNGLPOINTSIZEXOESPROC                              );
            self.glPolygonOffsetxOES                           = load!((load)(userptr, c"glPolygonOffsetxOES"                            .as_ptr()) as PFNGLPOLYGONOFFSETXOESPROC                          );
            self.glPrioritizeTexturesxOES                      = load!((load)(userptr, c"glPrioritizeTexturesxOES"                       .as_ptr()) as PFNGLPRIORITIZETEXTURESXOESPROC                     );
            self.glRasterPos2xOES                              = load!((load)(userptr, c"glRasterPos2xOES"                               .as_ptr()) as PFNGLRASTERPOS2XOESPROC                             );
            self.glRasterPos2xvOES                             = load!((load)(userptr, c"glRasterPos2xvOES"                              .as_ptr()) as PFNGLRASTERPOS2XVOESPROC                            );
            self.glRasterPos3xOES                              = load!((load)(userptr, c"glRasterPos3xOES"                               .as_ptr()) as PFNGLRASTERPOS3XOESPROC                             );
            self.glRasterPos3xvOES                             = load!((load)(userptr, c"glRasterPos3xvOES"                              .as_ptr()) as PFNGLRASTERPOS3XVOESPROC                            );
            self.glRasterPos4xOES                              = load!((load)(userptr, c"glRasterPos4xOES"                               .as_ptr()) as PFNGLRASTERPOS4XOESPROC                             );
            self.glRasterPos4xvOES                             = load!((load)(userptr, c"glRasterPos4xvOES"                              .as_ptr()) as PFNGLRASTERPOS4XVOESPROC                            );
            self.glRectxOES                                    = load!((load)(userptr, c"glRectxOES"                                     .as_ptr()) as PFNGLRECTXOESPROC                                   );
            self.glRectxvOES                                   = load!((load)(userptr, c"glRectxvOES"                                    .as_ptr()) as PFNGLRECTXVOESPROC                                  );
            self.glRotatexOES                                  = load!((load)(userptr, c"glRotatexOES"                                   .as_ptr()) as PFNGLROTATEXOESPROC                                 );
            self.glScalexOES                                   = load!((load)(userptr, c"glScalexOES"                                    .as_ptr()) as PFNGLSCALEXOESPROC                                  );
            self.glTexCoord1xOES                               = load!((load)(userptr, c"glTexCoord1xOES"                                .as_ptr()) as PFNGLTEXCOORD1XOESPROC                              );
            self.glTexCoord1xvOES                              = load!((load)(userptr, c"glTexCoord1xvOES"                               .as_ptr()) as PFNGLTEXCOORD1XVOESPROC                             );
            self.glTexCoord2xOES                               = load!((load)(userptr, c"glTexCoord2xOES"                                .as_ptr()) as PFNGLTEXCOORD2XOESPROC                              );
            self.glTexCoord2xvOES                              = load!((load)(userptr, c"glTexCoord2xvOES"                               .as_ptr()) as PFNGLTEXCOORD2XVOESPROC                             );
            self.glTexCoord3xOES                               = load!((load)(userptr, c"glTexCoord3xOES"                                .as_ptr()) as PFNGLTEXCOORD3XOESPROC                              );
            self.glTexCoord3xvOES                              = load!((load)(userptr, c"glTexCoord3xvOES"                               .as_ptr()) as PFNGLTEXCOORD3XVOESPROC                             );
            self.glTexCoord4xOES                               = load!((load)(userptr, c"glTexCoord4xOES"                                .as_ptr()) as PFNGLTEXCOORD4XOESPROC                              );
            self.glTexCoord4xvOES                              = load!((load)(userptr, c"glTexCoord4xvOES"                               .as_ptr()) as PFNGLTEXCOORD4XVOESPROC                             );
            self.glTexEnvxOES                                  = load!((load)(userptr, c"glTexEnvxOES"                                   .as_ptr()) as PFNGLTEXENVXOESPROC                                 );
            self.glTexEnvxvOES                                 = load!((load)(userptr, c"glTexEnvxvOES"                                  .as_ptr()) as PFNGLTEXENVXVOESPROC                                );
            self.glTexGenxOES                                  = load!((load)(userptr, c"glTexGenxOES"                                   .as_ptr()) as PFNGLTEXGENXOESPROC                                 );
            self.glTexGenxvOES                                 = load!((load)(userptr, c"glTexGenxvOES"                                  .as_ptr()) as PFNGLTEXGENXVOESPROC                                );
            self.glTexParameterxOES                            = load!((load)(userptr, c"glTexParameterxOES"                             .as_ptr()) as PFNGLTEXPARAMETERXOESPROC                           );
            self.glTexParameterxvOES                           = load!((load)(userptr, c"glTexParameterxvOES"                            .as_ptr()) as PFNGLTEXPARAMETERXVOESPROC                          );
            self.glTranslatexOES                               = load!((load)(userptr, c"glTranslatexOES"                                .as_ptr()) as PFNGLTRANSLATEXOESPROC                              );
            self.glVertex2xOES                                 = load!((load)(userptr, c"glVertex2xOES"                                  .as_ptr()) as PFNGLVERTEX2XOESPROC                                );
            self.glVertex2xvOES                                = load!((load)(userptr, c"glVertex2xvOES"                                 .as_ptr()) as PFNGLVERTEX2XVOESPROC                               );
            self.glVertex3xOES                                 = load!((load)(userptr, c"glVertex3xOES"                                  .as_ptr()) as PFNGLVERTEX3XOESPROC                                );
            self.glVertex3xvOES                                = load!((load)(userptr, c"glVertex3xvOES"                                 .as_ptr()) as PFNGLVERTEX3XVOESPROC                               );
            self.glVertex4xOES                                 = load!((load)(userptr, c"glVertex4xOES"                                  .as_ptr()) as PFNGLVERTEX4XOESPROC                                );
            self.glVertex4xvOES                                = load!((load)(userptr, c"glVertex4xvOES"                                 .as_ptr()) as PFNGLVERTEX4XVOESPROC                               );
        }
    }
}
