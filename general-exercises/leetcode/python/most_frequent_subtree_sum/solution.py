from __future__ import annotations

from collections import defaultdict
from typing import DefaultDict, List, Optional, TypeVar


class TreeNode:
    def __init__(
        self,
        x: int = 0,
        left: Optional[TreeNode] = None,
        right: Optional[TreeNode] = None,
    ):
        self.val: int = x
        self.left: Optional[TreeNode] = left
        self.right: Optional[TreeNode] = right


class Solution:
    def findFrequentSubtreeSum(self, root: TreeNode) -> List[int]:
        """
        Recursive definition:

        - Let ``val(n)`` be a function that returns the value of a given node ``n``
        - The subtree sum of a node is it's value plus the sum of the subtrees
          rooted at each child:

            subtree_sum(n) = val(n) + sum(subtree_sum(c) for c in children(n))

        - The subtree sum of a leaf node ``l`` is it's value:

            subtree_sum(l) = val(n)

        Assumptions:

        - The sum of values in any subtree is within the range of a 32-bit signed
          integer


        Complexity:

        Since a **full** traversal of the tree is required, this algorithm is
        ``O(n)`` where ``n`` is the number of nodes.

        Since the problem provides no guarantees about the *distribution* of
        nodes within the tree (e.g. balanced, complete e.t.c.), the degenerate
        tree must be considered the worst case; for each parent node there is
        only a single child. In this case, the height of tree is ``O(n)``,
        where ``n`` is the number of nodes.

        Although this solution **does pass** the LeetCode test cases, it is not
        suitable (in Python at least) for all possible inputs according to the
        guarantees provided by the problem specification. We know that the
        sum of values in any subtree is within the range of a 32-bit signed
        integer, a concrete upper bound of ``2^31`` can be determined for the
        maximum height of the tree:

            - Range of 32-bit signed integer = ``-2^31`` through ``2^31 - 1``.

            - A subtree sum having the minimum value of this range with the
              most possible nodes can be achieved with a degenerate tree where
              all nodes have a value of ``-1``.

        This leads to a tree height of ``2^31`` which is far greater than the
        maximum recursion limit in Python (1000 by default).
        """
        if not root:
            return []

        sub_sums: DefaultDict[int, int] = defaultdict(int)

        def recur(root):
            this_sum = root.val

            if root.left:
                this_sum += recur(root.left)
            if root.right:
                this_sum += recur(root.right)

            sub_sums[this_sum] += 1
            return this_sum

        recur(root)

        # This additional loop could be avoided by tracking the maximum frquency
        # throughout the recursion.
        max_freq = max(sub_sums.values())
        return [sub_sum for sub_sum, freq in sub_sums.items() if freq == max_freq]
