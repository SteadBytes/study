import pytest

from .solution import Solution, TreeNode


def test_example_1():
    root = TreeNode(5, left=TreeNode(2), right=TreeNode(-3))

    assert Solution().findFrequentSubtreeSum(root) == [2, -3, 4]


def test_example_2():
    root = TreeNode(5, left=TreeNode(2), right=TreeNode(-5))

    assert Solution().findFrequentSubtreeSum(root) == [2]


def test_worst_case_reaches_recursion_limit():
    """
    Demonstration of hitting recursion limit for the worst case tree depth
    """
    MAX_NODES = 2 ** 31

    class LazyTreeNode:
        """
        Lazily build a degenerate tree where all values are ``-1`` up to a
        maximum depth of ``MAX_NODES``. This allows a very large tree to be
        generated without building the entire structure before beginning the
        traversal.
        """

        def __init__(self, val, node_number):
            self.val = val

            self._n = node_number
            self._left = None

            self.right = None

            self._left_eval = False

        @property
        def left(self):
            if self._n < MAX_NODES and not self._left_eval:
                self._left = LazyTreeNode(-1, self._n + 1)
                self._left_eval = True

            return self._left

    root = LazyTreeNode(-1, 0)
    with pytest.raises(RecursionError):
        Solution().findFrequentSubtreeSum(root)
