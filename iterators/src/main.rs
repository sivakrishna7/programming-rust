// An iterator is a value that produces a sequences of values,
// typically for a loop to operate on.

// fn triangle(n: i32) -> i32 {
//     let mut sum = 0;
//     for i in 1..=n {
//         sum += i;
//     }
//     sum
// }

fn triangle(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

//  An iterator is any value that implements the std::iter::Iterator trait

// Item is the type of value the iterator produces.
// `next` method either returns `Some(v)`, where `v` is the iterator's
// next value, or returns `None` to indicate the end of the sequence.
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ... many default methods
}

// `IntoIter` is the type of the iterator value iteself, and `Item` is the
// type of value it produces.

// {
//     println!("There's:");
//     let v = vec!["antimony", "arsenic", "aluminum", "selenium"];

//     for element in &v {
//         println!("{}", element);
//     }
// }

// {
//     let mut iterator = (&v).into_iter();
//     while let Some(element) = iterator.next() {
//         println!("{}", element);
//     }
// }

// An `iterator` is any type that implements `Iterator`.
// An `iterable` is any type that implements `IntoIterator`: you can get an iterator
// over it by calling its `into_iter` method. The vector reference `&v` is the
// iterable in this case.
// An iterator produces values.
//  The values an iterator produces are items.
// The code that receives the items an iterator produces is the consumer. "For loop above"

fn main() {
    let v = vec![4, 20, 12, 8, 6];
    let mut iterator = v.iter();
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&12));
    assert_eq!(iterator.next(), Some(&8));
    assert_eq!(iterator.next(), Some(&6));
    assert_eq!(iterator.next(), None);


    use std::ffi::OsStr;
    use std::path::Path;

    let path = Path::new("C:/Users/Siva/Downloads/shambho.jpg");
    let mut iterator = path.iter();
    assert_eq!(iterator.next(), Some(OsStr::new("C:")));
    assert_eq!(iterator.next(), Some(OsStr::new("\\")));
    assert_eq!(iterator.next(), Some(OsStr::new("Users")));
    assert_eq!(iterator.next(), Some(OsStr::new("Siva")));
    assert_eq!(iterator.next(), Some(OsStr::new("Downloads")));
    assert_eq!(iterator.next(), Some(OsStr::new("shambho.jpg")));
    assert_eq!(iterator.next(), None);

    // You should usually use HashSet, but its iteration order is
    // nondeterministic, so BTreeSet works better in examples.
    use std::collections::BTreeSet;
    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the Sky with Diamonds".to_string());
    favorites.insert("Liebestraume No. 3".to_string());

    let mut it = favorites.into_iter();
    assert_eq!(it.next(), Some("Liebestraume No. 3".to_string()));
    assert_eq!(it.next(), Some("Lucy in the Sky with Diamonds".to_string()));
    assert_eq!(it.next(), None);
}