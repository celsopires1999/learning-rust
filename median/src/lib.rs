pub fn median(array:  &mut Vec<f64>) -> Option<f64> {
        if array.is_empty() {
        return None;
    }

    array.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let element = array.len() / 2;

    if array.len() % 2 == 0 {
        return Some((array[element] + array[element - 1]) / 2.0);
    }
    Some(array[element])
}

#[cfg(test)]
mod tests {
     // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut numbers = vec![];
        assert_eq!(median(&mut numbers), None);
    }

    #[test]
    fn test_odd_array_unsorted (){
        let mut numbers = vec![3.0, 1.0, 5.0];
        assert_eq!(median(&mut numbers), Some(3.0));
    }

    #[test]
    fn test_odd_array_sorted (){
        let mut numbers = vec![1.0, 3.0, 5.0];
        assert_eq!(median(&mut numbers), Some(3.0));
    }

    #[test]
    fn test_even_array_unsorted() {
        let mut numbers = vec![5.0, 4.0, 1.0, 3.0];
        assert_eq!(median(&mut numbers), Some(3.5));
    }

    #[test]
    fn test_even_array_sorted() {
        let mut numbers = vec![1.0, 3.0, 4.0, 5.0];
        assert_eq!(median(&mut numbers), Some(3.5));
    }
 
}