# glInvalidateSubFramebuffer
invalidate the content of a region of some or all of a framebuffer's
  attachments

## Parameters
- `target`
  Specifies the target to which the framebuffer object is attached for
  [`Gl::invalidate_sub_framebuffer`].

## Description
[`Gl::invalidate_sub_framebuffer`] and
  [`Gl::invalidate_named_framebuffer_sub_data`] invalidate the contents
  of a specified region of a specified set of attachments of a
  framebuffer.
For [`Gl::invalidate_sub_framebuffer`], the framebuffer object is that
  bound to `target`, which must be one of [`gl::FRAMEBUFFER`],
  [`gl::READ_FRAMEBUFFER`] or [`gl::DRAW_FRAMEBUFFER`].
  [`gl::FRAMEBUFFER`] is equivalent to [`gl::DRAW_FRAMEBUFFER`]. Default
  framebuffers may also be invalidated if bound to `target`.
For [`Gl::invalidate_named_framebuffer_sub_data`], `framebuffer` is
  the name of the framebuffer object. If `framebuffer` is zero, the
  default draw framebuffer is affected.
The set of attachments of which a region is to be invalidated are
  specified in the `attachments` array, which contains `numAttachments`
  elements.
If the specified framebuffer is a framebuffer object, each element of
  `attachments` must be one of [`gl::DEPTH_ATTACHMENT`],
  [`gl::STENCIL_ATTACHMENT`] [`gl::DEPTH_STENCIL_ATTACHMENT`], or
  [`gl::COLOR_ATTACHMENT`]*i*, where *i* is between zero and the value
  of [`gl::MAX_FRAMEBUFFER_ATTACHMENTS`] minus one.
If the specified framebuffer is a default framebuffer, each element of
  `attachments` must be one of [`gl::FRONT_LEFT`], [`gl::FRONT_RIGHT`],
  [`gl::BACK_LEFT`], [`gl::BACK_RIGHT`], [`gl::AUX`]*i*, [`gl::ACCUM`],
  [`gl::COLOR`], [`gl::DEPTH`], or [`gl::STENCIL`]. [`gl::COLOR`], is
  treated as [`gl::BACK_LEFT`] for a double-buffered context and
  [`gl::FRONT_LEFT`] for a single-buffered context. The other
  attachments identify the corresponding specific buffer.
The contents of the specified region of each specified attachment
  become undefined after execution of [`Gl::invalidate_sub_framebuffer`]
  or [`Gl::invalidate_named_framebuffer_sub_data`]. The region to be
  invalidated is specified by `x`, `y`, `width` and `height` where `x`
  and `y` give the offset from the origin (with lower-left corner at
  $(0,0)$) and `width` and `height` are the width and height,
  respectively, of the region. Any pixels lying outside of the window
  allocated to the current GL context (for the default framebuffer), or
  outside of the attachments of the framebuffer object, are ignored. If
  the framebuffer object is not complete, these commands may be ignored.
If the framebuffer object is not complete,
  [`Gl::invalidate_sub_framebuffer`] and
  [`Gl::invalidate_named_framebuffer_sub_data`] may be ignored. This is
  not an error.

## Errors
- [`gl::INVALID_ENUM`] by [`Gl::invalidate_sub_framebuffer`] if `target`
  is not one of the accepted framebuffer targets.
- [`gl::INVALID_OPERATION`] by
  [`Gl::invalidate_named_framebuffer_sub_data`] if `framebuffer` is not
  zero of the name of an existing framebuffer object.
- [`gl::INVALID_VALUE`] is generated if `numAttachments`, `width` or
  `height` is negative.
- [`gl::INVALID_ENUM`] is generated if any element of `attachments` is
  not one of the accepted framebuffer attachment points, as described
  above.
- [`gl::INVALID_OPERATION`] is generated if element of `attachments` is
  [`gl::COLOR_ATTACHMENT`]*m* where *m* is greater than or equal to the
  value of [`gl::MAX_COLOR_ATTACHMENTS`].

## See Also
- [`Gl::invalidate_tex_sub_image`]
- [`Gl::invalidate_tex_image`]
- [`Gl::invalidate_buffer_sub_data`]
- [`Gl::invalidate_buffer_data`]
- [`Gl::invalidate_framebuffer`]
