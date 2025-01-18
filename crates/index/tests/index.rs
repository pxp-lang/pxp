use discoverer::discover;
use pxp_index::{Index, ReflectionFunctionLike, ReflectsParameters};
use pxp_type::Type;

#[test]
fn it_indexes_plain_functions() {
    let index = index();

    let a = index.get_function("a").unwrap();

    assert_eq!(a.get_name(), b"a");
    assert_eq!(a.get_short_name(), b"a");
    assert!(a.get_return_type().is_none());
    assert!(a.get_parameters().is_empty());
}

#[test]
fn it_indexes_functions_with_parameters() {
    let index = index();

    let b = index.get_function("b").unwrap();

    assert_eq!(b.get_number_of_parameters(), 2);
    
    let parameters = b.get_parameters();

    assert_eq!(parameters[0].get_name(), b"a");
    assert!(parameters[0].get_type().is_some());
    assert!(parameters[0].get_type().unwrap().is(&Type::String));

    assert_eq!(parameters[1].get_name(), b"b");
    assert!(parameters[1].get_type().is_some());
    assert!(parameters[1].get_type().unwrap().is(&Type::Integer));
}

#[test]
fn it_indexes_functions_with_return_type() {
    let index = index();

    let c = index.get_function("c").unwrap();

    assert!(c.get_return_type().is_some());
    assert!(c.get_return_type().unwrap().is(&Type::Void));
}

#[test]
fn it_indexes_functions_that_return_by_ref() {
    let index = index();

    let d = index.get_function("d").unwrap();

    assert!(d.returns_reference());
}

fn index() -> Index {
    let mut index = Index::new();
    let files = discover(&["php"], &["./tests/fixtures"]).expect("Failed to load fixture files.");

    for file in files.iter() {
        index.index_file(&file);
    }

    index
}
