pub mod models;
pub mod schema;

use biocrs::interactions::*;

fn main() {
    println!("Hello, world!");

    let mut conn = establish_connection();

    println!("Hello, world 2!");
    let _ = run_migrations(&mut conn);

    // coment create_post(&mut conn, "fsdsa", "body_qf23", "this/is/my.path.txt");
    show_posts(&mut conn);
}
