use std::path::PathBuf;

use pxp_parser::parse;

use snappers::{snap, Snapper};

// Tags
snap!(snapper, empty_file, process("fixtures/tags/empty-file.php"));
snap!(snapper, tag, process("fixtures/tags/tag.php"));
snap!(snapper, short_tag, process("fixtures/tags/short-tag.php"));
snap!(snapper, echo_tag, process("fixtures/tags/echo-tag.php"));

// Echo
snap!(
    snapper,
    simple_echo,
    process("fixtures/echo/simple-echo.php")
);
snap!(snapper, multi_echo, process("fixtures/echo/multi-echo.php"));
snap!(
    snapper,
    echo_no_value,
    process("fixtures/echo/echo-no-value.php")
);
snap!(
    snapper,
    echo_single_value_trailing_comma,
    process("fixtures/echo/echo-single-value-trailing-comma.php")
);
snap!(
    snapper,
    echo_missing_semicolon,
    process("fixtures/echo/echo-missing-semicolon.php")
);

// Assignments
snap!(snapper, assign, process("fixtures/assignments/assign.php"));
snap!(
    snapper,
    multi_assign,
    process("fixtures/assignments/multi-assign.php")
);
snap!(
    snapper,
    add_assign,
    process("fixtures/assignments/add-assign.php")
);
snap!(
    snapper,
    sub_assign,
    process("fixtures/assignments/sub-assign.php")
);
snap!(
    snapper,
    mul_assign,
    process("fixtures/assignments/mul-assign.php")
);
snap!(
    snapper,
    div_assign,
    process("fixtures/assignments/div-assign.php")
);
snap!(
    snapper,
    mod_assign,
    process("fixtures/assignments/mod-assign.php")
);
snap!(
    snapper,
    exp_assign,
    process("fixtures/assignments/exp-assign.php")
);
snap!(
    snapper,
    concat_assign,
    process("fixtures/assignments/concat-assign.php")
);
snap!(
    snapper,
    bitwise_and_assign,
    process("fixtures/assignments/bitwise-and-assign.php")
);
snap!(
    snapper,
    bitwise_or_assign,
    process("fixtures/assignments/bitwise-or-assign.php")
);
snap!(
    snapper,
    bitwise_xor_assign,
    process("fixtures/assignments/bitwise-xor-assign.php")
);
snap!(
    snapper,
    bitwise_left_shift_assign,
    process("fixtures/assignments/bitwise-left-shift-assign.php")
);
snap!(
    snapper,
    bitwise_right_shift_assign,
    process("fixtures/assignments/bitwise-right-shift-assign.php")
);
snap!(
    snapper,
    coalesce_assign,
    process("fixtures/assignments/coalesce-assign.php")
);

// Arithmetic
snap!(snapper, add, process("fixtures/arithmetic/add.php"));
snap!(snapper, sub, process("fixtures/arithmetic/sub.php"));
snap!(snapper, mul, process("fixtures/arithmetic/mul.php"));
snap!(snapper, div, process("fixtures/arithmetic/div.php"));
snap!(snapper, mod_, process("fixtures/arithmetic/mod.php"));
snap!(snapper, exp, process("fixtures/arithmetic/exp.php"));
snap!(
    snapper,
    post_inc,
    process("fixtures/arithmetic/post-inc.php")
);
snap!(
    snapper,
    post_dec,
    process("fixtures/arithmetic/post-dec.php")
);
snap!(snapper, pre_inc, process("fixtures/arithmetic/pre-inc.php"));
snap!(snapper, pre_dec, process("fixtures/arithmetic/pre-dec.php"));

// Bitwise
snap!(
    snapper,
    bitwise_and,
    process("fixtures/bitwise/bitwise-and.php")
);
snap!(
    snapper,
    bitwise_or,
    process("fixtures/bitwise/bitwise-or.php")
);
snap!(
    snapper,
    bitwise_xor,
    process("fixtures/bitwise/bitwise-xor.php")
);
snap!(
    snapper,
    bitwise_not,
    process("fixtures/bitwise/bitwise-not.php")
);
snap!(
    snapper,
    bitwise_left_shift,
    process("fixtures/bitwise/bitwise-left-shift.php")
);
snap!(
    snapper,
    bitwise_right_shift,
    process("fixtures/bitwise/bitwise-right-shift.php")
);

