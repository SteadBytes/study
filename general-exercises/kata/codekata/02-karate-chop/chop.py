from typing import List

import pytest

from hypothesis import given
from hypothesis import strategies as st


def chop_iterative(x: int, lst: List[int]):
    """
    Classic iterative binary search implementation - no imagination here!
    No problems encountered due to familiarity with the algorithm, however from
    past experience off by one error are typical with this version as well as
    incorrect comparison operators i.e. `high = len(lst)` vs `high - len(lst) -1`
    or <` instead of `<=` 
    """
    low, high = 0, len(lst) - 1
    while low <= high:
        mid = (low + high) // 2
        v = lst[mid]
        if v == x:
            return mid
        elif v < x:
            low = mid + 1
        else:
            high = mid - 1
    return -1


def chop_recursive(x: int, lst: List[int]):
    """
    Classic recursive binary search implementation. Again, no imagination nor
    implementation problems here due to familiarity with the algorithm. The
    recursive version is a very natural expression of the binary chop due to
    it's recursive definition. An issue commonly encountered with this
    implementation (as with all recursive algorithms) is getting the termination
    case `low > high` incorrect and causing an infinite recursion/stack overflow.
    """

    def recur(low: int, high: int):
        if low > high:
            return -1
        mid = (low + high) // 2
        v = lst[mid]
        if v == x:
            return mid
        elif v < x:
            return recur(mid + 1, high)
        else:
            return recur(low, mid - 1)

    return recur(0, len(lst) - 1)


def chop_slices(x: int, lst: List[int]):
    """
    Recursive implementation using decreasing list slices instead of indexing
    directly into the source list. Complicates returning the final return index
    as the middle index of a slice will not be the index of that position in the
    entire original list. To calculate the correct index within the original
    list, the recursion takes an `offset` parameter which tracks the starting
    position of the slice within the original list.
    """

    def recur(_l, offset):
        if not _l:
            return -1
        mid = len(_l) // 2
        v = _l[mid]
        if v == x:
            return mid + offset
        elif v < x:
            return recur(_l[mid + 1 :], offset + mid + 1)
        else:
            return recur(_l[:mid], offset)

    return recur(lst, 0)


CHOP_FUNCS = [chop_iterative, chop_recursive, chop_slices]


@pytest.mark.parametrize("chop", CHOP_FUNCS)
@pytest.mark.parametrize(
    "expected,x,lst",
    [
        (-1, 3, []),
        (-1, 3, [1]),
        (0, 1, [1]),
        #
        (0, 1, [1, 3, 5]),
        (1, 3, [1, 3, 5]),
        (2, 5, [1, 3, 5]),
        (-1, 0, [1, 3, 5]),
        (-1, 2, [1, 3, 5]),
        (-1, 4, [1, 3, 5]),
        (-1, 6, [1, 3, 5]),
        #
        (0, 1, [1, 3, 5, 7]),
        (1, 3, [1, 3, 5, 7]),
        (2, 5, [1, 3, 5, 7]),
        (3, 7, [1, 3, 5, 7]),
        (-1, 0, [1, 3, 5, 7]),
        (-1, 2, [1, 3, 5, 7]),
        (-1, 4, [1, 3, 5, 7]),
        (-1, 6, [1, 3, 5, 7]),
        (-1, 8, [1, 3, 5, 7]),
    ],
)
def test_chop(chop, expected, x, lst):
    """
    Test cases provided by the Kata.
    """
    assert chop(x, lst) == expected


@st.composite
def sorted_list_strategy(draw):
    lst = draw(st.lists(st.integers(), min_size=1, max_size=100000, unique=True))
    lst.sort()
    return lst


@pytest.mark.parametrize("chop", CHOP_FUNCS)
@given(st.data(), sorted_list_strategy())
def test_chop_in_list(chop, data, lst):
    """
    Property based test for finding index of items known to exist in the list.
    """
    i = data.draw(st.integers(min_value=0, max_value=len(lst) - 1))
    assert chop(lst[i], lst) == i


@pytest.mark.parametrize("chop", CHOP_FUNCS)
@given(st.data(), sorted_list_strategy())
def test_chop_not_in_list(chop, data, lst):
    """
    Property based test for items known *not* to exist in the list.
    """
    x = data.draw(st.integers().filter(lambda x: x not in lst))
    assert chop(x, lst) == -1
