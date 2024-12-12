use discoverer::discover;
use pxp_bytestring::ByteString;
use pxp_index::{Index, Indexer};
use pxp_lexer::Lexer;
use pxp_parser::Parser;
use pxp_type::Type;

#[test]
fn it_indexes_classes() {
    let index = index();

    let a = index.get_class(&ByteString::from(b"A")).unwrap();

    assert!(a.get_name() == &ByteString::from(b"A"));
    assert!(a.get_short_name() == &ByteString::from(b"A"));
    assert!(a.get_namespace().is_none());

    let b = index.get_class(&ByteString::from(b"B")).unwrap();

    assert!(b.get_name() == &ByteString::from(b"B"));
    assert!(b.get_short_name() == &ByteString::from(b"B"));
    assert!(b.get_namespace().is_none());

    let parent = b.get_parent(&index).unwrap();
    assert!(parent.get_name() == &ByteString::from(b"A"));

    let c = index.get_class(&ByteString::from(b"C")).unwrap();

    assert!(c.get_name() == &ByteString::from(b"C"));
    assert!(c.get_short_name() == &ByteString::from(b"C"));
    assert!(c.get_namespace().is_none());

    let parent = c.get_parent(&index).unwrap();
    assert!(parent.get_name() == &ByteString::from(b"B"));

    // FIXME: Add in tests here for interfaces.

    let properties = index.get_class(&ByteString::from(b"Properties")).unwrap();

    assert!(properties.get_name() == &ByteString::from(b"Properties"));
    assert!(properties.get_short_name() == &ByteString::from(b"Properties"));
    assert!(properties.get_namespace().is_none());

    let properties_properties = properties.get_own_properties();

    assert!(properties_properties.len() == 6);

    let a = properties.get_property(&ByteString::from(b"a")).unwrap();
    assert!(a.is_public());
    assert!(a.get_type() == &Type::Mixed);

    let b = properties.get_property(&ByteString::from(b"b")).unwrap();
    assert!(b.is_public());
    assert!(b.get_type() == &Type::String);

    let c = properties.get_property(&ByteString::from(b"c")).unwrap();
    assert!(c.is_protected());
    assert!(c.get_type() == &Type::Integer);

    let d = properties.get_property(&ByteString::from(b"d")).unwrap();
    assert!(d.is_private());
    assert!(d.get_type() == &Type::Boolean);

    let e = properties.get_property(&ByteString::from(b"e")).unwrap();
    assert!(e.is_static());

    let f = properties.get_property(&ByteString::from(b"f")).unwrap();
    assert!(f.is_public());
    assert!(f.get_type() == &Type::Mixed);

    let methods = index.get_class(&ByteString::from(b"Methods")).unwrap();

    assert!(methods.get_name() == &ByteString::from(b"Methods"));
    assert!(methods.get_short_name() == &ByteString::from(b"Methods"));
    assert!(methods.get_namespace().is_none());

    let methods_methods = methods.get_own_methods();

    assert!(methods_methods.len() == 5);

    let a = methods.get_method(&ByteString::from(b"a")).unwrap();
    assert!(a.is_public());
    assert!(a.get_return_type() == &Type::Mixed);

    let b = methods.get_method(&ByteString::from(b"b")).unwrap();
    assert!(b.is_public());
    assert!(b.get_return_type() == &Type::String);

    let c = methods.get_method(&ByteString::from(b"c")).unwrap();
    assert!(c.is_protected());
    assert!(c.get_return_type() == &Type::Integer);

    let d = methods.get_method(&ByteString::from(b"d")).unwrap();
    assert!(d.is_private());
    assert!(d.get_return_type() == &Type::Boolean);

    let e = methods.get_method(&ByteString::from(b"e")).unwrap();
    assert!(e.is_static());

    let constants = index.get_class(&ByteString::from(b"Constants")).unwrap();

    assert!(constants.get_name() == &ByteString::from(b"Constants"));
    assert!(constants.get_short_name() == &ByteString::from(b"Constants"));
    assert!(constants.get_namespace().is_none());
    assert!(constants.is_class());

    let constants = constants.get_constants();

    assert!(constants.len() == 5);

    assert!(constants[0].get_name() == &ByteString::from(b"A"));
    assert!(constants[0].get_type() == &Type::Mixed);

    assert!(constants[1].get_name() == &ByteString::from(b"B"));
    assert!(constants[1].get_type() == &Type::Integer);
    assert!(constants[1].is_public());

    assert!(constants[2].get_name() == &ByteString::from(b"C"));
    assert!(constants[2].get_type() == &Type::String);
    assert!(constants[2].is_protected());

    assert!(constants[3].get_name() == &ByteString::from(b"D"));
    assert!(constants[3].is_private());

    assert!(constants[4].get_name() == &ByteString::from(b"E"));
    assert!(constants[4].is_final());
}

