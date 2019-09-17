from typing import List

from hypothesis import given
from hypothesis import strategies as st


def sift(a: List, start: int, end: int):
    """
    Repair the min-heap property of a min heap between a[start:end] by sifting
    down the root element into the correct place.
    """
    root = start
    while True:
        child = 2 * root + 1
        if child > end:
            return
        if child + 1 <= end and a[child] < a[child + 1]:
            child += 1
        if a[root] < a[child]:
            a[root], a[child] = a[child], a[root]
            root = child
        else:
            return


def heap_sort(a: List):
    """
    In place sort of `a` using heap sort
    """
    # arrange a into a min heap
    for left in range((len(a) - 2) // 2, -1, -1):
        sift(a, left, len(a) - 1)

    # construct the sorted array
    # a[0:right] contains the min heap of unsorted elements
    # a[right:] contains the sorted elements
    for right in range(len(a) - 1, 0, -1):
        # swap root item of heap into sorted position
        a[right], a[0] = a[0], a[right]
        # maintain heap property of reduced size heap
        sift(a, 0, right - 1)


@given(st.lists(st.integers(), min_size=1))
def test_sort(l):
    """
    Compare to Python built in sorting
    """
    heap_sort(l)
    assert l == sorted(l)
