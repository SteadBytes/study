1. 
```scala
scala> 3.
!=   <     abs            equals       isInstanceOf    isWhole     shortValue       toFloat         unary_-
##   <<    asInstanceOf   floatValue   isNaN           longValue   signum           toHexString     unary_~
%    <=    byteValue      floor        isNegInfinity   max         synchronized     toInt           underlying
&    ==    ceil           formatted    isPosInfinity   min         to               toLong          until
*    >     compare        getClass     isValidByte     ne          toBinaryString   toOctalString   wait
+    >=    compareTo      hashCode     isValidChar     notify      toByte           toRadians       |
-    >>    doubleValue    intValue     isValidInt      notifyAll   toChar           toShort         →
->   >>>   ensuring       isInfinite   isValidLong     round       toDegrees        toString
/    ^     eq             isInfinity   isValidShort    self        toDouble         unary_+
```

2. 

```scala
scala> import scala.math._
import scala.math._

scala> sqrt(3)
res0: Double = 1.7320508075688772

scala> 3 - res0
res1: Double = 1.2679491924311228
```

3. `res` variables are `val`:
```scala
scala> res0 = 1
<console>:15: error: reassignment to val
       res0 = 1
            ^
```

4. `"crazy" * 3` concatenates `"crazy"` with itself three times:

```scala
scala> "crazy" * 3
res2: String = crazycrazycrazy
```

Documentation is found in the `*` method of `scala.collection.StringOps` -
https://www.scala-lang.org/api/current/https://www.scala-lang.org/api/current/scala/collection/StringOps.html#*(n:Int):Stringscala/collection/StringOps.html#*(n:Int):String

5. `max` returns the maximum value of it's two operands:

```scala
scala> 10 max 2
res3: Int = 10
```

`max` is defined on the `RichInt` class and is added to the `Int` class via
implicit conversion from `Int` to `RichInt` - https://www.scala-lang.org/api/current/scala/Int.html#max(that:Int):Int

6. 
```scala
scala> BigInt(2) pow 1024
res8: scala.math.BigInt = 179769313486231590772930519078902473361797697894230657273430081157732675805500963132708477322407536021120113879871393357658789768814416622492847430639474124377767893424865485276302219601246094119453082952085005768838150682342462881473913110540827237163350510684586298239947245938479716304835356329624224137216
```

Explicit use of `BigInt` is necessary as `pow` returns a `Double` by default
which is overflowed by 2<sup>1024</sup>.

```scala
scala> pow(2, 1024)
res11: Double = Infinity
```

7. `BigInt.probablePrime` and `util.Random`

8. Generate a random `BigInt`, then use the `toString` method of `BigInt` with
a base of `36`.

```scala
scala> probablePrime(100, Random).toString(36)
res12: String = 2r2q2273iltnr5j2j9nd
```

9.
```scala
scala> val s = "hello"
s: String = hello

scala> s.head
res15: Char = h

scala> s(0)
res16: Char = h

scala> s.last
res17: Char = o

scala> s(s.length - 1)
res18: Char = o
```

10. `take` returns a string of the first `n` characters of a string, `drop`
returns a string of the last `l - n` characters where `l` is the length of the
string. `takeRight` and `dropRight` are the "right to left" equivalents of
these. They are more concise and declarative than substring and reduce the risk
of indexing errors.

```scala
scala> val s = "hello world"
s: String = hello world

scala> s.take(3)
res19: String = hel

scala> s.drop(3)
res20: String = lo world

scala> s.takeRight(3)
res21: String = rld

scala> s.dropRight(3)
res22: String = hello wo
```
