# glReleaseShaderCompiler
release resources consumed by the implementation's shader compiler

## Description
[`Gl::release_shader_compiler`] provides a hint to the implementation
  that it may free internal resources associated with its shader
  compiler. [`Gl::compile_shader`] may subsequently be called and the
  implementation may at that time reallocate resources previously freed
  by the call to [`Gl::release_shader_compiler`].

## See Also
- [`Gl::compile_shader`]
- [`Gl::link_program`]
