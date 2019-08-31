# Second Largest Item in a Binary Search Tree

[Source](https://www.interviewcake.com/question/python3/second-largest-item-in-bst)

Write a function to find the 2nd largest element in a _binary search tree_.

Sample binary tree node class:

```python
class BinaryTreeNode(object):

    def __init__(self, value):
        self.value = value
        self.left  = None
        self.right = None

    def insert_left(self, value):
        self.left = BinaryTreeNode(value)
        return self.left

    def insert_right(self, value):
        self.right = BinaryTreeNode(value)
        return self.right
```

## Notes

Simple solution:

- In-order traversal -> return second to last item
- _O(n)_

Improved:

- Second largest will be the node (in-order) previous to the largest:
  - Case 1: Largest node has a left subtree -> 2nd largest is largest node in left subtree
  - Case 2: Largest node has no children -> 2nd largest is parent of largest node
- From root, follow right pointers
  - If largest node is reached (no right node) and it has a left node return the largest node of the left subtree
  - If largest node not reached (has a right node) and the right node has no left or right pointers return current node value
- _O(h)_ where _h_ = height of tree
