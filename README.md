# Build your own blockchain

## 1. Implement a binary trie with integer keys

A trie is an ordered tree data structure where the keys are strings. Your task is to implement a trie data structure where each node has at most two children and the keys are of unsigned integer type.

Define a class `TrieNode` and add the following methods on the `TrieNode` class:

```
Insert(uint key, string value)
Get(uint key) string
```

The `Insert` function adds a key/value pair to the trie rooted at the `TrieNode`. If the key already exists, the new value should replace the old value. in the trie. The `Get` function returns the value for the given key or the empty string if the key is not in the trie node rooted at the `TrieNode`.

For example, inserting pairs `(4, "foo")` and `(2, "bar")` in a trie node should decode the integers `4` and `2` into their binary representations (least significant bit first): `[0, 0, 1]` and `[0, 1]` respectively. The resulting trie looks like the following ("`-`" represents a node with an empty value):

```
        -
       /
      -
    /   \
   -    "bar"
    \ 
     "foo"
```

## 2. Calculate the Merkle root

The Merkle Root of a trie is a hash that uniquely identifies any binary trie up to a hash collision.

The Merkle Root of a TrieNode is calculated recursively as follows:
- The Merkle Root of a null node is the hash of the empty string
- The Merkle Root of a leaf node is the hash of the value of the node
- The Merkle Root of an inner node is the is the hash of the concatenation of the following:
  - the hash of the node's value
  - the Merkle Root of the node's left child, and
  - the Merkle Root of the node's right child

For this task you are required to implement a method `MerkleRoot()` that returns the MerkleRoot of a TrieNode.

Any hashing algorithm can be used in this task.

Example of MerkleRoot:

The trie below has the following Merkle Root:
```
                      -
                    /   \
                  -      "foo"
                /   \
                    "bar"

hash(
   // this is the has of the value of the root node
  hash("") +

  // this is the hash of the Merkle Root of the left node
  hash(
    // this is the hash of the value of the node
    hash("") +

    // this is the Merkle Root of the node's left child,
    // which is a nil node
    hash("") +

    // this is the Merkle Root of the node's right child,
    // which is a leaf node
    hash("bar")
  ) +

 // this is the Merkle Root of the root's right node, which
 // is a leaf node
 hash ("foo")
)
```

## 3. Optimize Merkle Calculation

A common use-case is for the Merkle Trie data structure is to call the Insert, Get, and Merkle Root methods multiple times in any order. While Insert and Get run in O(1), the implementation of MerkleRoot runs in O(N), where N is the number of key:value pairs in the trie. However, when inserting a few keys into a large trie, the Merkle Hash for most of the inner nodes does not have to be recalculated.

For this task, you are required to cache data from the MerkleRoot calculation and optimize re-calculating the MerkleRoot.

For example, for any given TrieNode:
- If MerkleRoot is called two times in a row without any Insert calls in-between, the second call should complete in O(1).
- If MerkleRoot is called once, followed by K inserts followed by a second call to MerkleRoot, the second call should complete in O(K).

The Insert and Get methods should still have constant time complexity.

You may add additional member fields to the TrieNode class to complete this task. 

