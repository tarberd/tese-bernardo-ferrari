# Minute from 2020-10-27

## What have I done: 

1:15 hrs: Reading TaPL book ch. 3:

1. The name of the game is operational semantics;
2. Meta-variables are the names on the left side of productions and generate sets;
3. Rules are meta rules;
4. Grammar relates to the AST and not to the syntax.

3:00 hrs: Expanded the operational semantics to include:

1. return statements;
2. expression statements;
3. block double pass.

### AST
```
Module ::= StatementList

StatementList ::= extern fn Id ( ParameterList ) : Type ; StatementList
                | fn Id ( ParameterList ) : Type => Expression ; StatementList
                | let Id : Type = Expression ; StatementList
                | return Expression ; StatementList
                | Expression ; StatementList
                | epsilon

Expression ::= BlockExpression
             | Expression BinOp Expression
             | UnaryOp Expression
             | Id ( ArgumentList )
             | if Expression BlockExpression else BlockExpression 
             | Id
             | BooleanLiteral
             | IntegerLiteral
             | ()

BlockExpression ::= { StatementList Expression }

BinOp ::= || | && | > | >= | == | <= | < | != | + | - | * | /

UnaryOp ::= - | !

ArgumentList ::= Expression , ArgumentList
               | Expression
               | epsilon

BooleanLiteral ::= true | false

IntegerLiteral ::= NumToken IntegerType

Type ::= IntegerType
       | bool 
       | ()
       | fn(TypeList) -> Type

IntegerType ::= i8 | u8 | i16 | u16 | i32 | u32 | i64 | u64 | isize | usize

TypeList ::= Type , TypeList
           | epsilon
```

### Context

```
Γ ::= empty
    | Γ, Expression: Type
    | Γ, StatementList valid
    | Γ, StatementList validF
    | Γ, StatementList1 forward StatementList2
    | Γ, StatementList valid_block Expression Type
    | Γ, StatementList validF_block Expression Type
    | Γ, StatementList1 forward_block StatementList2 Expression Type
```


### Expressions

#### Literals 

```
n in NumToken
it in IntegerType
----------------- (integer literal)
Γ |- n it : it

---------------------- (true) 
Γ |- true : bool

---------------------- (false)
Γ |- false : bool

---------------------- (unit)
Γ |- () : ()
```

#### Operators

```
Γ |- b1 : bool
Γ |- b2 : bool
bop in { ==, !=, ||, && } 
------------------------- (bool bin algebra)
Γ |- b1 bop b2 : bool

Γ |- b: bool
--------------- (bool-not)
Γ |- ! b : bool 

Γ |- n1: it
Γ |- n2: it
it in IntegerType
bop in { >, >=, ==, !=, <=, < }
------------------------------- (integer compare)
Γ |- b1 bop b2 : bool

Γ |- n1: it
Γ |- n2: it
it in IntegerType
bop in { +, -, *, / }
--------------------- (integer bin algebra)
Γ |- n1 bop n2 : it

Γ |- n: it
it in IntegerType
----------------- (integer minus)
Γ |- - n: T
```

#### if

```
Γ |- condition: bool
Γ |- then_expr: T
Γ |- else_expr: T
----------------------------------------------- (if expression)
Γ |- (if condition then_expr else else_expr): T
```

#### function call

```
f: fn(A1, A2, ... An) -> B in Γ
Γ |- e1: A1 ... Γ |- en: An
-------------------------------- (function call expression)
Γ |- f(e1, e2, ... en): B
```

#### block

```
Γ |- s validF_block e T
-------------------------------------------------- (start block double pass)
Γ |- { s e } : T

Γ |- e: T
-------------------------------------------------- (expression block)
Γ |- s valid_block e T
```

### Statements (first pass block)

