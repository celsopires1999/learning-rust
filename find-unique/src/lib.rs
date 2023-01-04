use std::collections::HashSet;
use std::hash::Hash;

pub fn dedup<T: Eq + Hash + Copy>(v: &mut Vec<T>) {
    // note the Copy constraint
    let mut uniques = HashSet::new();
    v.retain(|e| uniques.insert(*e));
}

pub fn unique<T>(array: Vec<T>) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    if array.is_empty() {
        return array;
    }

    let mut output = Vec::<T>::new();

    output.push(array[0]);

    for i in 1..array.len() {
        let mut found = false;
        for j in 0..output.len() {
            if array[i] == output[j] {
                found = true;
                break;
            }
        }
        if !found {
            output.push(array[i]);
        }
    }

    output
}

pub fn unique_iterators(_array: Vec<i32>) -> Vec<i32> {
    // To be done
    let output: Vec<i32> = vec![2, 1, 3];
    output
}

pub fn unique_functions<T>(mut array: Vec<T>) -> Vec<T>
where
    T: Ord,
{
    array.sort();
    array.dedup();
    array
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
