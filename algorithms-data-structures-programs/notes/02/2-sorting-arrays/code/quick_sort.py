from typing import List, NamedTuple, Tuple

import pytest
from hypothesis import given
from hypothesis import strategies as st


def partition(lst: List, l: int, r: int) -> Tuple[int, int]:
    """
    Partition `lst` *in place* into two sub arrays `a[l:i], a[i:r]` using the
    *middle* element as a pivot `x` such that:
    ```
    all(lst[k] <= x for k in range(l, i)) and
    all(lst[k] >= x for k in range(j + 1, r)) and
    all(lst[k] == x for k in range(j + 1, i - 1))
    ```

    Translated from program 2.9 in book

    Returns:
        (tuple): i, j indices of left, right partitions
    """

    i = l
    j = r
    # partition lst[l:r] into two sub arrays
    x = lst[(l + r) // 2]  # middle element as pivot
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


def quick_sort(lst: List):
    """
    In place sort of `lst` using recursive quicksort

    Translated from program 2.10 in book
    """

    def sort(l: int, r: int):
        i, j = partition(lst, l, r)
        # sort left subarray
        if l < j:
            sort(l, j)
        # sort right subarray
        if i < r:
            sort(i, r)

    sort(0, len(lst) - 1)


class StackItem(NamedTuple):
    l: int
    r: int


def quick_sort_1(lst: List):
    """
    In place sort `lst` using iterative quicksort

    Translated from program 2.11 in book
    """
    # initialise stack with first partition request of entire array
    stack: List[StackItem] = [StackItem(0, len(lst) - 1)]
    while True:
        # take top request from stack
        l, r = stack.pop()
        # partition: split a[l] ... a[r] into two sub arrays
        while True:
            i, j = partition(lst, l, r)
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
def test_sort(sort_fn, lst):
    """
    Compare to Python built in sorting
    """
    sort_fn(lst)
    assert lst == sorted(lst)
