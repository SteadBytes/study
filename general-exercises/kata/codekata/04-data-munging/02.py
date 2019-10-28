from pathlib import Path

FOOTBALL_DATA_FILE = Path("./data/football.dat")


def ident(x):
    return x


def dat_to_dict(f, headers=None, has_header=True, clean_f=None, extract_cols_f=None):
    if headers is None:
        headers = f.readline().strip().split()
    elif has_header:
        f.readline()  # skip headers within file
    clean_f = ident if clean_f is None else clean_f
    lines = (l for l in (l.strip() for l in f) if l)
    extract_cols_f = ident if extract_cols_f is None else extract_cols_f
    row_dicts = (dict(zip(headers, extract_cols_f(l))) for l in lines)
    return (d for d in (clean_f(d) for d in row_dicts) if d)


def get_cols(l):
    """
    Skip row numbers column (index 0) and '-' column (index 7).
    """
    cols = l.strip().split()
    return cols[1:7] + cols[8:]


with FOOTBALL_DATA_FILE.open() as f:
    d = dat_to_dict(f, extract_cols_f=get_cols)
    print(min(d, key=lambda x: abs(int(x["F"]) - int(x["A"])))["Team"])
