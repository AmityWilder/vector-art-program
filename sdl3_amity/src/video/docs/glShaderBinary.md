# glShaderBinary
load pre-compiled shader binaries

## Parameters
- `count`
  Specifies the number of shader object handles contained in `shaders`.

## Description
[`Gl::shader_binary`] loads pre-compiled shader binary code into the
  `count` shader objects whose handles are given in `shaders`. `binary`
  points to `length` bytes of binary shader code stored in client
  memory. `binaryFormat` specifies the format of the pre-compiled code.
The binary image contained in `binary` will be decoded according to
  the extension specification defining the specified `binaryFormat`
  token. OpenGL does not define any specific binary formats, but it does
  provide a mechanism to obtain token vaues for such formats provided by
  such extensions.
Depending on the types of the shader objects in `shaders`,
  [`Gl::shader_binary`] will individually load binary vertex or fragment
  shaders, or load an executable binary that contains an optimized pair
  of vertex and fragment shaders stored in the same binary.

## Errors
- [`gl::INVALID_OPERATION`] is generated if more than one of the handles
  in `shaders` refers to the same shader object.
- [`gl::INVALID_ENUM`] is generated if `binaryFormat` is not an accepted
  value.
- [`gl::INVALID_VALUE`] is generated if the data pointed to by `binary`
  does not match the format specified by `binaryFormat`.

## See Also
- [`Gl::get_program`]
- [`Gl::get_program_binary`]
- [`Gl::program_binary`]
