use futures::executor::block_on;
// 不用futures

struct Song {
    author: String,
    name: String,
}

async fn learn_sone() -> Song {
    Song {
        author: String::from("刘德华"),
        name: String::from("暗里着迷"),
    }
}

async fn sing_song(song: &Song) {
    println!("给大家献上{}的{}", song.author, song.name);
}

async fn dance() {
    println!("开始跳舞!")
}

// 异步处理
async fn learn_and_sing() {
    let song = learn_sone().await;
    sing_song(&song).await;
}

pub fn start() {
    let song = block_on(learn_sone());
    block_on(sing_song(&song));
    block_on(dance());
}

// 异步函数,可以包裹之后对外处理
pub fn async_start() {
    async fn start() {
        let f1 = learn_and_sing();
        let f2 = dance();
        futures::join!(f1, f2);
    }
    block_on(start());
}
