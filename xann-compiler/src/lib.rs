//! Native compiler backend for xann (not implemented yet).
//!
//! This crate will lower the same checked program accepted by the interpreter
//! from `xann-core` to LLVM, emit object code, and invoke a linker to produce
//! an executable. LLVM and linking dependencies belong here.
