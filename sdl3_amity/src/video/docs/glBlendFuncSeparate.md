# glBlendFuncSeparate
specify pixel arithmetic for RGB and alpha components separately

## Parameters
- `buf`
  For [`Gl::blend_func_separatei`], specifies the index of the draw
  buffer for which to set the blend functions.

## Description
Pixels can be drawn using a function that blends the incoming (source)
  RGBA values with the RGBA values that are already in the frame buffer
  (the destination values). Blending is initially disabled. Use
  [`Gl::enable`] and [`Gl::disable`] with argument [`gl::BLEND`] to
  enable and disable blending.
[`Gl::blend_func_separate`] defines the operation of blending for all
  draw buffers when it is enabled. [`Gl::blend_func_separatei`] defines
  the operation of blending for a single draw buffer specified by `buf`
  when enabled for that draw buffer. `srcRGB` specifies which method is
  used to scale the source RGB-color components. `dstRGB` specifies
  which method is used to scale the destination RGB-color components.
  Likewise, `srcAlpha` specifies which method is used to scale the
  source alpha color component, and `dstAlpha` specifies which method is
  used to scale the destination alpha component. The possible methods
  are described in the following table. Each method defines four scale
  factors, one each for red, green, blue, and alpha.
In the table and in subsequent equations, first source, second source
  and destination color components are referred to as $None$, $$ $$ ```c
  ``` _{None} *R* *s0* _{None} *G* *s0* _{None} *B* *s0* _{None} *A*
  *s0* $None$, and $$ $$ ```c ``` _{None} *R* *s1* _{None} *G* *s1*
  _{None} *B* *s1* _{None} *A* *s1* $None$, respectively. The color
  specified by $$ $$ ```c ``` _{None} *R* *d* _{None} *G* *d* _{None}
  *B* *d* _{None} *A* *d* [`Gl::blend_color`] is referred to as $None$.
  They are understood to have integer values between 0 and $$ $$ ```c
  ``` _{None} *R* *c* _{None} *G* *c* _{None} *B* *c* _{None} *A* *c*
  $None$, where $$ $$ ```c ``` _{None} *k* *R* _{None} *k* *G* _{None}
  *k* *B* _{None} *k* *A*
$None$ $$ $$ _{None} *k* *c* *=* ^{None} 2 ```c ``` _{None} *m* *c*
  *-* 1
and $None$ is the number of red, green, blue, and alpha bitplanes. $$
  $$ ```c ``` _{None} *m* *R* _{None} *m* *G* _{None} *m* *B* _{None}
  *m* *A*
Source and destination scale factors are referred to as $None$ and $$
  $$ ```c ``` _{None} *s* *R* _{None} *s* *G* _{None} *s* *B* _{None}
  *s* *A* $None$. All scale factors have range $$ $$ ```c ``` _{None}
  *d* *R* _{None} *d* *G* _{None} *d* *B* _{None} *d* *A* $None$. $$ $$
  ```c ``` 0 1

In the table,
$None$ $$ $$ *i* *=* *min* *\u{2061}* ```c ``` _{None} *A* *s* 1 *-*
  ```c ``` _{None} *A* *d*
To determine the blended RGBA values of a pixel, the system uses the
  following equations:
$None$ $$ $$ _{None} *R* *d* *=* *min* *\u{2061}* ```c ``` _{None} *k*
  *R* _{None} *R* *s* *\u{2062}* _{None} *s* *R* *+* _{None} *R* *d*
  *\u{2062}* _{None} *d* *R* $None$ $$ $$ _{None} *G* *d* *=* *min*
  *\u{2061}* ```c ``` _{None} *k* *G* _{None} *G* *s* *\u{2062}* _{None}
  *s* *G* *+* _{None} *G* *d* *\u{2062}* _{None} *d* *G* $None$ $$ $$
  _{None} *B* *d* *=* *min* *\u{2061}* ```c ``` _{None} *k* *B* _{None}
  *B* *s* *\u{2062}* _{None} *s* *B* *+* _{None} *B* *d* *\u{2062}*
  _{None} *d* *B* $None$ $$ $$ _{None} *A* *d* *=* *min* *\u{2061}* ```c
  ``` _{None} *k* *A* _{None} *A* *s* *\u{2062}* _{None} *s* *A* *+*
  _{None} *A* *d* *\u{2062}* _{None} *d* *A*
Despite the apparent precision of the above equations, blending
  arithmetic is not exactly specified, because blending operates with
  imprecise integer color values. However, a blend factor that should be
  equal to 1 is guaranteed not to modify its multiplicand, and a blend
  factor equal to 0 reduces its multiplicand to 0. For example, when
  `srcRGB` is [`gl::SRC_ALPHA`], `dstRGB` is
  [`gl::ONE_MINUS_SRC_ALPHA`], and $None$ is equal to $$ $$ _{None} *A*
  *s* $None$, the equations reduce to simple replacement: $$ $$ _{None}
  *k* *A*
$None$ $$ $$ _{None} *R* *d* *=* _{None} *R* *s* $None$ $$ $$ _{None}
  *G* *d* *=* _{None} *G* *s* $None$ $$ $$ _{None} *B* *d* *=* _{None}
  *B* *s* $None$ $$ $$ _{None} *A* *d* *=* _{None} *A* *s*


## Notes
Incoming (source) alpha is correctly thought of as a material opacity,
  ranging from 1.0 ( $None$), representing complete opacity, to 0.0 (0),
  representing complete transparency. $$ $$ _{None} *K* *A*
When more than one color buffer is enabled for drawing, the GL
  performs blending separately for each enabled buffer, using the
  contents of that buffer for destination color. (See
  [`Gl::draw_buffer`].)
When dual source blending is enabled (i.e., one of the blend factors
  requiring the second color input is used), the maximum number of
  enabled draw buffers is given by [`gl::MAX_DUAL_SOURCE_DRAW_BUFFERS`],
  which may be lower than [`gl::MAX_DRAW_BUFFERS`].

## Errors
- [`gl::INVALID_ENUM`] is generated if either `srcRGB` or `dstRGB` is
  not an accepted value.
- [`gl::INVALID_VALUE`] is generated by [`Gl::blend_func_separatei`] if
  `buf` is greater than or equal to the value of
  [`gl::MAX_DRAW_BUFFERS`].

## See Also
- [`Gl::blend_color`]
- [`Gl::blend_func`]
- [`Gl::blend_equation`]
- [`Gl::clear`]
- [`Gl::draw_buffer`]
- [`Gl::enable`]
- [`Gl::logic_op`]
- [`Gl::stencil_func`]
