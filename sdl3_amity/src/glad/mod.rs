#![allow(nonstandard_style)]
use std::ffi::*;
use crate::amy_util::cstr_from;

#[macro_use]
pub mod items;
use items::*;

mod GLAD_PLATFORM_H_ {
    use std::ffi::*;

    #[inline(always)]
    pub const fn GLAD_MAKE_VERSION(major: c_int, minor: c_int) -> c_int {
        major * 10000 + minor
    }
    #[inline(always)]
    pub const fn GLAD_VERSION_MAJOR(version: c_int) -> c_int {
        version / 10000
    }
    #[inline(always)]
    pub const fn GLAD_VERSION_MINOR(version: c_int) -> c_int {
        version % 10000
    }

    pub const GLAD_GENERATOR_VERSION: &CStr = c"2.0.0-beta";

    pub type GLADapiproc = unsafe extern "C" fn();

    pub type GLADloadfunc = unsafe extern "C" fn(name: *const c_char) -> Option<GLADapiproc>;
    pub type GLADuserptrloadfunc = unsafe fn(userptr: *mut c_void, name: *const c_char) -> Option<GLADapiproc>;

    pub type GLADprecallback = unsafe extern "C" fn(name: *const c_char, apiproc: Option<GLADapiproc>, len_args: c_int, ...);
    pub type GLADpostcallback = unsafe extern "C" fn(ret: *mut c_void, name: *const c_char, apiproc: Option<GLADapiproc>, len_args: c_int, ...);
}
pub use GLAD_PLATFORM_H_::*;

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
pub type GLchar = c_char;
pub type GLcharARB = c_char;
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
pub type GLDEBUGPROC      = unsafe extern "C" fn(source: GLenum, r#type: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *const c_void);
pub type GLDEBUGPROCARB   = unsafe extern "C" fn(source: GLenum, r#type: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *const c_void);
pub type GLDEBUGPROCKHR   = unsafe extern "C" fn(source: GLenum, r#type: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *const c_void);
pub type GLDEBUGPROCAMD   = unsafe extern "C" fn(id: GLuint, category: GLenum, severity: GLenum,               length: GLsizei, message: *const GLchar, userParam: *mut   c_void);
pub type GLhalfNV         = c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;
pub type GLVULKANPROCNV   = unsafe extern "C" fn();

pub const GLAD_GL_IS_SOME_NEW_VERSION: bool = true;

unsafe fn glad_gl_has_extension(version: c_int, exts: &Option<&'static CStr>, exts_i: &Option<Box<[Option<CString>]>>, ext: &CStr) -> bool {
    unsafe {
        if GLAD_VERSION_MAJOR(version) < 3 || !GLAD_GL_IS_SOME_NEW_VERSION {
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

unsafe fn glad_gl_get_proc_from_userptr(userptr: *mut c_void, name: *const c_char) -> GLADapiproc {
    unsafe {
        let f: unsafe extern "C" fn(*const c_char) -> GLADapiproc = std::mem::transmute(userptr);
        f(name)
    }
}

impl Glad {
    unsafe fn gl_get_extensions(&self, version: c_int) -> Result<(Option<&'static CStr>, c_uint, Option<Box<[Option<CString>]>>), ()> {
        unsafe {
            if GLAD_VERSION_MAJOR(version) < 3 {
                if let Some(glGetString) = self.glGetString {
                    let ptr: *const c_char = glGetString(GL_EXTENSIONS as GLenum).cast();
                    Ok((
                        (!ptr.is_null()).then(|| CStr::from_ptr(ptr)),
                        0,
                        None,
                    ))
                } else {
                    Err(())
                }
            } else {
                if let (Some(glGetStringi), Some(glGetIntegerv)) = (self.glGetStringi, self.glGetIntegerv) {
                    let mut num_exts_i: c_uint = 0;
                    glGetIntegerv(GL_NUM_EXTENSIONS as GLenum, &raw mut num_exts_i as *mut c_int);
                    if num_exts_i > 0 {
                        let mut exts_i_arr: Box<[Option<CString>]> = std::iter::from_fn(|| None).take(num_exts_i as usize).collect();

                        for index in 0..num_exts_i {
                            let gl_str_tmp: *const c_char = glGetStringi(GL_EXTENSIONS as GLenum, index).cast();
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

    unsafe fn gl_find_extensions_gl(&mut self, version: c_int) -> Result<GladExtCompat, ()> {
        unsafe {
            if let Ok((exts, _num_exts_i, exts_i)) = self.gl_get_extensions(version) {
                Ok(GladExtCompat {
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

    unsafe fn gl_find_core_gl(glGetString: &PFNGLGETSTRINGPROC) -> Result<(c_int, GladVersionCompat), ()> {
        let prefixes: [&CStr; 4] = [
            c"OpenGL ES-CM ",
            c"OpenGL ES-CL ",
            c"OpenGL ES ",
            c"OpenGL SC ",
        ];
        unsafe {
            let version = cstr_from(glGetString(GL_VERSION.into()).cast());
            if let Some(version) = version {
                let version_num = prefixes.iter().find_map(|prefix| {
                    version.to_bytes().starts_with(prefix.to_bytes())
                        .then(|| &version[prefix.count_bytes()..])
                });
                if let Some(version) = version_num {
                    let version_bytes = version.to_bytes();
                    let split_point = version_bytes.iter().position(|&c| c == b'.').ok_or(())?;
                    let (major_str, minor_str) = version_bytes.split_at(split_point);
                    let major: c_int = str::from_utf8(major_str).map_err(|_| ())?.parse().map_err(|_| ())?;
                    let minor: c_int = str::from_utf8(minor_str).map_err(|_| ())?.parse().map_err(|_| ())?;

                    let compat = GladVersionCompat {
                        GL_VERSION_1_0: !matches!((major, minor), (..1, _) | (1, ..0)),
                        GL_VERSION_1_1: !matches!((major, minor), (..1, _) | (1, ..1)),
                        GL_VERSION_1_2: !matches!((major, minor), (..1, _) | (1, ..2)),
                        GL_VERSION_1_3: !matches!((major, minor), (..1, _) | (1, ..3)),
                        GL_VERSION_1_4: !matches!((major, minor), (..1, _) | (1, ..4)),
                        GL_VERSION_1_5: !matches!((major, minor), (..1, _) | (1, ..5)),

                        GL_VERSION_2_0: !matches!((major, minor), (..2, _) | (2, ..0)),
                        GL_VERSION_2_1: !matches!((major, minor), (..2, _) | (2, ..1)),

                        GL_VERSION_3_0: !matches!((major, minor), (..3, _) | (3, ..0)),
                        GL_VERSION_3_1: !matches!((major, minor), (..3, _) | (3, ..1)),
                        GL_VERSION_3_2: !matches!((major, minor), (..3, _) | (3, ..2)),
                        GL_VERSION_3_3: !matches!((major, minor), (..3, _) | (3, ..3)),

                        GL_VERSION_4_0: !matches!((major, minor), (..4, _) | (4, ..0)),
                        GL_VERSION_4_1: !matches!((major, minor), (..4, _) | (4, ..1)),
                        GL_VERSION_4_2: !matches!((major, minor), (..4, _) | (4, ..2)),
                        GL_VERSION_4_3: !matches!((major, minor), (..4, _) | (4, ..3)),
                    };

                    return Ok((GLAD_MAKE_VERSION(major, minor), compat));
                }
            }
            Err(())
        }
    }

    unsafe fn gladLoadGLUserPtr(&mut self, load: GLADuserptrloadfunc, userptr: *mut c_void) -> Result<(c_int, GladCompat), ()> {
        unsafe {
            if let Some(glGetString) = load!((load)(userptr, c"glGetString".as_ptr()) as PFNGLGETSTRINGPROC) {
                if !glGetString(GL_VERSION as GLenum).is_null() {
                    let (version, ver_compat) = Glad::gl_find_core_gl(&glGetString)?;

                    self.gl_load_GL_VERSION_1_0(&ver_compat, load, userptr);
                    self.gl_load_GL_VERSION_1_1(&ver_compat, load, userptr);
                    self.gl_load_GL_VERSION_1_2(&ver_compat, load, userptr);
                    self.gl_load_GL_VERSION_1_3(&ver_compat, load, userptr);
                    self.gl_load_GL_VERSION_1_4(&ver_compat, load, userptr);
                    self.gl_load_GL_VERSION_1_5(&ver_compat, load, userptr);
                    self.gl_load_GL_VERSION_2_0(&ver_compat, load, userptr);
                    self.gl_load_GL_VERSION_2_1(&ver_compat, load, userptr);
                    self.gl_load_GL_VERSION_3_0(&ver_compat, load, userptr);
                    self.gl_load_GL_VERSION_3_1(&ver_compat, load, userptr);
                    self.gl_load_GL_VERSION_3_2(&ver_compat, load, userptr);
                    self.gl_load_GL_VERSION_3_3(&ver_compat, load, userptr);
                    self.gl_load_GL_VERSION_4_0(&ver_compat, load, userptr);
                    self.gl_load_GL_VERSION_4_1(&ver_compat, load, userptr);
                    self.gl_load_GL_VERSION_4_2(&ver_compat, load, userptr);
                    self.gl_load_GL_VERSION_4_3(&ver_compat, load, userptr);

                    if let Ok(ext_compat) = self.gl_find_extensions_gl(version.into()) {
                        self.gl_load_GL_ARB_ES2_compatibility            (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_ES3_1_compatibility          (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_ES3_2_compatibility          (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_blend_func_extended          (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_buffer_storage               (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_clear_buffer_object          (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_clear_texture                (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_color_buffer_float           (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_compute_shader               (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_compute_variable_group_size  (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_copy_buffer                  (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_copy_image                   (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_debug_output                 (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_direct_state_access          (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_draw_buffers                 (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_draw_buffers_blend           (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_draw_elements_base_vertex    (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_draw_indirect                (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_draw_instanced               (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_fragment_program             (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_framebuffer_no_attachments   (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_framebuffer_object           (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_geometry_shader4             (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_get_program_binary           (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_get_texture_sub_image        (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_gl_spirv                     (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_gpu_shader_fp64              (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_gpu_shader_int64             (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_instanced_arrays             (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_internalformat_query         (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_internalformat_query2        (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_map_buffer_range             (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_multi_bind                   (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_multi_draw_indirect          (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_multisample                  (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_multitexture                 (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_occlusion_query              (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_sample_locations             (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_sample_shading               (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_shader_atomic_counters       (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_shader_image_load_store      (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_shader_objects               (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_shader_storage_buffer_object (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_shading_language_include     (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_tessellation_shader          (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_texture_compression          (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_texture_multisample          (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_texture_storage              (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_texture_view                 (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_timer_query                  (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_transpose_matrix             (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_uniform_buffer_object        (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_vertex_array_object          (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_vertex_attrib_binding        (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_vertex_buffer_object         (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_vertex_program               (&ext_compat, load, userptr);
                        self.gl_load_GL_ARB_vertex_shader                (&ext_compat, load, userptr);
                        self.gl_load_GL_EXT_draw_instanced               (&ext_compat, load, userptr);
                        self.gl_load_GL_EXT_fog_coord                    (&ext_compat, load, userptr);
                        self.gl_load_GL_EXT_framebuffer_blit             (&ext_compat, load, userptr);
                        self.gl_load_GL_EXT_framebuffer_multisample      (&ext_compat, load, userptr);
                        self.gl_load_GL_EXT_framebuffer_object           (&ext_compat, load, userptr);
                        self.gl_load_GL_OES_fixed_point                  (&ext_compat, load, userptr);

                        return Ok((version, GladCompat(ver_compat, ext_compat)));
                    }
                }
            }
            Err(())
        }
    }

    pub fn load(load: GLADloadfunc) -> Result<(Self, c_int, GladCompat), ()> {
        let mut glad = Self::default();
        let (version, compat) = unsafe {
            let userptr: *mut c_void = std::mem::transmute(load);
            let get_proc: unsafe fn(userptr: *mut c_void, name: *const c_char) -> GLADapiproc = glad_gl_get_proc_from_userptr;
            let load: GLADuserptrloadfunc = std::mem::transmute(get_proc);
            glad.gladLoadGLUserPtr(load, userptr)?
        };
        Ok((glad, version, compat))
    }
}
