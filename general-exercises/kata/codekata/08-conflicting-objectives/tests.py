import pytest

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
