mod code150;

fn main() {
    use code150::two_sum::Solution;
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("Output: {:?}\n", result);

}
