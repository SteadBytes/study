#! /usr/bin/env python3

"""
Scans source files for camelCase strings and reports on their locations.
Yes I'm aware this can be achieved with a grep one-liner.
"""

import re
import sys
from pathlib import Path
from typing import Generator, Iterable, List, NamedTuple, Tuple

# matches valid camelCase strings
# see tests for examples of matching/non-matching strings
CAMEL_RE = re.compile(r"\b[a-z]+((\d)|([A-Z0-9][a-z0-9]+))+([A-Z])?")


def err_exit(msg, status=1):
    print(msg)
    exit(status)


class Match(NamedTuple):
    lineno: int
    start: int
    end: int
    line: str


def pretty_match(m: Match, filename: str = None):
    pretty_name = f"\033[35m{filename}\033" if filename else ""
    return (
        f"{pretty_name}\033[32m{m.lineno}\033[m:"
        f"\033[32m{m.start}\033[m:{m.line[:m.start]}"
        f"\033[31m{m.line[m.start:m.end]}\033[m{m.line[m.end:]}"
    )


def find_camel(lines: Iterable[str]) -> Generator[Match, None, None]:
    for i, line in enumerate(lines):
        for m in CAMEL_RE.finditer(line):
            yield Match(i, *m.span(), line)


def main(files: List[Path]):
    show_filenames = len(files) > 1
    for file in files:
        with file.open() as f:
            for m in find_camel(f):
                print(pretty_match(m, filename=file if show_filenames else None))


if __name__ == "__main__":
    if len(sys.argv) == 1:
        err_exit("Usage: ./camel_finder.py FILES...")

    main([Path(f) for f in sys.argv[1:]])
