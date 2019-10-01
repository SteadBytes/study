"""
Find maximal runs of an iterable.

For example, natural merge-sort (chapter 2.3.2) uses maximal runs to improve
sorting efficiency.
"""
from itertools import chain, tee, zip_longest
from typing import Iterable, Iterator

import pytest
from hypothesis import given, settings
from hypothesis import strategies as st


def pairwise_longest(iterable: Iterable, fillvalue=None) -> Iterator[tuple]:
    """
    "s -> (s0,s1), (s1,s2), (s2, s3), ..."

    `fillvalue` substituted for the final value of `iterable`; the *final* pair
    of iterable with length `n` will be `(sn-1, fillvalue)`.

    Adapted from https://docs.python.org/3.7/library/itertools.html#itertools-recipes
    to include *all* elements by using `zip_longest`.
    """
    a, b = tee(iterable)
    next(b, None)
    return zip_longest(a, b, fillvalue=fillvalue)


def run_grouper(iterable: Iterable) -> Iterator[list]:
    """
    Yields consecutive *maximal* runs from `iterable`.

    Maximal run invariant:
    - `a[k] <= a[k+1] for k=i...j-1`
    - `a[i-1] > a[i]`
    - `a[j] > a[j+1]`

    >>> my_iter = iter([3, 4, 2, 1, 7, 5, 8, 9, 0, 6])
    >>> runs = run_grouper(my_iter)
    >>> list(runs)
    [[3, 4], [2], [1, 7], [5, 8, 9], [0, 6]]
    """
    run = []
    for a, b in pairwise_longest(iterable):
        run.append(a)
        if b is None:
            # end of iterable
            yield run
            return
        if a > b:
            # end of run
            yield run
            run = []


@pytest.mark.parametrize(
    "elements", [st.integers, st.floats, st.dates, st.datetimes, st.characters, st.text]
)
@settings(deadline=600)
@given(st.data())
def test_gen_runs(elements, data):
    """
    Test correctness of run generation according to properties:
    1. All elements in input present once across all runs
    2. Each run maintains the maximal run invariant (see run_grouper docstring)

    NOTE: Hypothesis deadline setting overriden as single character values i.e. ['0']
    cause Hypothesis to raise unreliable test timings exceptions. Can't repro slow
    times outside of tests - bug in Hypothesis? increase deadline to allow tests to run.
    """
    lst = data.draw(st.lists(elements()))
    # gen_runs expects and iterable
    iterable = iter(lst)
    # cast to list for multiple assertions
    runs = list(run_grouper(iterable))

    # all elements in input present in output
    assert list(chain.from_iterable(runs)) == lst

    # run invariant:
    for r1, r2 in zip(runs, runs[1:]):
        assert r1[-1] > r2[0]