#[test]
fn it_indexes_interfaces() {
    let index = index();

    let i = index.get_class(&ByteString::from(b"I")).unwrap();

    assert!(i.get_name() == &ByteString::from(b"I"));
    assert!(i.get_short_name() == &ByteString::from(b"I"));
    assert!(i.get_namespace().is_none());
    assert!(i.is_interface());

    let j = index.get_class(&ByteString::from(b"J")).unwrap();

    assert!(j.get_name() == &ByteString::from(b"J"));
    assert!(j.get_short_name() == &ByteString::from(b"J"));
    assert!(j.get_namespace().is_none());
    assert!(j.is_interface());

    let k = index.get_class(&ByteString::from(b"K")).unwrap();

    assert!(k.get_name() == &ByteString::from(b"K"));
    assert!(k.get_short_name() == &ByteString::from(b"K"));
    assert!(k.get_namespace().is_none());
    assert!(k.is_interface());

    let k_interfaces = k.get_interfaces(&index);

    assert!(k_interfaces.len() == 2);
    assert!(k_interfaces[0].get_name() == &ByteString::from(b"I"));
    assert!(k_interfaces[1].get_name() == &ByteString::from(b"J"));

    let l = index.get_class(&ByteString::from(b"L")).unwrap();

    assert!(l.get_name() == &ByteString::from(b"L"));
    assert!(l.get_short_name() == &ByteString::from(b"L"));
    assert!(l.get_namespace().is_none());
    assert!(l.is_interface());

    let l_constants = l.get_constants();

    assert!(l_constants.len() == 1);
    assert!(l_constants[0].get_name() == &ByteString::from(b"A"));

    let l_methods = l.get_own_methods();

    assert!(l_methods.len() == 3);

    let a = l.get_method(&ByteString::from(b"a")).unwrap();
    assert!(a.is_public());
    assert!(a.get_return_type() == &Type::Mixed);

    let b = l.get_method(&ByteString::from(b"b")).unwrap();
    assert!(b.is_public());
    assert!(b.get_return_type() == &Type::String);

    let c = l.get_method(&ByteString::from(b"c")).unwrap();

    assert!(c.get_return_type() == &Type::Mixed);
    assert!(c.is_static());
    assert!(c.is_abstract());
}

#[test]
fn it_indexes_traits() {
    let index = index();

    let trait_a = index.get_class(&ByteString::from(b"TraitA")).unwrap();

    assert!(trait_a.get_name() == &ByteString::from(b"TraitA"));
    assert!(trait_a.get_short_name() == &ByteString::from(b"TraitA"));
    assert!(trait_a.get_namespace().is_none());
    assert!(trait_a.is_trait());

    let trait_b = index.get_class(&ByteString::from(b"TraitB")).unwrap();

    assert!(trait_b.get_name() == &ByteString::from(b"TraitB"));
    assert!(trait_b.get_short_name() == &ByteString::from(b"TraitB"));
    assert!(trait_b.get_namespace().is_none());
    assert!(trait_b.is_trait());

    let trait_c = index.get_class(&ByteString::from(b"TraitC")).unwrap();

    assert!(trait_c.get_name() == &ByteString::from(b"TraitC"));
    assert!(trait_c.get_short_name() == &ByteString::from(b"TraitC"));
    assert!(trait_c.get_namespace().is_none());
    assert!(trait_c.is_trait());

    let trait_c_traits = trait_c.get_traits(&index);

    assert!(trait_c_traits.len() == 2);
    assert!(trait_c_traits[0].get_name() == &ByteString::from(b"TraitA"));
    assert!(trait_c_traits[1].get_name() == &ByteString::from(b"TraitB"));

    let trait_d = index.get_class(&ByteString::from(b"TraitD")).unwrap();

    assert!(trait_d.get_name() == &ByteString::from(b"TraitD"));
    assert!(trait_d.get_short_name() == &ByteString::from(b"TraitD"));
    assert!(trait_d.get_namespace().is_none());
    assert!(trait_d.is_trait());

    let trait_d_constants = trait_d.get_constants();

    assert!(trait_d_constants.len() == 1);
    assert!(trait_d_constants[0].get_name() == &ByteString::from(b"A"));

    let trait_d_methods = trait_d.get_own_methods();

    assert!(trait_d_methods.len() == 4);

    let a = trait_d.get_method(&ByteString::from(b"a")).unwrap();
    assert!(a.is_public());
    assert!(a.get_return_type() == &Type::Mixed);

    let b = trait_d.get_method(&ByteString::from(b"b")).unwrap();
    assert!(b.is_public());
    assert!(b.get_return_type() == &Type::String);

    let c = trait_d.get_method(&ByteString::from(b"c")).unwrap();
    assert!(c.is_protected());
    assert!(c.is_static());

    let d = trait_d.get_method(&ByteString::from(b"d")).unwrap();
    assert!(d.is_private());
    assert!(d.is_abstract());
}

