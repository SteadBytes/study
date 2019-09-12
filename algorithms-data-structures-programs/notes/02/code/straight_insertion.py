from typing import List

import pytest
from hypothesis import given
from hypothesis import strategies as st


def straight_insertion(a: List):
    """
    In place sort of `a` using straight insertion with a sentinel value.

    list.insert and del are O(n - 1) operations, thus using the sentinel value
    to optimise the inner loop by removing the need to check j > 0 may not be
    effective on lists of large n.
    """
    # insert sentinel value
    a.insert(0, float("-inf"))
    for i in range(2, len(a)):
        x = a[i]
        j = i - 1
        while x < a[j]:
            a[j + 1] = a[j]
            j = j - 1
        a[j + 1] = x
    # remove sentinel value
    del a[0]


def straight_insertion_no_sentinel(a: List):
    """
    In place sort of `a` using straight insertion without a sentinel value.
    """
    for i in range(1, len(a)):
        x = a[i]
        j = i
        # inner loop less optimised (j > 0 check required)
        while j > 0 and a[j - 1] > x:
            a[j] = a[j - 1]
            j = j - 1
        a[j] = x


@pytest.mark.parametrize("f", [straight_insertion, straight_insertion_no_sentinel])
@given(st.lists(st.integers(), min_size=1))
def test_sort(f, l):
    """
    Compare to Python built in sorting
    """
    f(l)
    assert l == sorted(l)
