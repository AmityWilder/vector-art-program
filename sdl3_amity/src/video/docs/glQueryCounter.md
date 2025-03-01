# glQueryCounter
record the GL time into a query object after all previous commands
  have reached the GL server but have not yet necessarily executed.

## Parameters
- `id`
  Specify the name of a query object into which to record the GL time.

## Description
[`Gl::query_counter`] causes the GL to record the current time into
  the query object named `id`. `target` must be [`gl::TIMESTAMP`]. The
  time is recorded after all previous commands on the GL client and
  server state and the framebuffer have been fully realized. When the
  time is recorded, the query result for that object is marked
  available. [`Gl::query_counter`] timer queries can be used within a
  [`Gl::begin_query`] / [`Gl::end_query`] block where the target is
  [`gl::TIME_ELAPSED`] and it does not affect the result of that query
  object.

## Notes
[`Gl::query_counter`] is available only if the GL version is 3.3 or
  higher.

## Errors
- [`gl::INVALID_OPERATION`] is generated if `id` is the name of a query
  object that is already in use within a [`Gl::begin_query`] /
  [`Gl::end_query`] block.
- [`gl::INVALID_VALUE`] is generated if `id` is not the name of a query
  object returned from a previous call to [`Gl::gen_queries`].
- [`gl::INVALID_ENUM`] is generated if `target` is not
  [`gl::TIMESTAMP`].

## See Also
- [`Gl::gen_queries`]
- [`Gl::begin_query`]
- [`Gl::delete_queries`]
- [`Gl::get_query_object`]
- [`Gl::get_queryiv`]
- [`Gl::get`]
