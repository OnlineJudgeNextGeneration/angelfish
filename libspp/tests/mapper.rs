extern crate libspp;

#[test]
fn test_mapper() {
    let mut mapper = libspp::new_mapper();
    ["hi", "hello"].iter().for_each(
        |e| mapper.define_string_id(e));
    let hello = mapper.string_to_integer("hello");
    let hi = mapper.string_to_integer("hi");
    assert_eq!(hello, mapper.string_to_integer("hello"));
    assert_ne!(hi, mapper.string_to_integer("hello"));
    assert_ne!(hello, mapper.string_to_integer("hi"));
    assert_eq!(None, mapper.string_to_integer("not-found"));
    assert_eq!("hello", mapper.integer_to_string(hello.unwrap()).unwrap());
    assert_eq!("hi", mapper.integer_to_string(hi.unwrap()).unwrap());
}
