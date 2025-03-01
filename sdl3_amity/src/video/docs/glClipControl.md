# glClipControl
control clip coordinate to window coordinate behavior

## Parameters
- `origin`
  Specifies the clip control origin. Must be one of [`gl::LOWER_LEFT`]
  or [`gl::UPPER_LEFT`].

## Description
[`Gl::clip_control`] controls the clipping volume behavior and the
  clip coordinate to window coordinate transformation behavior.
The view volume is defined by $$z_{min} \leq z_c \leq w_c$$ where
  $z_{min} = -w_c$ when `depth` is [`gl::NEGATIVE_ONE_TO_ONE`], and
  $z_{min} = 0$ when `depth` is [`gl::ZERO_TO_ONE`].
The normalized device coordinate $y_d$ is given by $$y_d = { { f
  \times y_c } \over w_c }$$ where $f = 1$ when `origin` is
  [`gl::LOWER_LEFT`], and $f = -1$ when `origin` is [`gl::UPPER_LEFT`].
The window coordinate $z_w$ is given by $$z_w = s \times z_d + b$$
  where $s = { { f - n } \over 2 }$ and $b = { {n + f} \over 2 }$ when
  `depth` is [`gl::NEGATIVE_ONE_TO_ONE`], and $s = f - n$ and $b = n$
  when `depth` is [`gl::ZERO_TO_ONE`]. $n$ and $f$ are the near and far
  depth range values set with [`Gl::depth_range`].
Finally, the polygon area computation defined by [`Gl::front_facing`]
  to determine if a polygon is front- or back-facing has its sign
  negated when `origin` is [`gl::UPPER_LEFT`].

## Notes
The default GL clip volume definition is for a `origin` of
  [`gl::LOWER_LEFT`] and a `depth` of [`gl::NEGATIVE_ONE_TO_ONE`].
An `origin` of [`gl::UPPER_LEFT`] and a `depth` of [`gl::ZERO_TO_ONE`]
  corresponds to Direct3D's clip volume definition.
An `origin` of [`gl::UPPER_LEFT`] and a `depth` of
  [`gl::NEGATIVE_ONE_TO_ONE`] corresponds to the upper-left origin of
  the window coordinate system of Microsoft Windows and the X Window
  System.
There is extensive discussion of the uses and further consequences of
  the different clip volume settings in the ```c GL_ARB_clip_control ```
  extension specification in the OpenGL Registry at URL
  [http://www.opengl.org/registry/](http://www.opengl.org/registry/).

## Errors
- An [`gl::INVALID_ENUM`] error is generated if `origin` is not
  [`gl::LOWER_LEFT`] or [`gl::UPPER_LEFT`].
- An [`gl::INVALID_ENUM`] error is generated if `depth` is not
  [`gl::NEGATIVE_ONE_TO_ONE`] or [`gl::ZERO_TO_ONE`].

## See Also
- [`Gl::clip_distance`]
- [`Gl::cull_distance`]
- [`Gl::front_facing`]
- [`Gl::depth_range`]
