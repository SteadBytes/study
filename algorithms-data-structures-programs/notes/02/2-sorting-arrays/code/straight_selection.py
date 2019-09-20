from itertools import islice
from typing import List

import pytest
from hypothesis import given
from hypothesis import strategies as st


def straight_selection(lst: List):
    """
    In place sort of `lst` using straight selection.

    Translated from program 2.3 in book
    """
    for i in range(len(lst) - 1):
        k = i
        x = lst[i]
        # find smallest item in source sequence
        for j in range(i + 1, len(lst)):
            if lst[j] < x:
                k = j
                x = lst[j]
        # exchange into correct position in full array
        lst[k] = lst[i]
        lst[i] = x


def straight_selection_pythonic(lst: List):
    """
    In place sort of `lst` using straight selection.

    Implemented in a more 'pythonic' fashion using generators and builtins
    """
    for i in range(len(lst)):
        # (index, value)
        source_items = enumerate(islice(lst, i, len(lst)))
        # index of smallest value in source sequence
        j_min, _ = min(source_items, key=lambda x: x[1])
        # position in full array
        j_min += i
        # exchange into correct position in full array
        lst[i], lst[j_min] = lst[j_min], lst[i]


@pytest.mark.parametrize("f", [straight_selection, straight_selection_pythonic])
@given(st.lists(st.integers(), min_size=1))
def test_sort(sort_fn, lst):
    """
    Compare to Python built in sorting
    """
    sort_fn(lst)
    assert lst == sorted(lst)
