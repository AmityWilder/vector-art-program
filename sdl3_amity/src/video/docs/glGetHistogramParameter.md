# glGetHistogramParameter
get histogram parameters

## Parameters
- `target`
  Must be one of [`gl::HISTOGRAM`] or [`gl::PROXY_HISTOGRAM`].

## Description
[`Gl::get_histogram_parameter`] is used to query parameter values for
  the current histogram or for a proxy. The histogram state information
  may be queried by calling [`Gl::get_histogram_parameter`] with a
  `target` of [`gl::HISTOGRAM`] (to obtain information for the current
  histogram table) or [`gl::PROXY_HISTOGRAM`] (to obtain information
  from the most recent proxy request) and one of the following values
  for the `pname` argument:



## Notes
[`Gl::get_histogram_parameter`] is present only if ```c ARB_imaging
  ``` is returned when [`Gl::get_string`] is called with an argument of
  [`gl::EXTENSIONS`].

## Errors
- [`gl::INVALID_ENUM`] is generated if `target` is not one of the
  allowable values.
- [`gl::INVALID_ENUM`] is generated if `pname` is not one of the
  allowable values.
- [`gl::INVALID_OPERATION`] is generated if
  [`Gl::get_histogram_parameter`] is executed between the execution of
  [`Gl::begin`] and the corresponding execution of [`Gl::end`].

## See Also
- [`Gl::get_histogram`]
- [`Gl::histogram`]
