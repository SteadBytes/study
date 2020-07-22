"""
Smart Game Format (SGF) string parsing.

SGF
---

- Single tree of nodes
- Each node is a property list
- Property list contains key value pairs
    - Unique keys
    - Multiple values per key


Examples
========

`(;FF[4]C[root]SZ[19];B[aa];W[ab])`

Single tree:

1. (root)
    - key = "FF", value = "4"
    - key = "C", value = "root"
    - key = "SZ", value = "19"

    2.
        - key = "B", value = "aa"
    3.
        - key = "W", value = "ab"


`(;FF[4](;B[aa];W[ab])(;B[dd];W[ee]))`

Multiple trees:

1. (root)
    - key = "FF", value = "4"

    2. (subtree 1)
        1. key = "B", value = "aa"
        2. key = "W", value = "ab"

    3. (subtree 2)
        1. key = "B", value = "dd"
        2. key = "W", value = "ee"


`(;FF[4];AB[aa][ab][ba])`

Single tree:
1. (root)
    - key = "FF", value = "4"

    2.
        - key = "AB"
            - value = "aa"
            - value = "ab"
            - value = "ab"


Grammar
=======

sgf_tree ::= tree+
tree ::= "(" sequence tree* ")"
sequence ::= node+
node ::= ";" property+
property ::= key value+
key ::= upper_char [lower_char | digit]?
value ::= "[" text "]"
text ::= (char; "\\]" = "]", "\\\\" = "\\")*
"""

from __future__ import annotations

from enum import Enum
from typing import (
    DefaultDict,
    Generic,
    Iterable,
    Iterator,
    NamedTuple,
    TypeVar,
)

TokenType = Enum(
    "TokenType",
    ["TREE_START", "TREE_END", "VALUE_START", "VALUE_END", "NODE_DELIM", "TEXT"],
)


class Token(NamedTuple):
    type: TokenType
    string: str
    start: int
    end: int


# Semantic indication that a single character is expected (e.g. `str` of length
# 1). *Not* enforced at runtime or during type checking.
Char = str
TokenStream = Iterator[Token]

NON_TEXT_TOKEN_MAP = {
    "(": TokenType.TREE_START,
    ")": TokenType.TREE_END,
    "[": TokenType.VALUE_START,
    "]": TokenType.VALUE_END,
    ";": TokenType.NODE_DELIM,
}

ESCAPE = "\\"


T = TypeVar("T")


# > White spaces other than linebreaks are converted to space
# https://www.red-bean.com/sgf/sgf4.html#text
WS_TRANS = str.maketrans("\t\v", "  ")


class SgfTree:
    def __init__(self, properties=None, children=None):
        self.properties = properties or {}
        self.children = children or []

    def __eq__(self, other):
        if not isinstance(other, SgfTree):
            return False
        for k, v in self.properties.items():
            if k not in other.properties:
                return False
            if other.properties[k] != v:
                return False
        for k in other.properties.keys():
            if k not in self.properties:
                return False
        if len(self.children) != len(other.children):
            return False
        for a, b in zip(self.children, other.children):
            if a != b:
                return False
        return True

    def __ne__(self, other):
        return not self == other


class Peekable(Generic[T]):
    def __init__(self, it: Iterable[T]):
        self.it = iter(it)
        self.peeked = False

    def __next__(self) -> T:
        if self.peeked:
            self.peeked = False
            return self.head
        else:
            return next(self.it)

    def peek(self) -> T:
        if not self.peeked:
            self.peeked = True
            self.head = next(self.it)
        return self.head

    def __iter__(self) -> Peekable[T]:
        return self