```
Γ |- s forward_block s e T
---------------------- (kickstarts double pass)
Γ |- s validF_block e T

Γ |- s valid_block e T
-------------------------------- (empty statement forward)
Γ |- epsilon forward_block s e T

Γ |- next forward_block s e T
--------------------------------------------- (let statement forward)
Γ |- (let x: T = e; next) forward_block s e T

Γ, x: fn(a1: T1, ... an: Tn) -> T |- next forward_block s e T
-------------------------------------------------------------------- (function statement forward)
Γ |- (fn x(a1: T1, ... an: Tn): T => expr; next) forward_block s e T

Γ, x: fn(a1: T1, ... an: Tn) -> T |- next forward_block s e T
------------------------------------------------------------------- (extern-def forward)
Γ |- (extern fn x(a1: T1, ... an: Tn): T; next) forward_block s e T

Γ |- next forward_block s e T
---------------------------------- (expression statement forward)
Γ |- (e; next) forward_block s e T

Γ |- next forward_block s e T
----------------------------------------- (return statement forward)
Γ |- (return e; next) forward_block s e T
```

### Statements (second pass block)

```
---------------------------- (empty statement)
Γ |- epsilon valid_block e T

Γ |- e: T1
Γ, x: T1 |- next valid_block e TB
----------------------------------------- (let statement)
Γ |- (let x: T1 = e; next) valid_block e TB

Γ, a1: T1, ... an: Tn, expected_return T |- expr: T 
Γ |- next valid_block e TB
------------------------------------------------------ (function Statement)
Γ |- (fn x(a1: T1, ... an: Tn): T => expr; next) valid_block e TB

Γ |- next valid_block e TB
----------------------------------------------------- (extern-def)
Γ |- (extern fn x(a1: T1, ... an: Tn): T; next) valid_block e TB

Γ |- next valid_block e TB
Γ |- e : T
T in Type
------------------------------- (expression statement)
Γ |- (e; next) valid_block e TB

Γ |- next valid_block eb TB
Γ |- e : T
expected_return T in Γ
--------------------------------------- (return statement forward)
Γ |- (return e; next) valid_block eb TB
```

### Statements (first pass)

```
Γ |- s forward s
---------------------- (kick-starts double pass)
Γ |- s validF 

Γ |- s valid
---------------------- (empty statement forward)
Γ |- epsilon forward s 

Γ, x: T |- next forward s
----------------------------------- (let statement forward)
Γ |- (let x: T = e; next) forward s

Γ, x: fn(a1: T1, ... an: Tn) -> T |- next forward s
---------------------------------------------------------- (function statement forward)
Γ |- (fn x(a1: T1, ... an: Tn): T => expr; next) forward s

Γ, x: fn(a1: T1, ... an: Tn) -> T |- next forward s
--------------------------------------------------------- (extern-def forward)
Γ |- (extern fn x(a1: T1, ... an: Tn): T; next) forward s

Γ |- next forward s
------------------------ (expression statement forward)
Γ |- (e; next) forward s

Γ |- next forward s
------------------------ (return statement forward)
Γ |- (return e; next) forward s
```

### Statements (second pass)

```
------------------ (empty statement)
Γ |- epsilon valid

Γ |- e: T
Γ |- next valid
------------------------------- (let statement)
Γ |- (let x: T = e; next) valid

Γ, a1: T1, ... an: Tn, expected_return T |- expr: T      Γ |- next valid
------------------------------------------------------ (function Statement)
Γ |- (fn x(a1: T1, ... an: Tn): T => expr; next) valid

Γ |- next valid
----------------------------------------------------- (extern-def)
Γ |- (extern fn x(a1: T1, ... an: Tn): T; next) valid

Γ |- next valid  Γ |- e : T
---------------------------- (expression statement)
Γ |- (e; next) valid

Γ |- next valid
Γ |- e : T
expected_return T in Γ
------------------------ (return statement forward)
Γ |- (return e; next) forward s
```

# Dificuldades

1. Expression statement T if free? each variable should be bound to a set?
2. is it ok to keep piling up expected_return on the context? function inside function return

