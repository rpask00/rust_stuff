mod post;
mod state;

use crate::post::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("lorem");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("lorem", post.content());
}
