# Read contents of a file into a string
let entireFile = readFile("kittens.txt")
echo entireFile # print the entire file

proc readKittens() =
  let f = open("kittens.txt")
  # Close the file at the end of the current block
  defer: f.close()
  let firstLine = f.readLine()
  echo firstLine # --> Spitfire

readKittens()

# Write a string into a file, creating it if necessary.
let text = "Cats are very cool!"
writeFile("cats.txt", text)

proc writeCatActivities() =
  let lines = ["Play", "Eat", "Sleep"]
  # `fmWrite` indicates the file should be opened for reading
  let f = open("catactivities.txt", fmWrite)
  defer: f.close()

  # Write to a file line by line
  for line in lines:
    f.writeLine(line)

writeCatActivities()
