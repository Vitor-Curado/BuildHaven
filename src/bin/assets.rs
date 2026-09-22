use buildhaven::{assets::Assets, error::AppResult};

fn main() -> AppResult<()> {
    Assets::new()?.generate_all_static_pages()
}