# Nim objects define a _grouping of fields_ e.g. C structs
# - Traced by the garbage collector (by default at least)

type
  # Exported object type
  Animal* = object
    # Exported fields
    name*, species*: string
    # Private fields
    age: int

proc sleep*(a: var Animal) =
  a.age += 1

proc dead*(a: Animal): bool =
  result = a.age > 20

# Created on the stack, initialised to zeros
var carl: Animal
# `carl` is mutable
carl = Animal(name: "Carl",
              species: "L. glama",
              age: 12)

# Created on the stack, initialised with the given values
let joe = Animal(name: "Joe",
                 species: "H. sapiens",
                 age: 23)
  # Won't compile - `joe` is immutable
  # joe.sleep()
  # joe.species = "Dog"

assert(not carl.dead)
for i in 0..10:
  carl.sleep()
assert carl.dead

# *Reference* to a heap-allocated object
let mittens: ref Animal = new(Animal)

# Won't compile - the value of `mittens` is immutable so it can never point to
# anything else
# mittens = new(Animal)

# The object being pointed to, however, *is* mutable
mittens.name = "Mittens"
mittens.species = "P. leo"
mittens.age = 6

# Naming the reference type allows for more concise initialisation
type
  AnimalRef* = ref Animal

let spot = AnimalRef(name: "Spot",
                   species: "C. lupus",
                   age: 1)

# Type declartion using `ref object` makes the object *always* a reference type
type
  Thing* = ref object
    positionX*, positionY*: int

let it = Thing(1, 2)
