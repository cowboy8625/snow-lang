```ebnf
program              ::= function_definition*
function_definition  ::= ident ident* ( ":" type )? "=" expression
expression           ::= binary_operation | atom
binary_operation     ::= expression operator expression
unary_operation      ::= ("!" | "-") unary_operation | app
app                  ::= ident atom* | primary
atom                 ::= int | bool | string | ident | "(" expression ")"
int                  ::= digit+
bool                 ::= "true" | "false"
string               ::= '"' [^"]* '"'
ident                ::= letter (letter | digit)*
type_specifier      ::= "Int" | "Bool" | "String" | "IO" | ident
type                ::= type_specifier ("->" type)?
operator             ::= "+" | "-" | "*" | "/" | "="
letter              ::= "a".."z" | "A".."Z"
digit               ::= "0".."9"
```
