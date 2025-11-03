#include <iostream>

using namespace std;

struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right) : val(x), left(left), right(right) {}
};

#include "code150/balanced_binary_tree.cpp"

int main() {
  Solution sol;

  TreeNode* node7 = new TreeNode(7);
  TreeNode* node15 = new TreeNode(15);
  TreeNode* node20 = new TreeNode(20, node15, node7);
  TreeNode* node9 = new TreeNode(9);
  TreeNode* root1 = new TreeNode(3, node9, node20);

  bool result1 = sol.isBalanced(root1);
  cout << "Tree [3,9,20,null,null,15,7] is balanced: " << (result1 ? "true" : "false") << endl;

  TreeNode* node4a = new TreeNode(4);
  TreeNode* node4b = new TreeNode(4);
  TreeNode* node3a = new TreeNode(3, node4a, node4b);
  TreeNode* node3b = new TreeNode(3);
  TreeNode* node2a = new TreeNode(2, node3a, node3b);
  TreeNode* node2b = new TreeNode(2);
  TreeNode* root2 = new TreeNode(1, node2a, node2b);

  bool result2 = sol.isBalanced(root2);
  cout << result2 ? "true" : "false") << endl;

  return 0;
}
