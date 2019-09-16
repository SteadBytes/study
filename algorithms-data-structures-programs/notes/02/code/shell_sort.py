from functools import wraps
from itertools import count
from math import floor
from typing import List, Callable, Iterable

import pytest
from hypothesis import given
from hypothesis import strategies as st

# Selection of gap sequences to compare
# https://en.wikipedia.org/wiki/Shellsort#Gap_sequences


def gap_seq(prefix=None, reverse=True):
    """
    Decorator to produce a generator of a gap sequence from it's general term
    function.

    General term functions must be of the form `f(n, k) -> int`, where `n` is the
    length of the array and `k` is the index of the current term in the sequence.

    Args:
        prefix (int): Optionally yield a specified prefix value as the first sequence element
            *before* calculating values using the general term function.
        reverse (bool): Yield values from `n` to `1` instead of `1` to `n`, this is
            necessary if a general term function produces *increasing* values as the
            gap sequence must strictly decrease to `1`.
    """

    def decorator(fn: Callable[[int, int], int]):
        @wraps(fn)
        def wrapper(n: int, *args, **kwargs):
            if prefix:
                yield prefix
            for k in range(n, 0, -1) if reverse else range(1, n):
                yield fn(n, k, *args, **kwargs)

        return wrapper

    return decorator


@gap_seq()
def shell_gap(n, k):
    """
    Gap sequence proposed by Shell.
    General term: floor(n/2^k)
    """
    return floor(n / pow(2, k))


@gap_seq()
def frank_lazarus_gap(n, k):
    """
    Gap sequence proposed by Frank & Lazarus.
    General term: 2*floor(n/(2*(k+1)))+1
    """
    return 2 * floor(n / pow(2, k + 1)) + 1


@gap_seq(reverse=True)
def A168604(n, k):
    """
    General term: 2^k - 1
    """
    return pow(2, k) - 1


@gap_seq(reverse=True, prefix=1)
def A083318(n, k):
    """
    General term: 2^k + 1, prefixed with 1
    """
    return pow(2, k) + 1


@gap_seq(reverse=True, prefix=1)
def A0356562(n, k):
    """
    General term: 4^k + 3 * 2^(k-1) + 1, prefixed with 1
    """
    return pow(4, k) + 3 * pow(2, k - 1) + 1


def shell_sort(a: List, gap_fn: Callable[[int], Iterable[int]] = shell_gap):
    """
    In place sort of `a` using shell sort.
    
    Args:
        a (list): List to sort in place
        gap_fn (callable): Function which generates a valid sequence of 'gaps' to
            use for each h-sort; given the length of `a`. Valid sequence *must*
            decrease from an initial value to 1.
    """
    for gap in gap_fn(len(a)):
        for i in range(gap, len(a)):
            x = a[i]
            j = i
            while j >= gap and a[j - gap] > x:
                a[j] = a[j - gap]
                j -= gap
            a[j] = x


@pytest.mark.parametrize(
    "gap_fn", [shell_gap, frank_lazarus_gap, A168604, A083318, A0356562]
)
@given(st.lists(st.integers(), min_size=1))
def test_shell_sort(gap_fn, l):
    """
    Compare to Python built in sorting
    """
    shell_sort(l, gap_fn)
    assert l == sorted(l)
