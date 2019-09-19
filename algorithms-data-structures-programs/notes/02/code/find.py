from typing import List, Tuple

import pytest
from hypothesis import given
from hypothesis import strategies as st


def partition(a: List, l: int, r: int, k: int) -> Tuple[int, int]:
    """
    Partition `a` *in place* into two sublists `a[l:i], a[i:r]` using the
    `k`th element as a pivot `x` such that:
    ```
    all(a[k] <= x for k in range(l, i)) and
    all(a[k] >= x for k in range(j + 1, r)) and
    all(a[k] == x for k in range(j + 1, i - 1))
    ```

    Extracted from program 2.12 in book

    Returns:
        (tuple): i, j indices of left, right partitions
    """

    i = l
    j = r
    # partition a[l:r] into two sub arrays
    x = a[k]
    while True:
        # scan from left
        while a[i] < x:
            i += 1
        # scan from right
        while x < a[j]:
            j -= 1
        # a[i] is > a and a[j] < x and a[i] is currently before the pivot
        # -> exchange positions so values of a[i] and a[j] are on correct
        # sides of the pivot
        if i <= j:
            a[i], a[j] = a[j], a[i]
            i += 1
            j -= 1
        # entire array scanned
        if i > j:
            return i, j


def find(a: List, k: int):
    """
    Find the `k`th smallest element in `a` using quickselect/Hoare's find algorithm

    Will partially sort `a` *in place*

    Translated from program 2.12 in book
    """
    l = 0
    r = len(a) - 1
    while l < r:
        i, j = partition(a, l, r, k)
        if j < k:
            l = i
        if k < i:
            r = j
    return a[k]


def median(a: List):
    """
    Return the median item of `a`
    """
    return find(a, len(a) // 2)


@given(st.lists(st.integers(), min_size=1), st.data())
def test_find(l, data):
    """
    Compare to retrieving the `k`th item of fully sorted list
    """
    k = data.draw(st.integers(min_value=0, max_value=len(l) - 1))
    kth = find(l, k)

    assert kth == sorted(l)[k]


@given(st.lists(st.integers(), min_size=1))
def test_median(l):
    """
    Compare to retrieving the middle item of fully sorted list
    """
    m = median(l)

    assert m == sorted(l)[len(l) // 2]
