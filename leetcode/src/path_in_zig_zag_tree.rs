// 二叉树寻路

// 在一棵无限的二叉树上，每个节点都有两个子节点，树中的节点 逐行 依次按 “之” 字形进行标记。

// 如下图所示，在奇数行（即，第一行、第三行、第五行……）中，按从左到右的顺序进行标记；

// 而偶数行（即，第二行、第四行、第六行……）中，按从右到左的顺序进行标记

// 给你树上某一个节点的标号 label，请你返回从根节点到该标号为 label 节点的路径，该路径是由途经的节点标号所组成的。

//

// 示例 1：

// 输入：label = 14
// 输出：[1,3,4,14]
// 示例 2：

// 输入：label = 26
// 输出：[1,2,6,10,26]
pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
    let mut ret = vec![label];
    let mut value: i32 = label;
    while value != 1 {
        value = (value >> 1) ^ (1 << format!("{:b}", (value >> 1)).chars().count() - 1) - 1;
        ret.push(value);
    }
    ret.reverse();
    ret
}

#[test]
fn test_path_in_zig_zag_tree() {
    let result = path_in_zig_zag_tree(14);
    assert_eq!(result, vec![1, 3, 4, 14])
}
