#![feature(allow_internal_unstable)]
#![feature(bench_black_box)]
#![feature(const_panic)]
#![feature(extend_one)]
#![feature(iter_zip)]
#![feature(unboxed_closures)]
#![feature(test)]
#![feature(fn_traits)]
#![feature(trusted_step)]

pub mod bit_set;
pub mod vec;

// FIXME(#56935): Work around ICEs during cross-compilation.
#[allow(unused)]
extern crate rustc_macros;
