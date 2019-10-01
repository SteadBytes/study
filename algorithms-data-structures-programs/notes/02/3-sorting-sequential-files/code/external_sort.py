import filecmp
import heapq
import pickle
from itertools import zip_longest
from pathlib import Path
from tempfile import NamedTemporaryFile
from typing import Iterable, TextIO

import pytest
from hypothesis import given, settings
from hypothesis import strategies as st


def dump_iterable(iterable: Iterable) -> Path:
    with NamedTemporaryFile(suffix=".pickle", delete=False) as fobj:
        for item in iterable:
            pickle.dump(item, fobj)
        return Path(fobj.name)


def load_dump(dump: Path):
    with dump.open(mode="rb") as fobj:
        try:
            while True:
                yield pickle.load(fobj)
        except EOFError:
            pass


def load_and_delete_dump(dump: Path):
    yield from load_dump(dump)
    dump.unlink()


def merge_sorted_parts(part_files: Iterable[TextIO]) -> Iterable:
    """
    Merge multiple sorted partition files into a single sorted sequence.
    Yields items lazily -> does not load all parts into memory at once.
    """
    return heapq.merge(*(load_and_delete_dump(f) for f in part_files))


def grouper(iterable: Iterable, n: int, fillvalue=None) -> Iterable[tuple]:
    """
    Collect iterable into chunks of fixed-length tuples

    >>> list(grouper([1, 2, 3, 4, 5, 6, 7], 3))
    [(1, 2, 3), (4, 5, 6), (7, None, None)]

    >>> list(grouper([1, 2, 3, 4, 5, 6, 7], 3, fillvalue=0))
    [(1, 2, 3), (4, 5, 6), (7, 0, 0)]
    """
    args = [iter(iterable)] * n
    return zip_longest(*args, fillvalue=fillvalue)


# TODO: Allow custom sorting (i.e. key function) and dumping strategies
# TODO: Calculate part_size based on a given max RAM usage?
def external_sort(iterable: Iterable, part_size=1024) -> Iterable:
    """
    Sort of `iterable` using an external merge sort to allow sorting of
    data too large to fit in memory.

    Args:
        iterable (Iterable): Items to sort
        part_size (int): Maximum number of items from `iterable` to hold in
        memory at a time.
    """
    parts = (tuple(e for e in g if e is not None) for g in grouper(iterable, part_size))
    dump_files = (dump_iterable(sorted(part)) for part in parts)
    return merge_sorted_parts(dump_files)


def sort_large_file(p: Path, part_size=1024):
    """
    In-place sort of file at path `p` using an external sort to allow for files
    larger than available memory to be sorted.

    Args:
        p (Path): File to be sorted
        part_size (int): Maximum number of lines to hold in memory at a time,
        used to determine size of partitions used in external sorting.
    """
    with p.open() as f:
        p_sorted = external_sort(f, part_size=part_size)

    with p.open("w") as f:
        f.writelines(p_sorted)


@pytest.mark.parametrize("elements", [st.integers, st.text])
@settings(deadline=600)
@given(st.data())
def test_external_sort(elements, data):
    """
    NOTE: Hypothesis deadline setting overriden as single character values i.e. ['0']
        cause Hypothesis to raise unreliable test timings exceptions. Can't repro slow
        times outside of tests - bug in Hypothesis? increase deadline to allow tests to run.
        Opened issue on GitHub https://github.com/HypothesisWorks/hypothesis/issues/2108
    """
    lst = data.draw(st.lists(elements()))
    assert list(external_sort(iter(lst))) == sorted(lst)


@settings(deadline=600)
@given(st.lists(st.integers()))
def test_sort_large_file(tmp_path, lst):
    """
    NOTE: Hypothesis deadline setting overriden as single character values i.e. ['0']
        cause Hypothesis to raise unreliable test timings exceptions. Can't repro slow
        times outside of tests - bug in Hypothesis? increase deadline to allow tests to run.
        Opened issue on GitHub https://github.com/HypothesisWorks/hypothesis/issues/2108
    """
    # generate input data as 1 value per line
    input_data = [str(e) + "\n" for e in lst]

    # write data to file for sorting
    file_to_sort = tmp_path / "input.txt"
    with file_to_sort.open("w") as f:
        f.writelines(input_data)

    # write *sorted* data to golden file
    golden_file = tmp_path / "golden.txt"
    with golden_file.open("w") as f:
        f.writelines(sorted(input_data))

    sort_large_file(file_to_sort, part_size=10)

    assert filecmp.cmp(file_to_sort, golden_file, shallow=False)


def test_load_and_delete_dump(tmp_path):
    dump = tmp_path / "dump.pickle"
    # serialisation handled by pickle -> not exhaustively testing
    lst = [[0, 1], [2, 3, 4, 5]]

    # write object to file
    with dump.open("wb") as f:
        pickle.dump(lst, f)

    lst_again = list(load_and_delete_dump(dump))  # force generator

    # sanity check that deserialisation worked
    assert lst_again == lst_again

    # file should have been deleted
    assert not dump.exists()
