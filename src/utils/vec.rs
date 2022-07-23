use std::cmp::PartialEq;

// Code gottten from
// https://users.rust-lang.org/t/idiomatic-way-to-get-difference-between-two-vecs/48396/9
// Thanks to @pcpthm
pub fn vec_diff<T: PartialEq>(new_items: Vec<T>, previous_items: Vec<T>) -> Vec<T> {
    new_items
        .into_iter()
        .filter(|item| !previous_items.contains(item))
        .collect()
}