#[test]
fn it_indexes_functions() {
    let index = index();

    let a = index.get_function(&ByteString::from(b"a")).unwrap();

    assert!(a.get_name() == &ByteString::from(b"a"));
    assert!(a.get_short_name() == &ByteString::from(b"a"));
    assert!(a.get_namespace().is_none());

    let c = index.get_function(&ByteString::from(b"c")).unwrap();

    assert!(c.get_name() == &ByteString::from(b"c"));
    assert!(c.get_short_name() == &ByteString::from(b"c"));
    assert!(c.get_namespace().is_none());

    let c_parameters = c.get_parameters();

    assert!(c_parameters.len() == 1);
    assert!(c_parameters[0].get_name() == &ByteString::from(b"a"));
    assert!(c_parameters[0].get_type() == &Type::String);

    let d = index.get_function(&ByteString::from(b"d")).unwrap();

    assert!(d.get_return_type() == &Type::Integer);

    let e = index.get_function(&ByteString::from(b"e")).unwrap();

    assert!(e.returns_by_reference());

    let b = index.get_function(&ByteString::from(b"A\\b")).unwrap();

    assert!(b.get_name() == &ByteString::from(b"A\\b"));
    assert!(b.get_short_name() == &ByteString::from(b"b"));
    assert!(b.get_namespace() == Some(&ByteString::from(b"A")));
}

#[test]
fn it_indexes_enums() {
    let index = index();

    let role = index.get_class(&ByteString::from(b"Role")).unwrap();

    assert!(role.get_name() == &ByteString::from(b"Role"));
    assert!(role.get_short_name() == &ByteString::from(b"Role"));
    assert!(role.get_namespace().is_none());
    assert!(role.is_enum());

    let cases = role.get_cases();

    assert!(cases.len() == 2);
    assert!(cases[0].get_name() == &ByteString::from(b"Admin"));
    assert!(cases[1].get_name() == &ByteString::from(b"User"));

    let status = index.get_class(&ByteString::from(b"Status")).unwrap();

    assert!(status.get_name() == &ByteString::from(b"Status"));
    assert!(status.get_short_name() == &ByteString::from(b"Status"));
    assert!(status.get_namespace().is_none());
    assert!(status.is_enum());

    let cases = status.get_cases();

    assert!(cases.len() == 2);
    assert!(cases[0].get_name() == &ByteString::from(b"Active"));
    assert!(cases[1].get_name() == &ByteString::from(b"Inactive"));

    let color = index.get_class(&ByteString::from(b"Color")).unwrap();

    assert!(color.get_name() == &ByteString::from(b"Color"));
    assert!(color.get_short_name() == &ByteString::from(b"Color"));
    assert!(color.get_namespace().is_none());
    assert!(color.is_enum());

    let cases = color.get_cases();

    assert!(cases.len() == 3);
    assert!(cases[0].get_name() == &ByteString::from(b"Red"));
    assert!(cases[1].get_name() == &ByteString::from(b"Green"));
    assert!(cases[2].get_name() == &ByteString::from(b"Blue"));

    let methods = color.get_own_methods();

    assert!(methods.len() == 1);

    let get_hex = &methods[0];

    assert!(get_hex.get_name() == &ByteString::from(b"getHex"));
    assert!(get_hex.get_return_type() == &Type::String);
    assert!(get_hex.is_public());
}

fn index() -> Index {
    let mut index = Index::new();
    let mut indexer = Indexer::new(&mut index);
    let files = discover(&["php"], &["tests/fixtures"]).expect("failed to discover files");

    for file in files.iter() {
        let contents = std::fs::read(file).unwrap();
        let result = Parser::parse(Lexer::new(&contents));
        indexer.index(&result.ast);
    }

    index
}
