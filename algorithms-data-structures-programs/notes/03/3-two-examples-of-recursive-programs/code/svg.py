EMPTY_PATH = ""


def path_exp(letter, path, *coordinates):
    """
    SVG render path
    """
    return letter + " " + " ".join([str(c) for c in coordinates]) + " " + path


def rel_ver(y, path):
    """
    Relative vertical path
    """
    return path_exp("v", path, y)


def rel_horiz(x, path):
    """
    Relative horizontal path
    """
    return path_exp("h", path, x)


def abs_move(x, y, path):
    """
    Absolute move start of path to x y
    """
    return f"M {x} {y} {path}"
