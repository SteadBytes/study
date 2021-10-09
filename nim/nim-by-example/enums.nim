# `enum` types are analogous to C enums + type checking.

type
  # By default, references to enum values do not need to be qualified - `cd`
  # prefix is used to avoid name conflicts
  CompassDirections = enum
    cdNorth, cdEast, cdSouth, cdWest
  # `{.pure.}` pragma *requires* that all ambiguous references to enum values
  # are qualified.
  Colours {.pure.} = enum
    Red = "FF0000", Green = (1, "00FF00"), Blue = "OOOOFF"

  OtherColours {.pure.} = enum
    Red = 0xFF0000, Orange = 0xFFA500, Yellow = 0xFFFF00

  # Disjoint values -> not ordinal (discouraged, only possible for C compatibility)
  Signals = enum
    sigQuit = 3, sigAbort = 6, sigKill = 9

# Won't compile - ambiguous identifier (`{.pure.}`)
# echo Red
echo Colours.Red
echo OtherColours.Red
# Need not be qualified as there is only one `Orange` enum value in scope
echo OtherColours.Orange
echo Orange

# Enums are *ordinal* - pre-defined methods:
# - `low` -> lowest possible value
# -`high` -> highest possible value
# - `inc` -> increment
# - `dec` -> decrement
# - `ord` -> integer value of an enum
for direction in ord(low(CompassDirections))..ord(high(CompassDirections)):
  # Cast from integer -> enum value
  echo CompassDirections(direction), " ord: ", direction

# Equivalent to the above
for direction in cdNorth..cdWest:
  echo direction, " ord: ", ord(direction)



var ordinal = low(int)
echo ordinal
inc ordinal
echo ordinal
dec ordinal
echo ordinal
echo high(char)

# Won't compile - `Signals` is not ordinal -> ordinal methods not defined
# var nonOrdinal = sigQuit
# inc nonOrdinal
# dec nonOrdinal
