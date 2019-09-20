from typing import List

import pytest
from hypothesis import given
from hypothesis import strategies as st


def straight_insertion(lst: List):
    """
    In place sort of `lst` using straight insertion with a sentinel value.

    list.insert and del are O(n - 1) operations, thus using the sentinel value
    to optimise the inner loop by removing the need to check j > 0 may not be
    effective on lists of large n.
    """
    # insert sentinel value
    lst.insert(0, float("-inf"))
    for i in range(2, len(lst)):
        x = lst[i]
        j = i - 1
        while x < lst[j]:
            lst[j + 1] = lst[j]
            j = j - 1
        lst[j + 1] = x
    # remove sentinel value
    del lst[0]


def straight_insertion_no_sentinel(lst: List):
    """
    In place sort of `lst` using straight insertion without a sentinel value.
    """
    for i in range(1, len(lst)):
        x = lst[i]
        j = i
        # inner loop less optimised (j > 0 check required)
        while j > 0 and lst[j - 1] > x:
            lst[j] = lst[j - 1]
            j = j - 1
        lst[j] = x


@pytest.mark.parametrize("f", [straight_insertion, straight_insertion_no_sentinel])
@given(st.lists(st.integers(), min_size=1))
def test_sort(sort_fn, lst):
    """
    Compare to Python built in sorting
    """
    sort_fn(lst)
    assert lst == sorted(lst)