// Comparison
snap!(snapper, equal, process("fixtures/comparison/equal.php"));
snap!(
    snapper,
    not_equal,
    process("fixtures/comparison/not-equal.php")
);
snap!(
    snapper,
    identical,
    process("fixtures/comparison/identical.php")
);
snap!(
    snapper,
    not_identical,
    process("fixtures/comparison/not-identical.php")
);
snap!(
    snapper,
    less_than,
    process("fixtures/comparison/less-than.php")
);
snap!(
    snapper,
    less_than_or_equal,
    process("fixtures/comparison/less-than-or-equal.php")
);
snap!(
    snapper,
    greater_than,
    process("fixtures/comparison/greater-than.php")
);
snap!(
    snapper,
    greater_than_or_equal,
    process("fixtures/comparison/greater-than-or-equal.php")
);
snap!(
    snapper,
    spaceship,
    process("fixtures/comparison/spaceship.php")
);

// Logical
snap!(snapper, and, process("fixtures/logical/and.php"));
snap!(snapper, or, process("fixtures/logical/or.php"));
snap!(snapper, xor, process("fixtures/logical/xor.php"));
snap!(snapper, not, process("fixtures/logical/not.php"));
snap!(
    snapper,
    logical_and,
    process("fixtures/logical/logical-and.php")
);
snap!(
    snapper,
    logical_or,
    process("fixtures/logical/logical-or.php")
);
snap!(
    snapper,
    logical_xor,
    process("fixtures/logical/logical-xor.php")
);

// Literals
snap!(snapper, int, process("fixtures/literals/int.php"));
snap!(snapper, float, process("fixtures/literals/float.php"));
snap!(snapper, string, process("fixtures/literals/string.php"));
snap!(snapper, null, process("fixtures/literals/null.php"));
snap!(snapper, bool, process("fixtures/literals/bool.php"));
snap!(
    snapper,
    empty_array,
    process("fixtures/literals/empty-array.php")
);
snap!(
    snapper,
    single_item_array,
    process("fixtures/literals/single-item-array.php")
);
snap!(
    snapper,
    multi_item_array,
    process("fixtures/literals/multi-item-array.php")
);
snap!(
    snapper,
    nested_array,
    process("fixtures/literals/nested-array.php")
);
snap!(
    snapper,
    more_nested_array,
    process("fixtures/literals/more-nested-array.php")
);
snap!(
    snapper,
    array_trailing_comma,
    process("fixtures/literals/array-trailing-comma.php")
);
snap!(
    snapper,
    legacy_array,
    process("fixtures/literals/legacy-array.php")
);
snap!(
    snapper,
    legacy_array_single_item,
    process("fixtures/literals/legacy-array-single-item.php")
);

// Doc-strings
snap!(
    snapper,
    empty_heredoc,
    process("fixtures/docstrings/empty-heredoc.php")
);
snap!(
    snapper,
    simple_heredoc,
    process("fixtures/docstrings/simple-heredoc.php")
);
snap!(
    snapper,
    heredoc_interpolation,
    process("fixtures/docstrings/heredoc-interpolation.php")
);
snap!(
    snapper,
    empty_nowdoc,
    process("fixtures/docstrings/empty-nowdoc.php")
);
snap!(
    snapper,
    simple_nowdoc,
    process("fixtures/docstrings/simple-nowdoc.php")
);

// Variables
snap!(
    snapper,
    variable,
    process("fixtures/variables/variable.php")
);
snap!(
    snapper,
    variable_variable,
    process("fixtures/variables/variable-variable.php")
);
snap!(
    snapper,
    variable_variable_complex,
    process("fixtures/variables/variable-variable-complex.php")
);

// Constants
snap!(
    snapper,
    constant,
    process("fixtures/constants/constant.php")
);
snap!(
    snapper,
    magic_constants,
    process("fixtures/constants/magic-constants.php")
);

