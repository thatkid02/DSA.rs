#include <iostream>

using namespace std;

#include "code150/subtree_another_tree.cpp"

int main() {
  Solution sol;

  TreeNode* s1 = new TreeNode(3);
  s1->left = new TreeNode(4);
  s1->right = new TreeNode(5);
  s1->left->left = new TreeNode(1);
  s1->left->right = new TreeNode(2);
  s1->left->right->left = new TreeNode(0);

  TreeNode* t1 = new TreeNode(4);
  t1->left = new TreeNode(1);
  t1->right = new TreeNode(2);  
  cout << sol.isSubtree(s1, t1) << endl;

  return 0;
}
