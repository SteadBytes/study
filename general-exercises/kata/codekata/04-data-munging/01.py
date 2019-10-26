from pathlib import Path

import re

WEATHER_DATA_FILE = Path("./data/weather.dat")


def ident(x):
    return x


def dat_to_dict(f, headers=None, has_header=True, clean_f=None):
    if headers is None:
        headers = f.readline().split()
    elif has_header:
        f.readline()  # skip headers within file
    clean_f = ident if clean_f is None else clean_f
    lines = (l for l in (l.strip() for l in f) if l)
    row_dicts = (dict(zip(headers, l.split())) for l in lines)
    return (d for d in (clean_f(d) for d in row_dicts) if d)


def clean(row_dict):
    if not re.match(r"\d+", row_dict["day"]):
        return None
    else:
        return {k: int(v.strip("*")) for k, v in row_dict.items()}


with WEATHER_DATA_FILE.open() as f:
    d = dat_to_dict(f, headers=["day", "min_t", "max_t"], clean_f=clean)
    print(min(d, key=lambda x: int(x["max_t"]) - int(x["min_t"]))["day"])
