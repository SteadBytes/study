from itertools import islice
from typing import List

import pytest
from hypothesis import given
from hypothesis import strategies as st


def straight_selection(a: List):
    """
    In place sort of `a` using straight selection.

    Translated from program 2.3 in book
    """
    for i in range(len(a) - 1):
        k = i
        x = a[i]
        # find smallest item in source sequence
        for j in range(i + 1, len(a)):
            if a[j] < x:
                k = j
                x = a[j]
        # exchange into correct position in full array
        a[k] = a[i]
        a[i] = x


def straight_selection_pythonic(a: List):
    """
    In place sort of `a` using straight selection.

    Implemented in a more 'pythonic' fashion using generators and builtins
    """
    for i in range(len(a)):
        # (index, value)
        source_items = enumerate(islice(a, i, len(a)))
        # index of smallest value in source sequence
        j_min, _ = min(source_items, key=lambda x: x[1])
        # position in full array
        j_min += i
        # exchange into correct position in full array
        a[i], a[j_min] = a[j_min], a[i]


@pytest.mark.parametrize("f", [straight_selection, straight_selection_pythonic])
@given(st.lists(st.integers(), min_size=1))
def test_sort(f, l):
    """
    Compare to Python built in sorting
    """
    f(l)
    assert l == sorted(l)
