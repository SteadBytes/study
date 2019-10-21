import pytest
from random import shuffle
import strings_fsm

from hypothesis import given, strategies as st


def text_no_quotes():
    """
    `st.text` strategy without '"' character or empty strings.
    """
    return st.text(st.characters(blacklist_characters='"'), min_size=1)


@st.composite
def quoted_string(draw):
    """
    Quoted `st.text` strategy.
    """
    s = draw(text_no_quotes())
    return f'"{s}"'


@st.composite
def quoted_unquoted(draw):
    """
    Generate a 2-tuple containing a list of quoted (i.e. '"foo"') strings and a
    list of unquoted (i.e. 'bar') strings. The unquoted list has length >= that
    of the quoted list.
    Returns:
        (tuple): (quoted, unquoted)
    """
    quoted = draw(st.lists(quoted_string()))
    unquoted = draw(st.lists(text_no_quotes(), min_size=len(quoted)))
    return quoted, unquoted


@pytest.mark.parametrize(
    "find_strings", [strings_fsm.find_strings_original, strings_fsm.find_strings]
)
@pytest.mark.parametrize(
    "text,expected",
    [
        ('"foo" bar', ["foo"]),
        ('"foo" bar "baz"', ["foo", "baz"]),
        ('"foo" bar "baz bat"', ["foo", "baz bat"]),
        ("foo bar baz bat", []),
    ],
)
def test_find_strings(find_strings, text, expected):
    assert list(find_strings(text)) == expected


@pytest.mark.parametrize(
    "find_strings", [strings_fsm.find_strings_original, strings_fsm.find_strings]
)
@given(quoted_unquoted())
def test_find_strings_generative(find_strings, qu):
    # build an input text of mixed quoted and unquoted strings
    quoted, unquoted = qu
    mixed = list(zip(quoted, unquoted))
    shuffle(mixed)
    text = "".join(a + b for a, b in mixed)

    found = list(strings_fsm.find_strings(text))

    assert set(found) == set(q.strip('"') for q in quoted)
