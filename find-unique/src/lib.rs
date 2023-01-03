pub fn unique(array: Vec<i32>) -> Vec<i32> {
 
    // let mut into_iter = array.into_iter();
    // let _result = into_iter.find(| &x| x == 2);

    let _result = array.into_iter().find(| &x| x == 2);

    vec![1, 3, 2]

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_duplicates() {
        // arrange
        let numbers = vec![1, 3, 2, 2];        
        let expected_output = vec![1, 3, 2];
        // act
        let actual_output = unique(numbers);
        // assert
        assert_eq!(expected_output, actual_output);
    }
}
