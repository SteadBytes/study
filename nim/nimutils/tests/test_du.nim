import unittest, utils
import os, osproc, strutils, macros, strformat, sugar

import du

proc execDu(workingDir: string = "", args: openArray[string] = []): string =
  execProcess("du", workingDir = workingDir, args = args, options = {
      poStdErrToStdOut, poUsePath})

proc duSize(duOut: string): BiggestInt =
  duOut
    .strip()
    .splitlines()[^1]
    .split()[0]
    .parseInt()

# TODO: Cache oracle results? 
template duOracle(tmpdir: string, body: untyped): untyped =
  check dirExists(tmpdir)
  try:
    body
    check du(tmpdir, (path: string, sum: BiggestInt) => (discard)).blocks == duSize(execDu(workingDir = dir))
  finally:
    removeDir(tmpdir)

suite "du oracle tests":
  ## "Oracle" tests against coreutils implementation. These tests run coreutils
  ## du as an external process, capture the output, parse results and compare.
  ## This is intended to avoid potential issues around running the tests on
  ## different filesystems (e.g. cannot hard-code sizes) - trading off increased
  ## runtime overhead for (hopefully) improved stability.
  ##
  ## Note: These are known to pass using GNU coreutils du 8.32. Other versions/implementations
  ## are likely to be fine, but not guaranteed.

  # Basic sanity checking of the environment before running these tests. This
  # can definitely be made more robust but it's better than nothing - *shrugs*
  echo "checking for GNU coreutils du on PATH"
  try:
    let outp = execDu(args = ["--version"])
    if not outp.contains("du (GNU coreutils)"):
      echo "warning: unable to detect whether 'du' on PATH is GNU coreutils"
  except OSError:
    echo "GNU coreutils is not on PATH"
    fail()

  test "empty dir":
    let dir = tempdir()
    duOracle(dir):
      discard

  test "1 empty file":
    let dir = tempdir()
    duOracle(dir):
      writeFile(dir / "foo.txt", "")

  test "2 empty files":
    let dir = tempdir()
    duOracle(dir):
      writeFile(dir / "a.txt", "")
      writeFile(dir / "b.txt", "")

  test "1 file":
    let dir = tempdir()
    duOracle(dir):
      writeFile(dir / "a.txt", "hello")

  test "2 files":
    let dir = tempdir()
    duOracle(dir):
      writeFile(dir / "a.txt", "hello")
      writeFile(dir / "b.txt", "world")

  test "subdirs":
    let dir = tempdir()
    duOracle(dir):
      writeFile(dir / "a.txt", "hello")
      writeFile(dir / "b.txt", "world")
      let sub1 = dir / "dir1"
      createDir(sub1)
      writeFile(sub1 / "c.txt", "this")
      writeFile(sub1 / "d.txt", "is")
      let sub2 = dir / "dir2"
      createDir(sub2)
      writeFile(sub2 / "e.txt", "a")
      writeFile(sub2 / "f.txt", "test")

