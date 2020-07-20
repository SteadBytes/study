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



Grammar (EBNF)
==============

TODO: Fix this

sgf_tree ::= tree*
tree ::= "(" sequence tree* ")"
sequence ::= node+
node ::= ";" property*
property ::= key value+
key ::= upper_char [upper_char | digit]?
value ::= "[" text "]"
text ::= (char; "\\]" = "]", "\\\\" = "\\")*
"""

from typing import DefaultDict


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


class SourceStream:
    def __init__(self, input_string: str):
        self.src = input_string
        self.pos = -1

    def __next__(self) -> str:
        self.pos += 1
        return self.src[self.pos]

    def __iter__(self) -> Iterator[str]:
        return self


def _value(curr_token: str, stream: SourceStream, trans_tbl=WS_TRANS):
    if curr_token != VALUE_START:
        raise ValueError(
            f"Invalid input at position {stream.pos}: expected {VALUE_START}, found {curr_token}"
        )

    curr_token = next(stream)
    val_tokens = []
    while curr_token != VALUE_END:
        ch = next(stream) if curr_token == ESCAPE else curr_token
        val_tokens.append(ch)
        curr_token = next(stream)
    return "".join(val_tokens).translate(trans_tbl)


def _key(curr_token: str, stream: SourceStream):
    if not curr_token.isupper():
        raise ValueError(
            f"Invalid input at position {stream.pos}: expected upper case char, found {curr_token}"
        )

    key = curr_token
    curr_token = next(stream)

    if curr_token != VALUE_START:
        raise ValueError(
            f"Invalid input at position {stream.pos}: expected {VALUE_START}, found {curr_token}"
        )

    return (curr_token, key)


def _properties(curr_token: str, stream: SourceStream):
    properties = DefaultDict(list)
    while curr_token not in (TREE_START, TREE_END, NODE_DELIM):
        curr_token, key = _key(curr_token, stream)

        while curr_token == VALUE_START:
            val = _value(curr_token, stream)
            properties[key].append(val)
            curr_token = next(stream)

    return curr_token, properties


def _node(curr_token: str, stream: SourceStream):
    if curr_token != NODE_DELIM:
        raise ValueError(
            f"Invalid input at position {stream.pos}: expected {NODE_DELIM}, found {curr_token}"
        )
    curr_token, properties = _properties(next(stream), stream)
    return curr_token, SgfTree(properties)


def _tree(curr_token: str, stream: SourceStream):
    if curr_token != TREE_START:
        raise ValueError(
            f"Invalid input at position {stream.pos}: expected {TREE_START}, found {curr_token}"
        )

    root = None
    curr_token = next(stream)

    while curr_token != TREE_END:
        if curr_token == NODE_DELIM:
            curr_token, node = _node(curr_token, stream)
            if root is None:
                root = node
            else:
                root.children.append(node)
        elif curr_token == TREE_START:
            root.children.append(_tree(curr_token, stream))
            curr_token = next(stream)
        else:
            raise ValueError(
                f"Invalid input at position {stream.pos}: expected one of ({TREE_DELIM}, {TREE_START}), found {curr_token}"
            )
    return root


def parse(input_string):
    if not input_string:
        raise ValueError("Empty input")

    stream = SourceStream(input_string)

    try:
        root = _tree(next(stream), stream)
    except StopIteration as e:
        raise ValueError("Incomplete SGF tree definition") from e

    if root is None:
        raise ValueError("Incomplete SGF tree definition")

    return root
