import pytest


class BinaryTreeNode(object):
    def __init__(self, value):
        self.value = value
        self.left = None
        self.right = None

    def insert_left(self, value):
        self.left = BinaryTreeNode(value)
        return self.left

    def insert_right(self, value):
        self.right = BinaryTreeNode(value)
        return self.right


def insert(root, node):
    if root is None:
        return node

    if root.value > node.value:
        root.left = insert(root.left, node)
    else:
        root.right = insert(root.right, node)
    return root


def build_tree(values):
    r = BinaryTreeNode(values[0])
    for v in values[1:]:
        insert(r, BinaryTreeNode(v))
    return r


def in_order(root):
    if root:
        yield from in_order(root.left)
        yield root.value
        yield from in_order(root.right)


def largest(root):
    node = root
    while node:
        if node.right is None:
            return node.value
        node = node.right


def second_largest(root):
    node = root
    while node:
        # largest node has left subtree
        # second largest -> largest node in left subtree
        if node.left and node.right is None:
            return largest(node.left)
        # largest node has no children
        # second largest -> parent of largest node
        if node.right and node.right.right is None and node.right.left is None:
            return node.value
        node = node.right


def test_insert_single_level():
    r = BinaryTreeNode(5)
    a = BinaryTreeNode(7)
    b = BinaryTreeNode(3)
    insert(r, a)
    insert(r, b)

    assert r.left == b
    assert r.right == a


def test_insert_two_levels():
    r = BinaryTreeNode(5)
    a = BinaryTreeNode(7)
    b = BinaryTreeNode(3)
    c = BinaryTreeNode(1)
    d = BinaryTreeNode(15)
    insert(r, a)
    insert(r, b)
    insert(r, c)
    insert(r, d)

    assert r.left == b
    assert r.right == a
    assert b.left == c
    assert a.right == d


@pytest.mark.parametrize(
    "values",
    [[4, 2, 1, 3, 5, 6], [5, 7, 3, 1, 15], [1, 2, 10, 5, 3, 20], [10, -5, 0, 1, 6]],
)
def test_largest(values):
    r = build_tree(values)
    assert largest(r) == max(values)


@pytest.mark.parametrize(
    "values,expected",
    [
        ([4, 2, 1, 3, 5, 6], 5),
        ([5, 7, 3, 1, 15], 7),
        ([1, 2, 10, 5, 3, 20], 10),
        ([10, -5, 0, 1, 6], 6),
    ],
)
def test_second_largest(values, expected):
    r = build_tree(values)
    assert second_largest(r) == expected
