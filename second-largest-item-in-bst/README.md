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
- *O(n)*
