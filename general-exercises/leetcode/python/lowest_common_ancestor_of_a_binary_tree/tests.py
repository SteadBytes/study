from .solution import TreeNode, Solution
import pytest


@pytest.fixture
def example_tree():
    return TreeNode(
        3,
        left=TreeNode(
            5, left=TreeNode(6), right=TreeNode(2, left=TreeNode(7), right=TreeNode(4))
        ),
        right=TreeNode(1, left=TreeNode(0), right=TreeNode(8)),
    )


def test_example_1(example_tree):
    p = example_tree.left
    q = example_tree.right
    lca = example_tree

    assert Solution().lowestCommonAncestor(example_tree, p, q) is lca


def test_example_2(example_tree):
    p = lca = example_tree.left
    q = example_tree.left.right.right

    assert Solution().lowestCommonAncestor(example_tree, p, q) is lca

