# glPushClientAttrib
push and pop the client attribute stack

## Parameters
- `mask`
  Specifies a mask that indicates which attributes to save. Values for
  `mask` are listed below.

## Description
[`Gl::push_client_attrib`] takes one argument, a mask that indicates
  which groups of client-state variables to save on the client attribute
  stack. Symbolic constants are used to set bits in the mask. `mask` is
  typically constructed by specifying the bitwise-or of several of these
  constants together. The special mask [`gl::CLIENT_ALL_ATTRIB_BITS`]
  can be used to save all stackable client state.
The symbolic mask constants and their associated GL client state are
  as follows (the second column lists which attributes are saved):
[`gl::CLIENT_PIXEL_STORE_BIT`] Pixel storage modes
  [`gl::CLIENT_VERTEX_ARRAY_BIT`] Vertex arrays (and enables)
[`Gl::pop_client_attrib`] restores the values of the client-state
  variables saved with the last [`Gl::push_client_attrib`]. Those not
  saved are left unchanged.
It is an error to push attributes onto a full client attribute stack
  or to pop attributes off an empty stack. In either case, the error
  flag is set, and no other change is made to GL state.
Initially, the client attribute stack is empty.

## Notes
[`Gl::push_client_attrib`] is available only if the GL version is 1.1
  or greater.
Not all values for GL client state can be saved on the attribute
  stack. For example, select and feedback state cannot be saved.
The depth of the attribute stack depends on the implementation, but it
  must be at least 16.
Use [`Gl::push_attrib`] and [`Gl::pop_attrib`] to push and restore
  state that is kept on the server. Only pixel storage modes and vertex
  array state may be pushed and popped with [`Gl::push_client_attrib`]
  and [`Gl::pop_client_attrib`].
For OpenGL versions 1.3 and greater, or when the ```c ARB_multitexture
  ``` extension is supported, pushing and popping client vertex array
  state applies to all supported texture units, and the active client
  texture state.

## Errors
- [`gl::STACK_OVERFLOW`] is generated if [`Gl::push_client_attrib`] is
  called while the attribute stack is full.
- [`gl::STACK_UNDERFLOW`] is generated if [`Gl::pop_client_attrib`] is
  called while the attribute stack is empty.

## See Also
- [`Gl::color_pointer`]
- [`Gl::disable_client_state`]
- [`Gl::edge_flag_pointer`]
- [`Gl::enable_client_state`]
- [`Gl::fog_coord_pointer`]
- [`Gl::get`]
- [`Gl::get_error`]
- [`Gl::index_pointer`]
- [`Gl::normal_pointer`]
- [`Gl::new_list`]
- [`Gl::pixel_store`]
- [`Gl::push_attrib`]
- [`Gl::tex_coord_pointer`]
- [`Gl::vertex_pointer`]
