use super::*;
#[test]
fn test_dedup() {
    let mut v = vec!["b", "b", "a"];
    dedup(&mut v);

    assert_eq!(&v, &vec!["b", "a"]);
}

#[test]
fn test_with_empty_array() {
    // arrange
    let empty_array: Vec<i32> = vec![];
    let expected_output: Vec<i32> = vec![];
    // act
    let actual_output = unique(empty_array);
    // assert
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_with_duplicates() {
    // arrange
    let numbers = vec![2, 1, 3, 2];
    let expected_output = vec![2, 1, 3];
    // act
    let actual_output = unique(numbers);
    // assert
    assert_eq!(expected_output, actual_output);
}
#[test]
fn test_without_duplicates() {
    // arrange
    let numbers = vec![1, 3, 2];
    let expected_output = vec![1, 3, 2];
    // act
    let actual_output = unique(numbers);
    // assert
    assert_eq!(expected_output, actual_output);
}
#[test]
fn test_with_float() {
    // arrange
    let numbers = vec![3.1, 3.2, 3.1, 1.0];
    let expected_output = vec![3.1, 3.2, 1.0];
    // act
    let actual_output = unique(numbers);
    // assert
    assert_eq!(expected_output, actual_output);
}
#[test]
fn test_with_char() {
    // arrange
    let numbers = vec!['x', 'y', 'x', 'a'];
    let expected_output = vec!['x', 'y', 'a'];
    // act
    let actual_output = unique(numbers);
    // assert
    assert_eq!(expected_output, actual_output);
}
#[test]
fn test_with_string() {
    // arrange
    let numbers = vec!["a", "b"];
    let expected_output = vec!["a", "b"];
    // act
    let actual_output = unique(numbers);
    // assert
    assert_eq!(expected_output, actual_output);
}
