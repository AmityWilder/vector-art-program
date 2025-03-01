# glGetError
return error information

## Description
[`Gl::get_error`] returns the value of the error flag. Each detectable
  error is assigned a numeric code and symbolic name. When an error
  occurs, the error flag is set to the appropriate error code value. No
  other errors are recorded until [`Gl::get_error`] is called, the error
  code is returned, and the flag is reset to [`gl::NO_ERROR`]. If a call
  to [`Gl::get_error`] returns [`gl::NO_ERROR`], there has been no
  detectable error since the last call to [`Gl::get_error`], or since
  the GL was initialized.
To allow for distributed implementations, there may be several error
  flags. If any single error flag has recorded an error, the value of
  that flag is returned and that flag is reset to [`gl::NO_ERROR`] when
  [`Gl::get_error`] is called. If more than one flag has recorded an
  error, [`Gl::get_error`] returns and clears an arbitrary error flag
  value. Thus, [`Gl::get_error`] should always be called in a loop,
  until it returns [`gl::NO_ERROR`], if all error flags are to be reset.
Initially, all error flags are set to [`gl::NO_ERROR`].
The following errors are currently defined:
When an error flag is set, results of a GL operation are undefined
  only if [`gl::OUT_OF_MEMORY`] has occurred. In all other cases, the
  command generating the error is ignored and has no effect on the GL
  state or frame buffer contents. If the generating command returns a
  value, it returns 0. If [`Gl::get_error`] itself generates an error,
  it returns 0.

## See Also
