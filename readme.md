# Rust Lexical Analyzer
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.63+-lightgray.svg)](https://github.com/markhj/rust-config-reader)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)

> **Important!**
> This project is still being developed. It currently contains a **Tokenizer**.
> More features for lexical analysis will be added.

This Rust-base lexical analyzer is used to tokenize and interpret a custom-designed interpreted language
in your Rust project. 


# Installing

Add this in your ``Cargo.toml`` file:

````
[dependencies.rust-config-reader]
git = "https://github.com/markhj/rust-lexical-analyzer"
````

To upgrade to the latest version run ``cargo update``.

# How to use
> **Gude will be completed when project is closer to being finished**.
> To account for changes which could result from later stages of the project,
> the _How to use_ section is kept to a bare minimum.

## Basic overview
The first step is to create a language definition. This struct describes how you want
your language to look and behave. For example, you define which keywords exist.

````rust
let langdef = LanguageDefinition::new(
    vec!["if", "match", "else", "let"],
);
````

Now, you can tokenize a String (typically obtained from a physical file, but not necessarily).

````rust
let result: Result<TokenStream, ()> = Tokenizer::tokenize(
    &langdef,
    "String to be tokenized",
);
````

This returns a ``Result`` which contains a ``TokenStream``. A token stream is basically a ``Vec`` with an ordered
set of ``Token`` structs.  