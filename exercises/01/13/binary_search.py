import pytest
from hypothesis import given
from hypothesis import strategies as st, settings


def binary_search(a, x):
    """
    Implementation given in 1.17
    """
    i = 0
    j = len(a) - 1
    while True:
        k = (i + j) // 2
        if x > a[k]:
            i = k + 1
        else:
            j = k - 1
        if a[k] == x or i > j:
            return k


def A(a, x):
    """
    Not correct: infinite loop if x is first or last element
    """
    i = 0
    j = len(a) - 1
    while True:
        k = (i + j) // 2
        if a[k] < x:
            i = k
        else:
            j = k
        if a[k] == x or i >= j:
            return k


def B(a, x):
    i = 0
    j = len(a) - 1
    while True:
        k = (i + j) // 2
        if x <= a[k]:
            j = k - 1
        if a[k] <= x:
            i = k + 1
        if i > j:
            return k


def C(a, x):
    """
    Incorrect, for example C([-1, 1], 1) != 1
    """
    i = 0
    j = len(a) - 1
    while True:
        k = (i + j) // 2
        print("Entering", i, j, k)
        if x < a[k]:
            j = k
        else:
            i = k + 1
        print("Before conditional", i, j, k)
        if i >= j:
            return k


@pytest.mark.timeout(5)  # break out of infinite loop
@pytest.mark.xfail
def test_A_incorrect_with_infinite_loop():
    a = [0, 1, 2, 3, 4, 5]
    a.sort()
    assert a[A(a, a[-1])] == a[-1]


@pytest.mark.xfail
def test_C_incorrect_case():
    a = [-1, 1]
    a.sort()
    assert a[C(a, 1)] == 1


@pytest.mark.parametrize("f", [binary_search, B])
@given(st.lists(st.integers(), min_size=1), st.data())
def test_correct_alogrithms(f, a, data):
    a.sort()
    in_a = data.draw(st.sampled_from(a))
    not_in_a = data.draw(st.integers().filter(lambda x: x not in a))
    assert a[f(a, in_a)] == in_a
    assert a[f(a, not_in_a)] != not_in_a
