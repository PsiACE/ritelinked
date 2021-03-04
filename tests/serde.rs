#![cfg(feature = "serde")]

use core::hash::BuildHasherDefault;
use fnv::FnvHasher;

use ritelinked::{LinkedHashMap, LinkedHashSet};
use serde_test::{assert_tokens, Token};

type FnvHashMap<K, V> = LinkedHashMap<K, V, BuildHasherDefault<FnvHasher>>;
type FnvHashSet<T> = LinkedHashSet<T, BuildHasherDefault<FnvHasher>>;

#[test]
fn map_serde_tokens_empty() {
    let map = FnvHashMap::<char, u32>::default();

    assert_tokens(&map, &[Token::Map { len: Some(0) }, Token::MapEnd]);
}

#[test]
fn map_serde_tokens() {
    let mut map = FnvHashMap::default();
    map.insert('a', 10);
    map.insert('b', 20);
    map.insert('c', 30);

    assert_tokens(
        &map,
        &[
            Token::Map { len: Some(3) },
            Token::Char('a'),
            Token::I32(10),
            Token::Char('b'),
            Token::I32(20),
            Token::Char('c'),
            Token::I32(30),
            Token::MapEnd,
        ],
    );
}

#[test]
fn set_serde_tokens_empty() {
    let set = FnvHashSet::<u32>::default();

    assert_tokens(&set, &[Token::Seq { len: Some(0) }, Token::SeqEnd]);
}

#[test]
fn set_serde_tokens() {
    let mut set = FnvHashSet::default();
    set.insert(10);
    set.insert(20);
    set.insert(30);

    assert_tokens(
        &set,
        &[
            Token::Seq { len: Some(3) },
            Token::I32(10),
            Token::I32(20),
            Token::I32(30),
            Token::SeqEnd,
        ],
    );
}
