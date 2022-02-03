struct Solution {}

impl Solution {
    //offer 10 https://leetcode-cn.com/problems/qing-wa-tiao-tai-jie-wen-ti-lcof
    pub fn num_ways(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;
        let mut i = 1;
        while i <= n {
            let mut tmp = b;
            // a = b;
            b = a + b;
            a = tmp;
            i = i + 1;
        }
        b
    }
}

fn main_num_ways() {
    let r = Solution::num_ways(4);
    println!("res is {}", r);
}

fn quick_sort(nums: &mut Vec<i32>, l: usize, r: usize) {
    if l >= r {
        return;
    }
    let mut i = l;
    let mut j = r;
    while i < j {
        while i < j && nums[j] >= nums[l] { j -= 1; };
        while i < j && nums[i] < nums[l] { i += 1; }
        nums.swap(i, j);
    }
    nums.swap(l, i);
    if i > 1 {
        quick_sort(nums, l, i - 1);
    }

    quick_sort(nums, i + 1, r);
}

fn main_quick_sort() {
    let mut v = vec![4, 2, 8, 5, 6, 1];
    let right = v.len() - 1;
    quick_sort(&mut v, 0, right);
    println!("sorted v is {:?}", v);
}

fn main() {}