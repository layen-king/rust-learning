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

pub fn start() {
    let song = block_on(learn_sone());
    block_on(sing_song(&song));
    block_on(dance());
}
