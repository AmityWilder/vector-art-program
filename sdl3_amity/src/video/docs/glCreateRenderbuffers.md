# glCreateRenderbuffers
create renderbuffer objects

## Parameters
- `n`
  Number of renderbuffer objects to create.

## Description
[`Gl::create_renderbuffers`] returns `n` previously unused
  renderbuffer object names in `renderbuffers`, each representing a new
  renderbuffer object initialized to the default state.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::bind_renderbuffer`]
- [`Gl::delete_renderbuffers`]
- [`Gl::gen_renderbuffers`]
- [`Gl::is_renderbuffer`]
- [`Gl::renderbuffer_storage`]
- [`Gl::renderbuffer_storage_multisample`]
