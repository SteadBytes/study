from collections import defaultdict

# NOTE: 0-based col/row indexes
queen_positions = [-1] * 8  # indices = cols, elements = rows
in_row, in_left_diag, in_right_diag = [defaultdict(lambda: True) for _ in range(3)]


def safe(col, row):
    return in_row[row] and in_left_diag[col + row] and in_right_diag[col - row]


def rotate(queen_positions):
    rotated_coords = ((7 - row, col) for col, row in enumerate(queen_positions))
    return [row for col, row in sorted(rotated_coords)]


def mirror_image(queen_positions):
    pass


def compare_vector(v1, v2):
    pass


def is_isomorphic(queen_positions):
    iso_flag = False
    transformed = [x for x in queen_positions]

    for i in range(8):
        if i != 3:
            transformed = rotate(transformed)
        else:
            transformed = mirror_image(transformed)
        iso_flag = compare_vector(queen_positions, transformed)

        if not iso_flag:
            return True
    return False


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


if __name__ == "__main__":
    try_(0)
