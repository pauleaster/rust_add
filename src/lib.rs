

#[no_mangle] 
pub extern "C" fn add(left: i8, right: i8) -> i8 {
    println!("left = {}, right={}, left + right = {}",left, right,left + right);
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
