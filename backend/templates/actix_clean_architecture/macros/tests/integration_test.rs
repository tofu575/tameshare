use macros::NewFunction;

#[derive(NewFunction)]
struct TestStruct {
    field1: String,
    field2: i32,
}

#[test]
fn ok_new_function() {
    // newがマクロ経由で生成されていることを確認
    let instance = TestStruct::new("Hello".to_string(), 42);
    assert_eq!(instance.field1, "Hello");
    assert_eq!(instance.field2, 42);
}
