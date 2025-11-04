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

#include "code150/same_tree.cpp"

int main() {
  Solution sol;

  TreeNode* p1 = new TreeNode(1);
  TreeNode* p2 = new TreeNode(1);
  TreeNode* p3 = new TreeNode(2);
  TreeNode* p4 = new TreeNode(2);
  p1->left = p2;
  p1->right = p3;
  p4->left = p2;
  p4->right = p3;

  bool result = sol.isSameTree(p1, p4);
  cout << "Trees are the same: " << (result ? "true" : "false") << endl;

  return 0;
}
