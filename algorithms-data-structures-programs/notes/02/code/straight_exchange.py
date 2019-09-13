from typing import List

import pytest
from hypothesis import given
from hypothesis import strategies as st


def bubble_sort(a: List):
    """
    In place sort of `a` using basic bubblesort.

    Translated from program 2.4 in book
    """
    for i in range(len(a)):
        for j in range(len(a) - 1, i, -1):
            if a[j - 1] > a[j]:
                a[j - 1], a[j] = a[j], a[j - 1]


def bubble_sort_improved(a: List):
    """
    In place sort of `a` using improved bubblesort, terminating
    early if no exchanges made in the previous pass.
    """
    for i in range(len(a)):
        swapped = False
        for j in range(len(a) - 1, i, -1):
            if a[j - 1] > a[j]:
                swapped = True
                a[j - 1], a[j] = a[j], a[j - 1]
        if not swapped:
            return


@pytest.mark.parametrize("f", [bubble_sort, bubble_sort_improved])
@given(st.lists(st.integers(), min_size=1))
def test_sort(f, l):
    """
    Compare to Python built in sorting
    """
    f(l)
    assert l == sorted(l)
