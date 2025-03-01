# imageSize
retrieve the dimensions of an image

## Parameters
- `image`
  Specifies the image to which the texture whose dimensions to retrieve
  is bound.

## Description
[`Gl::image_size`] returns the dimensions of the image bound to
  `image`. The components in the return value are filled in, in order,
  with the width, height and depth of the image. For the array forms,
  the last component of the return value is the number of layers in the
  texture array.

## See Also
- [`Gl::texture_size`]
- [`Gl::image_load`]
- [`Gl::image_store`]
