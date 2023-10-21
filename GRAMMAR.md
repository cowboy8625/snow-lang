```ebnf
program              ::= function_definition*
function_definition  ::= ident ident* ( ":" type )? "=" expression
expression           ::= if_expression | binary_operation | atom
if_expression        ::= "if" expression "then" expression "else" expression
binary_operation     ::= expression operator expression
unary_operation      ::= ("!" | "-") unary_operation | app
app                  ::= ident atom* | primary
atom                 ::= int | bool | string | ident | array_literal
array_literal        ::= "[" (expression ("," expression)*)? "]"
int                  ::= digit+
bool                 ::= "true" | "false"
string               ::= '"' [^"]* '"'
ident                ::= letter (letter | digit)*
type_specifier       ::= "Int" | "Bool" | "String" | "IO" | array_type | ident
array_type           ::= "Array" "<" type ">"
type                ::= type_specifier ("->" type)?
operator             ::= "+" | "-" | "*" | "/" | "="
letter               ::= "a".."z" | "A".."Z"
digit                ::= "0".."9"
```
