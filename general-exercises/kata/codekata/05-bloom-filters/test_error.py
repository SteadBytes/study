from hypothesis import given, event, settings
from hypothesis import strategies as st

from spell_check import WORDS_PATH, check

with WORDS_PATH.open() as f:
    VALID_WORDS = {l.strip() for l in f}


@settings(max_examples=1000)
@given(st.text(min_size=5, max_size=5))
def test_spell_check_false_positives(word):
    """
    Slightly abusing pytest & hypothesis to test false positive rate. The test
    will always pass but marks a 'False positive` event to be output using
    --hypothesis-show-statistics pytest argument.

    Tests using random 5-character words as specified in the Kata.
    """
    if check(word) is True and word not in VALID_WORDS:
        event("False positive")
