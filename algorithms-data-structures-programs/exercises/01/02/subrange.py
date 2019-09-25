from typing import Union, Type


def _subrange__new__(base, low, high):
    def __new__(cls, value):
        if low <= value and value <= high:
            x = base.__new__(cls, value)
            return x
        else:
            raise ValueError(f"value {value} not in range {low}..{high}")

    return __new__


# TODO: add __str__, __repr__ e.t.c
def _mixin(base, low, high):
    if low > high:
        raise ValueError(f"low must be <= high")

    class SubrangeMixin:
        def __new__(cls, value):
            if low <= value and value <= high:
                x = base.__new__(cls, value)
                return x
            else:
                raise ValueError(f"value {value} not in range {low}..{high}")

    return SubrangeMixin


def IntSubrange(low, high):
    mixin = _mixin(int, low, high)

    class IntSubrange(mixin, int):
        def __add__(self, other):
            return self.__new__(IntSubrange, int(self) + other)

        def __sub__(self, other):
            return self.__new__(IntSubrange, int(self) - other)

        def __mul__(self, other):
            return self.__new__(IntSubrange, int(self) * other)

        def __truediv__(self, other):
            return self.__new__(IntSubrange, int(self) / other)

        def __floordiv__(self, other):
            return self.__new__(IntSubrange, int(self) / other)

    return IntSubrange


# TODO: change to appropriate magic methods for str
def StrSubrange(low, high):
    mixin = _mixin(str, low, high)

    class StrSubrange(mixin, int):
        def __add__(self, other):
            return self.__new__(StrSubrange, int(self) + other)

        def __sub__(self, other):
            return self.__new__(StrSubrange, int(self) - other)

        def __mul__(self, other):
            return self.__new__(StrSubrange, int(self) * other)

        def __truediv__(self, other):
            return self.__new__(StrSubrange, int(self) / other)

        def __floordiv__(self, other):
            return self.__new__(StrSubrange, int(self) / other)

    return StrSubrange


SUPPORTED_TYPES = {int: IntSubrange, str: StrSubrange}


def subrange(base: Union[Type[int], Type[str]], low, high):
    if base not in SUPPORTED_TYPES:
        raise ValueError(f"base must be one of {SUPPORTED_TYPES.keys()}")
    return SUPPORTED_TYPES[base](low, high)


Letter = subrange(str, "A", "Z")

Digit = subrange(int, 0, 9)
