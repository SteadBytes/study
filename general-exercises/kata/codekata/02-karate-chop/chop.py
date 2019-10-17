from typing import List

import pytest


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


@pytest.mark.parametrize("chop", [chop_iterative, chop_recursive])
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
    assert chop(x, lst) == expected
