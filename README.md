# egl-sys

Automatically generated bindings for EGL 1.4 and later. Binding generation can
be customized with selecting crate features. Binding generation for EGL extensions
is also supported.

Binding generation is done with [gl_generator](https://github.com/brendanzab/gl-rs/tree/master/gl_generator) crate.

## Generated bindings

Bindings will be created to module named `ffi`. By default bindings
in this module are for EGL 1.4 and require dynamic linking to EGL library. You can
change bindings in this module to use function pointer loading instead
of dynamic linking. That disables dynamic linking to EGL library.

Note that if bindings are function pointer loading bindings, you must
fist load the function pointer before calling that function. That means
you must load EGL library at runtime for example with [libloading](https://github.com/nagisa/rust_libloading) crate. Also note that
`GetProcAddr` function supports EGL functions only from
EGL version 1.5 or if extension `EGL_KHR_get_all_proc_addresses` is
supported.

There is also optional
module named `extensions`. It contains bindings to latest EGL version
and selected extensions. These bindings always require function pointer
loading.

## Crate features

### `function-pointer-loading`

Generate bindings which require function pointer loading.
Disables dynamic linking to EGL library.

### `EGL-version-*`

EGL version for bindings in module `ffi`.

### `extensions-module`

Enable `extensions` module even if there is not
any extensions selected.

### `raspberry-pi-broadcom`

Sets EGL native types to match Raspberry Pi Broadcom EGL implementation.

### EGL extensions

EGL extension names are crate features. Selecting any extensions
enables `extensions` module. See `Cargo.toml` for list of extensions
that this crate currently supports.

## EGL documentation

See [https://www.khronos.org/registry/EGL/](https://www.khronos.org/registry/EGL/) for
documentation and specifications about EGL. EGL extensions are documented there as well.

## License

This crate is licensed under terms of

* Apache 2.0 license or
* MIT license

at your opinion.

## Contributions

Contributions will be licensed as stated in License section
of this file unless otherwise specified.
