# glXIntro
Introduction to OpenGL in the X window system

## Overview

OpenGL (called GL in other pages) is a high-performance 3D-oriented
  renderer. It is available in the X window system through the GLX
  extension. To determine whether the GLX extension is supported by an X
  server, and if so, what version is supported, call
  [`Gl::x_query_extension`] and [`Gl::x_query_version`].
GLX extended X servers make a subset of their visuals available for
  OpenGL rendering. Drawables created with these visual can also be
  rendered into using the core X renderer and or any other X extension
  that is compatible with all core X visuals.
GLX extends a drawable's standard color buffer with additional
  buffers. These buffers include back and auxiliary color buffers, a
  depth buffer, a stencil buffer, and a color accumulation buffer. Some
  or all of the buffers listed are included in each X visual that
  supports OpenGL.
GLX supports rendering into three types of drawables: windows,
  pixmaps, and pbuffers (pixel buffers). GLX windows and pixmaps are X
  resources, and capable of accepting core X rendering as well as OpenGL
  rendering. GLX-pbuffers are GLX only resources and might not accept
  core X rendering.
To render using OpenGL into a GLX drawable, you must determine the
  appropriate GLXFBConfig that supports the rendering features your
  application requires. [`Gl::x_choose_fb_config`] returns a GLXFBConfig
  matching the required attributes or [`NULL`] if no match is found. A
  complete list of GLXFBConfigs supported by a server can be obtained by
  calling [`Gl::x_get_fb_configs`]. Attributes of a particular
  GLXFBConfig can be queried by calling [`Gl::x_get_fb_config_attrib`].
For GLX windows and pixmaps, a suitable X drawable (using either
  [`Gl::x_create_window`] or [`Gl::x_create_pixmap`], respectively) with
  a matching visual must be created first. Call
  [`Gl::x_get_visual_from_fb_config`] to obtain the necessary
  XVisualInfo structure for creating the X drawable. For pbuffers, no
  underlying X drawable is required.
To create a GLX window from an X window, call [`Gl::x_create_window`].
  Likewise, to create a GLX pixmap, call [`Gl::x_create_pixmap`].
  Pbuffers are created by calling [`Gl::x_create_pbuffer`]. Use
  [`Gl::x_destroy_window`], [`Gl::x_destroy_pixmap`], and
  [`Gl::x_destroy_pbuffer`] to release previously allocated resources.
A GLX context is required to bind OpenGL rendering to a GLX resource.
  A GLX resource and rendering context must have compatible
  GLXFBConfigs. To create a GLX context, call
  [`Gl::x_create_new_context`]. A context may be bound to a GLX drawable
  by using [`Gl::x_make_context_current`]. This context/drawable pair
  becomes the current context and current drawable, and is used by all
  OpenGL rendering commands until [`Gl::x_make_context_current`] is
  called with different arguments.
Both core X and OpenGL commands can be used to operate on drawables;
  however, the X and OpenGL command streams are not synchronized.
  Synchronization can be explicitly specified using by calling
  [`Gl::x_wait_gl`], [`Gl::x_wait_x`], [`Gl::x_sync`], and
  [`Gl::x_flush`].


## Examples
Below is a minimal example of creating an RGBA-format X window that's
  compatible with OpenGL using GLX 1.3 commands. The window is cleared
  to yellow when the program runs. The program does minimal error
  checking; all return values should be checked.
