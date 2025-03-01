# glPushAttrib
push and pop the server attribute stack

## Parameters
- `mask`
  Specifies a mask that indicates which attributes to save. Values for
  `mask` are listed below.

## Description
[`Gl::push_attrib`] takes one argument, a mask that indicates which
  groups of state variables to save on the attribute stack. Symbolic
  constants are used to set bits in the mask. `mask` is typically
  constructed by specifying the bitwise-or of several of these constants
  together. The special mask [`gl::ALL_ATTRIB_BITS`] can be used to save
  all stackable states.
The symbolic mask constants and their associated GL state are as
  follows (the second column lists which attributes are saved):

[`Gl::pop_attrib`] restores the values of the state variables saved
  with the last [`Gl::push_attrib`] command. Those not saved are left
  unchanged.
It is an error to push attributes onto a full stack or to pop
  attributes off an empty stack. In either case, the error flag is set
  and no other change is made to GL state.
Initially, the attribute stack is empty.

## Notes
Not all values for GL state can be saved on the attribute stack. For
  example, render mode state, and select and feedback state cannot be
  saved. Client state must be saved with [`Gl::push_client_attrib`].
The depth of the attribute stack depends on the implementation, but it
  must be at least 16.
For OpenGL versions 1.3 and greater, or when the ```c ARB_multitexture
  ``` extension is supported, pushing and popping texture state applies
  to all supported texture units.

## Errors
- [`gl::STACK_OVERFLOW`] is generated if [`Gl::push_attrib`] is called
  while the attribute stack is full.
- [`gl::STACK_UNDERFLOW`] is generated if [`Gl::pop_attrib`] is called
  while the attribute stack is empty.
- [`gl::INVALID_OPERATION`] is generated if [`Gl::push_attrib`] or
  [`Gl::pop_attrib`] is executed between the execution of [`Gl::begin`]
  and the corresponding execution of [`Gl::end`].

## See Also
- [`Gl::get`]
- [`Gl::get_clip_plane`]
- [`Gl::get_error`]
- [`Gl::get_light`]
- [`Gl::get_map`]
- [`Gl::get_material`]
- [`Gl::get_pixel_map`]
- [`Gl::get_polygon_stipple`]
- [`Gl::get_string`]
- [`Gl::get_tex_env`]
- [`Gl::get_tex_gen`]
- [`Gl::get_tex_image`]
- [`Gl::get_tex_level_parameter`]
- [`Gl::get_tex_parameter`]
- [`Gl::is_enabled`]
- [`Gl::push_client_attrib`]
