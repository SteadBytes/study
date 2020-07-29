from __future__ import annotations
from itertools import dropwhile

from typing import Generic, Iterator, Optional, TypeVar, Dict

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
        ``q`` to ``root``. This solution constructs a mapping from each node
        to its parent in a Depth First Search. The paths from ``p`` and ``q``
        are then constructed by looking up parents in the mapping until the
        root is reached. The LCA is the first node in both of these paths.

        This is an ``O(n)`` solution, where the worst case arises from ``p``
        and ``q`` being leaf nodes in an unbalanced tree e.g ``p`` = 6 and
        ``q``= 2 in the following tree:

                                       3
                                        5
                                         1
                                          4
                                         6 2

       To be more specific, this is ``O(h)`` where ``h`` is the height of the
       tree (of which the worst case is ``O(n)``).

        Note: This algorithm relies on the assumptions given in the problem
        description and is therfore not (at least without modification)
        applicable to a more general version of the problem:

        - All of the nodes' values will be unique.

        - ``p`` and ``q`` are different and both values will exist in the
          binary tree.
        """

        stack = [root]
        parents: Dict[TreeNode, Optional[TreeNode]] = {root: None}

        # DFS to build mapping from nodes to parents. The search can be
        # terminated early once both p and q have been found as only the path
        # from these nodes to the root is required.
        while p not in parents or q not in parents:
            node = stack.pop()
            for child in filter(None, (node.left, node.right)):
                parents[child] = node
                stack.append(child)

        # Since node values are unique, the ordering of the path itself doesn't
        # matter - only the presence of a common ancestor node. So a set is
        # used here for fast membership tests.
        p_ancestors = set(self._construct_path(parents, p))

        # Find the first ancestor of q that is also an ancestor of p
        return next(
            dropwhile(lambda n: n not in p_ancestors, self._construct_path(parents, q))
        )

    def _construct_path(self, parents, start) -> Iterator[TreeNode]:
        node = start
        while node is not None:
            yield node
            node = parents[node]