```c #include <stdio.h> #include <stdlib.h> #include <GL/gl.h>
  #include <GL/glx.h> int singleBufferAttributess[] = {
  GLX_DRAWABLE_TYPE, GLX_WINDOW_BIT, GLX_RENDER_TYPE, GLX_RGBA_BIT,
  GLX_RED_SIZE, 1, /* Request a single buffered color buffer */
  GLX_GREEN_SIZE, 1, /* with the maximum number of color bits */
  GLX_BLUE_SIZE, 1, /* for each component */ None }; int
  doubleBufferAttributes[] = { GLX_DRAWABLE_TYPE, GLX_WINDOW_BIT,
  GLX_RENDER_TYPE, GLX_RGBA_BIT, GLX_DOUBLEBUFFER, True, /* Request a
  double-buffered color buffer with */ GLX_RED_SIZE, 1, /* the maximum
  number of bits per component */ GLX_GREEN_SIZE, 1, GLX_BLUE_SIZE, 1,
  None }; static Bool WaitForNotify( Display *dpy, XEvent *event,
  XPointer arg ) { return (event->type == MapNotify) &&
  (event->xmap.window == (Window) arg); } int main( int argc, char
  *argv[] ) { Display *dpy; Window xWin; XEvent event; XVisualInfo
  *vInfo; XSetWindowAttributes swa; GLXFBConfig *fbConfigs; GLXContext
  context; GLXWindow glxWin; int swaMask; int numReturned; int swapFlag
  = True; /* Open a connection to the X server */ dpy = XOpenDisplay(
  NULL ); if ( dpy == NULL ) { printf( "Unable to open a connection to
  the X server\n" ); exit( EXIT_FAILURE ); } /* Request a suitable
  framebuffer configuration - try for a double ** buffered configuration
  first */ fbConfigs = glXChooseFBConfig( dpy, DefaultScreen(dpy),
  doubleBufferAttributes, &numReturned ); if ( fbConfigs == NULL ) { /*
  no double buffered configs available */ fbConfigs = glXChooseFBConfig(
  dpy, DefaultScreen(dpy), singleBufferAttributess, &numReturned );
  swapFlag = False; } /* Create an X colormap and window with a visual
  matching the first ** returned framebuffer config */ vInfo =
  glXGetVisualFromFBConfig( dpy, fbConfigs[0] ); swa.border_pixel = 0;
  swa.event_mask = StructureNotifyMask; swa.colormap = XCreateColormap(
  dpy, RootWindow(dpy, vInfo->screen), vInfo->visual, AllocNone );
  swaMask = CWBorderPixel | CWColormap | CWEventMask; xWin =
  XCreateWindow( dpy, RootWindow(dpy, vInfo->screen), 0, 0, 256, 256, 0,
  vInfo->depth, InputOutput, vInfo->visual, swaMask, &swa ); /* Create a
  GLX context for OpenGL rendering */ context = glXCreateNewContext(
  dpy, fbConfigs[0], GLX_RGBA_TYPE, NULL, True ); /* Create a GLX window
  to associate the frame buffer configuration ** with the created X
  window */ glxWin = glXCreateWindow( dpy, fbConfigs[0], xWin, NULL );
  /* Map the window to the screen, and wait for it to appear */
  XMapWindow( dpy, xWin ); XIfEvent( dpy, &event, WaitForNotify,
  (XPointer) xWin ); /* Bind the GLX context to the Window */
  glXMakeContextCurrent( dpy, glxWin, glxWin, context ); /* OpenGL
  rendering ... */ glClearColor( 1.0, 1.0, 0.0, 1.0 ); glClear(
  GL_COLOR_BUFFER_BIT ); glFlush(); if ( swapFlag ) glXSwapBuffers( dpy,
  glxWin ); sleep( 10 ); exit( EXIT_SUCCESS ); } ```


## Notes
An X color map must be created and passed to [`Gl::x_create_window`].
A GLX context must be created and bound to a GLX drawable before
  OpenGL commands can be executed. OpenGL commands executed while no
  context/drawable pair is current result in undefined behavior.
Exposure events indicate that *all* buffers associated with the
  specified window may be damaged and should be repainted. Although
  certain buffers of some visuals on some systems may never require
  repainting (the depth buffer, for example), it is incorrect to write a
  program assuming that these buffers will not be damaged.
GLX commands utilize XVisualInfo structures rather than pointers to
  visuals or visualIDs directly. XVisualInfo structures contain
  *visual*, *visualID*, *screen*, and *depth* elements, as well as other
  X-specific information.


