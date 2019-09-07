import string
from enum import Enum
from typing import NamedTuple, Set

from subrange import subrange


class Sex(Enum):
    MALE = 1
    FEMALE = 2


class Boolean(Enum):
    FALSE = 0
    TRUE = 1


# or just use actual bool types
a_bool = True
another_bool = False


class Weekday(Enum):
    MONDAY = 0
    TUESDAY = 1
    WEDNESDAY = 2
    THURSDAY = 3
    FRIDAY = 4
    SATURDAY = 5
    SUNDAY = 6


Letter = subrange(str, "A", "Z")

Digit = subrange(int, 0, 9)


class Officer(Enum):
    LIEUTENANT = 0
    CAPTAIN = 1
    MAJOR = 2
    LIEUTENANT_COLONEL = 3
    COLONEL = 4
    BRIGADIER = 5
    MAJOR_GENERAL = 6
    LIEUTENANT_GENERAL = 7
    GENERAL = 8


class Complex(NamedTuple):
    re: float
    im: float


Day = subrange(int, 1, 31)
Month = subrange(int, 1, 12)
Year = subrange(int, 1, 2000)


class Date:
    def __init__(self, day: Day, month: Month, year: Year):
        self.day = Day(day)
        self.month = Month(month)
        self.year = Year(year)


class MarriageStatus(Enum):
    SINGLE = 0
    MARRIED = 1
    WIDOWED = 2
    DIVORCED = 3


class Person(NamedTuple):
    name: str
    firstname: str
    birthdate: Date
    sex: Sex
    marstatus: MarriageStatus


# Python has no built in char type -> use length 1 strings
charset: Set[str] = set()


class TapeException(Enum):  # renamed to not collide with python Exception class
    unloaded = 0
    manual = 1
    parity = 2
    skew = 3


tapestatus: Set[TapeException] = set()