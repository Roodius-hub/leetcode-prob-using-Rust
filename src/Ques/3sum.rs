pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let n = nums.len(); // len
    let mut result = Vec::new();

    if n < 3 {
        return result;
    }

    for i in 0..(n - 2) {
        // array  now is sorted if first element greater then 0 break it
        if nums[i] > 0 {
            break;
        }

        //Skip Duplicates for First Element
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        // two pointer
        let mut j = i + 1;
        let mut k = n - 1;

        // until they collapse
        while j < k {
            let x = nums[i] + nums[j] + nums[k];
            if x < 0 {
                j += 1;
            } else if x > 0 {
                k -= 1;
            } else {
                result.push(vec![nums[i], nums[j], nums[k]]);
                //Move both pointers
                j += 1;
                k -= 1;

                // Skip duplicate values for the second number
                while j < k && nums[j] == nums[j - 1] {
                    j += 1;
                }

                // Skip duplicate values for the third number
                while j < k && nums[k] == nums[k + 1] {
                    if k == n - 1 {
                        break;
                    }
                    k -= 1;
                }
            }
        }
    }

    result
}

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];

    let ans:Vec<Vec<i32>> = three_sum(nums);

    for inner in &ans {
        for val  in inner {
            print!("{} , ", val);
        };
    }
}
