use std::{
    pin::Pin,
    task::{Context, Poll},
};

#[allow(unused)]
use futures::{channel::mpsc, executor::block_on, io, Future, SinkExt, StreamExt};

// `foo()`返回一个`Future<Output = u8>`,
// 当调用`foo().await`时，该`Future`将被运行，当调用结束后我们将获取到一个`u8`值
#[allow(dead_code)]
async fn foo() -> u8 {
    5
}
#[allow(dead_code)]
fn bar() -> impl Future<Output = u8> {
    async {
        // 下面的`async`语句块返回`Future<Output = u8>`
        let x: u8 = foo().await;
        x + 5
    }
}

// async fn 函数如果拥有引用类型的参数，那它返回的 Future 的生命周期就会被这些参数的生命周期所限制:
#[allow(dead_code)]
async fn foo_1(x: &u8) -> u8 {
    *x
}
// 上面的函数跟下面的函数是等价的:
#[allow(dead_code)]
fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move { *x }
}

// 意味着 async fn 函数返回的 Future 必须满足以下条件: 当 x 依然有效时，
// 该 Future 就必须继续等待( .await ), 也就是说x 必须比 Future活得更久。

// 在一般情况下，在函数调用后就立即 .await 不会存在任何问题，例如foo(&x).await。
// 但是，若 Future 被先存起来或发送到另一个任务或者线程，就可能存在问题了:
// #[allow(dead_code)]
// fn bad() -> impl Future<Output = u8> {
//     let x = 5;
//     borrow_x(&x)
// }
#[allow(dead_code)]
async fn borrow_x(x: &u8) -> u8 {
    *x
}

// 其中一个常用的解决方法就是将具有引用参数的 async fn 函数转变成一个具有 'static 生命周期的 Future 。
// 以上解决方法可以通过将参数和对 async fn 的调用放在同一个 async 语句块来实现:
#[allow(dead_code)]
fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;
        borrow_x(&x).await
    }
}

// async 允许我们使用 move 关键字来将环境中变量的所有权转移到语句块内，就像闭包那样，
// 好处是你不再发愁该如何解决借用生命周期的问题，坏处就是无法跟其它代码实现对变量的共享:
async fn block() {
    let str = String::from("my string");
    let f1 = async {
        println!("f1:{}", str);
    };
    let f2 = async {
        println!("f2:{}", str);
    };
    let ((), ()) = futures::join!(f1, f2);
}
/// 触发future
pub fn start() {
    block_on(block());
}

// 当.await 遇见多线程执行器
// 需要注意的是，当使用多线程 Future 执行器( executor )时， Future 可能会在线程间被移动，因此 async 语句块中的变量必须要能在线程间传递。 至于 Future 会在线程间移动的原因是：它内部的任何.await都可能导致它被切换到一个新线程上去执行。

// 由于需要在多线程环境使用，意味着 Rc、 RefCell 、没有实现 Send 的所有权类型、没有实现 Sync 的引用类型，它们都是不安全的，因此无法被使用

// 需要注意！实际上它们还是有可能被使用的，只要在 .await 调用期间，它们没有在作用域范围内。

// 类似的原因，在 .await 时使用普通的锁也不安全，例如 Mutex 。原因是，它可能会导致线程池被锁：当一个任务获取锁 A 后，若它将线程的控制权还给执行器，然后执行器又调度运行另一个任务，该任务也去尝试获取了锁 A ，结果当前线程会直接卡死，最终陷入死锁中。

// 因此，为了避免这种情况的发生，我们需要使用 futures 包下的锁 futures::lock 来替代 Mutex 完成任务。

trait Stream {
    // Stream生成的值的类型
    type Item;
    // 尝试去解析Stream中的下一个值,
    // 若无数据，返回`Poll::Pending`, 若有数据，返回 `Poll::Ready(Some(x))`, `Stream`完成则返回 `Poll::Ready(None)`
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}

// 关于 Stream 的一个常见例子是消息通道（ futures 包中的）的消费者 Receiver。
// 每次有消息从 Send 端发送后，它都可以接收到一个 Some(val) 值，
//  一旦 Send 端关闭(drop)，且消息通道中没有消息后，它会接收到一个 None 值。
#[allow(dead_code)]
async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);
    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);
    // `StreamExt::next` 类似于 `Iterator::next`, 但是前者返回的不是值，而是一个 `Future<Output = Option<T>>`，
    // 因此还需要使用`.await`来获取具体的值
    assert_eq!(Some(1), rx.next().await);
    assert_eq!(Some(2), rx.next().await);
    assert_eq!(None, rx.next().await);
}

// 迭代和并发
// 跟迭代器类似，我们也可以迭代一个 Stream。
// 例如使用map，filter，fold方法，以及它们的遇到错误提前返回的版本： try_map，try_filter，try_fold。

// 但是跟迭代器又有所不同，
// for 循环无法在这里使用，但是命令式风格的循环while let是可以用的，同时还可以使用next 和 try_next 方法:
// async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
//   use futures::stream::StreamExt; // 引入 next // 方法存在问题
//   let mut sum = 0;
//   while let Some(item) = stream.next().await {
//       sum += item;
//   }
//   sum
// }
// try_next异步处理流 ,同样存在try_next错误
// async fn sum_with_try_next(
//     mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
// ) -> Result<i32, io::Error> {
//     use futures::stream::TryStreamExt;
//     let mut sum = 0;
//     while let Some(value) = stream.try_next().await? {
//         sum += value;
//     }
//     Ok(sum)
// }

// 上面代码是一次处理一个值的模式，但是需要注意的是：
// 如果你选择一次处理一个值的模式，可能会造成无法并发，这就失去了异步编程的意义。
//  因此，如果可以的话我们还是要选择从一个 Stream 并发处理多个值的方式，通过 for_each_concurrent 或 try_for_each_concurrent 方法来实现:
// 使用并发处理stream 存在函数引入错误
// async fn jump_around(
//     mut stream: Pin<&mut dyn Stream<Item = Result<u8, io::Error>>>,
// ) -> Result<(), io::Error> {
//     use futures::stream::TryStreamExt;
//     const MAX_CURRENT_JUMPERS: usize = 100;
//     stream
//         .try_for_each_concurrent(MAX_CURRENT_JUMPERS, |nums| async move {
//             jump_n_times(nums).await?;
//             report_n_jumpers(nums).await?;
//             Ok(())
//         })
//         .await?;
//     Ok(())
// }
