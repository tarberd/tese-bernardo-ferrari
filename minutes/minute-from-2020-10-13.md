# Minute from 2020-10-13:

## Grammar

```
Module ::= StatementList

StatementList ::= epsilon 
                | Statement StatementList

Statement ::= ExternFuncionDeclaration ';'
            | FunctionDefinition ';'
            | VariableDefinition ';'
            | Expression ';'

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

BinaryBooleanOperator ::= '&&' | '||' | '==' | '!='

OrderingOperator ::=  '>' | '>=' | '==' | '!=' | '<=' | '<'

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


# Types Theory

## Types

```
Types ::= IntegerTypes | FunctionTypes | bool | valid | ()

FunctionTypes ::= (TypeList) -> Types | () -> Types

TypeList ::= Types , TypeList | Types

IntegerTypes ::= i8 | u8 | i16 | u16 | i32 | u32 | i64 | u64 | isize | usize

```

## Contexts

```
s = StatementList
x = Expression
```

```
Γ ::= empty
    | Γ, x: Types
    | Γ, s valid
    | Γ, s validF
    | Γ, s1 forward s2
```

`Γ |- s validF` <-- indica que s é válido mesmo com forward declarations.

`Γ |- s1 forward s2` <-- indica que vamos carregar as declarações todas de s1, antes de avançarmos para verificar s2.

`Γ |- s valid` <-- Vamos verificar que s é válido, sem olhar para a frente (porque já o fizemos no validF que usou o s forward s)

## Literals

```
it ∈ IntegerTypes
----------------------
Γ |- n str(it) : it

----------------------
Γ |- true : bool

----------------------
Γ |- false : bool
```

## Operators
```
BinaryBooleanOperator ::= '&&' | '||' | '==' | '!='

OrderingOperator ::=  '>' | '>=' | '==' | '!=' | '<=' | '<'
```

```
Γ |- b1: bool  Γ |- b2: bool
------------------------------- (bool and or)
Γ |- b1 BinaryBooleanOperator b2:bool

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
```

## If expressions

```
Γ |- condition: bool  Γ |- then_expr: T  Γ |- else_expr: T
---------------------------------------------------------- (if expression)
Γ |- (if condition then_expr else else_expr): T
```

## Function calls

```
f: (A1, A2, ... An) -> B in Γ        Γ |- e1: A1 ... Γ |- en: An
---------------------------------------------------------------- (function call expression)
Γ |- f(e1, e2, ... en): B

```


## Statements

### Blocks

```
-------------------------- (empty block)
Γ |- {} : ()

Γ |- s validF  Γ |- e:T
-------------------------- (expression block)
Γ |- { s e } :T

Γ |- s validF 
-------------------------- (statements-only block)
Γ |- {s} : ()
```

### Statements (first-pass)

```
Γ |- s forward s
---------------------- (kickstarts double pass)
Γ |- s validF 

Γ |- s valid
---------------------- (empty statement forward)
Γ |- epsilon forward s 

Γ, x: T |- next forward s
----------------------------------- (let statement forward)
Γ |- (let x: T = e; next) forward s

Γ, x: (a1: T1, ... an: Tn) -> T |- next forward s
------------------------------------------------------------ (function statement forward)
Γ |- (fn x(a1: T1, ... an: Tn): T => expr; next) forward s

Γ, x: (a1: T1, ... an: Tn) -> T |- next forward s
----------------------------------------------------------------- (extern-def forward)
Γ |- (extern fn x(a1: T1, ... an: Tn): T; next) forward s
```

### Statements (second-pass)

```
------------------- (empty statement)
Γ |- epsilon valid


Γ |- e: T    Γ |- next valid
----------------------------------- (let statement)
Γ |- (let x: T = e; next) valid


Γ, a1: T1, ... an: Tn |- expr: T      Γ |- next valid
------------------------------------------------------------- (function Statement)
Γ |- (fn x(a1: T1, ... an: Tn): T => expr; next): valid


Γ |- next valid
----------------------------------------------------------------- (extern-def)
Γ |- (extern fn x(a1: T1, ... an: Tn): T; next) valid

```

# Dificuldades

1. forward dentro do expression block precisa ter semantica diferente do forward pra modulo
2. expression statements 

```
Γ |- e: T    T in Types
----------------------------------------------------------------- (expression statement)
Γ |- (e; next) valid
```

3. o quão próximo eu preciso estar da gramatica? eu posso marcar a ultima expression com um return como é na AST.

```
Γ |- (let y: i32; epsilon) validF  Γ |- x + y: i32
-------------------------------------------------------------------------
empty, foo: () -> i32, x: i32, goo: () -> i32 |- {let y: i32; x + y}: i32
```

## Example
```
                                                                                                                                empty, foo: () -> i32, x: i32, goo: () -> i32 |- {let y: i32; x + y}: i32   Γ |- epsilon valid
                                                                                                                                --------------------------------------------------------------------------------------------------------- ( function statement )
                                                                empty, foo: () -> i32, x: i32, goo:() -> i32 |- 5: i32          empty, foo: () -> i32, x: i32, goo: () -> i32 |- (fn goo(): i32 => {let y: i32 = 6; y}; epsilon) valid
                                                                ------------------------------------------------------------------------------------------------------------------------ (let statement)
empty, foo: () -> i32, x: i32, goo: () -> i32 |- goo(): i32      empty, foo: () -> i32, x: i32, goo: () -> i32 |- (let x: i32 = 5; fn goo(): i32 => {let y: i32 = 6; y}; epsilon) valid
---------------------------------------------------------------------------------------------------------------------------------------------- (function statement)
empty, foo: () -> i32, x: i32, goo: () -> i32 |- (fn foo(): i32 => goo(); let x: i32 = 5; fn goo(): i32 => {let y: i32 = 6; y}; epsilon) valid
-------------------------------------------------------------------------------------------------------------------------------------------------------- (empty statement forward)
empty, foo: () -> i32, x: i32, goo: () -> i32 |- epsilon forward (fn foo(): i32 => goo(); let x: i32 = 5; fn goo(): i32 => {let y: i32 = 6; y}; epsilon)
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- (function statement forward)
empty, foo: () -> i32, x: i32 |- (fn goo(): i32 => {let y: i32 = 6; y}; epsilon) forward (fn foo(): i32 => goo(); let x: i32 = 5; fn goo(): i32 => {let y: i32 = 6; y}; epsilon)
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- (let statement forward)
empty, foo: () -> i32 |- (let x: i32 = 5; fn goo(): i32 => {let y: i32 = 6; y}; epsilon) forward (fn foo(): i32 => goo(); let x: i32 = 5; fn goo(): i32 => {let y: i32 = 6; y}; epsilon)
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ (function statement forward)
empty |- (fn foo(): i32 => goo(); let x: i32 = 5; fn goo(): i32 => {let y: i32 = 6; y}; epsilon) forward (fn foo(): i32 => goo(); let x: i32 = 5; fn goo(): i32 => {let y: i32 = 6; y}; epsilon) 
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ (kickstart forward)
empty |- (fn foo(): i32 => goo(); let x: i32 = 5; fn goo(): i32 => {let y: i32 = 6; y}; epsilon) validF
```

# Proxima semana

1. Pedir ao professor por material de estudo, TaPL? Refinement Types in ML? other?
2. Finalizar essa etapa da teoria de tipos com o forwarding pra block;
3. Lembrar de não economizar com eletricistas.
