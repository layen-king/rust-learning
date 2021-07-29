use std::collections::HashMap;
fn dp(coins: &Vec<usize>, n: i32, coins_map: &mut HashMap<usize, i32>) -> i32 {
    if n == 0 {
        return 0;
    }
    if n < 0 {
        return -1;
    }
    if !coins_map.get(&(n as usize)).is_none() {
        return *coins_map.get(&(n as usize)).unwrap();
    }
    let mut res = i32::MAX;
    for coin in coins {
        let result = dp(coins, n as i32 - *coin as i32, coins_map);
        if result == -1 {
            continue;
        }
        res = i32::min(res, result + 1);
    }
    let res = if res == i32::MAX { -1 } else { res };
    coins_map.insert(n as usize, res);
    return *coins_map.get(&(n as usize)).unwrap();
}

/// ## 动态规划凑硬币,自顶而下
pub fn coin_change(coins: Vec<usize>, i: usize) -> i32 {
    let mut coins_map: HashMap<usize, i32> = HashMap::new();
    dp(&coins, i as i32, &mut coins_map)
}

/// 从小到大迭代
pub fn coin_change_1(coins: Vec<usize>, i: usize) -> i32 {
    let mut dp = vec![i + 1, i + 1];
    dp[0] = 0;
    for k in 0..=i {
        for coin in &coins {
            if k < *coin {
                continue;
            }
            dp[k] = dp[k].min(1 + dp[k - coin]);
        }
    }
    if dp[i] == i + 1 {
        return -1 as i32;
    } else {
        dp[i] as i32
    }
}

#[test]
fn test_dp() {
    let coins = vec![1, 2, 5];
    let result = coin_change(coins, 11);
    assert_eq!(3, result);

    let coins = vec![1, 2, 5];
    let result = coin_change_1(coins, 11);
    assert_eq!(3, result);
}
