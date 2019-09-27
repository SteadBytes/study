from typing import List, Optional

from hypothesis import given
from hypothesis import strategies as st


def merge_sort(lst: List, left: int = 0, right: Optional[int] = None):
    """
    In place sort of `lst` using a recursive merge sort.

    This is **not** the bufferless implementation presented in program 2.13 in
    the book - `merge` uses a temporary buffer. Bufferless implementation is
    complex and non-trivial to implement in the general case. Furthermore, it is
    unstable and requires additional swap operations (but mainly I couldn't bring
    myself to translate the mass of index variables and corresponding index
    arithemetic from program 2.13).
    """
    # initial call
    if right is None:
        right = len(lst)
    # left and right runs have converged -> complete
    if right - left <= 1:
        return
    # split into two runs and sort individually
    mid = left + (right - left) // 2
    merge_sort(lst, left, mid)
    merge_sort(lst, mid, right)
    # merge runs
    merge(lst, left, mid, right)


def merge(lst: List, left: int, mid: int, right: int):
    """
    In place merge of sorted runs (sublists) `lst[left:mid]` and `lst[mid:right]`
    to form a sorted sublist at `lst[left:right]`.

    >>> lst = [27, 38, 3, 43, 9, 82, 10, 15]
    >>> merge(lst, 0, 2, 3)
    >>> lst == [3, 27, 38, 43, 9, 82, 10, 15]
    True
    >>> merge(lst, 4, 6, 8)
    >>> lst == [3, 27, 38, 43, 9, 10, 15, 82]
    True
    >>> merge(lst, 0, 4, 8)
    lst == [3, 9, 10, 15, 27, 38, 43, 82]
    """
    # copy left run into temporary buffer
    buf = lst[left:mid]
    # indexes for reading from lst and buf
    read_l = 0
    read_r = mid
    # index at which elements will placed into lst; either from lst or buf
    write = left
    while read_l < len(buf) and read_r < right:
        if lst[read_r] < buf[read_l]:
            # element in right run less than element in left run
            # -> move element from right run into place
            lst[write] = lst[read_r]
            read_r += 1
        else:
            # element in left run less than element in right run
            # -> move from left run into place
            lst[write] = buf[read_l]
            read_l += 1
        write += 1

    # copy any remaining elements from left run into place
    while read_l < len(buf):
        lst[write] = buf[read_l]
        read_l += 1
        write += 1
    # more pythonic but will allocate extra list
    # for i, v in enumerate(buf[l_read : len(buf)]):
    #     lst[i + write] = v


@given(st.lists(st.integers(), min_size=1))
def test_merge_sort(lst):
    """
    Compare to builtin `sorted`
    """
    merge_sort(lst)
    assert lst == sorted(lst)
