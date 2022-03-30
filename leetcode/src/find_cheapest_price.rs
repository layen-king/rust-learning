/// ## K 站中转内最便宜的航班
/// ### 中等

/// ### 有 n 个城市通过一些航班连接。给你一个数组 flights ，其中 flights\[i\] = \[fromi, toi, pricei\] ，表示该航班都从城市 fromi 开始，以价格 pricei 抵达 toi。

/// ### 现在给定所有的城市和航班，以及出发城市 src 和目的地 dst，你的任务是找到出一条最多经过 k 站中转的路线，使得从 src 到 dst 的 价格最便宜 ，并返回该价格。 如果不存在这样的路线，则输出 -1。

/// ###

/// ### 示例 1：

/// ### 输入:
/// ### n = 3, edges = [[0,1,100],[1,2,100],[0,2,500]]
/// ### src = 0, dst = 2, k = 1
/// ### 输出: 200
/// ### 解释:
/// ### 城市航班图如下

/// ### 从城市 0 到城市 2 在 1 站中转以内的最便宜价格是 200，如图中红色所示。
/// ### 示例 2：

/// ### 输入:
/// ### n = 3, edges = [[0,1,100],[1,2,100],[0,2,500]]
/// ### src = 0, dst = 2, k = 0
/// ### 输出: 500
/// ### 解释:
/// ### 城市航班图如下

/// ### 从城市 0 到城市 2 在 0 站中转以内的最便宜价格是 500，如图中蓝色所示。
/// ###

/// ### 提示：

/// ### 1 <= n <= 100
/// ### 0 <= flights.length <= (n * (n - 1) / 2)
/// ### flights[i].length == 3
/// ### 0 <= fromi, toi < n
/// ### fromi != toi
/// ### 1 <= pricei <= 104
/// ### 航班没有重复，且不存在自环
/// ### 0 <= src, dst, k < n
/// ### src != dst

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/cheapest-flights-within-k-stops
pub fn find_cheapest_price(n:i32,flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let inf = 100 * 10000 + 1i32;
    let n = n as usize;
    let mut map = vec![inf; n];
    let src = src.clone() as usize;
    map[src] = 0;
    let mut ans = inf;
    for _ in 0..k + 1 {
        let mut tem = vec![inf; n];
        for flight in flights.iter() {
            let s = flight.get(0).unwrap();
            let d = flight.get(1).unwrap();
            let c = flight.get(2).unwrap();
            tem[*d as usize] = (tem[*d as usize] as i32).min(map[*s as usize] + (*c));
        }
        let tem1 = tem[dst as usize];
        map = tem;
        ans = ans.min(tem1);
    }
    if ans == inf {
       return -1;
    } else {
       return ans;
    };
}

#[test]
fn test_find_cheapest_price() {
    let mut map = vec![];
    map.push(vec![0, 1, 100]);
    map.push(vec![1, 2, 100]);
    map.push(vec![0, 2, 500]);
    map.push(vec![2, 3, 100]);
    let result = find_cheapest_price(4,map, 0, 3, 2);
    println!("result: {}", result);
}
