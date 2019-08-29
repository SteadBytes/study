import string
from enum import Enum
from typing import NamedTuple, Set

import pytest
from hypothesis import given, settings
from hypothesis import strategies as st


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


def subrange(base, low, high):
    if low > high:
        raise ValueError(f"low must be <= high")

    class SubRange(base):
        def __new__(cls, value):
            if low <= value and value <= high:
                x = base.__new__(cls, value)
                return x
            else:
                raise ValueError(f"value {value} not in range {low}..{high}")

    return SubRange


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


class TestSubrange:
    @given(st.data())
    def test_int_in_range(self, data):
        low = data.draw(st.integers())
        high = data.draw(st.integers(min_value=low))
        val = data.draw(st.integers(min_value=low, max_value=high))
        IntSubrange = subrange(int, low, high)
        i = IntSubrange(val)
        assert i == val

    @given(st.data())
    def test_int_out_of_range(self, data):
        low = data.draw(st.integers())
        high = data.draw(st.integers(min_value=low))
        val = data.draw(st.integers().filter(lambda x: x < low or x > high))
        IntSubrange = subrange(int, low, high)
        with pytest.raises(ValueError):
            IntSubrange(val)

    @given(st.data())
    # increase as drawing char values can be slow
    # TODO: is this a hypothesis bug?
    @settings(deadline=500)
    def test_char_in_range(self, data):
        low = data.draw(st.characters())
        high = data.draw(st.characters(min_codepoint=ord(low)))
        val = data.draw(st.characters(min_codepoint=ord(low), max_codepoint=ord(high)))
        CharSubrange = subrange(str, low, high)
        c = CharSubrange(val)
        assert c == val

    @given(st.data())
    def test_char_out_of_range(self, data):
        low = data.draw(st.characters())
        high = data.draw(st.characters(min_codepoint=ord(low)))
        val = data.draw(st.characters().filter(lambda c: c < low or c > high))
        CharSubrange = subrange(str, low, high)
        with pytest.raises(ValueError):
            CharSubrange(val)


class TestLetter:
    @pytest.mark.parametrize("ch", string.ascii_uppercase)
    def test_letters_in_range(self, ch):
        l = Letter(ch)
        assert l == ch

    @given(st.characters(blacklist_characters=string.ascii_uppercase))
    def test_letters_out_of_range(self, ch):
        with pytest.raises(ValueError):
            Letter(ch)


class TestDigit:
    @pytest.mark.parametrize("x", range(10))
    def test_digits_in_range(self, x):
        d = Digit(x)
        assert d == x

    @given(st.integers().filter(lambda x: x < 0 or x > 9))
    def test_digits_out_of_range(self, x):
        with pytest.raises(ValueError):
            Digit(x)
