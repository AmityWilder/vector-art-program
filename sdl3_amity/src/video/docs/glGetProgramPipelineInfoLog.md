# glGetProgramPipelineInfoLog
retrieve the info log string from a program pipeline object

## Parameters
- `pipeline`
  Specifies the name of a program pipeline object from which to retrieve
  the info log.

## Description
[`Gl::get_program_pipeline_info_log`] retrieves the info log for the
  program pipeline object `pipeline`. The info log, including its null
  terminator, is written into the array of characters whose address is
  given by `infoLog`. The maximum number of characters that may be
  written into `infoLog` is given by `bufSize`, and the actual number of
  characters written into `infoLog` is returned in the integer whose
  address is given by `length`. If `length` is [`NULL`], no length is
  returned.
The actual length of the info log for the program pipeline may be
  determined by calling [`Gl::get_program_pipeline`] with `pname` set to
  [`gl::INFO_LOG_LENGTH`].

## Errors
- [`gl::INVALID_OPERATION`] is generated if `pipeline` is not a name
  previously returned from a call to [`Gl::gen_program_pipelines`] or if
  such a name has been deleted by a call to
  [`Gl::delete_program_pipelines`].

## See Also
- [`Gl::gen_program_pipelines`]
- [`Gl::bind_program_pipeline`]
- [`Gl::delete_program_pipelines`]
- [`Gl::get_program_pipeline`]
