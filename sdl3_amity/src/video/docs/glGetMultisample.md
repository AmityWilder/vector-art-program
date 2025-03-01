# glGetMultisamplefv
retrieve the location of a sample

## Parameters
- `pname`
  Specifies the sample parameter name. `pname` must be
  [`gl::SAMPLE_POSITION`].

## Description
[`Gl::get_multisamplefv`] queries the location of a given sample.
  `pname` specifies the sample parameter to retrieve and must be
  [`gl::SAMPLE_POSITION`]. `index` corresponds to the sample for which
  the location should be returned. The sample location is returned as
  two floating-point values in `val[0]` and `val[1]`, each between 0 and
  1, corresponding to the `x` and `y` locations respectively in the GL
  pixel space of that sample. (0.5, 0.5) this corresponds to the pixel
  center. `index` must be between zero and the value of [`gl::SAMPLES`]
  minus one.
If the multisample mode does not have fixed sample locations, the
  returned values may only reflect the locations of samples within some
  pixels.

## Errors
- [`gl::INVALID_ENUM`] is generated if `pname` is not one
  [`gl::SAMPLE_POSITION`].
- [`gl::INVALID_VALUE`] is generated if `index` is greater than or equal
  to the value of [`gl::SAMPLES`].

## See Also
- [`Gl::gen_framebuffers`]
- [`Gl::bind_framebuffer`]
