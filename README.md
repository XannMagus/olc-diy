# xann

A learning project for a custom language, originally inspired by OneLoneCoder's
DIY language series. The front end currently tokenizes source and uses a Pratt
parser to produce a **raw AST**. Semantic analysis and execution are future work.

## Versioning

All workspace crates share a version following [SemVer 2.0.0](https://semver.org/spec/v2.0.0.html).
The language is currently an alpha with no stable public API. Its version sequence
starts with the first commit unique to `custom-lang` (`52932af`), excluding the
earlier calculator/shunting-yard development from the version count.

For this initial alpha series, each commit is a revision toward `0.0.1`:
`0.0.1-alpha.1`, `0.0.1-alpha.2`, and so on. The historical branch head
`b3d828e` is revision 17. These are retrospective version labels, not published
releases or Git tags. The initial workspace version is `0.0.1-alpha.17`.
Alpha continues until a complete vertical slice works from source through parsing
and semantic analysis to both interpreter execution and compiler-produced native
binary execution. A complete language is not required for that milestone, but
the compiler must be included. Beta and release-candidate stages will use `beta.N`
and `rc.N`; those stages are chosen by readiness, not commit count. Later release
targets get their own series.

## Workspace

| Crate | Responsibility |
| --- | --- |
| `xann-core` | Shared language definitions and the future checked program representation. |
| `xann-frontend` | Lexing, raw AST construction, and future semantic analysis producing a checked program. |
| `xann-interpreter` | Future execution of the checked program. |
| `xann-compiler` | Future LLVM code generation, object emission, and linking into a native binary. |
| `xann-cli` | The `xann` executable: source input, orchestration, and displaying results/errors. |

The front end and both execution backends depend on `xann-core`. The interpreter
and compiler do not depend on front-end internals. The CLI wires the stages
together. LLVM dependencies and linker invocation will belong to `xann-compiler`.

The checked program is not implemented yet. When introduced, it will express
resolved names, types, and validated operations and will be the same input to both
backends. A raw parsed AST is not evidence of semantic validity.

## Current playground

```sh
cargo run -p xann-cli
```

Enter a semicolon-terminated expression or declaration, such as `let x = 1 + 2;`.
`file`, `file1`, and `file2` load `examples/00.lang`, `examples/01.lang`, and
`examples/02.lang` respectively. Enter `quit` or `exit` to leave.

The current front end supports a subset of expressions, assignments, typed
variable declarations (including array types), and struct declarations. Examples
01 and 02 parse; the class/function example 00 describes planned syntax.

This split preserves the existing playground behavior, including debug-only AST
display. Parser correctness fixes and CLI redesign are separate work; there are
known lexer failures, and EOF currently does not exit the playground.

## Checks

```sh
cargo fmt --all --check
cargo check --workspace --locked
cargo test --workspace --locked
```

The inherited implementation has no automated behavioral tests yet.
