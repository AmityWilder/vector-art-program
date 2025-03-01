# glClearColor
specify clear values for the color buffers

## Parameters
- `red`
  Specify the red, green, blue, and alpha values used when the color
  buffers are cleared. The initial values are all 0.

## Description
[`Gl::clear_color`] specifies the red, green, blue, and alpha values
  used by [`Gl::clear`] to clear the color buffers. Values specified by
  [`Gl::clear_color`] are clamped to the range $None$. $$ $$ ```c ``` 0
  1

## Notes
The type of the `red`, `green`, `blue`, and `alpha` parameters was
  changed from GLclampf to GLfloat. This change is transparent to user
  code and is described in detail on the [`Gl::removed_types`] page.

## See Also
- [`Gl::clear`]
- [`Gl::removed_types`]
