## Test utilities
import os, strutils, random, sequtils, strformat

const
  TempDirNameLength = 12
  TempDirMaxAttempts = high(int)
  TempDirMaxAttemptsExceededMsg = &"failed to generate tempdir name after {TempDirMaxAttempts} attempts"

# TODO: Use destructors for removing the directory?

## Create a temporary directory.
##
## The following example code creates a directory with a random name in the
## system temporary directory (e.g. /tmp):
##
## .. code-block:: nim
##  import os
##  import utils
##
##  let dir = tempdir()
##  defer: removeDir(dir)
##
proc tempdir*(prefix: string = "", basePath: string = ""): string =
  let base =
    if len(basePath) < 1:
      getTempDir()
    elif isAbsolute(basePath):
      basePath
    else:
      getCurrentDir() / basePath

  var
    path: string
    name: string = prefix.alignLeft(len(prefix)+TempDirNameLength)

  for i in 0..TempDirMaxAttempts:
    name[len(prefix)..^1] = TempDirNameLength.newSeqWith(sample(HexDigits)).join
    path = base / name

    if not dirExists(path):
      createDir(path)
      return path

  raise newException(IOError, TempDirMaxAttemptsExceededMsg)
