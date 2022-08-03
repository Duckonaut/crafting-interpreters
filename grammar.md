program         -> declaration* EOF ;
declaration     -> fnDecl | varDecl | statement ;
fnDecl          -> "fn" function;
function        -> IDENTIFIER "(" parameters? ")" block;
parameters      -> IDENTIFIER ( "," IDENTIFIER )* ;
varDecl         -> "var" IDENTIFIER ( "=" expression )? ";" ;
statement       -> expressionStmt | forStmt | ifStmt | printStmt | whileStmt | block ;
ifStmt          -> "if" expression block ( else block )? ;
forStmt         -> "for" "(" ( varDecl | exprStmt | ";" ) expression? ";" expression? ")" block ;
expressionStmt  -> expression ";" ;
printStmt       -> "print" expression ";" ;


expression  -> assignment ;
assignment  -> IDENTIFIER "=" assignment | logic_or;
logic_or    -> logic_and ( "or" logic_and )* ;
logic_and   -> equality ( "and" equality )* ;
equality    -> comparison ( (  "!=" | "==" ) comparison )* ;
comparison  -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term        -> factor ( ( "-" | "+" ) factor )* ;
factor      -> unary ( ( "/" | "*" ) unary )* ;
unary       ->  ( "!" | "-" ) unary | call;
call        -> primary ( "(" arguments? ")" )* ;
primary     -> "true" | "false" | "nil" | NUMBER | STRING | "(" expression ")" | IDENTIFIER ;

arguments   -> expression ( "," expression )* ;