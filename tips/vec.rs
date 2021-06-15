/// 向量,双端队列，链表，映射表，集合，优先队列，指针
pub fn run_vec() {
  run_binary_heap();
  run_set();
  vec_run();
  vec_deque_run();
  list_run();
  map_run()
}

/// 向量vec
fn vec_run() {
  let mut v1 = vec![];
  v1.push(1);
  v1.push(2);
  for (i, el) in v1.iter().enumerate() {
      println!("The current element is {}", el);
      println!("The current index is {}", i);
  }
  // 创建多个
  let mut v2 = vec![1; 10];
  loop {
      match v2.pop() {
          Some(x) => println!("v2 is {}", x),
          None => break,
      }
  }

  let mut v3: Vec<i32> = Vec::new();
  v3.push(11);
  v3.push(12);
  while let Some(v3) = v3.pop() {
      println!("v3 is {}", v3);
  }
}

use std::collections::VecDeque;
/// 线性序列:双端队列
fn vec_deque_run() {
  let mut buf = VecDeque::new();
  buf.push_front(1); //新进后出,正面压入
  buf.push_front(2);
  buf.push_back(3); // 新进先出,最后压入
  buf.push_back(4);
  buf.push_back(5);
  assert_eq!(buf.get(0), Some(&2)); // 使用get获取元素, Option类型,Some or None
  for (i, el) in buf.iter().enumerate() {
      println!("current i is: {}, current el is :{}", i, el)
  }
}

use std::collections::LinkedList;
/// 线性序列:链表
fn list_run() {
  let mut list1 = LinkedList::new();
  list1.push_back('a');
  list1.push_back('b');
  let mut list2 = LinkedList::new();
  list2.push_back('c');
  list1.append(&mut list2);

  println!("{:?}", list1);
}

use std::collections::BTreeMap;
use std::collections::HashMap;
/// hasMap和bTreeMap
fn map_run() {
  let mut hash_map = HashMap::new();
  let mut btree_map = BTreeMap::new();
  hash_map.insert('c', 'c');
  hash_map.insert('b', 'b');
  hash_map.insert('a', 'a');
  btree_map.insert(1, 'a');
  btree_map.insert(2, 'b');
  btree_map.insert(3, 'c');
  let str = hash_map.get(&'a');
  println!("str is {:?}", str);
  println!("has_map is :{:?}", hash_map); // has_map没有顺序
  println!("btree_map is :{:?}", btree_map); // btree_map 有顺序
}

use std::collections::BTreeSet;
use std::collections::HashSet;

/// 集合:hashSet 和 BTreeSet
fn run_set() {
  let mut has_set = HashSet::new();
  let mut btree_set = BTreeSet::new();
  has_set.insert("layen");
  has_set.insert("king");
  if has_set.contains("layen") {
      println!("layen is in {:?}", has_set);
  }
  btree_set.insert("btree_set");
  println!("btree_set:{:?}", btree_set)
}

use std::collections::BinaryHeap;
/// 优先队列:binary heap ；peak 取最大堆
fn run_binary_heap() {
  let mut heap = BinaryHeap::new();
  println!("heap is {:?}", heap);
  let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 100];
  for &i in arr.iter() {
      heap.push(i);
  }
  println!("heap is {:?},arr is : {:?}", heap, arr);
  let max = heap.peek();
  println!("max is {:?}", max);
}
