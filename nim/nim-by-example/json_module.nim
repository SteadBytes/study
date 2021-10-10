import json

let element = "Hydrogen"
let atomicNumber = 1

# Create JSON from Nim objects
block creating:
  # `json` module `%*` operator *creates* JSON objects
  let jsonObject = %* {"element": element, "atomicNumber": atomicNumber}
  # `$` to convert to string
  echo $jsonObject # --> {"element":"Hydrogen", "atomicNumber": 1}

# Parse raw JSON strings
block parsing:
  let rawObject = """{"name": "Sky", "age": 32}"""
  let rawArray = """[7, 8, 9]"""

  # `parseJson` parses a raw JSON string into a `JsonNode` object
  let parsedObject = parseJson(rawObject)
  # `JsonNode` has accessor methods for converting values into Nim data types
  let name = parsedObject["name"].getStr()
  echo name # --> Sky

  let parsedArray = parseJson(rawArray)
  let eight = parsedArray[1].getInt()
  echo eight # --> 8

# Unmarshal JSON into Nim Objects
block unmarshalling:
  type
    Element = object
      name: string
      atomicNumber: int

  # `to` unmarshals JSON into an object of the given type
  let x = to(parseJson("""{"name": "Carbon", "atomicNumber": 6}"""), Element)
  echo x.name # --> Carbon
  echo x.atomicNumber # --> 6

  # Additional fields are ignored
  let y = to(parseJson("""{"name": "Carbon", "atomicNumber": 6, "notAField": 1234}"""), Element)
  assert (x == y)

  # Invalid field values raise an exception
  doAssertRaises(ValueError):
    let z = to(parseJson("""{"name": "Carbon", "atomicNumber": "six"}"""), Element)

