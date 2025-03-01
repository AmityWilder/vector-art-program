# glDeleteQueries
delete named query objects

## Parameters
- `n`
  Specifies the number of query objects to be deleted.

## Description
[`Gl::delete_queries`] deletes `n` query objects named by the elements
  of the array `ids`. After a query object is deleted, it has no
  contents, and its name is free for reuse (for example by
  [`Gl::gen_queries`]).
[`Gl::delete_queries`] silently ignores 0's and names that do not
  correspond to existing query objects.

## Errors
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::begin_query`]
- [`Gl::gen_queries`]
- [`Gl::get_queryiv`]
- [`Gl::get_query_object`]
