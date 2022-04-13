use std::collections::HashMap;

pub struct RandomizedSet {
    nums: Vec<i32>,
    maps: HashMap<i32, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/// ## O(1) 时间插入、删除和获取随机元素

/// ### 中等

/// ### 实现RandomizedSet 类：

/// ### RandomizedSet() 初始化 RandomizedSet 对象
/// ### bool insert(int val) 当元素 val 不存在时，向集合中插入该项，并返回 true ；否则，返回 false 。
/// ### bool remove(int val) 当元素 val 存在时，从集合中移除该项，并返回 true ；否则，返回 false 。
/// ### int getRandom() 随机返回现有集合中的一项（测试用例保证调用此方法时集合中至少存在一个元素）。每个元素应该有 相同的概率 被返回。
/// ### 你必须实现类的所有函数，并满足每个函数的 平均 时间复杂度为 O(1) 。

/// ###  

/// ### 示例：

/// ### 输入
/// ### ["RandomizedSet", "insert", "remove", "insert", "getRandom", "remove", "insert", "getRandom"]
/// ### [[], [1], [2], [2], [], [1], [2], []]
/// ### 输出
/// ### [null, true, false, true, 2, true, false, 2]

/// ### 解释
/// ### RandomizedSet randomizedSet = new RandomizedSet();
/// ### randomizedSet.insert(1); /// ### 向集合中插入 1 。返回 true 表示 1 被成功地插入。
/// ### randomizedSet.remove(2); /// ### 返回 false ，表示集合中不存在 2 。
/// ### randomizedSet.insert(2); /// ### 向集合中插入 2 。返回 true 。集合现在包含 [1,2] 。
/// ### randomizedSet.getRandom(); /// ### getRandom 应随机返回 1 或 2 。
/// ### randomizedSet.remove(1); /// ### 从集合中移除 1 ，返回 true 。集合现在包含 [2] 。
/// ### randomizedSet.insert(2); /// ### 2 已在集合中，所以返回 false 。
/// ### randomizedSet.getRandom(); /// ### 由于 2 是集合中唯一的数字，getRandom 总是返回 2 。
/// ###  

/// ### 提示：

/// ### -231 <= val <= 231 - 1
/// ### 最多调用 insert、remove 和 getRandom 函数 2 * 105 次
/// ### 在调用 getRandom 方法时，数据结构中 至少存在一个 元素。
/// ### 通过次数50,617提交次数99,052

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://eetcode-cn.com/problems/insert-delete-getrandom-o1
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
impl RandomizedSet {
    #[allow(dead_code)]
    fn new() -> Self {
        RandomizedSet {
            nums: vec![],
            maps: HashMap::new(),
        }
    }
    #[allow(dead_code)]
    fn insert(&mut self, val: i32) -> bool {
        if self.maps.get(&val).is_some() {
            return false;
        }
        self.maps.insert(val, self.nums.len());
        self.nums.push(val);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(index) = self.maps.get(&val) {
            let &last = self.nums.last().unwrap();
            self.nums[*index] = last;
            *self.maps.get_mut(&last).unwrap() = *index;
            self.nums.pop().unwrap();
            self.maps.remove(&val);
            return true;
        }
        false
    }
    #[allow(dead_code)]
    fn get_random(&self) -> i32 {
        let index = rand::random::<usize>() % self.nums.len();
        self.nums[index]
    }
}

#[test]
fn test_randomi_zed_set() {
    let mut s = RandomizedSet::new();
    s.insert(100);
    s.insert(101);
    s.remove(102);
    s.remove(100);
    let n = s.get_random();
    assert_eq!(n, 102)
}