// Functions
snap!(
    snapper,
    simple_function,
    process("fixtures/functions/simple-function.php")
);
snap!(
    snapper,
    simple_function_call,
    process("fixtures/functions/simple-function-call.php")
);
snap!(
    snapper,
    simple_function_call_args,
    process("fixtures/functions/simple-function-call-args.php")
);
snap!(
    snapper,
    simple_function_call_args_trailing_comma,
    process("fixtures/functions/simple-function-call-args-trailing-comma.php")
);
snap!(
    snapper,
    function_with_parameter_types,
    process("fixtures/functions/function-with-parameter-types.php")
);
snap!(
    snapper,
    function_with_return_type,
    process("fixtures/functions/function-with-return-type.php")
);

// Control Structures
snap!(
    snapper,
    if_statement,
    process("fixtures/control/if-statement.php")
);
snap!(
    snapper,
    if_else_statement,
    process("fixtures/control/if-else-statement.php")
);
snap!(
    snapper,
    if_elseif_else_statement,
    process("fixtures/control/if-elseif-else-statement.php")
);
snap!(
    snapper,
    if_elseif_else_statement_no_else,
    process("fixtures/control/if-elseif-statement-no-else.php")
);
snap!(
    snapper,
    switch_statement,
    process("fixtures/control/switch-statement.php")
);
snap!(
    snapper,
    switch_statement_no_case,
    process("fixtures/control/switch-statement-no-case.php")
);
snap!(
    snapper,
    while_statement,
    process("fixtures/control/while-statement.php")
);
snap!(
    snapper,
    do_while_statement,
    process("fixtures/control/do-while-statement.php")
);
snap!(
    snapper,
    for_statement,
    process("fixtures/control/for-statement.php")
);
snap!(
    snapper,
    foreach_statement,
    process("fixtures/control/foreach-statement.php")
);
snap!(
    snapper,
    foreach_statement_with_key,
    process("fixtures/control/foreach-statement-with-key.php")
);
snap!(
    snapper,
    match_expression,
    process("fixtures/control/match-expression.php")
);
snap!(
    snapper,
    match_expression_no_default,
    process("fixtures/control/match-expression-no-default.php")
);

// Class
snap!(
    snapper,
    simple_class,
    process("fixtures/classes/simple-class.php")
);
snap!(
    snapper,
    class_with_properties,
    process("fixtures/classes/class-with-properties.php")
);
snap!(
    snapper,
    class_with_methods,
    process("fixtures/classes/class-with-methods.php")
);
snap!(
    snapper,
    class_with_constants,
    process("fixtures/classes/class-with-constants.php")
);
snap!(
    snapper,
    class_with_static_properties,
    process("fixtures/classes/class-with-static-properties.php")
);
snap!(
    snapper,
    class_with_static_methods,
    process("fixtures/classes/class-with-static-methods.php")
);
snap!(
    snapper,
    class_with_extends,
    process("fixtures/classes/class-with-extends.php")
);
snap!(
    snapper,
    class_with_implements,
    process("fixtures/classes/class-with-implements.php")
);
snap!(
    snapper,
    class_with_extends_and_implements,
    process("fixtures/classes/class-with-extends-and-implements.php")
);
snap!(
    snapper,
    final_class,
    process("fixtures/classes/final-class.php")
);
snap!(
    snapper,
    abstract_class,
    process("fixtures/classes/abstract-class.php")
);
snap!(
    snapper,
    readonly_class,
    process("fixtures/classes/readonly-class.php")
);
snap!(
    snapper,
    class_with_traits,
    process("fixtures/classes/class-with-traits.php")
);
snap!(
    snapper,
    class_with_traits_and_alias,
    process("fixtures/classes/class-with-traits-and-alias.php")
);
snap!(
    snapper,
    class_with_traits_and_insteadof,
    process("fixtures/classes/class-with-traits-and-insteadof.php")
);
snap!(
    snapper,
    class_with_traits_and_visibility,
    process("fixtures/classes/class-with-traits-and-visibility.php")
);
snap!(
    snapper,
    class_with_attributes,
    process("fixtures/classes/class-with-attributes.php")
);

