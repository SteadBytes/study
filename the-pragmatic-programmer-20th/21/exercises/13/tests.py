import pytest

import camel_finder as cf

CAMEL_CASE_WORDS = {
    "camelCase": "camel_case",
    "camelCamelCase": "camel_camel_case",
    "camel1Case": "camel1_case",
    "camel1234CamelCase": "camel1234_camel_case",
    "camelCase1234": "camel_case1234",
    "camelCase1234D": "camel_case1234_d",
}

NOT_CAMEL_CASE_WORDS = ["word", "PascalCase", "snake_case", "multiple words", "word!"]

JAVA_LINES = [
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
]


@pytest.mark.parametrize("s", CAMEL_CASE_WORDS)
def test_CAMEL_RE_match(s):
    assert cf.CAMEL_RE.match(s)


@pytest.mark.parametrize("s", NOT_CAMEL_CASE_WORDS)
def test_CAMEL_RE_no_match(s):
    assert cf.CAMEL_RE.match(s) is None


@pytest.mark.parametrize(
    "s,match_groups",
    [(w, [cf.MatchGroup(0, len(w))]) for w in CAMEL_CASE_WORDS]
    + [(w, []) for w in NOT_CAMEL_CASE_WORDS]
    + [
        (
            "System.out.println(Arrays.toString(myArray))",
            [cf.MatchGroup(26, 34), cf.MatchGroup(35, 42)],
        )
    ],
)
def test_find_match_groups(s, match_groups):
    assert cf.find_match_groups(s) == match_groups


@pytest.mark.parametrize(
    "s,match_list",
    [
        (
            CAMEL_CASE_WORDS,
            [
                cf.Match(i, [cf.MatchGroup(0, len(w))], w)
                for i, w in enumerate(CAMEL_CASE_WORDS)
            ],
        ),
        (NOT_CAMEL_CASE_WORDS, []),
        (
            ["System.out.println(Arrays.toString(myArray))"],
            [
                cf.Match(
                    0,
                    [cf.MatchGroup(26, 34), cf.MatchGroup(35, 42)],
                    "System.out.println(Arrays.toString(myArray))",
                )
            ],
        ),
        (
            JAVA_LINES,
            [
                cf.Match(24, [cf.MatchGroup(15, 23)], "\tpublic Object getValue() {\n"),
                cf.Match(
                    28, [cf.MatchGroup(17, 24)], "\tpublic ListNode getNext() {\n"
                ),
                cf.Match(
                    32,
                    [cf.MatchGroup(13, 21)],
                    "\tpublic void setValue(Object value) {\n",
                ),
                cf.Match(
                    36,
                    [cf.MatchGroup(13, 20)],
                    "\tpublic void setNext(ListNode next) {\n",
                ),
            ],
        ),
    ],
)
def test_find_camel(s, match_list):
    assert list(cf.find_camel(s)) == match_list


@pytest.mark.parametrize(
    "m,expected",
    [
        (
            cf.Match(24, [cf.MatchGroup(15, 23)], "\tpublic Object getValue() {"),
            "\033[32m24\033[m:\tpublic Object \033[31mgetValue\033[m() {",
        ),
        (
            cf.Match(
                0,
                [cf.MatchGroup(26, 34), cf.MatchGroup(35, 42)],
                "System.out.println(Arrays.toString(myArray))",
            ),
            "\033[32m0\033[m:System.out.println(Arrays.\033[31mtoString\033[m(\033[31mmyArray\033[m))",
        ),
    ],
)
def test_pretty_match_no_filename(m, expected):
    assert cf.pretty_match(m) == expected


@pytest.mark.parametrize(
    "m,filename,expected",
    [
        (
            cf.Match(24, [cf.MatchGroup(15, 23)], "\tpublic Object getValue() {\n"),
            "data/LinkedList.java",
            "\033[35mdata/LinkedList.java\033[m:\033[32m24\033[m:\tpublic Object \033[31mgetValue\033[m() {\n",
        ),
        (
            cf.Match(
                0,
                [cf.MatchGroup(26, 34), cf.MatchGroup(35, 42)],
                "System.out.println(Arrays.toString(myArray))",
            ),
            "data/ArrayPrint.java",
            "\033[35mdata/ArrayPrint.java\033[m:\033[32m0\033[m:System.out.println(Arrays.\033[31mtoString\033[m(\033[31mmyArray\033[m))",
        ),
    ],
)
def test_pretty_match_with_filename(m, filename, expected):
    assert cf.pretty_match(m, filename=filename) == expected


@pytest.mark.parametrize("s,expected", CAMEL_CASE_WORDS.items())
def test_convert_camel_word(s, expected):
    assert cf.convert_camel_word(s) == expected


@pytest.mark.parametrize("s", JAVA_LINES)
def test_convert_camel_line_result_has_no_camel_matches(s):
    converted = cf.convert_camel_line(s)
    assert not cf.find_match_groups(converted)


@pytest.mark.parametrize(
    "s",
    NOT_CAMEL_CASE_WORDS
    + [
        "a sentence without camel case",
        "a sentence with snake_case",
        "a sentence with numbers 1234",
        "a sentence with radnom punctuation !?@?#';",
    ],
)
def test_convert_camel_line_noop_if_no_camel(s):
    converted = cf.convert_camel_line(s)
    assert converted == s
