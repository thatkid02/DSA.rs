#include <iostream>

using namespace std;

#include "code150/lowest_common_binary.cpp"

int main() {
  Solution sol;

  TreeNode* root = new TreeNode(6);
  TreeNode* p = new TreeNode(2);
  TreeNode* q = new TreeNode(8);
  root->left = p;
  root->right = q;
  p->left = new TreeNode(0);
  p->right = new TreeNode(4);
  p->right->left = new TreeNode(3);
  p->right->right = new TreeNode(5);
  q->left = new TreeNode(7);
  q->right = new TreeNode(9);

  TreeNode* ancestor = sol.lowestCommonAncestor(root, p, q);
  if (ancestor) {
    cout << "Lowest Common Ancestor: " << ancestor->val << endl;
  } else {
    cout << "No Common Ancestor found." << endl;
  }

  return 0;
}
