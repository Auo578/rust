///adds two number together
/// 
/// #Example
/// ```
/// let sum = test_study::cargo cadd(2, 2);
/// assert_eq!(sum, 4);
/// ```
/// # panics






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

    
    #[test]
    fn feasure_sum2(){
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    #[should_panic]
    fn feasure_sum3(){
        assert_eq!(add(2, 2), 5);
    }
}
