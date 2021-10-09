# Distinct types are similar to type aliases, however they provide type safety
# and cannot be implicitly converted into their base type.

type
  Dollars* = distinct float

var a = 20.Dollars

# Won't compile - `25` is of type `float`, not `Dollars`
# a = 25
a = 25.Dollars

# None of the base type's procs are carried over. The `{.borrow.} pragma can be
# used to automate generation of procs from the base type.

# Won't compile - `*` and `+` are not defined for `Dollars`
#a = 20.Dollars * 20.Dollars
#a = 20.Dollars + 20.Dollars

# Define `*` and `+` to be the same as the base type
proc `*` *(a, b: Dollars): Dollars {.borrow.}
proc `+` *(a, b: Dollars): Dollars {.borrow.}

a = 20.Dollars * 20.Dollars
a = 20.Dollars + 20.Dollars

# Base type fields are also not carried over.

type
  Foo = object
    a: int

  MyFoo = distinct Foo

# Won't compile - `MyFoo` has not field `a`
# var x: MyFoo
# echo x.a

# Use `{.borrow.}` to define *all* of the fields from `Foo` for `OtherFoo`
# - This borrows the "dot accessor" from `Foo` to allow access to it's fields
type
  OtherFoo {.borrow: `.`.} = distinct Foo

var x: OtherFoo
echo x.a
