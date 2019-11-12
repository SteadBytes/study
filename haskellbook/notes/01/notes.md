# Lambda Calculus

**Lambda terms**:

- Expressions
- Variables
- Abstractions
  - _Function_

**Expression** = _superset_ of the lambda terms

- Simplest is a single variable

**Variable** = name for potential inputs to functions

- No meaning or value

**Function** = Abstraction -> lambda term with a _head_ and body that is applied to an _argument_

- _head_ = λ followed by variable name
- _body_ = another expression

## Alpha Equivalence

Equivalence between lambda terms due to variables not being semantically meaningful other than their roles within a single expression

_λx.x = λd.d = λz.z_

## Beta Reduction

Process of applying a lambda term to an argument, replacing bound variables with the value of the argument and _eliminating the head_ (function has been applied)

- Stops when no more heads or lambdas left to apply _or_ no more arguments to apply functions to

Applications are **left associative**

- _λx.x+1_
  - Apply to _2_:
    - _λ2.2+1_
    - _2+1_
    - _3_
  - Apply to _10_:
    - _λ10.10+1_
    - _10+1_
    - _11_

Applying identity function to another lambda abstraction:

- _(λx.x)(λy.y)_
- _[x := (λy.y)]_
  - Indicates substitution
- _λy.y_

- _(λx.x)(λy.y)z_
- _[x := (λy.y)]_
- _(λy.y)z_
- _[y := z]_
- _x_

## Free Variables

Variables _not_ named in the head of a function:

- _(λx.xy)z_
- _(λ[x := z].xy)_
- _zy_

Alpha equivalence **does not apply**:

- _λx.xz != λx.xy_
  - _z_ and _y_ may be different

## Intermission: Equivalence Exercises

1. _λxy.xz = λmn.mz_
2. _λxy.xxy = λa(λb.aab)_
3. _λxyz.zx = λtos.st_

## Beta Normal Form

Cannot reduce the terms any further

- Unable to apply lambdas to arguments

**Fully evaluated expression**

## Combinators

Lambda term with **no free variables**

- Every term in the body occurs in the head
  - No _free variables_

Only purpose is to _combine_ their arguments

Examples:

- _λx.x_
- _λxy.x_
- _λxyz.xz(yz)_

## Divergence

Non-terminating reduction process

- Don't reduce to beta normal form (convergence)

_Omega_ lambda example:

- _(λx.xx)(λx.xx)_
- _([x := (λx.xx)]xx)_
- _(λx.xx)(λx.xx)_
  - Back to original expression -> diverges

Divergent terms don't produce an _answer_ or meaningful result
