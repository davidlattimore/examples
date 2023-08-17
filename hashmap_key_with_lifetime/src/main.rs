use std::collections::HashMap;

fn main() {
    let s = "Hello, World!".to_owned();
    let mut m = HashMap::new();
    m.insert(&s[..5], "s1".to_owned());
    println!("{:?}", get_hello(&m));
}

fn get_hello<'a>(m: &'a HashMap<&str, String>) -> Option<&'a String> {
    let s = "Hello, Earth".to_owned();
    let hello = &s[..5];
    get_value(m, hello)
}

fn get_value<'a>(m: &'a HashMap<&str, String>, key: &str) -> Option<&'a String> {
    // Challenge: Make this code compile without changing the signature of this function and without
    // doing heap allocation.
    //
    // The problem is the (unnamed) lifetime parameter on `key`. HashMap::get requires that the keys
    // of the map implement Borrow<Q>, where Q is the type of the argument passed to HashMap::get.
    // This causes lifetime of `key` to be tied with the lifetime of the HashMap's keys for the
    // purposes of determining the lifetime of the return type.
    //
    // See https://github.com/rust-lang/rust/issues/103289
    m.get(&key)

    // One way is to scan the entire hashmap - horribly inefficient.

    // for (k, v) in m {
    //     if k == &key {
    //         return Some(v);
    //     }
    // }
    // None

    // If we allow heap allocation, then we can make a temporary String object and use it to get the
    // value we want. It's kind of funny that this works because here we're passing a key with an
    // even shorter lifetime. This is probably the best workaround that I know of, since often the
    // optimiser will be smart enough to eliminated the heap allocation.

    // m.get(key.to_owned().as_str())
}
