#![cfg_attr(not(test), no_main, no_std)]

#[cfg(feature = "export-abi")]
fn main() {
    stylus_hello_world::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}
