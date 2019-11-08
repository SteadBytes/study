from collections import defaultdict

# NOTE: 0-based row/col indexes
queen_positions = [-1] * 8
in_row, in_left_diag, in_right_diag = [defaultdict(lambda: True) for _ in range(3)]


def safe(col, row):
    return in_row[row] and in_left_diag[col + row] and in_right_diag[col - row]


def try_(col):

    for row in range(8):
        if safe(col, row):
            # set queen
            queen_positions[col] = row
            in_row[row] = False
            in_left_diag[col + row] = False
            in_right_diag[col - row] = False
            if col < 7:
                try_(col + 1)
            else:
                print(queen_positions)
            # remove queen
            in_row[row] = True
            in_left_diag[col + row] = True
            in_right_diag[col - row] = True


try_(0)
