/// ## 组合总和

/// ### 中等

/// ### 给定一个无重复元素的正整数数组 candidates 和一个正整数 target ，找出 candidates 中所有可以使数字和为目标数 target 的唯一组合。

/// ### candidates 中的数字可以无限制重复被选取。如果至少一个所选数字数量不同，则两种组合是唯一的。

/// ### 对于给定的输入，保证和为 target 的唯一组合数少于 150 个。

/// ###

/// ### 示例 1：

/// ### 输入: candidates = [2,3,6,7], target = 7
/// ### 输出: [[7],[2,2,3]]
/// ### 示例 2：

/// ### 输入: candidates = [2,3,5], target = 8
/// ### 输出: [[2,2,2,2],[2,3,3],[3,5]]
/// ### 示例 3：

/// ### 输入: candidates = [2], target = 1
/// ### 输出: []
/// ### 示例 4：

/// ### 输入: candidates = [1], target = 1
/// ### 输出: [[1]]
/// ### 示例 5：

/// ### 输入: candidates = [1], target = 2
/// ### 输出: [[1,1]]
/// ###

/// ### 提示：

/// ### 1 <= candidates.length <= 30
/// ### 1 <= candidates[i] <= 200
/// ### candidate 中的每个元素都是独一无二的。
/// ### 1 <= target <= 500
/// ### 通过次数333,756提交次数458,747

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/combination-sum
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut v = candidates;
    v.sort();
    let mut res = vec![];
    let mut path = vec![];
    back_tracking(&mut res, &mut path, &v, 0, 0, target);
    res
}

fn back_tracking(
    res: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
    v: &Vec<i32>,
    j: i32,
    mut sum: i32,
    target: i32,
) {
    if sum > target {
        return;
    }
    if sum == target {
        res.push(path.clone());
        return;
    }
    let j = j as usize;
    for i in j..v.len() {
        if v[i] > target - sum {
            continue;
        }
        path.push(v[i]);
        sum = sum + v[i];
        back_tracking(res, path, v, i as i32, sum, target);
        path.pop();
        sum = sum - v[i];
    }
}

#[test]
fn test_combination_sum() {
    let result = combination_sum(vec![2, 3, 6, 7], 7);
    println!("result:{:?}", result)
}
