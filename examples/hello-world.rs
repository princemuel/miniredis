#[tokio::main]
async fn main() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    let message = say_world();

    // This println! comes first
    println!("hello");

    // Calling `.await` on `message` starts executing `say_world`.
    message.await;
}

async fn say_world() {
    println!("world");
}
