use futures::{Future, executor::block_on};

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
pub fn start(){
  block_on(block());
}
