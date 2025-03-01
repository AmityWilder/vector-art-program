# glRenderMode
set rasterization mode

## Parameters
- `mode`
  Specifies the rasterization mode. Three values are accepted:
  [`gl::RENDER`], [`gl::SELECT`], and [`gl::FEEDBACK`]. The initial
  value is [`gl::RENDER`].

## Description
[`Gl::render_mode`] sets the rasterization mode. It takes one
  argument, `mode`, which can assume one of three predefined values:
The return value of [`Gl::render_mode`] is determined by the render
  mode at the time [`Gl::render_mode`] is called, rather than by `mode`.
  The values returned for the three render modes are as follows:
See the [`Gl::select_buffer`] and [`Gl::feedback_buffer`] reference
  pages for more details concerning selection and feedback operation.

## Notes
If an error is generated, [`Gl::render_mode`] returns 0 regardless of
  the current render mode.

## Errors
- [`gl::INVALID_ENUM`] is generated if `mode` is not one of the three
  accepted values.
- [`gl::INVALID_OPERATION`] is generated if [`Gl::select_buffer`] is
  called while the render mode is [`gl::SELECT`], or if
  [`Gl::render_mode`] is called with argument [`gl::SELECT`] before
  [`Gl::select_buffer`] is called at least once.
- [`gl::INVALID_OPERATION`] is generated if [`Gl::feedback_buffer`] is
  called while the render mode is [`gl::FEEDBACK`], or if
  [`Gl::render_mode`] is called with argument [`gl::FEEDBACK`] before
  [`Gl::feedback_buffer`] is called at least once.
- [`gl::INVALID_OPERATION`] is generated if [`Gl::render_mode`] is
  executed between the execution of [`Gl::begin`] and the corresponding
  execution of [`Gl::end`].

## See Also
- [`Gl::feedback_buffer`]
- [`Gl::init_names`]
- [`Gl::load_name`]
- [`Gl::pass_through`]
- [`Gl::push_name`]
- [`Gl::select_buffer`]
