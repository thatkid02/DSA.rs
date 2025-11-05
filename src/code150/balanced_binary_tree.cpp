#include <algorithm>
#include <math.h>
using namespace std;

struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right) : val(x), left(left), right(right) {}
};

class Solution {
public:
    bool isBalanced(TreeNode* root) {
        int height = 0;
        return checkBalance(root, height);
    }
    
    bool checkBalance(TreeNode* root, int& height) {
        if (root == NULL) {
            height = 0;
            return true;
        }
        int leftHeight = 0;
        int rightHeight = 0;
        if (!checkBalance(root->left, leftHeight)) return false;
        if (!checkBalance(root->right, rightHeight)) return false;
        if ( abs(leftHeight - rightHeight) > 1 ) return false;
        height = max(leftHeight, rightHeight) + 1;
        return true;
    }
};