// Interfaces
snap!(
    snapper,
    simple_interface,
    process("fixtures/interfaces/simple-interface.php")
);
snap!(
    snapper,
    interface_with_constants,
    process("fixtures/interfaces/interface-with-constants.php")
);
snap!(
    snapper,
    interface_with_methods,
    process("fixtures/interfaces/interface-with-methods.php")
);
snap!(
    snapper,
    interface_with_extends,
    process("fixtures/interfaces/interface-with-extends.php")
);

// Traits
snap!(
    snapper,
    simple_trait,
    process("fixtures/traits/simple-trait.php")
);
snap!(
    snapper,
    trait_with_properties,
    process("fixtures/traits/trait-with-properties.php")
);
snap!(
    snapper,
    trait_with_methods,
    process("fixtures/traits/trait-with-methods.php")
);
snap!(
    snapper,
    trait_with_trait_use,
    process("fixtures/traits/trait-with-trait-use.php")
);

// Enums
snap!(
    snapper,
    simple_enum,
    process("fixtures/enums/simple-enum.php")
);
snap!(
    snapper,
    backed_enum_string,
    process("fixtures/enums/backed-enum-string.php")
);
snap!(
    snapper,
    backed_enum_int,
    process("fixtures/enums/backed-enum-int.php")
);
snap!(
    snapper,
    enum_with_constants,
    process("fixtures/enums/enum-with-constants.php")
);
snap!(
    snapper,
    enum_with_methods,
    process("fixtures/enums/enum-with-methods.php")
);
snap!(
    snapper,
    enum_with_implements,
    process("fixtures/enums/enum-with-implements.php")
);
snap!(
    snapper,
    enum_with_attributes,
    process("fixtures/enums/enum-with-attributes.php")
);

// Namespaces
snap!(
    snapper,
    unbraced_namespace,
    process("fixtures/namespaces/unbraced-namespace.php")
);
snap!(
    snapper,
    braced_namespace,
    process("fixtures/namespaces/braced-namespace.php")
);
snap!(
    snapper,
    global_namespace,
    process("fixtures/namespaces/global-namespace.php")
);

// Class Constants
snap!(
    snapper,
    class_const,
    process("fixtures/class-constants/class-const.php")
);
snap!(
    snapper,
    multi_class_const,
    process("fixtures/class-constants/multi-class-const.php")
);
snap!(
    snapper,
    typed_class_const,
    process("fixtures/class-constants/typed-class-const.php")
);
snap!(
    snapper,
    dynamic_class_const,
    process("fixtures/class-constants/dynamic-class-const.php")
);
snap!(
    snapper,
    class_const_with_attributes,
    process("fixtures/class-constants/class-const-with-attributes.php")
);

// Methods
snap!(snapper, method, process("fixtures/methods/method.php"));
snap!(
    snapper,
    method_with_visibility,
    process("fixtures/methods/method-with-visibility.php")
);
snap!(
    snapper,
    method_with_static,
    process("fixtures/methods/method-with-static.php")
);
snap!(
    snapper,
    method_with_final,
    process("fixtures/methods/method-with-final.php")
);
snap!(
    snapper,
    method_with_abstract,
    process("fixtures/methods/method-with-abstract.php")
);
snap!(
    snapper,
    method_with_return_type,
    process("fixtures/methods/method-with-return-type.php")
);
snap!(
    snapper,
    method_with_parameters,
    process("fixtures/methods/method-with-parameters.php")
);
snap!(
    snapper,
    method_with_parameters_with_default,
    process("fixtures/methods/method-with-parameters-with-default.php")
);
snap!(
    snapper,
    method_with_parameters_with_type,
    process("fixtures/methods/method-with-parameters-with-type.php")
);
snap!(
    snapper,
    method_with_parameters_with_type_and_default,
    process("fixtures/methods/method-with-parameters-with-type-and-default.php")
);
snap!(
    snapper,
    method_with_attributes,
    process("fixtures/methods/method-with-attributes.php")
);

