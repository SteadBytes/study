"""
n! perumations of n elements in situ.

Generating permutations of a[:m]:
    - m subtasks of generating permutations of a[:m - 1]
    - In the ith subtask, a[i] and a[m] are initially interchanged
"""

from itertools import permutations

from hypothesis import given
from hypothesis import strategies as st


def perms(l: list, i=0, q=print):
    if i == len(l) - 1:
        q(l)
    else:
        for j in range(i, len(l)):
            l[i], l[j] = l[j], l[i]
            perms(l, i + 1, q=q)
            l[j], l[i] = l[i], l[j]


@given(st.lists(st.integers(), min_size=2, max_size=8))
def test_perms(lst):
    results = set()
    perms(lst[:], q=lambda l: results.add(tuple(l)))
    # compare to built in itertools.permuations
    assert set(results) == set(permutations(lst))
