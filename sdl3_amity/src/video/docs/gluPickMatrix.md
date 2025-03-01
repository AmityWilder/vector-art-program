# gluPickMatrix
define a picking region

## Parameters
- `x`
  Specify the center of a picking region in window coordinates.

## Description
[`Gl::u_pick_matrix`] creates a projection matrix that can be used to
  restrict drawing to a small region of the viewport. This is typically
  useful to determine what objects are being drawn near the cursor. Use
  [`Gl::u_pick_matrix`] to restrict drawing to a small region around the
  cursor. Then, enter selection mode (with [`Gl::render_mode`]) and
  rerender the scene. All primitives that would have been drawn near the
  cursor are identified and stored in the selection buffer.
The matrix created by [`Gl::u_pick_matrix`] is multiplied by the
  current matrix just as if [`Gl::mult_matrix`] is called with the
  generated matrix. To effectively use the generated pick matrix for
  picking, first call [`Gl::load_identity`] to load an identity matrix
  onto the perspective matrix stack. Then call [`Gl::u_pick_matrix`],
  and, finally, call a command (such as [`Gl::u_perspective`]) to
  multiply the perspective matrix by the pick matrix.
When using [`Gl::u_pick_matrix`] to pick NURBS, be careful to turn off
  the NURBS property [`GLU_AUTO_LOAD_MATRIX`]. If
  [`GLU_AUTO_LOAD_MATRIX`] is not turned off, then any NURBS surface
  rendered is subdivided differently with the pick matrix than the way
  it was subdivided without the pick matrix.

## Example
When rendering a scene as follows: ```c glMatrixMode(GL_PROJECTION);
  glLoadIdentity(); gluPerspective(...); glMatrixMode(GL_MODELVIEW); /*
  Draw the scene */ ``` a portion of the viewport can be selected as a
  pick region like this: ```c glMatrixMode(GL_PROJECTION);
  glLoadIdentity(); gluPickMatrix(x, y, width, height, viewport);
  gluPerspective(...); glMatrixMode(GL_MODELVIEW); /* Draw the scene */
  ```

## See Also
- [`Gl::u_perspective`]
- [`Gl::get`]
- [`Gl::load_identity`]
- [`Gl::mult_matrix`]
- [`Gl::render_mode`]
