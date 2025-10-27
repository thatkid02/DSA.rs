mod code150;

fn main() {
    use code150::find_duplicate::Solution;

    let nums = vec![1, 3, 4, 2, 2];
    let result = Solution::find_duplicate(nums);
    println!("Output: {:?}", result);
}
