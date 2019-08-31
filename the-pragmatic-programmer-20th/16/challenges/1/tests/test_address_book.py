import pytest

from address_book import binary, plain_text
from address_book.models import Person, generate_people


@pytest.mark.parametrize("p", generate_people(50))
def test_to_bytes_inverts_from_bytes(p):
    p_bytes, size = binary.to_bytes(p)
    p_again = binary.from_bytes(p_bytes)
    assert p == p_again


@pytest.mark.parametrize("p", generate_people(50))
def test_to_dict_inverts_from_dict(p):
    p_dict = plain_text.to_dict(p)
    p_again = plain_text.from_dict(p_dict)
    assert p == p_again


@pytest.mark.parametrize(
    "module,fname", [(binary, "address-book.bin"), (plain_text, "address-book.json")]
)
def test_write_address_book_inverts_read_address_book(module, fname, tmp_path):
    db = tmp_path / fname
    # sanity check
    assert db.exists() is False

    people = list(generate_people(50))
    module.write_address_book(db, people)

    assert db.exists() is True
    assert db.stat().st_size > 0

    people_again = module.read_address_book(db)

    assert people == people_again
