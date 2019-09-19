from typing import List, Tuple

import pytest
from hypothesis import given
from hypothesis import strategies as st


def partition(lst: List, l: int, r: int, k: int) -> Tuple[int, int]:
    """
    Partition `lst` *in place* into two sublists `lst[l:i], lst[i:r]` using the
    `k`th element as a pivot `x` such that:
    ```
    all(lst[k] <= x for k in range(l, i)) and
    all(lst[k] >= x for k in range(j + 1, r)) and
    all(lst[k] == x for k in range(j + 1, i - 1))
    ```

    Extracted from program 2.12 in book

    Returns:
        (tuple): i, j indices of left, right partitions
    """

    i = l
    j = r
    # partition lst[l:r] into two sub arrays
    x = lst[k]
    while True:
        # scan from left
        while lst[i] < x:
            i += 1
        # scan from right
        while x < lst[j]:
            j -= 1
        # lst[i] is > x and lst[j] < x and lst[i] is currently before the pivot
        # -> exchange positions so values of lst[i] and lst[j] are on correct
        # sides of the pivot
        if i <= j:
            lst[i], lst[j] = lst[j], lst[i]
            i += 1
            j -= 1
        # entire array scanned
        if i > j:
            return i, j


def find(lst: List, k: int):
    """
    Find the `k`th smallest element in `lst` using quickselect/Hoare's find algorithm

    Will partially sort `lst` *in place*

    Translated from program 2.12 in book
    """
    l = 0
    r = len(lst) - 1
    while l < r:
        i, j = partition(lst, l, r, k)
        if j < k:
            l = i
        if k < i:
            r = j
    return lst[k]


def median(lst: List):
    """
    Return the median item of `lst`
    """
    return find(lst, len(lst) // 2)


@given(st.lists(st.integers(), min_size=1), st.data())
def test_find(lst, data):
    """
    Compare to retrieving the `k`th item of fully sorted list
    """
    k = data.draw(st.integers(min_value=0, max_value=len(lst) - 1))
    kth = find(lst, k)

    assert kth == sorted(lst)[k]


@given(st.lists(st.integers(), min_size=1))
def test_median(lst):
    """
    Compare to retrieving the middle item of fully sorted list
    """
    m = median(lst)

    assert m == sorted(lst)[len(lst) // 2]
