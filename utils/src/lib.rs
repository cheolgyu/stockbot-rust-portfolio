#[cfg(feature = "cmd")]
pub mod cmd;
#[cfg(feature = "db")]
pub mod db;
#[cfg(feature = "env")]
pub mod env;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
