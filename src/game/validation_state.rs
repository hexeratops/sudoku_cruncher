#[derive(Debug)]
pub enum ValidationState {
    NextMove,
    Complete,
    BadMove,
    Unsolvable
}
