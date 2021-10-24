import unittest
import os, strutils
import utils

template withCleanup(dir: string, body: untyped): untyped =
  check dirExists(dir)
  try:
    body
  finally:
    removeDir(dir)

suite "tempdir":
  test "defaults":
    let dir = tempdir()
    withCleanup(dir):
      check cmpPaths(parentDir(dir), os.getTempDir()) == 0

  test "basepath":
    let base = os.getTempDir() / "foobar"
    let dir = tempdir(basePath = base)
    withCleanup(dir):
      check parentDir(dir) == base

  test "prefix":
    let dir = tempdir(prefix = "foobar")
    withCleanup(dir):
      check cmpPaths(parentDir(dir), os.getTempDir()) == 0
      # Name uses prefix
      let (_, tail) = splitPath(dir)
      check tail.startsWith("foobar")

