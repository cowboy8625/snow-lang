# Snow Lang

Snow is a pure functional language with a lot of inspiration from Haskall and OCaml.

# Syntax

```
-- match statement
main :: IO;
main = print (fib 10);

-- match statement
fib :: Integer -> Integer
fib 0 = 0
fib 1 = 1
fib n = fib (n-1) + fib (n-2)
```

### Todo

###### Parser
- [ ] Make visual tree of ast

###### Error Messages
- [ ] formated error messages

###### Repl
- [X] Greeting Message of the version of snowc

###### Running
- [X] get a simple ast walker working.
- [ ] add passing functions as args
- [ ] partial function application

###### New Syntax
- [ ] String
- [ ] Char
- [ ] Generics
- [ ] Type variant access like `Type::Variant` like rust.
- [ ] List syntax
