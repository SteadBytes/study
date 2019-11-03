import pytest

from readable import compounds


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


def test_compounds(words):
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
