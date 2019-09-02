def subrange(base, low, high):
    if low > high:
        raise ValueError(f"low must be <= high")

    class SubRange(base):
        def __new__(cls, value):
            if low <= value and value <= high:
                x = base.__new__(cls, value)
                return x
            else:
                raise ValueError(f"value {value} not in range {low}..{high}")

    return SubRange


Letter = subrange(str, "A", "Z")

Digit = subrange(int, 0, 9)
