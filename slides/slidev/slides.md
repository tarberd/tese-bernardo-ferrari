---
layout: cover
---

# Ekitai: A Programming Language with Refinement Types and its LLVM-IR Front End Implementation

---

# Introdução

- Compilador
- Porque usar LLVM-IR
- Uso de Tipos
- Tipos Refinados 
- Geração de LLVM-IR

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

> Um método sintático tratável para provar a ausência de certos comportamentos de programa classificando frases de acordo com os tipos de valores que elas computam. -- Pierce (2002)

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
) -> { new_balance: Int | new_balance == amount } {
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






