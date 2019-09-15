#! /usr/bin/env python3

"""
Scan source files for camelCase strings, reporting (grep style) on locations or
converting to snake_case.

During conversion, orginal files are renamed with a ".backup" extension.

Yes I'm aware this can probably be achieved with a bash one-liner.
"""

import argparse
import re
import sys
from pathlib import Path
from typing import Generator, Iterable, List, NamedTuple, Optional

# matches valid lower camelCase strings according to the examples in Google
# Java style guide definition https://google.github.io/styleguide/javaguide.html#s5.3-camel-case
# see tests for examples of matching/non-matching strings
CAMEL_RE = re.compile(
    (
        # 1st character must be lowercase
        r"\b[a-z]+"
        # followed by a single digit
        # OR uppercase character/number followed by lower case characters or number
        r"((\d)|([A-Z0-9][a-z0-9]+))"
        # final character *may* be uppercase
        r"+([A-Z])?"
    )
)


def err_exit(msg, status=1):
    print(msg)
    exit(status)


class MatchGroup(NamedTuple):
    start: int
    end: int


def find_match_groups(s: str) -> list:
    return [MatchGroup(*m.span()) for m in CAMEL_RE.finditer(s)]


# Reporting


class Match(NamedTuple):
    lineno: int
    groups: List[MatchGroup]
    line: str


def pretty_match(m: Match, filename: str = None) -> str:
    """
    Build a 'pretty' string representation of `m`, with coloured text and line
    numbers; optionally prefixed with `filename`.

    Colours:
        - Line numbers = green
        - Matches = red
        - Filenames = purple
        - Non-matching text = white
    """
    pretty_name = f"\033[35m{filename}\033[m:" if filename else ""
    l = []
    prev = 0
    for g in m.groups:
        l.append(f"{m.line[prev:g.start]}\033[31m{m.line[g.start:g.end]}\033[m")
        prev = g.end
    l.append(m.line[prev:])
    return "".join([f"{pretty_name}\033[32m{m.lineno}\033[m:"] + l)


def find_camel(lines: Iterable[str]) -> Generator[Match, None, None]:
    for i, line in enumerate(lines):
        groups = find_match_groups(line)
        if groups:
            yield Match(i, groups, line)


def report_camel(file: Path, show_filenames=False):
    """
    Print a report on the locations of all camelCase strings in `file`. See
    `pretty_match` for output format.
    """
    with file.open() as f:
        for m in find_camel(f):
            print(pretty_match(m, filename=str(file) if show_filenames else None))


# Converting


CONVERT_CAMEL_RE = re.compile(r"((?<=[a-z0-9])[A-Z]|(?!^)(?<!_)[A-Z](?=[a-z]))")


def convert_camel_word(w: str) -> str:
    return CONVERT_CAMEL_RE.sub(r"_\1", w).lower()


def convert_camel_line(l: str) -> str:
    for g in find_match_groups(l):
        l = l[0 : g.start] + convert_camel_word(l[g.start : g.end]) + l[g.end :]
    return l


def convert_camel(lines: Iterable[str]) -> Generator[str, None, None]:
    return (convert_camel_line(l) for l in lines)


def transform_camel(file: Path):
    """
    Transform all occurences of camelCase strings in `file` to snake_case. The
    original file is renamed with a ".backup" extension to prevent data loss.
    """
    original = Path(str(file) + ".backup")
    file.rename(original)
    with original.open() as source:
        with file.open("w") as dest:
            dest.writelines(convert_camel(source))


# CLI


def main(files: List[Path], transform=False):
    for f in files:
        if transform:
            transform_camel(f)
        else:
            report_camel(f, show_filenames=len(files) > 1)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=sys.modules[__name__].__doc__)
    parser.add_argument("files", nargs="+", help="source files to scan for camelCase")
    parser.add_argument(
        "--convert",
        action="store_true",
        dest="convert",
        help="perform camelCase to snake_case conversion",
    )
    args = parser.parse_args()
    main([Path(f) for f in args.files], transform=args.convert)
