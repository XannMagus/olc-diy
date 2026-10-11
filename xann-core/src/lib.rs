//! Shared definitions for the xann language.
//!
//! Semantic analysis will introduce a checked program representation here,
//! shared by the interpreter and compiler. Raw syntax trees belong to
//! `xann-frontend`; this crate has no parser, execution, or LLVM dependencies.

mod operators;

pub use operators::OperatorKind;
