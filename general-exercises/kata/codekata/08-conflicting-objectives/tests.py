from itertools import chain
from math import factorial

import pytest
from hypothesis import given
from hypothesis import strategies as st

import extendible
import fast
import readable


@pytest.fixture
def words():
    return [
        "al",
        "bums",
        "albums",
        "bar",
        "ely",
        "barely",
        "be",
        "foul",
        "befoul",
        "con",
        "vex",
        "convex",
        "here",
        "by",
        "hereby",
        "jig",
        "saw",
        "jigsaw",
        "tail",
        "or",
        "tailor",
        "we",
        "aver",
        "weaver",
    ]


@pytest.mark.parametrize("compounds", [readable.compounds, fast.compounds])
def test_compounds(compounds, words):
    """
    See TestExtentible.test_compounds for the extendible version.
    """
    assert list(compounds(words)) == [
        ("al", "bums"),
        ("bar", "ely"),
        ("be", "foul"),
        ("con", "vex"),
        ("here", "by"),
        ("jig", "saw"),
        ("tail", "or"),
        ("we", "aver"),
    ]


def ncr(n: int, r: int) -> int:
    f = factorial
    return f(n) // f(r) // f(n - r)


class TestExtendible:
    @pytest.mark.parametrize(
        "n,expected",
        [
            (1, [("albums",)]),
            (
                2,
                [
                    ("a", "lbums"),
                    ("al", "bums"),
                    ("alb", "ums"),
                    ("albu", "ms"),
                    ("album", "s"),
                ],
            ),
            (
                3,
                [
                    ("a", "l", "bums"),
                    ("a", "lb", "ums"),
                    ("a", "lbu", "ms"),
                    ("a", "lbum", "s"),
                    ("al", "b", "ums"),
                    ("al", "bu", "ms"),
                    ("al", "bum", "s"),
                    ("alb", "u", "ms"),
                    ("alb", "um", "s"),
                    ("albu", "m", "s"),
                ],
            ),
            (6, [("a", "l", "b", "u", "m", "s")]),
            (7, []),
        ],
    )
    def test_splits_basic_examples(self, n, expected):
        assert list(extendible.n_splits("albums", n)) == expected

    # limit list max_size to avoid slow tests
    @given(st.data(), st.lists(st.integers(), min_size=2, max_size=15))
    def test_splits_concatenation_equals_input(self, data, lst):
        n = data.draw(st.integers(min_value=-1, max_value=len(lst)))
        splits = extendible.n_splits(lst, n)
        for split in splits:
            assert len(split) == n
            assert list(chain.from_iterable(split)) == lst

    @given(st.data(), st.lists(st.integers(), min_size=2, max_size=15))
    def test_correct_number_of_splits(self, data, lst):
        """
        n_splits should return all possible in order n-combinations of a sequence:
            - num_combinations = nCr(l-1, n-1)
            - l = length of sequence
        """
        n = data.draw(st.integers(min_value=1, max_value=len(lst)))
        splits = extendible.n_splits(lst, n)

        assert len(list(splits)) == ncr(len(lst) - 1, n - 1)

    def test_compounds(self, words):
        """
        Extendible version has a different API to the others -> not included
        in the parametrized test_compounds.
        """
        assert set(extendible.compounds(set(words))) == {
            ("al", "bums"),
            ("bar", "ely"),
            ("be", "foul"),
            ("con", "vex"),
            ("here", "by"),
            ("jig", "saw"),
            ("tail", "or"),
            ("we", "aver"),
        }
