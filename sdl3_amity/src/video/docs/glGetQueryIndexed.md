# glGetQueryIndexediv
return parameters of an indexed query object target

## Parameters
- `target`
  Specifies a query object target. Must be [`gl::SAMPLES_PASSED`],
  [`gl::ANY_SAMPLES_PASSED`], [`gl::ANY_SAMPLES_PASSED_CONSERVATIVE`]
  [`gl::PRIMITIVES_GENERATED`],
  [`gl::TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN`], [`gl::TIME_ELAPSED`],
  or [`gl::TIMESTAMP`].

## Description
[`Gl::get_query_indexediv`] returns in `params` a selected parameter
  of the indexed query object target specified by `target` and `index`.
  `index` specifies the index of the query object target and must be
  between zero and a target-specific maxiumum.
`pname` names a specific query object target parameter. When `pname`
  is [`gl::CURRENT_QUERY`], the name of the currently active query for
  the specified `index` of `target`, or zero if no query is active, will
  be placed in `params`. If `pname` is [`gl::QUERY_COUNTER_BITS`], the
  implementation-dependent number of bits used to hold the result of
  queries for `target` is returned in `params`.

## Notes
The target [`gl::ANY_SAMPLES_PASSED_CONSERVATIVE`] is available only
  if the GL version is 4.3 or greater.
If an error is generated, no change is made to the contents of
  `params`.
Calling [`Gl::get_queryiv`] is equivalent to calling
  [`Gl::get_query_indexediv`] with `index` set to zero.

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` or `pname` is not an
  accepted value.
- [`gl::INVALID_VALUE`] is generated if `index` is greater than or equal
  to the `target`-specific maximum.

## See Also
- [`Gl::get_query_object`]
- [`Gl::is_query`]
