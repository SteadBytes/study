import pytest

import camel_finder as cf

CAMEL_CASE_WORDS = [
    "camelCase",
    "camelCamelCase",
    "camel1Case",
    "camel1234CamelCase",
    "camelCase1234",
    "camelCase1234D",
]

NOT_CAMEL_CASE_WORDS = [
    "word",
    "PascalCase",
    "snake_case",
    "multiple words",
    "word!",
    "sort_of_snake_Case",
]


@pytest.mark.parametrize("s", CAMEL_CASE_WORDS)
def test_CAMEL_RE_match(s):
    assert cf.CAMEL_RE.match(s)


@pytest.mark.parametrize("s", NOT_CAMEL_CASE_WORDS)
def test_CAMEL_RE_no_match(s):
    assert cf.CAMEL_RE.match(s) is None


@pytest.mark.parametrize(
    "s,match_list",
    [
        (
            CAMEL_CASE_WORDS,
            [cf.Match(i, 0, len(w), w) for i, w in enumerate(CAMEL_CASE_WORDS)],
        ),
        (NOT_CAMEL_CASE_WORDS, []),
        (
            [
                "/**\n",
                " * Singly linked list\n",
                " */\n",
                "public class ListNode {\n",
                "\n",
                "\tprivate Object value;\n",
                "\n",
                "\tprivate ListNode next;\n",
                "\n",
                "\t/**\n",
                "\t * Default constructor for an empty node\n",
                "\t */\n",
                "\tpublic ListNode() {\n",
                "\t\tthis(null, null);\n",
                "\t}\n",
                "\n",
                "\t/**\n",
                "\t * Constructor to create node with data and next pointer\n",
                "\t */\n",
                "\tpublic ListNode(Object value, ListNode next) {\n",
                "\t\tthis.value = value;\n",
                "\t\tthis.next = next;\n",
                "\t}\n",
                "\n",
                "\tpublic Object getValue() {\n",
                "\t\treturn value;\n",
                "\t}\n",
                "\n",
                "\tpublic ListNode getNext() {\n",
                "\t\treturn next;\n",
                "\t}\n",
                "\n",
                "\tpublic void setValue(Object value) {\n",
                "\t\tthis.value = value;\n",
                "\t}\n",
                "\n",
                "\tpublic void setNext(ListNode next) {\n",
                "\t\tthis.next = next;\n",
                "\t}\n",
                "}",
            ],
            [
                cf.Match(
                    lineno=24, start=15, end=23, line="\tpublic Object getValue() {\n"
                ),
                cf.Match(
                    lineno=28, start=17, end=24, line="\tpublic ListNode getNext() {\n"
                ),
                cf.Match(
                    lineno=32,
                    start=13,
                    end=21,
                    line="\tpublic void setValue(Object value) {\n",
                ),
                cf.Match(
                    lineno=36,
                    start=13,
                    end=20,
                    line="\tpublic void setNext(ListNode next) {\n",
                ),
            ],
        ),
    ],
)
def test_find_camel(s, match_list):
    assert list(cf.find_camel(s)) == match_list


def test_pretty_match_no_filename():
    match = cf.Match(lineno=24, start=15, end=23, line="\tpublic Object getValue() {\n")
    assert (
        cf.pretty_match(match)
        == "\033[32m24\033[m:\033[32m15\033[m:\tpublic Object \033[31mgetValue\033[m() {\n"
    )


def test_pretty_match_with_filename():
    match = cf.Match(lineno=24, start=15, end=23, line="\tpublic Object getValue() {\n")
    assert (
        cf.pretty_match(match, filename="data/LinkedList.java")
        == "\033[35mdata/LinkedList.java\033\033[32m24\033[m:\033[32m15\033[m:\tpublic Object \033[31mgetValue\033[m() {\n"
    )
