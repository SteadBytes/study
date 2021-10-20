import os, strformat, parseopt, posix

const usage = """
Usage:
  du [OPTIONS] [FILE]...
Report file system space used by files and/or directories and their subdirectories.

Options:
  -b          print apparent sizes rather than disk usage
  -h, --help  show this help

With no FILE, report file system space for the current directory.
"""

proc blocks*(size: BiggestInt): BiggestInt =
  size shr 1

proc report(size: BiggestInt, path: string, apparent_size: bool = false) =
  let size = if apparent_size: size else: size.blocks
  echo &"{size} {path}"

proc du*(path: string; apparent_size: bool = false, depth = 0): BiggestInt =
  var statbuf: Stat
  if lstat(path, statbuf) < 0:
    # FIXME
    raiseOSError(osLastError(), path)

  # TODO: Optionally handle symlinks?

  var sum = BiggestInt(if apparent_size: statbuf.st_size else: statbuf.st_blocks)
  if S_ISDIR(statbuf.st_mode):
    # Adapted from os.WalkDir https://github.com/nim-lang/Nim/blob/80c8107c560d91afae2c7596ab196cb0f7c30860/lib/pure/os.nim#L2148
    var dir = opendir(path)
    if dir == nil:
      raiseOSError(osLastError(), path)
    while true:
      var entry = readdir(dir)
      if entry == nil: break
      when defined(nimNoArrayToCstringConversion):
        var name = $cstring(addr entry.d_name)
      else:
        var name = $x.d_name.cstring
      if name != "." and name != "..":
        let newpath = path / name
        sum += du(newpath, apparent_size = apparent_size, depth = depth+1)
  elif depth != 0:
    # Don't report individual files
    return sum
  report(sum, path, apparent_size = apparent_size)
  return sum

proc main() =
  var files: seq[string]
  var apparent_size, show_total = false
  for kind, key, value in getOpt():
    case kind
    of cmdEnd: doAssert(false) # not possible with getOpt
    of cmdArgument:
      # [FILE]...
      files.add(key)
      discard
    of cmdShortOption, cmdLongOption:
      # [OPTIONS]
      case key:
        of "b":
          apparent_size = true
        of "c":
          show_total = true
        of "h", "help":
          echo usage
          return
        else:
          quit(&"Unknown option '{key}'\nTry 'du --help' for more information.", 1)

  if files.len() == 0:
    files.add(".")

  var total = BiggestInt(0)
  for f in files:
    total += du(f, apparent_size = apparent_size)

  if show_total:
    report(total, "total", apparent_size = apparent_size)

when isMainModule:
  main()
