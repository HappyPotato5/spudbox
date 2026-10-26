<!-- cargo-rdme start -->

[![Crates.io](https://img.shields.io/badge/Crates.io-latest-blue?style=for-the-badge&logo=rust&logoColor=black&labelColor=BBBBBB)](https://crates.io/crates/spudbox)
[![Docs.rs](https://img.shields.io/badge/Documentation-SpudBox-blue?style=for-the-badge&logo=Docs.rs&logoColor=black&labelColor=BBBBBB)](https://docs.rs/spudbox/latest/spudbox/)
[![GitHub](https://img.shields.io/badge/Github-SpudBox-blue?style=for-the-badge&logo=github&logoColor=black&labelColor=BBBBBB)](https://github.com/HappyPotato5/spudbox)
[![License](https://img.shields.io/badge/License-GPL--v3-blue?style=for-the-badge&logo=readdotcv&logoColor=black&labelColor=BBBBBB)](https://github.com/HappyPotato5/spudbox/blob/master/LICENSE)

# Spudbox
A library that implements lots of useful **data structures** used in <!--vale off--> my <!--vale on--> **Rust** projects.

## Types

From **[`prelude`](https://docs.rs/spudbox/latest/spudbox/prelude/)**:
+ [`VecSet<T>`](https://docs.rs/spudbox/latest/spudbox/vecset/struct.VecSet.html): A [`Vec<T>`](https://doc.rust-lang.org/stable/alloc/vec/struct.Vec.html) that doesn't keep repetitions.
+ [`OrderedSet<T>`](https://docs.rs/spudbox/latest/spudbox/ordered_set/struct.OrderedSet.html): An **ordered** [`HashSet<T>`](https://doc.rust-lang.org/stable/std/collections/hash/set/struct.HashSet.html).
+ [`Arena<T>`](https://docs.rs/spudbox/latest/spudbox/arena/struct.Arena.html): An **Arena allocator** for any **T** type.

From **[`lexer`](https://docs.rs/spudbox/latest/spudbox/lexer/)**:
+ [`Lexer`](https://docs.rs/spudbox/latest/spudbox/lexer/lexer/struct.Lexer.html): A **lexer** for tokens matching a [`Pattern`](https://docs.rs/spudbox/latest/spudbox/lexer/pattern/trait.Pattern.html).
+ [`RawToken`](https://docs.rs/spudbox/latest/spudbox/lexer/raw_token/struct.RawToken.html): A **Token** which still contains the *lexeme*. 
+ [`Span`](https://docs.rs/spudbox/latest/spudbox/lexer/span/struct.Span.html): A struct that holds the starting and end position of a [`RawToken`](https://docs.rs/spudbox/latest/spudbox/lexer/raw_token/struct.RawToken.html).
+ [`Pattern`](https://docs.rs/spudbox/latest/spudbox/lexer/pattern/trait.Pattern.html): A **pattern** that can match a *substring*.
+ [`ConflictPolicy`](https://docs.rs/spudbox/latest/spudbox/lexer/pattern/enum.ConflictPolicy.html): An *enum* used to resolve conflicts.
+ [`Recipe`](https://docs.rs/spudbox/latest/spudbox/lexer/recipe/struct.Recipe.html): A **recipe** used by a [`Lexer`](https://docs.rs/spudbox/latest/spudbox/lexer/lexer/struct.Lexer.html) to build *tokens*.
+ [`recipe!`](https://docs.rs/spudbox/latest/spudbox/macro.recipe.html): A **macro** that constructs [`Recipes`](https://docs.rs/spudbox/latest/spudbox/lexer/recipe/struct.Recipe.html) in a simplified manner.
+ [`recipe_list!`](https://docs.rs/spudbox/latest/spudbox/macro.recipe_list.html): A **macro** that constructs [`Vec<Recipe>`](https://docs.rs/spudbox/latest/spudbox/lexer/recipe/struct.Recipe.html) in a simplified manner.

From **[`parse`](https://docs.rs/spudbox/latest/spudbox/parse/)**:
+ [`ParserStream`](https://docs.rs/spudbox/latest/spudbox/parse/type.ParserStream.html): A **stream** able to [parse](https://docs.rs/spudbox/latest/spudbox/parse/trait.Parse.html) items from *tokens*, in a *top-down manner*.
+ [`ParserBuffer`](https://docs.rs/spudbox/latest/spudbox/parse/struct.ParserBuffer.html): The buffer that [`ParserStream`](https://docs.rs/spudbox/latest/spudbox/parse/type.ParserStream.html) uses.
+ [`Parse`](https://docs.rs/spudbox/latest/spudbox/parse/trait.Parse.html): A **trait** implemented by any type that a [`ParserStream`](https://docs.rs/spudbox/latest/spudbox/parse/type.ParserStream.html) can construct from a *sequence of tokens*.
+ [`Kinded`](https://docs.rs/spudbox/latest/spudbox/parse/trait.Kinded.html): A **trait** implemented by a *token type*, providing a getter to its *kind*.
+ [`ParseError`](https://docs.rs/spudbox/latest/spudbox/parse/struct.ParseError.html): The *default* **error type** returned when the [`ParserStream`](https://docs.rs/spudbox/latest/spudbox/parse/type.ParserStream.html) couldn't parse an *item*.

## Features
This crate uses **features**, so you only import the submodules that **you use**.

### <!--vale off-->By Default<!--vale on-->:
+ **`vecset`**: Enables the [`vecset`](https://docs.rs/spudbox/latest/spudbox/vecset/) submodule.
+ **`ordered-set`**: Enables the [`ordered_set`](https://docs.rs/spudbox/latest/spudbox/ordered_set/) submodule.
+ **`arena`**: Enables the [`arena`](https://docs.rs/spudbox/latest/spudbox/arena/) submodule.

### Additionally:
+ **`lexer`**: Enables the [`lexer`](https://docs.rs/spudbox/latest/spudbox/lexer/) submodule.
+ **`parse`**: Enables the [`parse`](https://docs.rs/spudbox/latest/spudbox/parse/) submodule.

**Spudbox** crate automatically **re-exports** prelude contents, to avoid this you can use the **`no-prelude`** **feature**.

## Examples
You can find examples in the [GitHub repository](https://github.com/HappyPotato5/spudbox).

<!-- cargo-rdme end -->
