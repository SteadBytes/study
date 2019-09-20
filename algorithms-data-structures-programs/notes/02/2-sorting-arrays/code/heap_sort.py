from typing import List

from hypothesis import given
from hypothesis import strategies as st


def sift(lst: List, start: int, end: int):
    """
    Repair the min-heap property of a min heap between `lst[start:end]` by sifting
    down the root element into the correct place.
    """
    root = start
    while True:
        child = 2 * root + 1
        if child > end:
            return
        if child + 1 <= end and lst[child] < lst[child + 1]:
            child += 1
        if lst[root] < lst[child]:
            lst[root], lst[child] = lst[child], lst[root]
            root = child
        else:
            return


def heap_sort(lst: List):
    """
    In place sort of `lst` using heap sort
    """
    # arrange a into a min heap
    for left in range((len(lst) - 2) // 2, -1, -1):
        sift(lst, left, len(lst) - 1)

    # construct the sorted array
    # lst[0:right] contains the min heap of unsorted elements
    # lst[right:] contains the sorted elements
    for right in range(len(lst) - 1, 0, -1):
        # swap root item of heap into sorted position
        lst[right], lst[0] = lst[0], lst[right]
        # maintain heap property of reduced size heap
        sift(lst, 0, right - 1)


@given(st.lists(st.integers(), min_size=1))
def test_sort(lst):
    """
    Compare to Python built in sorting
    """
    heap_sort(lst)
    assert lst == sorted(lst)
