pub trait IteratorExample {
    type Item; // The Item type will be the type returned from the iterator.

    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        for val in v1_iter {
            println!("Got: {}", val);
        }
    }
}

#[test]
fn iterator_demo() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter(); // mut in order to use `.next()`

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iter_num() {
    let v1 = vec![1, 2, 3];

    let v1_i = v1.iter();

    let total: i32 = v1_i.sum();

    assert!(total == 6)
}

#[test]
fn add_one() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
