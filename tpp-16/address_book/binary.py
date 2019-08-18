import struct
from dataclasses import astuple
from functools import partial
from pathlib import Path
from typing import Iterable

from models import Person

PersonStruct = struct.Struct("50s50s15s10s50s50s10s36s")


def from_bytes(buffer: bytes) -> Person:
    return Person(
        *(x.decode("utf-8").rstrip("\x00") for x in PersonStruct.unpack(buffer))
    )


def to_bytes(p: Person) -> bytes:
    return PersonStruct.pack(*(s.encode("utf-8") for s in astuple(p)))


def read_address_book(db: Path):
    people = []
    with db.open("rb") as f:
        for chunk in iter(partial(f.read, PersonStruct.size), b""):
            people.append(from_bytes(chunk))
    return people


def write_address_book(db: Path, people: Iterable[Person]):
    with db.open("wb") as f:
        f.write(b"".join(to_bytes(p) for p in people))
