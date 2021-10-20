# Package

version       = "0.1.0"
author        = "Ben Steadman"
description   = "Re-implementing basic coreutils in Nim"
license       = "MIT"
srcDir        = "src"
binDir        = "bin"
installExt    = @["nim"]
bin           = @["wc", "du"]


# Dependencies

requires "nim >= 1.4.8"
