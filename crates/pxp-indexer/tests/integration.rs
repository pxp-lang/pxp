use pxp_indexer::Indexer;
use pxp_type::Type;
use std::path::PathBuf;

macro_rules! assert_matches {
    ($expression:expr, $pattern:pat) => {
        match $expression {
            $pattern => (),
            _ => panic!("assertion failed: expression doesn't match pattern"),
        }
    };
}

#[test]
fn it_indexes_correctly() {
    let mut indexer = Indexer::new();
    let (index, mut symbol_table) = indexer.index(&PathBuf::from(format!(
        "{}/{}",
        env!("CARGO_MANIFEST_DIR"),
        "example-php"
    )));

    assert_eq!(index.get_number_of_functions(), 2);
    assert_eq!(index.get_number_of_class_likes(), 8);
    assert_eq!(index.get_number_of_constants(), 2);

    let foo = match index.get_function(symbol_table.intern(b"foo")) {
        Some(foo) => foo,
        None => unreachable!("foo is not defined"),
    };

    assert_eq!(foo.name, symbol_table.intern(b"foo"));
    assert_eq!(foo.short_name, symbol_table.intern(b"foo"));
    assert_eq!(foo.parameters.len(), 0);
    assert_matches!(foo.return_type, Type::Mixed(_));

    let bar = match index.get_function(symbol_table.intern(b"bar")) {
        Some(foo) => foo,
        None => unreachable!("bar is not defined"),
    };

    assert_eq!(bar.name, symbol_table.intern(b"bar"));
    assert_eq!(bar.short_name, symbol_table.intern(b"bar"));
    assert_eq!(bar.parameters.len(), 1);
    assert_eq!(bar.parameters[0].name, symbol_table.intern(b"$baz"));
    assert_matches!(bar.parameters[0].r#type, Type::String(_));
    assert_matches!(bar.return_type, Type::Void(_));

    let model = match index.get_class_like(symbol_table.intern(b"Nested\\Interfaces\\Model")) {
        Some(model) => model,
        None => unreachable!("Nested\\Interfaces\\Model is not defined"),
    };

    assert_eq!(
        model.name,
        symbol_table.intern(b"Nested\\Interfaces\\Model")
    );
    assert_eq!(model.short_name, symbol_table.intern(b"Model"));

    let example = match index.get_class_like(symbol_table.intern(b"Nested\\Interfaces\\Example")) {
        Some(example) => example,
        None => unreachable!("Nested\\Interfaces\\Example is not defined"),
    };

    assert_eq!(
        example.name,
        symbol_table.intern(b"Nested\\Interfaces\\Example")
    );
    assert_eq!(example.short_name, symbol_table.intern(b"Example"));
    assert_eq!(example.methods.len(), 1);
    assert_eq!(example.methods[0].name, symbol_table.intern(b"example"));
    assert_eq!(example.methods[0].parameters.len(), 0);
    assert!(example.methods[0].r#virtual);
    assert_matches!(example.methods[0].return_type, Type::Void(_));

    let user = match index.get_class_like(symbol_table.intern(b"App\\User")) {
        Some(user) => user,
        None => unreachable!("App\\User is not defined"),
    };

    assert_eq!(user.name, symbol_table.intern(b"App\\User"));
    assert_eq!(user.short_name, symbol_table.intern(b"User"));
    assert_eq!(user.properties.len(), 3);
    assert_eq!(user.implements.len(), 1);
    assert_eq!(
        user.implements[0],
        symbol_table.intern(b"Nested\\Interfaces\\Model")
    );

    assert_eq!(user.properties[0].name, symbol_table.intern(b"$name"));
    assert_matches!(user.properties[0].r#type, Type::String(_));

    assert_eq!(user.properties[1].name, symbol_table.intern(b"$email"));
    assert_matches!(user.properties[1].r#type, Type::String(_));

    assert_eq!(user.properties[2].name, symbol_table.intern(b"$createdAt"));
    assert_matches!(user.properties[2].r#type, Type::Named(_, _));
    let created_at_type = match user.properties[2].r#type {
        Type::Named(_, created_at_type) => created_at_type,
        _ => unreachable!("createdAt is not defined"),
    };
    assert_eq!(created_at_type, symbol_table.intern(b"DateTimeInterface"));

    assert_eq!(user.methods.len(), 4);

    let controller = match index.get_class_like(symbol_table.intern(b"App\\Controller")) {
        Some(controller) => controller,
        None => unreachable!("App\\Controller is not defined"),
    };

    assert!(controller.r#abstract);
}
