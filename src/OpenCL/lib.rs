#[pkgid = "OpenCL"];
#[link(name = "OpenCL",
       version = "0.1",
       uuid = "26bc3ce8-7720-41a7-9280-c522285f2e70")];
#[crate_type = "lib"];
#[feature(macro_rules)];
#[feature(globs)];
#[feature(managed_boxes)];
#[feature(link_args)];

//! OpenCL bindings for Rust.

extern mod std;

#[nolink]
#[link_args = "-framework OpenCL"]
#[cfg(target_os = "macos")]
extern { }

#[link(name = "OpenCL")]
#[cfg(target_os = "linux")]
extern { }

/// Low-level OpenCL bindings. These should primarily be used by the
/// higher level features in this library.
pub mod CL;
pub mod error;
pub mod hl;
pub mod util;
pub mod mem;
pub mod array;
