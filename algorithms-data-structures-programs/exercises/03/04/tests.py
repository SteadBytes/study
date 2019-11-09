import pytest

from eight_queens import rotate


# TODO: Property tests
@pytest.mark.parametrize(
    "queen_positions,expected",
    [
        ([0, 4, 7, 5, 2, 6, 1, 3], [2, 5, 3, 1, 7, 4, 6, 0]),
        ([0, 5, 7, 2, 6, 3, 1, 4], [2, 4, 1, 7, 5, 3, 6, 0]),
        ([1, 3, 5, 7, 2, 0, 6, 4], [3, 6, 2, 7, 1, 4, 0, 5]),
    ],
)
def test_rotate(queen_positions, expected):
    assert rotate(queen_positions) == expected
