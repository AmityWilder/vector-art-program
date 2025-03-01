# glCreateQueries
create query objects

## Parameters
- `target`
  Specifies the target of each created query object.

## Description
[`Gl::create_queries`] returns `n` previously unused query object
  names in `ids`, each representing a new query object with the
  specified `target`.
`target` may be one of [`gl::SAMPLES_PASSED`],
  [`gl::ANY_SAMPLES_PASSED`], [`gl::ANY_SAMPLES_PASSED_CONSERVATIVE`],
  [`gl::TIME_ELAPSED`], [`gl::TIMESTAMP`], [`gl::PRIMITIVES_GENERATED`]
  or [`gl::TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not an accepted
  value.
- [`gl::INVALID_VALUE`] is generated if `n` is negative.

## See Also
- [`Gl::begin_query`]
- [`Gl::begin_query_indexed`]
- [`Gl::delete_queries`]
- [`Gl::gen_queries`]
- [`Gl::get_query_object`]
- [`Gl::get_queryiv`]
- [`Gl::is_query`]
