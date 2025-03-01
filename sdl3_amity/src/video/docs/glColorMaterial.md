# glColorMaterial
cause a material color to track the current color

## Parameters
- `face`
  Specifies whether front, back, or both front and back material
  parameters should track the current color. Accepted values are
  [`gl::FRONT`], [`gl::BACK`], and [`gl::FRONT_AND_BACK`]. The initial
  value is [`gl::FRONT_AND_BACK`].

## Description
[`Gl::color_material`] specifies which material parameters track the
  current color. When [`gl::COLOR_MATERIAL`] is enabled, the material
  parameter or parameters specified by `mode`, of the material or
  materials specified by `face`, track the current color at all times.
To enable and disable [`gl::COLOR_MATERIAL`], call [`Gl::enable`] and
  [`Gl::disable`] with argument [`gl::COLOR_MATERIAL`].
  [`gl::COLOR_MATERIAL`] is initially disabled.

## Notes
[`Gl::color_material`] makes it possible to change a subset of
  material parameters for each vertex using only the [`Gl::color`]
  command, without calling [`Gl::material`]. If only such a subset of
  parameters is to be specified for each vertex, calling
  [`Gl::color_material`] is preferable to calling [`Gl::material`].
Call [`Gl::color_material`] before enabling [`gl::COLOR_MATERIAL`].
Calling [`Gl::draw_elements`], [`Gl::draw_arrays`], or
  [`Gl::draw_range_elements`] may leave the current color indeterminate,
  if the color array is enabled. If [`Gl::color_material`] is enabled
  while the current color is indeterminate, the lighting material state
  specified by `face` and `mode` is also indeterminate.
If the GL version is 1.1 or greater, and [`gl::COLOR_MATERIAL`] is
  enabled, evaluated color values affect the results of the lighting
  equation as if the current color were being modified, but no change is
  made to the tracking lighting parameter of the current color.

## Errors
- [`gl::INVALID_ENUM`] is generated if `face` or `mode` is not an
  accepted value.
- [`gl::INVALID_OPERATION`] is generated if [`Gl::color_material`] is
  executed between the execution of [`Gl::begin`] and the corresponding
  execution of [`Gl::end`].

## See Also
- [`Gl::color`]
- [`Gl::color_pointer`]
- [`Gl::draw_arrays`]
- [`Gl::draw_elements`]
- [`Gl::draw_range_elements`]
- [`Gl::enable`]
- [`Gl::light`]
- [`Gl::light_model`]
- [`Gl::material`]
