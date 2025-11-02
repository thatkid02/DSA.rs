#include <iostream>

using namespace std;

#include "code150/diameter_of_binary_tree.cpp"

int main() {
  Solution sol;

  TreeNode* root = new TreeNode(1);
  root->left = new TreeNode(2);
  root->right = new TreeNode(3);
  root->left->left = new TreeNode(4);
  root->left->right = new TreeNode(5);

  int result = sol.diameterOfBinaryTree(root);

  cout << "res: " << result << endl;

  delete root->left->left;
  delete root->left->right;
  delete root->left;
  delete root->right;
  delete root;

  return 0;
}
