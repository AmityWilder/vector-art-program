# gl_FragDepth
establishes a depth value for the current fragment

## Description
Available only in the fragment language, [`Gl::frag_depth`] is an
  output variable that is used to establish the depth value for the
  current fragment. If depth buffering is enabled and no shader writes
  to [`Gl::frag_depth`], then the fixed function value for depth will be
  used (this value is contained in the z component of
  [`Gl::frag_coord`]) otherwise, the value written to [`Gl::frag_depth`]
  is used. If a shader statically assigns to [`Gl::frag_depth`], then
  the value of the fragment's depth may be undefined for executions of
  the shader that don't take that path. That is, if the set of linked
  fragment shaders statically contain a write to [`Gl::frag_depth`],
  then it is responsible for always writing it.

## See Also
- [`Gl::frag_coord`]
