import unittest

from sgf_parsing import Peekable, SgfTree, Token, TokenType, parse, tokenise

# Tests adapted from `problem-specifications//canonical-data.json` @ v1.2.0


class SgfParsingTest(unittest.TestCase):
    def test_empty_input(self):
        input_string = ""
        with self.assertRaisesWithMessage(ValueError):
            parse(input_string)

    def test_tree_with_no_nodes(self):
        input_string = "()"
        with self.assertRaisesWithMessage(ValueError):
            parse(input_string)

    def test_node_without_tree(self):
        input_string = ";"
        with self.assertRaisesWithMessage(ValueError):
            parse(input_string)

    def test_node_without_properties(self):
        input_string = "(;)"
        expected = SgfTree()
        self.assertEqual(parse(input_string), expected)

    def test_single_node_tree(self):
        input_string = "(;A[B])"
        expected = SgfTree(properties={"A": ["B"]})
        self.assertEqual(parse(input_string), expected)

    def test_multiple_properties(self):
        input_string = "(;A[b]C[d])"
        expected = SgfTree(properties={"A": ["b"], "C": ["d"]})
        self.assertEqual(parse(input_string), expected)

    def test_properties_without_delimiter(self):
        input_string = "(;A)"
        with self.assertRaisesWithMessage(ValueError):
            parse(input_string)

    def test_all_lowercase_property(self):
        input_string = "(;a[b])"
        with self.assertRaisesWithMessage(ValueError):
            parse(input_string)

    def test_upper_and_lowercase_property(self):
        input_string = "(;Aa[b])"
        with self.assertRaisesWithMessage(ValueError):
            parse(input_string)

    def test_two_nodes(self):
        input_string = "(;A[B];B[C])"
        expected = SgfTree(properties={"A": ["B"]}, children=[SgfTree({"B": ["C"]})])
        self.assertEqual(parse(input_string), expected)

    def test_two_child_trees(self):
        input_string = "(;A[B](;B[C])(;C[D]))"
        expected = SgfTree(
            properties={"A": ["B"]},
            children=[SgfTree({"B": ["C"]}), SgfTree({"C": ["D"]})],
        )
        self.assertEqual(parse(input_string), expected)

    def test_multiple_property_values(self):
        input_string = "(;A[b][c][d])"
        expected = SgfTree(properties={"A": ["b", "c", "d"]})
        self.assertEqual(parse(input_string), expected)

    def test_escaped_property(self):
        input_string = "(;A[\\]b\nc\nd\t\te \n\\]])"
        expected = SgfTree(properties={"A": ["]b\nc\nd  e \n]"]})
        self.assertEqual(parse(input_string), expected)

    # Utility functions
    def assertRaisesWithMessage(self, exception):
        return self.assertRaisesRegex(exception, r".+")


class TokenisationTest(unittest.TestCase):
    def test_Peekable(self):
        s = "abc"
        it = Peekable(iter(s))

        self.assertEqual(it.peek(), "a")
        self.assertEqual(it.peek(), "a")
        self.assertEqual(next(it), "a")

        self.assertEqual(it.peek(), "b")
        self.assertEqual(it.peek(), "b")
        self.assertEqual(next(it), "b")

        self.assertEqual(it.peek(), "c")
        self.assertEqual(next(it), "c")

        with self.assertRaises(StopIteration):
            next(it)

    def test_tokenise(self):
        test_cases = [
            (
                "[]();",
                [
                    Token(TokenType.VALUE_START, "[", 0, 1),
                    Token(TokenType.VALUE_END, "]", 1, 2),
                    Token(TokenType.TREE_START, "(", 2, 3),
                    Token(TokenType.TREE_END, ")", 3, 4),
                    Token(TokenType.NODE_DELIM, ";", 4, 5),
                ],
            ),
            (
                "A(b;",
                [
                    Token(TokenType.TEXT, "A", 0, 1),
                    Token(TokenType.TREE_START, "(", 1, 2),
                    Token(TokenType.TEXT, "b", 2, 3),
                    Token(TokenType.NODE_DELIM, ";", 3, 4),
                ],
            ),
            (
                "Ab(cDeF;",
                [
                    Token(TokenType.TEXT, "Ab", 0, 2),
                    Token(TokenType.TREE_START, "(", 2, 3),
                    Token(TokenType.TEXT, "cDeF", 3, 7),
                    Token(TokenType.NODE_DELIM, ";", 7, 8),
                ],
            ),
            (
                "Ab(cDeF\\];",
                [
                    Token(TokenType.TEXT, "Ab", 0, 2),
                    Token(TokenType.TREE_START, "(", 2, 3),
                    Token(TokenType.TEXT, "cDeF]", 3, 9),
                    Token(TokenType.NODE_DELIM, ";", 9, 10),
                ],
            ),
        ]
        for input_string, tokens in test_cases:
            with self.subTest(input_string=input_string):
                self.assertListEqual(list(tokenise(iter(input_string))), tokens)


if __name__ == "__main__":
    unittest.main()
