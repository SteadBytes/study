import pytest

from hits import *


@pytest.fixture
def responses():
    return [
        {
            "name": "steadman",
            "firstname": "ben",
            "s": "male",
            "age": 21,
            "choice": [1, 2, 3, 4, 5],
        },
        {
            "name": "Baggins",
            "firstname": "Bilbo",
            "s": "male",
            "age": 111,
            "choice": [2, 10, 11, 25, 4],
        },
    ]


def test_hit_totals(responses):
    assert hit_totals(responses) == {1: 1, 2: 2, 3: 1, 4: 2, 5: 1, 10: 1, 11: 1, 25: 1}


def test_most_popular_hits(responses):
    assert most_popular_hits(responses) == [
        (2, 2),
        (4, 2),
        (1, 1),
        (3, 1),
        (5, 1),
        (10, 1),
        (11, 1),
        (25, 1),
    ]


def test_categorise_responses():
    responses = [
        {
            "name": "Smith",
            "firstname": "John",
            "s": "male",
            "age": 18,
            "choice": [1, 2, 3, 4, 5],
        },
        {
            "name": "Baggins",
            "firstname": "Bilbo",
            "s": "male",
            "age": 111,
            "choice": [2, 10, 11, 25, 4],
        },
        {
            "name": "Undomiel",
            "firstname": "Arwen",
            "s": "female",
            "age": 2901,
            "choice": [1, 2, 3, 4, 5],
        },
        {
            "name": "Smith",
            "firstname": "Mary",
            "s": "female",
            "age": 18,
            "choice": [2, 10, 11, 25, 4],
        },
    ]

    assert categorise_responses(responses) == {
        "male_up_to_20": [
            {
                "name": "Smith",
                "firstname": "John",
                "s": "male",
                "age": 18,
                "choice": [1, 2, 3, 4, 5],
            }
        ],
        "male_over_20": [
            {
                "name": "Baggins",
                "firstname": "Bilbo",
                "s": "male",
                "age": 111,
                "choice": [2, 10, 11, 25, 4],
            }
        ],
        "female_up_to_20": [
            {
                "name": "Smith",
                "firstname": "Mary",
                "s": "female",
                "age": 18,
                "choice": [2, 10, 11, 25, 4],
            }
        ],
        "female_over_20": [
            {
                "name": "Undomiel",
                "firstname": "Arwen",
                "s": "female",
                "age": 2901,
                "choice": [1, 2, 3, 4, 5],
            }
        ],
    }