def tokenise(src: Iterable[Char]) -> TokenStream:
    """Yields ``Token``s parsed from ``src``.

    Note: I made this harder for myself by using ``Iterable[Char]`` instead of
    ``str``, however I feel this makes for a more useable implementation e.g. this
    can lazily consume and parse instead of loading everything into a single
    string.
    """
    # Rename required by MyPy https://github.com/python/mypy/issues/1174
    src_ = Peekable(enumerate(src))

    while True:
        try:
            pos, ch = next(src_)
        except StopIteration:
            break

        if ch in NON_TEXT_TOKEN_MAP:
            yield Token(NON_TEXT_TOKEN_MAP[ch], ch, pos, pos + 1)
        else:
            start = pos
            chars = []
            try:
                while True:
                    if ch == "\\":
                        pos, ch = next(src_)
                    chars.append(ch)
                    if src_.peek()[1] in NON_TEXT_TOKEN_MAP:
                        break
                    else:
                        pos, ch = next(src_)
            finally:
                yield Token(TokenType.TEXT, "".join(chars), start, pos + 1)


def _value(token: Token, tokens: TokenStream, trans_tbl=WS_TRANS):
    if token.type != TokenType.VALUE_START:
        raise ValueError(
            f"Invalid input at position {token.start}: expected {TokenType.VALUE_START}, found {token}"
        )
    token = next(tokens)
    if token.type != TokenType.TEXT:
        raise ValueError(
            f"Invalid input at position {token.start}: expected {TokenType.TEXT}, found {token}"
        )
    return next(tokens), token.string.translate(trans_tbl)


def _key(token: Token, tokens: TokenStream):
    if token.type != TokenType.TEXT:
        raise ValueError(
            f"Invalid input at position {token.start}: expected {TokenType.TEXT}, found {token}"
        )

    key = token.string
    if len(key) != 1 or not key.isupper():
        raise ValueError(
            f"Invalid input at position {token.start}: expected upper case char, found {token}"
        )

    token = next(tokens)

    if token.type != TokenType.VALUE_START:
        raise ValueError(
            f"Invalid input at position {token.start}: expected {TokenType.VALUE_START}, found {token}"
        )

    return (token, key)


def _properties(token: Token, tokens: TokenStream):
    properties = DefaultDict(list)
    while token.type not in (
        TokenType.TREE_START,
        TokenType.TREE_END,
        TokenType.NODE_DELIM,
    ):
        token, key = _key(token, tokens)

        while token.type == TokenType.VALUE_START:
            token, val = _value(token, tokens)
            properties[key].append(val)
            if token.type != TokenType.VALUE_END:
                raise ValueError(
                    f"Invalid input at position {token.start}: expected {TokenType.VALUE_END}, found {token}"
                )

            token = next(tokens)

    return token, properties


def _node(token: Token, tokens: TokenStream):
    if token.type != TokenType.NODE_DELIM:
        raise ValueError(
            f"Invalid input at position {token.start}: expected {TokenType.NODE_DELIM}, found {token}"
        )
    token, properties = _properties(next(tokens), tokens)
    return token, SgfTree(properties)


def _tree(token: Token, tokens: TokenStream):
    if token.type != TokenType.TREE_START:
        raise ValueError(
            f"Invalid input at position {token.start}: expected {TokenType.TREE_START}, found {token}"
        )

    root = None
    token = next(tokens)

    while token.type != TokenType.TREE_END:
        if token.type == TokenType.NODE_DELIM:
            token, node = _node(token, tokens)
            if root is None:
                root = node
            else:
                root.children.append(node)
        elif token.type == TokenType.TREE_START:
            assert root is not None
            root.children.append(_tree(token, tokens))
            token = next(tokens)
        else:
            raise ValueError(
                f"Invalid input at position {token.start}: expected one of ({TokenType.NODE_DELIM}, {TokenType.TREE_START}), found {token}"
            )
    return root


def parse(input_string: str):
    """Parse an ``SGFTree`` from ``input_string``

    Uses recursive-descent parsing to build an ``SGFTree`` *starting* at the
    root node.
    """
    if not input_string:
        raise ValueError("Empty input")

    tokens = tokenise(input_string)

    try:
        root = _tree(next(tokens), tokens)
    except StopIteration as e:
        raise ValueError("Incomplete SGF tree definition") from e

    if root is None:
        raise ValueError("Incomplete SGF tree definition")

    return root
