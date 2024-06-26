---
layout: cover
---

# Ekitai: A Programming Language with Refinement Types and its LLVM-IR Front End Implementation

---

# Introdução

- Compilador
- Sistema De Tipos 
- Tipos Refinados 
- LLVM
- O compilador Ekitai em funcionamento

---
layout: two-cols
---

<img src="/imgs/compiler_passes.png" style="height:75%;"/>

::right::

# O que é um Compilador?

- Uma serie de transformações que levam de um programa fonte a um programa alvo.
- Vantajoso separar um compilador em duas partes: "front end" e "back end".
- Front end:
  - verifica se o programa fonte esta *correto*
  - gera a representação intermediária.
- Back end:
  - optimiza a representação intermediária.
  - geral o programa alvo.

---

# Sistemas de Tipos

> Um método sintático tratável para provar a ausência de certos comportamentos de programa classificando sentensas de acordo com os tipos de valores que elas computam. -- Pierce (2002)

<br/>

Exemplo de comportamentos indesejaveis:

- Divisão por zero:
```
x: Int = 5
y: Int = 0

eval x / y // runtime error, divide by zero
```

<br/>

- Buffer Overflow:

```
array: [Int] = [0, 1, 2, 3]

eval array[4] // runtime error, access out of bounds
```

---

# Tipos refinados

- Utilização de predicados em Tipos.

```rust
fn safe_div(x: Int, y: {v: Int | v != 0}) -> Int {
  x / y
}

eval safe_div(5, 0) // Static error, y must be Int != 0
```

- Funções dependentes.

```rust
fn withdraw(
  balance: { x: Int | x >= 0},
  amount:  { y: Int | y <= x },
) -> { new_balance: Int | new_balance == balance - amount } {
//...
}

eval withdraw(50, 100) // compiler statically verifies this line to be incorrect
eval withdraw(100, 50) // compiler statically verifies this line to be correct
```

---

# Ecossistema LLVM

- Facil descrição de objetos em memória.
```
; Define a struct type named "Person" with two fields
%Person = type { i32, i8* }
``` 
- Facil construção de procedimentos atraves da sua representação em Static Single Assignment (SSA).
```
define i32 @absolute_value(i32 %x) {
entry:
  %is_negative = icmp slt i32 %x, 0
  br i1 %is_negative, label %negative, label %positive
negative:                                          ; preds = %entry
  %negated_value = sub i32 0, %x
  ret i32 %negated_value
positive:                                          ; preds = %entry
  ret i32 %x
}
``` 
- Facil aproveitamento das diverças optimizações de código existentes.
```
mem2reg; reg2mem; loop-unroll; inline; ...
```

--- 

# Problema da pesquisa 

- Multiplas implementações de refinamentos construidos em cima de linguagens existentes.
  - LiquidHaskell
- Ausencia de pesquisa sobre a integração de *tipos refinados* na etapa de geração de codigo intermediario de um front end de um compilador.

---

# A Linguagem Ekitai

Em seu core a linguagem ekitai é composta de um subset do lambda calculus com refinamentos.

<img src="/imgs/ekitai_ast.png" style="height:75%"/>

---

# A Linguagem Ekitai

Exemplo:

```rust
fn abs(x: {x2:i64| true}) -> {z:i64| z >= x}
{
  if x >= 0 {
    x
  } else { 
    -x 
  } 
}
```

---

# Type Checker Ekitai

<br/>

- Composto de um type checker que implementa regras de inferencias bidirecionais.

- Tem como objetivo gerar constraints que serão avaliadas por um SMT Solver durante a execução das regras de inferencia.

---

# Type Checker Ekitai

## Synthesis Rules  

<br/>
<br/>
<br/>

<img src="/imgs/syn_rule.png"/>

---

# Type Checker Ekitai

## Synthesis pseudo algoritimo

<img src="/imgs/syn_algo.png" style="height:75%"/>

---

# Type Checker Ekitai

## Checking Rules

<img src="/imgs/check_rule.png" style="height:75%"/>

---

# Type Checker Ekitai

## Checking pseudo algoritimo

<img src="/imgs/check_algo.png" style="height:75%"/>

---

# Type Checker Ekitai

## Entailment

<br/>
<br/>
<br/>

<img src="/imgs/entailment_rule.png"/>

---

# Type Checker Ekitai

<br/>
<br/>

## Aplicação de entailment para sintese

