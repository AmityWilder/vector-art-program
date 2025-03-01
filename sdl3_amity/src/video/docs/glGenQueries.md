# glGenQueries
generate query object names

## Parameters
- `n`
  Specifies the number of query object names to be generated.

## Description
[`Gl::gen_queries`] returns `n` query object names in `ids`. There is
  no guarantee that the names form a contiguous set of integers;
  however, it is guaranteed that none of the returned names was in use
  immediately before the call to [`Gl::gen_queries`].
Query object names returned by a call to [`Gl::gen_queries`] are not
  returned by subsequent calls, unless they are first deleted with
  [`Gl::delete_queries`].
No query objects are associated with the returned query object names
  until they are first used by calling [`Gl::begin_query`].

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::begin_query`]
- [`Gl::delete_queries`]
