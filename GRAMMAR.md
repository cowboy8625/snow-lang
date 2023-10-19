```ebnf
program              ::= function_definition* expression
function_definition  ::= ident pattern* "=" expression
expression           ::= function_call | binary_operation | atom
function_call        ::= ident atom*
binary_operation     ::= expression operator expression
operator             ::= "+" | "-" | "*" | "/" | "="
atom                 ::= int | bool | string | ident
int                  ::= [0-9]+
bool                 ::= "true" | "false"
string               ::= '"' [^"]* '"'
ident                ::= [a-zA-Z][a-zA-Z0-9]*
pattern              ::= ident
```
