use super::Searchable;

#[test]
fn find_before() {
    assert_eq!(
        (b"This is a sentence with an item, and a marker" as &[u8]).find_before(b"item", b"marker").expect("marker").expect("item"),
        b"This is a sentence with an ".len()
    );

    (b"This is a sentence with just an item" as &[u8]).find_before(b"item", b"marker").expect_err("marker absent");

    assert_eq!(
        (b"This is a sentence with a marker occurring before the item" as &[u8]).find_before(b"item", b"marker").expect("marker").is_none(),
        true
    );


}

#[test]
fn find_all() {
    let string: &[u8] = b"This is a sentence with as many as 5 a's in it";

    assert_eq!(
        string.find_all(b"a").len(),
        5
    )
}