use lox_shared::{mod_flat, mod_pub};

mod_flat!(interpreter resolver);
mod_pub!(eval std_lib value);
mod environment;
