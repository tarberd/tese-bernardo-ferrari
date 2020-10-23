# Minute from 2020-10-6:

```
Module ::= StatementList

StatementList ::= epsilon 
                | Statement StatementList

Statement ::= ExternFuncionDeclaration ';'
            | FunctionDefinition ';'
            | VariableDefinition ';'

ExternFunctionDeclaration ::= 'extern' 'fn' Identifier '(' ParameterList? ')' ':' Type

FunctionDefinition ::= 'fn' Identifier '(' ParameterList? ')' ':' Type => Expression

ParameterList ::= Parameter
                | Parameter ','
                | Parameter ',' ParameterList

Parameter ::= Identifier ':' Type

VariableDefinition ::= 'let' Identifier ':' Type '=' Expression

Type ::= IntegerType
       | 'bool'

IntegerType ::= 'i8'
              | 'i16'
              | 'i32'
              | 'i64'
              | 'isize'
              | 'u8'
              | 'u16'
              | 'u32'
              | 'u64'
              | 'usize'

Expression ::= BinaryBooleanExpression

BinaryBooleanExpression ::= BinaryBooleanExpression BinaryBooleanOperator UnaryBooleanExpression
                          | UnaryBooleanExpression

BinaryBooleanOperator ::= BooleanOperator | OrderingOperator

BooleanOperator ::= '&&' | '||' 

OrderingOperator ::=  '>' | '>=' | '==' | '<=' | '<'

UnaryBooleanExpression ::= UnaryBooleanOperator TermExpression
                         | TermExpression

UnaryBooleanOperator ::= '!'

TermExpression ::= TermExpression TermOperator FactorExpression
                 | FactorExpression

TermOperator ::= '+' | '-'

FactorExpression ::= FactorExpression FactorOperator UnaryFactorExpression
                   | UnaryFactorExpression

FactorOperator ::= '*' | '/'

UnaryFactorExpression ::= UnaryFactorOperator NestedExpression
                        | NestedExpression

UnaryFactorOperator ::=  '-'

NestedExpression ::= '(' Expression ')'
                   | BlockExpression
                   | FunctionCallExpression
                   | IfExpression
                   | LeafExpression

BlockExpression ::= '{' StatementList Expression? '}'

FunctionCallExpression ::= Identifier '(' ExpressionList? ')'

ExpressionList ::= Expression ','?
                | Expression ',' ExpressionList 

IfExpression ::= 'if' Expression BlockExpression 'else' BlockExpression

LeafExpression ::= IntegerLiteral
                 | BooleanLiteral
                 | Identifier

BooleanLiteral ::= 'true' | 'false'

IntegerLiteral ::=  NumToken IntegerType

NumToken ::= '([[:digit:]]+_*)+'

Identifier ::= '[_[:alpha:]][_[:alnum:]]*'
```

```
Types ::= IntegerTypes | bool | valid | ()

IntegerTypes ::= i8 | u8 | i16 | u16 | i32 | u32 | i64 | u64 | isize | usize

Γ ::= epsilon
    | Γ, x: Types
    | Γ, statement(StatementList)
```

```
----------------------
Γ |- NumToken'i8': i8

----------------------
Γ |- NumToken'u8': u8

----------------------
Γ |- NumToken'i16': i16

----------------------
Γ |- NumToken'u16': u16

----------------------
Γ |- NumToken'i32': i32

----------------------
Γ |- NumToken'u32': u32

----------------------
Γ |- NumToken'i64': i64

----------------------
Γ |- NumToken'u64': u64

----------------------
Γ |- NumToken'isize': isize

----------------------
Γ |- NumToken'usize': usize

----------------------
Γ |- 'true': bool

----------------------
Γ |- 'false': bool

Γ |- b1: bool  Γ |- b2: bool
------------------------------- (bool and or)
Γ |- b1 BooleanOperator b2:bool

Γ |- b: bool
--------------------------------- (bool not)
Γ |- UnaryBooleanOperator b: bool 

Γ |- n1: T  Γ |- n2: T  T in IntegerTypes
----------------------------------------- (integer ordering)
Γ |- b1 OrderingOperator b2: bool

Γ |- n1: T  Γ |- n2: T  T in IntegerTypes
----------------------------------------- (integer term binop)
Γ |- n1 TermOperator n2: T

Γ |- n1: T  Γ |- n2: T  T in IntegerTypes
----------------------------------------- (integer factor binop)
Γ |- n1 FactorOperator n2: T

Γ |- n: T  T in IntegerTypes
---------------------------- (integer factor unary)
Γ |- UnaryFactorOperator n: T

Γ |- nested: T
---------------------- (nested expression)
Γ |- (nested): T

Γ |- s: valid
--------------------------
Γ |- { s }:()

Γ |- s: valid  Γ |- e:T
--------------------------
Γ |- { s e }:T

Γ |- condition: bool  Γ |- then_expr: T  Γ |- else_expr: T
---------------------------------------------------------- (if expression)
Γ |- (if condition then_expr else else_expr): T

f: (A1, A2, ... An) -> B in Γ        Γ |- e1: A1 ... Γ |- en: An
---------------------------------------------------------------- (function call expression)
Γ |- f(e1, e2, ... en): B

------------------- (empty statement)
Γ |- epsilon: valid

Γ |- e: T    Γ, x: T |- next: valid
----------------------------------- (let statement)
Γ |- (let x: T = e; next): valid

Γ, x: (a1: T1, ... an: Tn) -> T, statement(next), a1: T1, ... an: Tn |- expr: T      Γ, x: T |- Next: valid
----------------------------------------------------------------------------------------------------------- (function Statement)
Γ |- (fn x(a1: T1, ... an: Tn): T => expr; next): valid

Γ, x: (a1: T1, ... an: Tn) -> T, statement(next) |- expr: T
----------------------------------------------------------------- ()
Γ, statement(extern fn x(a1: T1, ... an: Tn): T; next) |- expr: T

Γ, x: T, statement(next) |- e2: T
-------------------------------------------------------------- ()
Γ, statement(fn x(a1: T1, ... an: Tn): T => e1; next) |- e2: T

Γ, statement(next) |- expr: T
-------------------------------------------------------------- ()
Γ, statement(let x: T = e; next) |- expr: T

Γ |- expr: T
-------------------------------------------------------------- ()
Γ, statement(epsilon) |- expr: T
```

```
goo: () -> i32 in epsilon, goo epsilon, foo: () -> A, goo: () -> i32, statement(epsilon)
----------------------------------------------------------------------------------------     --------------------------------------------------------------------
epsilon, foo: () -> A, goo: () -> i32, statement(epsilon) |- goo(): i32                      epsilon, foo: () -> i32, goo: () -> i32, statement(epsilon) |- 5:i32
---------------------------------------------------------------------------                  --------------------------------------------------------------------
epsilon, foo: () -> A, statement(fn goo(): i32 => 5; epsilon) |- goo(): i32                  epsilon, foo: () -> i32 |- (fn goo(): i32 => 5; epsilon): valid
------------------------------------------------------------------------------------------------------------------------------------------------------------ (push next statement to context)
epsilon |- (fn foo(): i32 => goo(); let x: i32 = 5; fn goo(): i32 => 5; epsilon): valid
```

# Dificuldades:

* Fazer o contexto não precisar de forward declarations em uma statement list;
* Diferenciar uma statement list de uma expression block de uma statement list de um Module;
* Um simbolo let x de um modulo o faz accessível em todo o modulo enquanto dentro
  de um expression block ele não deveria poder ser capturado, somente em clojures;

