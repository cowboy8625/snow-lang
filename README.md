# Snow Lang

Snow is a pure functional language with a lot of inspiration from Haskall and OCaml.

# Syntax

```
-- match statement
main :: IO;
main = print (fib 10);

-- match statement
fib :: Int -> Int;
fib 0 = 0;
fib 1 = 1;
fib n = fib (n-1) + fib (n-2);
```

### Todo

- [ ] Generics
- [ ] Type variant access like `Type::Variant` like rust.
- [ ] List syntax
- [ ] Macros
- [ ] Add more info into Tokens about there surroundings.
