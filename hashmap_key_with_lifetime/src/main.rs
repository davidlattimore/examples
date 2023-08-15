use std::borrow::Cow;
use std::collections::HashMap;

fn main() {
    let s = "Hello, World!".to_owned();
    let mut m = HashMap::new();
    m.insert(Cow::Borrowed(&s[..5]), "s1".to_owned());
    println!("{:?}", get_hello(&m));
}

fn get_hello<'a>(m: &'a HashMap<Cow<str>, String>) -> Option<&'a String> {
    let s = "Hello, Earth".to_owned();
    let hello = Cow::Borrowed(&s[..5]);
    get_value(m, hello)
}

fn get_value<'a>(m: &'a HashMap<Cow<str>, String>, key: Cow<str>) -> Option<&'a String> {
    // Challenge: Make this code compile without changing the signature of this function and without
    // doing heap allocation.
    //
    // The problem is the lifetime parameter on `key`. Even though we're passing only shared
    // references to HashMap::get, the borrow checker is unifying the return value to include the
    // lifetime of the key that we passed in.
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
    // value we want.

    // m.get(&Cow::Owned(key.as_ref().to_owned()))
}
