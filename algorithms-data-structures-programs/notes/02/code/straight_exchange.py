from typing import List

import pytest
from hypothesis import given
from hypothesis import strategies as st


def bubble_sort(lst: List):
    """
    In place sort of `lst` using basic bubblesort.

    Translated from program 2.4 in book
    """
    for i in range(len(lst)):
        for j in range(len(lst) - 1, i, -1):
            if lst[j - 1] > lst[j]:
                lst[j - 1], lst[j] = lst[j], lst[j - 1]


def bubble_sort_improved(lst: List):
    """
    In place sort of `lst` using improved bubblesort, terminating
    early if no exchanges made in the previous pass.
    """
    for i in range(len(lst)):
        swapped = False
        for j in range(len(lst) - 1, i, -1):
            if lst[j - 1] > lst[j]:
                swapped = True
                lst[j - 1], lst[j] = lst[j], lst[j - 1]
        if not swapped:
            return


@pytest.mark.parametrize("f", [bubble_sort, bubble_sort_improved])
@given(st.lists(st.integers(), min_size=1))
def test_sort(sort_fn, lst):
    """
    Compare to Python built in sorting
    """
    sort_fn(lst)
    assert lst == sorted(lst)
