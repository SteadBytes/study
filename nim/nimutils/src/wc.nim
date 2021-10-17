import os, strutils, streams, strformat, parseopt

const usage = """
Usage:
  wc [OPTIONS] [FILE ...]
Print word count for each FILE, and a total line if more than one FILE is specified. A word is a non-zero-length sequence of characters delimited by white space.

With no FILE, or when FILE is -, read standard input.

Options:
  -h, --help  show this help
"""

proc iterLines(stream: Stream): iterator(): string =
  result = iterator(): string =
    while not stream.atEnd:
      yield stream.readLine()

proc wc*(lines: iterator(): string): int =
  ## Count words in `lines`. A word is defined as a sequence of non-whitespace
  ## characters.
  result = 0
  for l in lines():
    for w in l.split():
      result += 1

proc wc*(strm: FileStream): int =
  wc(iterLines(strm))

proc wc*(f: File): int =
  wc(newFileStream(f))

proc main() =
  if paramCount() == 0 or (paramCount() == 1 and paramStr(1) == "-"):
    echo wc(stdin)
    return

  var total = 0
  for kind, key, value in getOpt():
    case kind
    # TODO: Implement Rust `unreachable` macro?
    of cmdEnd: doAssert(false) # not possible with getOpt
    of cmdArgument:
      let strm = try:
        openFileStream(key)
      except:
        quit(&"cannot open {key}: {getCurrentExceptionMsg()}")
      let n = wc(strm)
      total += n
      echo n, " ", key
    of cmdShortOption, cmdLongOption:
      case key:
        of "h", "help":
          echo usage
          return
        else:
          quit(&"Unknown option '{key}'\nTry 'wc --help' for more information.", 1)

  if paramCount() > 1:
    echo total, " total"

when isMainModule:
  main()
