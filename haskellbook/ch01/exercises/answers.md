## Combinators

1. Yes
2. No
3. Yes
4. Yes
5. No

## Normal Form or Diverge?

1. Normal form
2. Diverge (Omega)
3. Normal form

## Beta Reduce

1. _(λabc.cba)zz(λwv.w)_
   - _(λa.λb.λc.cba)(z)z(λw.λv.w)_
   - _(λb.λc.cbz)(z)(λw.λv.w)_
   - _(λc.czz)(λw.λv.w)_
   - _(λw.λv.w)(z)z_
   - _(λv.z)(z)_
   - _z_
2. _(λx.λy.xyy)(λa.a)b_
   - _(λy(λa.a)yy)(b)_
   - _(λa.a)(b)b_
   - _bb_
3. _(λy.y)(λx.xx)(λz.zq)_
   - _(λx.xx)(λz.zq)_
   - _(λz.zq)(λz.zq)_
   - _(λz.zq)(q)_
   - _qq_
4. _(λz.z)(λz.zz)(λz.zy)_
   - _(λz.zy)(λz.zz)_
   - _(λz.zz)(y)_
   - _yy_
5. _(λx.λy.xyy)(λy.y)y_
   - _(λx.λy.xyy)(λy.y)y_
   - _(λy(λy.y)yy)y_
   - _(λy.y)(y)y_
   - _yy_
6. _(λa.aa)(λb.ba)c_
   - _(λb.ba)(λb.ba)c_
   - _(λb.ba)(a)c_
   - _(λb.ba)(a)c_
   - _aac_
7. _(λxyx.xz(yz))(λx.z)(λx.a)_
   - _(λx.λy.λz.xz(yz))(λx.z)(λx.a)_
   - _(λy.λz<sub>1</sub>.(λx.z)z<sub>1</sub>(yz<sub>1</sub>))(λx.a)_
   - _(λz<sub>1</sub>.(λx.z)z<sub>1</sub>((λx.a)z<sub>1</sub>))_
   - _(λz<sub>1</sub>.z((λx.a)z<sub>1</sub>)))_
   - _(λz<sub>1</sub>.za)_
