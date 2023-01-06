fn sum_with_missing_1(list: Vec<Option<i32>>) -> i32 {
    let mut sum = 0;
    for n in list {
        if let Some(value) = n {
            sum += value;
        };
    }
    sum
}

fn sum_with_missing_2(list: Vec<Option<i32>>) -> i32 {
    list.iter().map(|v| v.unwrap_or(0)).sum()
}

#[cfg(test)]
mod tests_1 {
    use super::*;

    #[test]
    fn should_be_5() {
        let list = vec![Some(4), None, Some(1)];
        let result = sum_with_missing_1(list);
        assert_eq!(result, 5);
    }

    #[test]
    fn should_be_0() {
        let list = vec![None, None, None];
        let result = sum_with_missing_1(list);
        assert_eq!(result, 0);
    }
}

mod tests_2 {
    use super::*;

    #[test]
    fn should_be_5() {
        let list = vec![Some(4), None, Some(1)];
        let result = sum_with_missing_2(list);
        assert_eq!(result, 5);
    }

    #[test]
    fn should_be_0() {
        let list = vec![None, None, None];
        let result = sum_with_missing_2(list);
        assert_eq!(result, 0);
    }
}
