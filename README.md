# Snow Lang

Snow is an emerging programming language that is firmly rooted in the principles of pure functional programming, drawing substantial inspiration from notable predecessors such as Haskell and OCaml. This heritage endows Snow with a robust foundation in functional programming paradigms, providing a rich set of expressive constructs for the development of concise and resilient software solutions.

**Syntax Enhancement on the Horizon:** Presently, Snow utilizes semicolons as delimiters to separate statements within its code. However, it is essential to recognize that the language is on a dynamic development trajectory. A significant refinement of the syntax is on the horizon, wherein the requirement for semicolons will be deprecated. Future usage of semicolons will result in syntax errors. This pivotal change aligns with Snow's unwavering commitment to cultivating a more intuitive and elegant syntax, which, in turn, promises an enhanced and more user-friendly programming experience.

**Growing Pains and Potential:** As it stands, Snow is in its infancy of development. As is often the case with evolving software projects, the presence of potential bugs is a part of the journey. Users are encouraged to engage with the language with patience and the understanding that ongoing improvements and debugging endeavors are essential components of Snow's progress toward maturity.

**The Current Landscape:** At its current stage, Snow is equipped with a tree-walking interpreter, enabling the execution of its code. Nonetheless, the development roadmap for Snow brims with exciting prospects. The language is poised for potential transformation, with plans to become a compiled language or explore the utilization of a virtual machine in forthcoming iterations. These strategic directions hold the promise of enhanced performance, further extending Snow's versatility as a tool for software development.

## Getting Started

While Snow is under active development, an interactive REPL (Read-Eval-Print Loop) is available for users to experiment with the language. To get started, follow these commands:

```sh
$ git clone http://github.com/cowboy8625/snow-lang.git
$ cd snow-lang
$ cargo run
```

This will place you in a REPL where you can explore the language interactively. If you wish to work with code from a file, use the following command:

```sh
$ git clone http://github.com/cowboy8625/snow-lang.git
$ cd snow-lang
$ cargo run -- file_name.snow -d
```

The `-d` flag disables type checking, as this aspect of the language is still a work in progress.

## Examples

Sample code can be found in the `samples` folder. Here are a few illustrative examples:

- [hello_world](./samples/hello_world.snow)
- [rule110](./samples/rule110.snow)
- [std](./samples/std.snow)

## Syntax

**Enums:**

```haskell
enum Option a = Some a | None
Option.map f = match self on
  | Some x -> Some (f x)
  | None -> None

enum Bool = True | False
```

**Functions:**

```haskell
max :> Int -> Int -> Int
max x y = if x > y then x else y

min :> Int -> Int -> Int
min x y = if x < y then x else y

clamp :> Int -> Int -> Int -> Int
clamp input low high = max low (min input high)

is_digit :> Char -> Bool
is_digit c = c >= '0' && c <= '9'
```

**Custom Operators:**

Snow allows the definition of custom operators to match the specific needs of your code:

```haskell
-- Prefix Operator
`!` :> a -> Bool
`!` x = core::not x True False

-- Infix Operator
`==` :> a -> a -> Bool
`==` x y = core::equal x y Bool::True Bool::False

`<=` :> a -> a -> Bool
`<=` x y = core::less_equal x y Bool::True Bool::False

`>=` :> a -> a -> Bool
`>=` x y = core::greater_equal x y Bool::True Bool::False

`>` :> a -> a -> Bool
`>` x y = core::greater x y Bool::True Bool::False

`<` :> a -> a -> Bool
`<` x y = core::less x y Bool::True Bool::False
```

The `:>` is used to define a type signature for functions, providing a clear specification for argument and return types.

Feel free to explore and experiment with Snow, and stay tuned for its evolving features and capabilities.
