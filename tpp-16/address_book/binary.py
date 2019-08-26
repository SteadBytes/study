import struct
from dataclasses import astuple
from functools import partial
from pathlib import Path
from typing import Iterable

from models import Person

PERSON_STRUCT_FMT = "50s50s30s10s50s50s10s{}s36s"


def from_bytes(buffer: bytes) -> Person:
    # calculate sizes of non-variable formats
    before_fmt, after_fmt = PERSON_STRUCT_FMT.split("{}s")
    before_start = struct.calcsize(before_fmt)
    after_start = len(buffer) - struct.calcsize(after_fmt)

    before, direction, after = (
        buffer[:before_start],
        buffer[before_start:after_start],
        buffer[after_start:],
    )

    # dynamically build struct format string for variable length field
    direction_fmt = "{}s".format(len(direction))
    data = (
        struct.unpack(before_fmt, before)
        + struct.unpack(direction_fmt, direction)
        + struct.unpack(after_fmt, after)
    )
    return Person(*(x.decode("utf-8").rstrip("\x00") for x in data))


def to_bytes(p: Person) -> bytes:
    # dynamically add size to format for variable length directions field
    fmt = PERSON_STRUCT_FMT.format(len(p.directions))
    return struct.pack(fmt, *(s.encode("utf-8") for s in astuple(p)))


def read_address_book(db: Path):
    people = []
    with db.open("rb") as f:
        for chunk in iter(partial(f.read, PersonStruct.size), b""):
            people.append(from_bytes(chunk))
    return people


def write_address_book(db: Path, people: Iterable[Person]):
    with db.open("wb") as f:
        f.write(b"".join(to_bytes(p) for p in people))
