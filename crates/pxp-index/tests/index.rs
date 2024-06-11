use discoverer::discover;
use pxp_index::{Index, Indexer};
use pxp_parser::parse;
use pxp_symbol::{Symbol, SymbolTable};
use pxp_type::Type;

#[test]
fn it_indexes_classes() {
    let index = index();

    let a = index.get_class(name(b"A")).unwrap();

    assert!(a.get_name() == name(b"A"));
    assert!(a.get_short_name() == name(b"A"));
    assert!(a.get_namespace() == None);

    let b = index.get_class(name(b"B")).unwrap();

    assert!(b.get_name() == name(b"B"));
    assert!(b.get_short_name() == name(b"B"));
    assert!(b.get_namespace() == None);
    
    let parent = b.get_parent().unwrap();
    assert!(parent.get_name() == name(b"A"));

    let c = index.get_class(name(b"C")).unwrap();

    assert!(c.get_name() == name(b"C"));
    assert!(c.get_short_name() == name(b"C"));
    assert!(c.get_namespace() == None);

    let parent = c.get_parent().unwrap();
    assert!(parent.get_name() == name(b"B"));

    // FIXME: Add in tests here for interfaces.

    let properties = index.get_class(name(b"Properties")).unwrap();

    assert!(properties.get_name() == name(b"Properties"));
    assert!(properties.get_short_name() == name(b"Properties"));
    assert!(properties.get_namespace() == None);

    let properties_properties = properties.get_properties().collect::<Vec<_>>();

    assert!(properties_properties.len() == 6);

    let a = properties.get_property(name(b"a")).unwrap();
    assert!(a.is_public());
    assert!(a.get_type() == &Type::Mixed);

    let b = properties.get_property(name(b"b")).unwrap();
    assert!(b.is_public());
    assert!(b.get_type() == &Type::String);

    let c = properties.get_property(name(b"c")).unwrap();
    assert!(c.is_protected());
    assert!(c.get_type() == &Type::Integer);

    let d = properties.get_property(name(b"d")).unwrap();
    assert!(d.is_private());
    assert!(d.get_type() == &Type::Boolean);

    let e = properties.get_property(name(b"e")).unwrap();
    assert!(e.is_static());

    let f = properties.get_property(name(b"f")).unwrap();
    assert!(f.is_public());
    assert!(f.get_type() == &Type::Mixed);

    let methods = index.get_class(name(b"Methods")).unwrap();

    assert!(methods.get_name() == name(b"Methods"));
    assert!(methods.get_short_name() == name(b"Methods"));
    assert!(methods.get_namespace() == None);

    let methods_methods = methods.get_methods().collect::<Vec<_>>();

    assert!(methods_methods.len() == 5);

    let a = methods.get_method(name(b"a")).unwrap();
    assert!(a.is_public());
    assert!(a.get_return_type() == &Type::Mixed);

    let b = methods.get_method(name(b"b")).unwrap();
    assert!(b.is_public());
    assert!(b.get_return_type() == &Type::String);

    let c = methods.get_method(name(b"c")).unwrap();
    assert!(c.is_protected());
    assert!(c.get_return_type() == &Type::Integer);

    let d = methods.get_method(name(b"d")).unwrap();
    assert!(d.is_private());
    assert!(d.get_return_type() == &Type::Boolean);

    let e = methods.get_method(name(b"e")).unwrap();
    assert!(e.is_static());

    let constants = index.get_class(name(b"Constants")).unwrap();

    assert!(constants.get_name() == name(b"Constants"));
    assert!(constants.get_short_name() == name(b"Constants"));
    assert!(constants.get_namespace() == None);
    assert!(constants.is_class());

    let constants = constants.get_constants().collect::<Vec<_>>();

    assert!(constants.len() == 5);

    assert!(constants[0].get_name() == name(b"A"));
    assert!(constants[0].get_type() == &Type::Mixed);

    assert!(constants[1].get_name() == name(b"B"));
    assert!(constants[1].get_type() == &Type::Integer);
    assert!(constants[1].is_public());

    assert!(constants[2].get_name() == name(b"C"));
    assert!(constants[2].get_type() == &Type::String);
    assert!(constants[2].is_protected());

    assert!(constants[3].get_name() == name(b"D"));
    assert!(constants[3].is_private());

    assert!(constants[4].get_name() == name(b"E"));
    assert!(constants[4].is_final());
}

