```ebnf
program             ::= function_definition*
function_definition ::= ident ident* type_info? "=" expression
type_info           ::= ":" type ( "->" type )*
expression          ::= if_expression | logic_or | lambda_expression
if_expression       ::= "if" expression "then" expression "else" expression
logic_or            ::= logic_and ( "or" logic_and )*
logic_and           ::= equality ( "and" equality )*
equality            ::= comparison ( ( "==" | "!=" ) comparison )*
comparison          ::= term ( ( ">" | ">=" | "<" | "<=" ) term )*
term                ::= factor ( ( "-" | "+" ) factor )*
factor              ::= unary ( ( "/" | "*" ) unary )*
unary               ::= ("!" | "-") unary | app
app                 ::= ident atom* | atom
atom                ::= int | bool | string | ident | array_literal
array_literal       ::= "[" (expression ("," expression)*)? "]"
lambda_expression   ::= ("λ" | "\") ident ( ":" type )? "->" expression
int                 ::= digit+
bool                ::= "true" | "false"
string              ::= '"' [^"]* '"'
ident               ::= letter (letter | digit)*
type_specifier      ::= "Int" | "Bool" | "String" | "IO" | array_type | ident
array_type          ::= "Array" "<" type ">"
type                ::= type_specifier ("->" type)?
letter              ::= "a".."z" | "A".."Z"
digit               ::= "0".."9"
comment             ::= line_comment | block_comment
line_comment        ::= "--" [^"\n"]* "\n"
block_comment       ::= "{-" [^"-}"]* "-}"
```
