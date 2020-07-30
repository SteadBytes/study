class Bit:
    def __init__(self, number):
        self.number = number

    def get_bit(self, index):
        return (self.number >> index) & 1

    def set_bit(self, index):
        return self.number | (1 << index)

    def clear_bit(self, index):
        return self.number & ~(1 << index)

    def clear_bits_msb_to_index(self, index):
        return self.number & (1 << index) - 1

    def clear_bits_index_to_lsb(self, index):
        return self.number & ~((1 << index + 1) - 1)

    def update_bit(self, index, value):
        # Sanity check
        assert value == 1 or value == 0
        return (self.number & ~(1 << index)) | (value << index)
