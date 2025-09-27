use std::
    {
        cmp::Ordering,
        collections::HashSet
    };

pub struct Solution;

impl Solution
{
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>>
    {
        nums.sort();
        let mut res = HashSet::<Vec<i32>>::new();
        
        for i in 0..nums.len()
        {
            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k
            {
                match (nums[i] + nums[j] + nums[k]).cmp(&0)
                {
                    Ordering::Equal =>
                        {
                            res.insert(vec![nums[i], nums[j], nums[k]]);
                            j += 1;
                            k -= 1;
                        },
                    Ordering::Less => j += 1,
                    Ordering::Greater => k -= 1
                }
            }
        }

        res.into_iter().collect::<Vec<Vec<i32>>>()
    }
}