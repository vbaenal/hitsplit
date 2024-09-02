pub mod counter;
pub mod functions;
pub mod hitsplit;
pub mod panels;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum ChangeImage {
    Game,
    Category,
    Split(String),
}
