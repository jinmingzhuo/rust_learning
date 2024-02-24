use state_pattern::version2::Post;

fn main() {
    println!("Hello, world!");
    let mut p = Post::new();
    p.add_text("jmz");
    // println!("{}",p.content)
    let p = p.request_review().approve();
    println!("{}", p.content());
}
