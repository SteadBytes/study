from __future__ import annotations

from typing import Generic, Iterator, Optional, TypeVar

T = TypeVar("T")


class TreeNode(Generic[T]):
    def __init__(
        self,
        x: T,
        left: Optional[TreeNode[T]] = None,
        right: Optional[TreeNode[T]] = None,
    ):
        self.val: T = x
        self.left: Optional[TreeNode[T]] = left
        self.right: Optional[TreeNode[T]] = right

    def __str__(self):
        return f"TreeNode({self.val}, left={self.left}, right={self.right})"

    def __repr__(self):
        return f"TreeNode({self.val}, left={repr(self.left)}, right={repr(self.right)})"


class Solution:
    def lowestCommonAncestor(
        self, root: TreeNode, p: TreeNode, q: TreeNode
    ) -> TreeNode:
        """Find the Lowest Common Ancestor (LCA) of ``p`` and ``q`` in the tree
        rooted at ``root``.

        LCA is found at the first intersection of the paths from ``p`` and
        ``q`` to ``root``. Starting from ``root``, this solution recursively
        searches left *and* right subtrees for ``p`` and ``q``. At any level
        of the search, there are 3 possible cases:

            1. The current node *is* ``p`` or ``q``.

            2. Both ``p`` and ``q`` are found in the left/right subtrees of the
               the current node.

            3. One of ``p`` and ``q`` are found in either the left/right
               subtree of the current node.

        In case 1, the current search branch of the recursion tree is complete
        and the current node is returned.

        In case 2, both the left and right subtree searches successfully found
        ``p`` and ``q``, therefore the LCA *must* be the current node.

        In case 3, the node that was not found in the subtree searches must
        be *below* the node that was found, therefore the found node is the
        LCA.

        This recursion is an ``O(n)`` solution, where the worst case arises
        from ``p`` and ``q`` being leaf nodes in an unbalanced tree e.g
        ``p`` = 6 and ``q``= 2 in the following tree:

                                       3
                                        5
                                         1
                                          4
                                         6 2

        Note: This algorithm relies on the assumptions given in the problem
        description and is therfore not (at least without modification)
        applicable to a more general version of the problem:

        - All of the nodes' values will be unique.

        - ``p`` and ``q`` are different and both values will exist in the
          binary tree.
        """
        # TODO: Non-recursive implementation
        def search(node) -> Optional[TreeNode]:

            # Case 1
            if node is p or node is q:
                return node

            # Search for p and q in left/right subtrees
            l = search(node.left) if node.left else None
            r = search(node.right) if node.right else None

            # Case 2
            if l and r:
                return node

            # Case 3
            return l or r

        lca = search(root)
        assert lca
        return lca
