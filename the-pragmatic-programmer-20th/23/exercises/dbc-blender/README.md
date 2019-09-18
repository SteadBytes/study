# dbc-blender

The Pragmatic Programmer Exercise 14

> Design an interface to a kitchen blender. It will eventually be a web-based, IoT-enabled blender, but for now we just need the interface to control it. It has ten speed settings (0 means off). You can’t operate it empty, and you can change the speed only one unit at a time (that is, from 0 to 1, and from 1 to 2, not from 0 to 2).
>
> Here are the methods. Add appropriate pre- and postconditions and an invariant.
>
> ```
> int getSpeed()
> void setSpeed(int x)
> boolean isFull()
> void fill()
> void empty()
> ```

Blender modelled as a `map`

`getSpeed` and `isFull` are not implemented -> use `(::speed blender)`/`(::full blender)`

`void` methods return blender maps -> no mutation

Implements design by contract using pre and post conditions along with [`spec`](https://clojure.org/guides/spec).
