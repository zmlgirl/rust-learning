use futures;

struct Song {
    name: String,
}

async fn learn_song() -> Song {
    println!("learnning song");
    Song {
        name: String::from("Whatever"),
    }
}

async fn dance() {
    println!("dance");
}

async fn sing_song(song: Song) {
    println!("sing song: {}", song.name);
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}

fn main() {
    futures::executor::block_on(async_main());
}
