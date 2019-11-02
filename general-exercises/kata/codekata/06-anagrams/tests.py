from collections import Counter
from itertools import chain, starmap
from operator import eq

import pytest
from hypothesis import given
from hypothesis import strategies as st

from anagrams import all_anagrams, build_index, longest_anagram, most_anagrams

# Example based tests


@pytest.fixture
def words():
    return [
        "kinship",
        "octopus",
        "pinkish",
        "enlist",
        "inlets",
        "listen",
        "silent",
        "boaster",
        "boaters",
        "borates",
        "fresher",
        "refresh",
        "hello",
        "sinks",
        "skins",
        "knits",
        "stink",
        "rots",
        "sort",
        "heel",
    ]


@pytest.fixture
def index(words):
    return build_index(words)


def test_build_index(index):
    assert list(index.values()) == [
        {"kinship", "pinkish"},
        {"octopus"},
        {"enlist", "inlets", "listen", "silent"},
        {"boaster", "boaters", "borates"},
        {"fresher", "refresh"},
        {"hello"},
        {"sinks", "skins"},
        {"knits", "stink"},
        {"rots", "sort"},
        {"heel"},
    ]


def test_longest_anagram(index):
    assert longest_anagram(index) == {"kinship", "pinkish"}


def test_most_anagrams(index):
    assert most_anagrams(index) == {"enlist", "inlets", "listen", "silent"}


def test_all_anagrams(index):
    assert list(all_anagrams(index)) == [
        {"kinship", "pinkish"},
        {"enlist", "inlets", "listen", "silent"},
        {"boaster", "boaters", "borates"},
        {"fresher", "refresh"},
        {"sinks", "skins"},
        {"knits", "stink"},
        {"rots", "sort"},
    ]


# TODO: Property based tests
