# glXCopyContext
copy state from one rendering context to another

## Parameters
- `dpy`
  Specifies the connection to the X server.

## Description
[`Gl::x_copy_context`] copies selected groups of state variables from
  `src` to `dst`. `mask` indicates which groups of state variables are
  to be copied. `mask` contains the bitwise OR of the same symbolic
  names that are passed to the GL command [`Gl::push_attrib`]. The
  single symbolic constant [`GLX_ALL_ATTRIB_BITS`] can be used to copy
  the maximum possible portion of rendering state.
The copy can be done only if the renderers named by `src` and `dst`
  share an address space. Two rendering contexts share an address space
  if both are nondirect using the same server, or if both are direct and
  owned by a single process. Note that in the nondirect case it is not
  necessary for the calling threads to share an address space, only for
  their related rendering contexts to share an address space.
Not all values for GL state can be copied. For example, pixel pack and
  unpack state, render mode state, and select and feedback state are not
  copied. The state that can be copied is exactly the state that is
  manipulated by the GL command [`Gl::push_attrib`].
An implicit [`Gl::flush`] is done by [`Gl::x_copy_context`] if `src`
  is the current context for the calling thread.

## Notes

A *process* is a single execution environment, implemented in a single
  address space, consisting of one or more threads.
A *thread* is one of a set of subprocesses that share a single address
  space, but maintain separate program counters, stack spaces, and other
  related global data. A *thread* that is the only member of its
  subprocess group is equivalent to a *process*.

## Errors
- [`BadMatch`] is generated if rendering contexts `src` and `dst` do not
  share an address space or were not created with respect to the same
  screen.
- [`BadAccess`] is generated if `dst` is current to any thread
  (including the calling thread) at the time [`Gl::x_copy_context`] is
  called.
- [`GLXBadCurrentWindow`] is generated if `src` is the current context
  and the current drawable is a window that is no longer valid.
- [`GLXBadContext`] is generated if either `src` or `dst` is not a valid
  GLX context.

## See Also
- [`Gl::push_attrib`]
- [`Gl::x_create_context`]
- [`Gl::x_is_direct`]