// Identifier Qualification
snap!(
    snapper,
    unqualified_identifier,
    process("fixtures/identifiers/unqualified-identifier.php")
);
snap!(
    snapper,
    qualified_identifier,
    process("fixtures/identifiers/qualified-identifier.php")
);
snap!(
    snapper,
    fully_qualified_identifier,
    process("fixtures/identifiers/fully-qualified-identifier.php")
);

// Inline HTML
snap!(snapper, html, process("fixtures/tags/html.php"));
snap!(
    snapper,
    inline_html_with_php,
    process("fixtures/html/inline-html-with-php.php")
);

// Name Resolving
snap!(
    snapper,
    class_in_namespace,
    process("fixtures/name-resolving/class-in-namespace.php")
);
snap!(
    snapper,
    trait_in_namespace,
    process("fixtures/name-resolving/trait-in-namespace.php")
);
snap!(
    snapper,
    interface_in_namespace,
    process("fixtures/name-resolving/interface-in-namespace.php")
);
snap!(
    snapper,
    enum_in_namespace,
    process("fixtures/name-resolving/enum-in-namespace.php")
);
snap!(
    snapper,
    class_extends,
    process("fixtures/name-resolving/class-extends.php")
);
snap!(
    snapper,
    class_extends_aliased,
    process("fixtures/name-resolving/class-extends-aliased.php")
);
snap!(
    snapper,
    class_extends_qualified_aliased,
    process("fixtures/name-resolving/class-extends-qualified-aliased.php")
);
snap!(
    snapper,
    interface_extends,
    process("fixtures/name-resolving/interface-extends.php")
);
snap!(
    snapper,
    class_implements,
    process("fixtures/name-resolving/class-implements.php")
);
snap!(
    snapper,
    class_implements_aliased,
    process("fixtures/name-resolving/class-implements-aliased.php")
);
snap!(
    snapper,
    class_implements_qualified_aliased,
    process("fixtures/name-resolving/class-implements-qualified-aliased.php")
);
snap!(
    snapper,
    function_in_namespace,
    process("fixtures/name-resolving/function-in-namespace.php")
);
snap!(
    snapper,
    attribute,
    process("fixtures/name-resolving/attribute.php")
);
snap!(
    snapper,
    attribute_use,
    process("fixtures/name-resolving/attribute-use.php")
);
snap!(
    snapper,
    attribute_use_aliased,
    process("fixtures/name-resolving/attribute-use-aliased.php")
);
snap!(
    snapper,
    new_class,
    process("fixtures/name-resolving/new-class.php")
);
snap!(
    snapper,
    new_class_fqn,
    process("fixtures/name-resolving/new-class-fqn.php")
);
snap!(
    snapper,
    new_class_aliased,
    process("fixtures/name-resolving/new-class-aliased.php")
);
snap!(
    snapper,
    new_class_qualified,
    process("fixtures/name-resolving/new-class-qualified.php")
);
snap!(
    snapper,
    new_class_qualified_aliased,
    process("fixtures/name-resolving/new-class-qualified-aliased.php")
);
snap!(
    snapper,
    new_self,
    process("fixtures/name-resolving/new-self.php")
);
snap!(
    snapper,
    new_static,
    process("fixtures/name-resolving/new-static.php")
);
snap!(
    snapper,
    new_parent,
    process("fixtures/name-resolving/new-parent.php")
);
snap!(
    snapper,
    function_call_unqualified,
    process("fixtures/name-resolving/function-call-unqualified.php")
);
snap!(
    snapper,
    function_call_qualified,
    process("fixtures/name-resolving/function-call-qualified.php")
);
snap!(
    snapper,
    function_call_use,
    process("fixtures/name-resolving/function-call-use.php")
);
snap!(
    snapper,
    enum_static,
    process("fixtures/name-resolving/enum-static.php")
);
snap!(
    snapper,
    enum_static_use,
    process("fixtures/name-resolving/enum-static-use.php")
);
snap!(
    snapper,
    enum_static_use_alias,
    process("fixtures/name-resolving/enum-static-use-alias.php")
);
snap!(
    snapper,
    from_static,
    process("fixtures/name-resolving/from-static.php")
);
snap!(
    snapper,
    from_static_use,
    process("fixtures/name-resolving/from-static-use.php")
);
snap!(
    snapper,
    from_static_use_alias,
    process("fixtures/name-resolving/from-static-use-alias.php")
);
snap!(
    snapper,
    function_argument_unqualified,
    process("fixtures/name-resolving/function-argument-unqualified.php")
);
snap!(
    snapper,
    function_argument_unqualified_use,
    process("fixtures/name-resolving/function-argument-unqualified-use.php")
);
snap!(
    snapper,
    method_argument_unqualified,
    process("fixtures/name-resolving/method-argument-unqualified.php")
);
snap!(
    snapper,
    method_argument_unqualified_use,
    process("fixtures/name-resolving/method-argument-unqualified-use.php")
);
snap!(
    snapper,
    constant_unnamespaced,
    process("fixtures/name-resolving/constant-unnamespaced.php")
);
snap!(
    snapper,
    constant_namespaced,
    process("fixtures/name-resolving/constant-namespaced.php")
);
snap!(
    snapper,
    constant_use,
    process("fixtures/name-resolving/constant-use.php")
);

