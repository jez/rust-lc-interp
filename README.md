# rust-lc-interp

I wrote a lambda calculus interpretter in Rust to get a feel for Rust.

If you care to look at the code, please know that I Have No Idea What I'm Doing.

## Usage

Clone the repo and then `cargo run` to start the REPL. Syntax is Haskell syntax
for lambdas and ReasonML syntax for let bindings:

```
❯ cargo run
...

λ> (\x -> x) (\x -> x)
...
=> \ -> #0
λ> let id = \x -> x; id id
...
=> \ -> #0
```

If you don't want to clone or don't have a cargo setup, you can play around with
a release from the Releases page.

## Notes

-   I had a really fun time with lalrpop. It feels really nice to use,
    especially the error messages for conflicts:

    ```
    ...
    src/lc.lalrpop:19:5: 19:57: Local ambiguity detected

      The problem arises after having observed the following symbols in the input:
        Atom Atom
      At that point, if the next token is a `"("`, then the parser can proceed in two different ways.

      First, the parser could execute the production at
      /Users/jez/Dropbox/Documents/Programming/rust/sandbox/src/lc.lalrpop:19:5: 19:57, which would consume
      the top 2 token(s) from the stack and produce a `Atom`. This might then yield a parse tree like
        Atom Atom Atom
        ├─Atom──┘    │
        └─Atom───────┘

      Alternatively, the parser could shift the `"("` token and later use it to construct a `Atom`. This might
      then yield a parse tree like
        Atom "(" Atom ")"
        │    └─Atom─────┤
        └─Atom──────────┘

      See the LALRPOP manual for advice on making your grammar LR(1).
    ...
    ```

    These are by far the best parser generator error messages I've ever seen.

-   There are still a bunch of things I miss coming from SML / OCaml / C++. On
    the SML side, I miss: global type inference, not having to worry about
    memory, not having to qualify names of patterns (`Lam` vs `Expr::Lam`), not
    having everything be mutually recursive by default, etc.

    On the C++ side, I find it's easier to do a mutable, in place traversal of a
    data structure. In rust, I found myself constantly trying to update a field
    of an enum in place, and fighting to make sure I was allowed to do so. What
    in C++ would have just been a field access + assignment, in Rust was either
    something like a `ref mut x` pattern match, and then trying to use that
    reference to update the field indirectly.


## License

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://jez.io/MIT-LICENSE.txt)