## Using GLX Extensions
All supported GLX extensions will have a corresponding definition in
  glx.h and a token in the extension string returned by
  [`Gl::x_query_extensions_string`]. For example, if the ```c
  EXT_visual_info ``` extension is supported, then this token will be
  defined in glx.h and ```c EXT_visual_info ``` will appear in the
  extension string returned by [`Gl::x_query_extensions_string`]. The
  definitions in glx.h can be used at compile time to determine if
  procedure calls corresponding to an extension exist in the library.
OpenGL itself is capable of being extended.


## GLX 1.1, GLX 1.2, and GLX 1.3
GLX 1.3 is now supported and is backward compatible with GLX 1.1 and
  GLX 1.2. It introduces new functionality (namely GLXFBConfigs) that
  supersedes the GLX 1.2 functionality. GLX 1.2 commands are supported,
  but their use in new application development is not recommended.
GLX 1.3 corresponds to OpenGL versions 1.2 and introduces the
  following new calls: [`Gl::x_get_fb_configs`],
  [`Gl::x_get_fb_config_attrib`], [`Gl::x_get_visual_from_fb_config`],
  [`Gl::x_create_window`], [`Gl::x_destroy_window`],
  [`Gl::x_create_pixmap`], [`Gl::x_destroy_pixmap`],
  [`Gl::x_create_pbuffer`], [`Gl::x_destroy_pbuffer`],
  [`Gl::x_query_drawable`], [`Gl::x_create_new_context`],
  [`Gl::x_make_context_current`], [`Gl::x_get_current_read_drawable`],
  [`Gl::x_get_current_display`], [`Gl::x_query_context`], and
  [`Gl::x_select_event`], [`Gl::x_get_selected_event`].
GLX 1.2 corresponds to OpenGL version 1.1 and introduces the following
  new call: [`Gl::x_get_current_display`].
GLX 1.1 corresponds to OpenGL version 1.0 and introduces the following
  new calls: [`Gl::x_query_extensions_string`],
  [`Gl::x_query_server_string`], and [`Gl::x_get_client_string`].
Call [`Gl::x_query_version`] to determine at runtime what version of
  GLX is available. [`Gl::x_query_version`] returns the version that is
  supported on the connection. Thus, if 1.3 is returned, both the client
  and server support GLX 1.3. You can also check the GLX version at
  compile time: GLX_VERSION_1_1 will be defined in glx.h if GLX 1.1
  calls are supported, GLX_VERSION_1_2 will be defined if GLX 1.2 calls
  are supported, and GLX_VERSION_1_3 will be defined if GLX 1.3 calls
  are supported.


## See Also
- [`Gl::finish`]
- [`Gl::flush`]
- [`Gl::x_choose_visual`]
- [`Gl::x_copy_context`]
- [`Gl::x_create_context`]
- [`Gl::x_create_glx_pixmap`]
- [`Gl::x_create_new_context`]
- [`Gl::x_create_pbuffer`]
- [`Gl::x_create_pixmap`]
- [`Gl::x_create_window`]
- [`Gl::x_destroy_context`]
- [`Gl::x_destroy_pbuffer`]
- [`Gl::x_destroy_pixmap`]
- [`Gl::x_destroy_window`]
- [`Gl::x_get_client_string`]
- [`Gl::x_get_config`]
- [`Gl::x_get_current_display`]
- [`Gl::x_get_current_read_drawable`]
- [`Gl::x_get_fb_config_attrib`]
- [`Gl::x_get_fb_configs`]
- [`Gl::x_get_proc_address`]
- [`Gl::x_get_selected_event`]
- [`Gl::x_get_visual_from_fb_config`]
- [`Gl::x_is_direct`]
- [`Gl::x_make_context_current`]
- [`Gl::x_make_current`]
- [`Gl::x_query_context`]
- [`Gl::x_query_drawable`]
- [`Gl::x_query_extension`]
- [`Gl::x_query_extensions_string`]
- [`Gl::x_query_server_string`]
- [`Gl::x_query_version`]
- [`Gl::x_select_event`]
- [`Gl::x_swap_buffers`]
- [`Gl::x_use_x_font`]
- [`Gl::x_wait_gl`]
- [`Gl::x_wait_x`]
