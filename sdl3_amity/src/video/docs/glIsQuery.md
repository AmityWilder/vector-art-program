# glIsQuery
determine if a name corresponds to a query object

## Parameters
- `id`
  Specifies a value that may be the name of a query object.

## Description
[`Gl::is_query`] returns [`gl::TRUE`] if `id` is currently the name of
  a query object. If `id` is zero, or is a non-zero value that is not
  currently the name of a query object, or if an error occurs,
  [`Gl::is_query`] returns [`gl::FALSE`].
A name returned by [`Gl::gen_queries`], but not yet associated with a
  query object by calling [`Gl::begin_query`], is not the name of a
  query object.

## See Also
- [`Gl::begin_query`]
- [`Gl::delete_queries`]
- [`Gl::gen_queries`]
