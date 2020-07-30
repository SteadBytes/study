import unittest

from .solution import Bit


class TestBit(unittest.TestCase):
    def test_bit(self):
        # Original cases
        number = int("10001110", base=2)
        bit = Bit(number)
        self.assertEqual(bit.get_bit(index=3), True)
        expected = int("10011110", base=2)
        self.assertEqual(bit.set_bit(index=4), expected)
        bit = Bit(number)
        expected = int("10000110", base=2)
        self.assertEqual(bit.clear_bit(index=3), expected)
        bit = Bit(number)
        expected = int("00000110", base=2)
        self.assertEqual(bit.clear_bits_msb_to_index(index=3), expected)
        bit = Bit(number)
        expected = int("10000000", base=2)
        self.assertEqual(bit.clear_bits_index_to_lsb(index=3), expected)
        bit = Bit(number)
        self.assertEqual(bit.update_bit(index=3, value=1), number)

    def test_new_cases(self):
        number = int("10001110", base=2)
        bit = Bit(number)
        expected = int("10000110", base=2)
        self.assertEqual(bit.update_bit(index=3, value=0), expected)
        bit = Bit(number)
        expected = int("10001111", base=2)
        self.assertEqual(bit.update_bit(index=0, value=1), expected)
        bit = Bit(number)
        expected = int("10011110", base=2)
        self.assertEqual(bit.update_bit(index=4, value=1), expected)
        bit = Bit(number)
        expected = int("10000110", base=2)
        self.assertEqual(bit.update_bit(index=3, value=0), expected)