#[test]
fn it_indexes_interfaces() {
    let index = index();

    let i = index.get_class(name(b"I")).unwrap();

    assert!(i.get_name() == name(b"I"));
    assert!(i.get_short_name() == name(b"I"));
    assert!(i.get_namespace() == None);
    assert!(i.is_interface());

    let j = index.get_class(name(b"J")).unwrap();

    assert!(j.get_name() == name(b"J"));
    assert!(j.get_short_name() == name(b"J"));
    assert!(j.get_namespace() == None);
    assert!(j.is_interface());

    let k = index.get_class(name(b"K")).unwrap();

    assert!(k.get_name() == name(b"K"));
    assert!(k.get_short_name() == name(b"K"));
    assert!(k.get_namespace() == None);
    assert!(k.is_interface());

    let k_interfaces = k.get_interfaces().collect::<Vec<_>>();

    assert!(k_interfaces.len() == 2);
    assert!(k_interfaces[0].get_name() == name(b"I"));
    assert!(k_interfaces[1].get_name() == name(b"J"));

    let l = index.get_class(name(b"L")).unwrap();

    assert!(l.get_name() == name(b"L"));
    assert!(l.get_short_name() == name(b"L"));
    assert!(l.get_namespace() == None);
    assert!(l.is_interface());

    let l_constants = l.get_constants().collect::<Vec<_>>();

    assert!(l_constants.len() == 1);
    assert!(l_constants[0].get_name() == name(b"A"));

    let l_methods = l.get_methods().collect::<Vec<_>>();

    assert!(l_methods.len() == 3);

    let a = l.get_method(name(b"a")).unwrap();
    assert!(a.is_public());
    assert!(a.get_return_type() == &Type::Mixed);

    let b = l.get_method(name(b"b")).unwrap();
    assert!(b.is_public());
    assert!(b.get_return_type() == &Type::String);

    let c = l.get_method(name(b"c")).unwrap();

    assert!(c.get_return_type() == &Type::Mixed);
    assert!(c.is_static());
    assert!(c.is_abstract());
}

#[test]
fn it_indexes_functions() {
    let index = index();

    let a = index.get_function(name(b"a")).unwrap();

    assert!(a.get_name() == name(b"a"));
    assert!(a.get_short_name() == name(b"a"));
    assert!(a.get_namespace() == None);

    let c = index.get_function(name(b"c")).unwrap();

    assert!(c.get_name() == name(b"c"));
    assert!(c.get_short_name() == name(b"c"));
    assert!(c.get_namespace() == None);

    let c_parameters = c.get_parameters().collect::<Vec<_>>();

    assert!(c_parameters.len() == 1);
    assert!(c_parameters[0].get_name() == name(b"a"));
    assert!(c_parameters[0].get_type() == &Type::String);

    let d = index.get_function(name(b"d")).unwrap();

    assert!(d.get_return_type() == &Type::Integer);

    let e = index.get_function(name(b"e")).unwrap();

    assert!(e.returns_by_reference());

    let b = index.get_function(name(b"A\\b")).unwrap();

    assert!(b.get_name() == name(b"A\\b"));
    assert!(b.get_short_name() == name(b"b"));
    assert!(b.get_namespace() == Some(name(b"A")));
}

#[test]
fn it_indexes_enums() {
    let index = index();

    let role = index.get_class(name(b"Role")).unwrap();

    assert!(role.get_name() == name(b"Role"));
    assert!(role.get_short_name() == name(b"Role"));
    assert!(role.get_namespace() == None);
    assert!(role.is_enum());

    let cases = role.get_cases().collect::<Vec<_>>();

    assert!(cases.len() == 2);
    assert!(cases[0].get_name() == name(b"Admin"));
    assert!(cases[1].get_name() == name(b"User"));

    let status = index.get_class(name(b"Status")).unwrap();

    assert!(status.get_name() == name(b"Status"));
    assert!(status.get_short_name() == name(b"Status"));
    assert!(status.get_namespace() == None);
    assert!(status.is_enum());

    let cases = status.get_cases().collect::<Vec<_>>();

    assert!(cases.len() == 2);
    assert!(cases[0].get_name() == name(b"Active"));
    assert!(cases[1].get_name() == name(b"Inactive"));

    let color = index.get_class(name(b"Color")).unwrap();

    assert!(color.get_name() == name(b"Color"));
    assert!(color.get_short_name() == name(b"Color"));
    assert!(color.get_namespace() == None);
    assert!(color.is_enum());

    let cases = color.get_cases().collect::<Vec<_>>();

    assert!(cases.len() == 3);
    assert!(cases[0].get_name() == name(b"Red"));
    assert!(cases[1].get_name() == name(b"Green"));
    assert!(cases[2].get_name() == name(b"Blue"));

    let methods = color.get_methods().collect::<Vec<_>>();

    assert!(methods.len() == 1);
    
    let get_hex = &methods[0];

    assert!(get_hex.get_name() == name(b"getHex"));
    assert!(get_hex.get_return_type() == &Type::String);
    assert!(get_hex.is_public());
}

fn name(name: &[u8]) -> Symbol {
    SymbolTable::the().intern(name)
}

fn index() -> Index {
    let mut indexer = Indexer::new();
    let files = discover(&["php"], &["tests/fixtures"]).expect("failed to discover files");

    for file in files.iter() {
        let result = parse(&std::fs::read(&file).expect("failed to read file"), SymbolTable::the());
        indexer.index(&result.ast);
    }

    indexer.get_index().clone()
}