# C-style arrays - fixed size specified at compile time
# - Size is encoded in the type

type
  ThreeStringAddress = array[3, string]

let names: ThreeStringAddress = ["Jasmine", "Ktisztina", "Kristof"]
let addresses: ThreeStringAddress = ["101 Betburweg", "66 Bellion Drive", "194 Laarderweg"]
  # Won't compile - wrong size
  # let twoNames: ThreeStringAddress = ["Ben", "Sara"]

  # Procs using arrays of variable length must encode the length as a type
  # parameter
proc zip[I, T](a, b: array[I, T]): array[I, tuple[a, b: T]] =
  for i in low(a)..high(a):
    result[i] = (a[i], b[i])

let nameAndAddresses = names.zip(addresses)
echo nameAndAddresses[1] # --> (a: "Ktisztina", b: "66 Bellion Drive")

# First type parameter is a *range*
# - An integer (as above) is syntax sugar for `0..N-1`
# - Can index arrays by other methods e.g. ordinal values to create a lookup table

type
  PartsOfSpeech {.pure.} = enum
    Pronoun, Verb, Article, Adjective, Noun, Adverb

let partOfSpeechExamples: array[PartsOfSpeech, string] = [
  "he", "reads", "the", "green", "book", "slowly"
]

echo partOfSpeechExamples[Verb] # --> "reads"

type
  Matrix[W, H: static[int]] =
    array[1..W, array[1..H, int]]

# Add two matrices of equal dimensions
proc `+`[W, H](a, b: Matrix[W, H]):
               Matrix[W, H] =
  for i in 1..high(a):
    for j in 1..high(a[0]):
      result[i][j] = a[i][j] + b[i][j]

let mat1: Matrix[2, 2] = [[1, 0],
                          [0, 1]]
let mat2: Matrix[2, 2] = [[0, 1],
                          [1, 0]]
let mat3: Matrix[2, 3] = [[0, 1, 0],
                          [1, 0, 1]]

# See `$` from the procs section

echo mat1 + mat2
# Won't compile - different dimensions
# echo mat1 + mat3
