# glLogicOp
specify a logical pixel operation for rendering

## Parameters
- `opcode`
  Specifies a symbolic constant that selects a logical operation. The
  following symbols are accepted: [`gl::CLEAR`], [`gl::SET`],
  [`gl::COPY`], [`gl::COPY_INVERTED`], [`gl::NOOP`], [`gl::INVERT`],
  [`gl::AND`], [`gl::NAND`], [`gl::OR`], [`gl::NOR`], [`gl::XOR`],
  [`gl::EQUIV`], [`gl::AND_REVERSE`], [`gl::AND_INVERTED`],
  [`gl::OR_REVERSE`], and [`gl::OR_INVERTED`]. The initial value is
  [`gl::COPY`].

## Description
[`Gl::logic_op`] specifies a logical operation that, when enabled, is
  applied between the incoming RGBA color and the RGBA color at the
  corresponding location in the frame buffer. To enable or disable the
  logical operation, call [`Gl::enable`] and [`Gl::disable`] using the
  symbolic constant [`gl::COLOR_LOGIC_OP`]. The initial value is
  disabled.

`opcode` is a symbolic constant chosen from the list above. In the
  explanation of the logical operations, *s* represents the incoming
  color and *d* represents the color in the frame buffer. Standard
  C-language operators are used. As these bitwise operators suggest, the
  logical operation is applied independently to each bit pair of the
  source and destination colors.

## Notes
When more than one RGBA color buffer is enabled for drawing, logical
  operations are performed separately for each enabled buffer, using for
  the destination value the contents of that buffer (see
  [`Gl::draw_buffer`]).
Logic operations have no effect on floating point draw buffers.
  However, if [`gl::COLOR_LOGIC_OP`] is enabled, blending is still
  disabled in this case.

## Errors
- [`gl::INVALID_ENUM`] is generated if `opcode` is not an accepted
  value.

## See Also
- [`Gl::blend_func`]
- [`Gl::draw_buffer`]
- [`Gl::enable`]
- [`Gl::stencil_op`]
