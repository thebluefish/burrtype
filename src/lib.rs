pub mod export;
#[path = "ir.rs"]
pub mod ir;
pub mod syn;
pub mod targets;
pub mod prelude;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}