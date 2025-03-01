# glClearDepth
specify the clear value for the depth buffer

## Parameters
- `depth`
  Specifies the depth value used when the depth buffer is cleared. The
  initial value is 1.

## Description
[`Gl::clear_depth`] specifies the depth value used by [`Gl::clear`] to
  clear the depth buffer. Values specified by [`Gl::clear_depth`] are
  clamped to the range $None$. $$ $$ ```c ``` 0 1

## Notes
The type of the `depth` parameter was changed from GLclampf to GLfloat
  for [`Gl::clear_depthf`] and from GLclampd to GLdouble for
  [`Gl::clear_depth`]. This change is transparent to user code and is
  described in detail on the [`Gl::removed_types`] page.

## See Also
- [`Gl::clear`]
- [`Gl::removed_types`]
