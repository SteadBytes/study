# Package

version       = "0.1.0"
author        = "Ben Steadman"
description   = "Re-implementing basic coreutils in Nim"
license       = "MIT"
srcDir        = "src"
binDir        = "bin"
bin           = @["wc"]


# Dependencies

requires "nim >= 1.4.8"
requires "osinfo >= 0.3.2"
