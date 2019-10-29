import re
from itertools import chain
from pathlib import Path
from typing import IO, Any, AnyStr, Callable, Iterable, List, Optional, Tuple


def extract_cols(l: str, *slices: Tuple, sep: Optional[str] = None) -> Iterable[str]:
    cols = l.split(sep)
    if slices:
        return chain.from_iterable(cols[slice(*s)] for s in slices)
    else:
        return iter(cols)


def ident(x: Any) -> Any:
    return x


def lines_to_cols(lines: Iterable[str], *slices: Tuple, sep: Optional[str] = None):
    lines_ = (l_ for l_ in (l.strip() for l in lines))
    return (
        cols for cols in (extract_cols(l_, *slices, sep=sep) for l_ in lines_) if cols
    )


def dat_to_dict(
    f: IO[AnyStr],
    *col_slices: Tuple,
    sep: Optional[str] = None,
    headers: Optional[List[str]] = None,
    has_header: bool = True,
    process_row_dict: Callable[[dict], dict] = ident
):
    if headers is None:
        headers = f.readline().strip().split()
    elif has_header:
        f.readline()  # skip headers within file

    lines = lines_to_cols(f, *col_slices, sep=sep)
    row_dicts = (dict(zip(headers, l)) for l in lines)
    return (d_ for d_ in (process_row_dict(d) for d in row_dicts if d) if d_)


def part_1(football_data_file: Path):
    with football_data_file.open() as f:
        d = dat_to_dict(f, (1, 7), (8, None))
        return min(d, key=lambda x: abs(int(x["F"]) - int(x["A"])))["Team"]


def process_weather_row(row_dict):
    """
    Skip rows not beginning with a valid day number, remove extraneous '*' chars.
    """
    if not re.match(r"\d+", row_dict["day"]):
        return None
    else:
        return {k: int(v.strip("*")) for k, v in row_dict.items()}


def part_2(weather_data_file: Path):
    with weather_data_file.open() as f:
        d = dat_to_dict(
            f, headers=["day", "min_t", "max_t"], process_row_dict=process_weather_row
        )
        return min(d, key=lambda x: int(x["max_t"]) - int(x["min_t"]))["day"]


if __name__ == "__main__":
    DATA_DIR = Path("./data")
    FOOTBALL_DATA_FILE = DATA_DIR / "football.dat"
    WEATHER_DATA_FILE = DATA_DIR / "weather.dat"

    print(part_1(FOOTBALL_DATA_FILE))
    print(part_2(WEATHER_DATA_FILE))
