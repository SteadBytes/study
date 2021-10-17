# Package

version       = "0.1.0"
author        = "Ben Steadman"
description   = "Re-implementing basic coreutils in Nim"
license       = "MIT"
srcDir        = "src"
installExt    = @["nim"]
bin           = @["wc"]


# Dependencies

requires "nim >= 1.4.8"
