import string

import pytest
from hypothesis import HealthCheck, given, settings, assume
from hypothesis import strategies as st

from q2 import Letter, Digit
from subrange import subrange


@st.composite
def int_lhv(draw, in_range=True):
    low = draw(st.integers())
    high = draw(st.integers(min_value=low))
    val = (
        draw(st.integers(min_value=low, max_value=high))
        if in_range
        else draw(st.integers().filter(lambda x: x < low or x > high))
    )
    return low, high, val


@st.composite
def str_lhv(draw, in_range=True):
    low = draw(st.characters())
    high = draw(st.characters(min_codepoint=ord(low)))
    val = (
        draw(st.characters(min_codepoint=ord(low), max_codepoint=ord(high)))
        if in_range
        else draw(st.characters().filter(lambda c: c < low or c > high))
    )
    return low, high, val


class TestSubrange:
    @given(int_lhv())
    def test_int_in_range(self, lhv):
        low, high, val = lhv
        IntSubrange = subrange(int, low, high)
        x = IntSubrange(val)
        assert x == val

    @given(int_lhv(in_range=False))
    def test_int_out_of_range_raises(self, lhv):
        low, high, val = lhv
        IntSubrange = subrange(int, low, high)
        with pytest.raises(ValueError):
            IntSubrange(val)

    @given(st.data())
    def test_int_add_in_range(self, data):
        low, high, val = data.draw(int_lhv())
        y = data.draw(st.integers().filter(lambda x: val + x in range(low, high)))
        IntSubrange = subrange(int, low, high)
        x = IntSubrange(val)
        z = x + y
        assert z == val + y
        assert type(z) == IntSubrange

    @given(st.data())
    def test_int_add_out_of_range_raises(self, data):
        low, high, val = data.draw(int_lhv())
        y = data.draw(st.integers().filter(lambda x: val + x < low or high < val + x))
        IntSubrange = subrange(int, low, high)
        x = IntSubrange(val)
        with pytest.raises(Exception):
            x + y

    @given(st.data())
    def test_int_sub_in_range(self, data):
        low, high, val = data.draw(int_lhv())
        y = data.draw(st.integers().filter(lambda x: val - x in range(low, high)))
        IntSubrange = subrange(int, low, high)
        x = IntSubrange(val)
        z = x - y
        assert z == val - y
        assert type(z) == IntSubrange

    @given(st.data())
    def test_int_sub_out_of_range_raises(self, data):
        low, high, val = data.draw(int_lhv())
        y = data.draw(st.integers().filter(lambda x: val - x < low or high < val - x))
        IntSubrange = subrange(int, low, high)
        x = IntSubrange(val)
        with pytest.raises(Exception):
            x - y

    @given(st.data())
    @settings(suppress_health_check=[HealthCheck.filter_too_much])
    def test_int_mul_in_range(self, data):
        low, high, val = data.draw(int_lhv())
        y = data.draw(st.integers().filter(lambda x: val * x in range(low, high)))
        IntSubrange = subrange(int, low, high)
        x = IntSubrange(val)
        z = x * y
        assert z == val * y
        assert type(z) == IntSubrange

    @given(st.data())
    def test_int_mul_out_of_range_raises(self, data):
        low, high, val = data.draw(int_lhv())
        y = data.draw(st.integers().filter(lambda x: val * x < low or high < val * x))
        IntSubrange = subrange(int, low, high)
        x = IntSubrange(val)
        with pytest.raises(Exception):
            x * y

    # TODO: Property to generate division tests
    # val / x >=low
    # val / x <= high
    # x != 0
    def test_int_div_in_range(self):
        low, high, val = (0, 10, 8)
        y = 2
        IntSubrange = subrange(int, low, high)
        x = IntSubrange(val)
        z = x / y
        assert z == val / y
        assert type(z) == IntSubrange

    def test_int_floor_div_in_range(self):
        low, high, val = (0, 10, 8)
        y = 2
        IntSubrange = subrange(int, low, high)
        x = IntSubrange(val)
        z = x // y
        assert z == val // y
        assert type(z) == IntSubrange

    @given(str_lhv())
    def test_str_in_range(self, lhv):
        low, high, val = lhv
        StrSubrange = subrange(str, low, high)
        c = StrSubrange(val)
        assert c == val

    @given(str_lhv(in_range=False))
    def test_str_out_of_range_raises(self, lhv):
        low, high, val = lhv
        StrSubrange = subrange(str, low, high)
        with pytest.raises(ValueError):
            StrSubrange(val)

    @given(st.data())
    def test_str_add(self, data):
        low, high, val = data.draw(str_lhv())
        t = data.draw(st.characters())
        StrSubrange = subrange(str, low, high)
        s = StrSubrange(val)
        if val + t < low or high < val + t:
            with pytest.raises(Exception):
                s + t
        else:
            z = s + t
            assert z == val + t
            assert type(z) == StrSubrange

    @given(st.data())
    @settings(suppress_health_check=[HealthCheck.filter_too_much])
    def test_str_add_out_of_range_raises(self, data):
        low, high, val = data.draw(str_lhv())
        # filters lots of examples, hence health check suppression
        t = data.draw(st.characters().filter(lambda s: val + s < low or high < val + s))
        StrSubrange = subrange(str, low, high)
        s = StrSubrange(val)
        with pytest.raises(Exception):
            s + t


class TestLetter:
    @pytest.mark.parametrize("ch", string.ascii_uppercase)
    def test_letters_in_range(self, ch):
        l = Letter(ch)
        assert l == ch

    @given(st.characters(blacklist_characters=string.ascii_uppercase))
    def test_letters_out_of_range(self, ch):
        with pytest.raises(ValueError):
            Letter(ch)


class TestDigit:
    @pytest.mark.parametrize("x", range(10))
    def test_digits_in_range(self, x):
        d = Digit(x)
        assert d == x

    @given(st.integers().filter(lambda x: x < 0 or x > 9))
    def test_digits_out_of_range(self, x):
        with pytest.raises(ValueError):
            Digit(x)