// Uses
snap!(snapper, simple_use, process("fixtures/uses/simple-use.php"));
snap!(
    snapper,
    qualified_use,
    process("fixtures/uses/qualified-use.php")
);
snap!(
    snapper,
    use_with_alias,
    process("fixtures/uses/use-with-alias.php")
);
snap!(snapper, group_use, process("fixtures/uses/group-use.php"));
snap!(
    snapper,
    group_use_multiple_types,
    process("fixtures/uses/group-use-multiple-types.php")
);

// Precedence Testing
snap!(
    snapper,
    binary_op_right_hand_assignment,
    process("fixtures/precedence/binary-op-right-hand-assignment.php")
);
snap!(
    snapper,
    binary_op_multi_right_hand_assignment,
    process("fixtures/precedence/binary-op-multi-right-hand-assignment.php")
);
snap!(
    snapper,
    multi_op_arithmetic,
    process("fixtures/precedence/multi-op-arithmetic.php")
);

// DocBlocks
snap!(snapper, docblock_empty, process("fixtures/docblocks/empty.php"));
snap!(snapper, docblock_empty_multiline, process("fixtures/docblocks/empty-multiline.php"));
snap!(snapper, docblock_text, process("fixtures/docblocks/text.php"));
snap!(snapper, docblock_text_multiline, process("fixtures/docblocks/text-multiline.php"));

snap!(snapper, docblock_param_empty, process("fixtures/docblocks/param-empty.php"));
snap!(snapper, docblock_param_type, process("fixtures/docblocks/param-type.php"));
snap!(snapper, docblock_param_nullable_type, process("fixtures/docblocks/param-nullable-type.php"));
snap!(snapper, docblock_param_union_type, process("fixtures/docblocks/param-union-type.php"));
snap!(snapper, docblock_param_intersection_type, process("fixtures/docblocks/param-intersection-type.php"));
snap!(snapper, docblock_param_dnf_type, process("fixtures/docblocks/param-dnf-type.php"));

snap!(snapper, docblock_param_variable, process("fixtures/docblocks/param-variable.php"));
snap!(snapper, docblock_param_variable_description, process("fixtures/docblocks/param-variable-description.php"));
snap!(snapper, docblock_param_type_variable, process("fixtures/docblocks/param-type-variable.php"));
snap!(snapper, docblock_param_type_variable_description, process("fixtures/docblocks/param-type-variable-description.php"));

fn snapper() -> Snapper {
    Snapper::new(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "tests/__snapshots__").into())
}

fn process(string_or_file: &str) -> String {
    let path = format!("{}/tests/{}", env!("CARGO_MANIFEST_DIR"), string_or_file);
    let path = PathBuf::from(path);
    let input = if path.exists() {
        std::fs::read(path).unwrap()
    } else {
        string_or_file.as_bytes().to_vec()
    };

    let result = parse(&input);
    let mut output = format!("{:#?}\n---\n", result.ast);

    if !result.diagnostics.is_empty() {
        output.push_str(&format!("{:#?}", &result.diagnostics));
    }

    output
}
