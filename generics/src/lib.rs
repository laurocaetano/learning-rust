fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_largest_number() {
        let numbers = vec![1, 2, 3, 33, 44, 101, 555];

        assert_eq!(largest(&numbers), 555);
    }

    #[test]
    fn returns_the_largest_char() {
        let chars = vec!['a', 'b', 'c', 'd'];

        assert_eq!(largest(&chars), 'd');
    }
}
