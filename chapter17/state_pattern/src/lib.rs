pub mod version1 {

    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }
    impl Post {
        pub fn new() -> Self {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }
        pub fn content(&self) -> &str {
            let a = self.state.as_ref().unwrap();
            ""
            // self.state.as_ref().unwrap().content(self)
        }
        // 试一下unwarp出来的是个啥
        pub fn request_review(&mut self) {
            if let Some(x) = self.state.take() {
                self.state = Some(x.request_review());
            }
        }
        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve());
            }
        }
    }

    struct Draft {}
    struct Published {}
    struct PendingReview {}
    trait Jmz {
        fn content(&self, post: &String) -> &str {
            ""
        }
        fn request_review(self: Box<Self>) -> Box<dyn Jmz>;
    }

    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }
    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }
    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
}

pub mod version2 {

    pub struct Post {
        content: String,
    }
    pub struct DraftPost {
        content: String,
    }
    pub struct PendingReviewPost {
        content: String,
    }
    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }
        pub fn content(&self) -> &str {
            &self.content
        }
    }
    impl DraftPost {
        pub fn add_text(&mut self, s: &str) {
            self.content.push_str(s);
        }
        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }
    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::version2::*;
    #[test]
    fn test1() {
        let mut p = Post::new();
        p.add_text("adfalkjdfladjlf");
        // println!("{}", p.content);
    }
}
