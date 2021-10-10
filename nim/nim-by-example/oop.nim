# Nim supports Object Oriented Programming via inheritance and methods. Methods
# are *virtual procedures* - same as procs except the runtime type of the
# object that the method is called with is used to determine which *version* of
# the method to call (virtual dispatch).

# `of` keyword in type declaration defines inheritance.  `RootObj` is the root
# of Nim's object heirarchy.
# - To use OOP, objects must inherit from it
# - Similar to inheriting from `object` in Python 2.
type Animal = ref object of RootObj
  name: string
  age: int
method vocalize(this: Animal): string {.base.} = "..."
method ageHumanYrs(this: Animal): int {.base.} = this.age

type Dog = ref object of Animal
 # Override methods by creating new methods with the parameter type of the
 # subtype
method vocalize(this: Dog): string = "woof"
method ageHumanYrs(this: Dog): int = this.age * 7

type Cat = ref object of Animal
method vocalize(this: Cat): string = "meow"


var animals: seq[Animal] = @[]
animals.add(Dog(name: "Sparky", age: 10))
animals.add(Cat(name: "Mitten", age: 10))

for a in animals:
  echo a.vocalize()
  echo a.ageHumanYrs()

# Testing for subtype relations
echo(animals[0] of Dog) # --> true
echo(animals[0] of Cat) # --> false
echo(animals[0] of Animal) # --> true