<br/>

```
if synth(Γ, e) = (c, t) and Γ ⊢ c then Γ ⊢ e ⇒ t
```

<br/>
<br/>

## Aplicação de entailment para checking

<br/>

```
if check(Γ, e, t) = c and Γ ⊢ c then Γ ⊢ e ⇐ t
```

---

# Type Checker Ekitai

Constraint generation example:

Função identidade com refinamentos:

```
fn id(x: {y: i64 | true}) -> {z: i64 | z == x} {
  x
}
```

---

# Type Checker Ekitai

Exemplo de geração de constraint:

Contexto após aplicação das regras de tipos refinados da função id:

```
Context { bindings: [
  ( Path { segments: [Name { id: "id" }] },
    Fn(DependentFunction { 
      parameter: (
        Name { id: "x" },
        RefinedBase { base: Scalar(Integer(I64)), binder: Name { id: "y" }, predicate: Boolean(true) }
      ),
      tail_type: Base(RefinedBase {
        base: Scalar(Integer(I64)), binder: Name { id: "z" },
        predicate: Binary(
          Compare(Equality { negated: false }), Variable(Name { id: "z" }), Variable(Name { id: "x" })
        ) 
      }) 
    })
  ), 
  ( Path { segments: [Name { id: "x" }] },
    Base(RefinedBase { base: Scalar(Integer(I64)), binder: Name { id: "y" }, predicate: Boolean(true) })
  ),
] }
```

---

# Type Checker Ekitai

Exemplo de geração de constraint:

Constraint após aplicação das regras de tipos refinados e construção do contexto da função id:

```
Implication { 
  binder: Name { id: "x" }, 
  base: Scalar(Integer(I64)), 
  antecedent: Boolean(true), 
  consequent: Implication { 
    binder: Name { id: "y" },
    base: Scalar(Integer(I64)),
    antecedent: Binary( Logic(And), Boolean(true), Binary( Compare(
        Equality { negated: false }),
        Variable(Name { id: "y" }),
        Variable(Name { id: "x" })
      )
    ),
    consequent: Predicate(Binary(Compare(
      Equality { negated: false }),
      Variable(Name { id: "y" }),
      Variable(Name { id: "x" }))
    )
  } 
}
```

---

# Type Checker Ekitai

Exemplo de validação de constraint em Z3:

```
(declare-fun x () Int)
(declare-fun y () Int)
(assert (and true true (= y x) true true (not (= y x))))

Result: Unsat
```

O resultado Unsat significa que não existe nenhuma variante onde a constraint é **falsa** logo o programa esta corréto.

---

# Type Checker Ekitai

Exemplo de erro:

Função identidade com refinamentos com erro:

```
fn id(x: {y: i64 | true}) -> {z: i64 | z == x} {
  -x
}
```

---

# Type Checker Ekitai

Exemplo de erro:

Prova do erro através do Z3:

```
Solver:
(declare-fun x () Int)
(declare-fun ret () Int)
(declare-fun __arg0 () Int)
(assert (and true
     (= ret (- __arg0))
     (and true (= __arg0 x))
     true
     (and true (= __arg0 x))
     true
     (not (= ret x))))

Result: Sat
```

---

# Codegen Ekitai


- Source:

```
fn id(x: {y: i64 | true}) -> {z: i64 | z == x} {
  x
}
```

- Código llvm gerado:

```
; ModuleID = 'ekitai_module'
source_filename = "ekitai_module"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

define i64 @id(i64 %x) {
  ret i64 %x
}
```

---

# Codegen Ekitai


- Source:

```
fn abs_liquid(x: {y: i64 | true}) -> {z: i64 | z >= 0} {
  if x > 0 {
    x
  } else {
    -x
  }
}
```

---

# Codegen Ekitai

- Código llvm gerado:

```
; ModuleID = 'ekitai_module'
source_filename = "ekitai_module"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

define i64 @abs_liquid(i64 %x) {
  %1 = icmp sgt i64 %x, 0
  br i1 %1, label %then, label %else

then:                                             ; preds = %0
  br label %merge

else:                                             ; preds = %0
  %2 = sub i64 0, %x
  br label %merge

merge:                                            ; preds = %else, %then
  %phi = phi i64 [ %x, %then ], [ %2, %else ]
  ret i64 %phi
}
```

---

# Demonstração

---

# Obrigado!
