from typing import List, NamedTuple, Tuple

import pytest
from hypothesis import given
from hypothesis import strategies as st


def partition(a: List, l: int, r: int) -> Tuple[int, int]:
    """
    Partition `a` *in place* into two sub arrays `a[l:i], a[i:r]` using the
    *middle* element as a pivot `x` such that:
    ```
    all(a[k] <= x for k in range(l, i)) and
    all(a[k] >= x for k in range(j + 1, r)) and
    all(a[k] == x for k in range(j + 1, i - 1))
    ```

    Translated from program 2.9 in book

    Returns:
        (tuple): i, j indices of left, right partitions
    """

    i = l
    j = r
    # partition a[l:r] into two sub arrays
    x = a[(l + r) // 2]  # middle element as pivot
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


def quick_sort(a: List):
    """
    In place sort of `a` using recursive quicksort

    Translated from program 2.10 in book
    """

    def sort(l: int, r: int):
        i, j = partition(a, l, r)
        # sort left subarray
        if l < j:
            sort(l, j)
        # sort right subarray
        if i < r:
            sort(i, r)

    sort(0, len(a) - 1)


class StackItem(NamedTuple):
    l: int
    r: int


def quick_sort_1(a: List):
    """
    In place sort `a` using iterative quicksort

    Translated from program 2.11 in book
    """
    # initialise stack with first partition request of entire array
    stack: List[StackItem] = [StackItem(0, len(a) - 1)]
    while True:
        # take top request from stack
        l, r = stack.pop()
        # partition: split a[l] ... a[r] into two sub arrays
        while True:
            i, j = partition(a, l, r)
            if i < r:
                # stack request to sort right partition
                stack.append(StackItem(i, r))
            # sort left partition on next iteration
            r = j
            if l >= r:
                break
        if not stack:
            break


@pytest.mark.parametrize("f", [quick_sort, quick_sort_1])
@given(st.lists(st.integers(), min_size=1))
def test_sort(f, l):
    """
    Compare to Python built in sorting
    """
    f(l)
    assert l == sorted(l)
