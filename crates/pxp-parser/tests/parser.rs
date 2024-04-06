use std::path::PathBuf;

use pxp_parser::parse;
use pxp_symbol::SymbolTable;
use snappers::{snap, Snapper};

// Tags & Inline HTML
snap!(snapper, empty_file, process("fixtures/tags/empty-file.php"));
snap!(snapper, tag, process("fixtures/tags/tag.php"));
snap!(snapper, short_tag, process("fixtures/tags/short-tag.php"));
snap!(snapper, echo_tag, process("fixtures/tags/echo-tag.php"));
snap!(snapper, html, process("fixtures/tags/html.php"));

// Echo
snap!(snapper, simple_echo, process("fixtures/echo/simple-echo.php"));
snap!(snapper, multi_echo, process("fixtures/echo/multi-echo.php"));
snap!(snapper, echo_no_value, process("fixtures/echo/echo-no-value.php"));
snap!(snapper, echo_single_value_trailing_comma, process("fixtures/echo/echo-single-value-trailing-comma.php"));
snap!(snapper, echo_missing_semicolon, process("fixtures/echo/echo-missing-semicolon.php"));

// Assignments
snap!(snapper, assign, process("fixtures/assignments/assign.php"));
snap!(snapper, multi_assign, process("fixtures/assignments/multi-assign.php"));
snap!(snapper, add_assign, process("fixtures/assignments/add-assign.php"));
snap!(snapper, sub_assign, process("fixtures/assignments/sub-assign.php"));
snap!(snapper, mul_assign, process("fixtures/assignments/mul-assign.php"));
snap!(snapper, div_assign, process("fixtures/assignments/div-assign.php"));
snap!(snapper, mod_assign, process("fixtures/assignments/mod-assign.php"));
snap!(snapper, exp_assign, process("fixtures/assignments/exp-assign.php"));
snap!(snapper, concat_assign, process("fixtures/assignments/concat-assign.php"));
snap!(snapper, bitwise_and_assign, process("fixtures/assignments/bitwise-and-assign.php"));
snap!(snapper, bitwise_or_assign, process("fixtures/assignments/bitwise-or-assign.php"));
snap!(snapper, bitwise_xor_assign, process("fixtures/assignments/bitwise-xor-assign.php"));
snap!(snapper, bitwise_left_shift_assign, process("fixtures/assignments/bitwise-left-shift-assign.php"));
snap!(snapper, bitwise_right_shift_assign, process("fixtures/assignments/bitwise-right-shift-assign.php"));
snap!(snapper, coalesce_assign, process("fixtures/assignments/coalesce-assign.php"));

// Arithmetic
snap!(snapper, add, process("fixtures/arithmetic/add.php"));
snap!(snapper, sub, process("fixtures/arithmetic/sub.php"));
snap!(snapper, mul, process("fixtures/arithmetic/mul.php"));
snap!(snapper, div, process("fixtures/arithmetic/div.php"));
snap!(snapper, mod_, process("fixtures/arithmetic/mod.php"));
snap!(snapper, exp, process("fixtures/arithmetic/exp.php"));
snap!(snapper, post_inc, process("fixtures/arithmetic/post-inc.php"));
snap!(snapper, post_dec, process("fixtures/arithmetic/post-dec.php"));
snap!(snapper, pre_inc, process("fixtures/arithmetic/pre-inc.php"));
snap!(snapper, pre_dec, process("fixtures/arithmetic/pre-dec.php"));

// Bitwise
snap!(snapper, bitwise_and, process("fixtures/bitwise/bitwise-and.php"));
snap!(snapper, bitwise_or, process("fixtures/bitwise/bitwise-or.php"));
snap!(snapper, bitwise_xor, process("fixtures/bitwise/bitwise-xor.php"));
snap!(snapper, bitwise_not, process("fixtures/bitwise/bitwise-not.php"));
snap!(snapper, bitwise_left_shift, process("fixtures/bitwise/bitwise-left-shift.php"));
snap!(snapper, bitwise_right_shift, process("fixtures/bitwise/bitwise-right-shift.php"));

// Comparison
snap!(snapper, equal, process("fixtures/comparison/equal.php"));
snap!(snapper, not_equal, process("fixtures/comparison/not-equal.php"));
snap!(snapper, identical, process("fixtures/comparison/identical.php"));
snap!(snapper, not_identical, process("fixtures/comparison/not-identical.php"));
snap!(snapper, less_than, process("fixtures/comparison/less-than.php"));
snap!(snapper, less_than_or_equal, process("fixtures/comparison/less-than-or-equal.php"));
snap!(snapper, greater_than, process("fixtures/comparison/greater-than.php"));
snap!(snapper, greater_than_or_equal, process("fixtures/comparison/greater-than-or-equal.php"));
snap!(snapper, spaceship, process("fixtures/comparison/spaceship.php"));

// Logical
snap!(snapper, and, process("fixtures/logical/and.php"));
snap!(snapper, or, process("fixtures/logical/or.php"));
snap!(snapper, xor, process("fixtures/logical/xor.php"));
snap!(snapper, not, process("fixtures/logical/not.php"));
snap!(snapper, logical_and, process("fixtures/logical/logical-and.php"));
snap!(snapper, logical_or, process("fixtures/logical/logical-or.php"));
snap!(snapper, logical_xor, process("fixtures/logical/logical-xor.php"));

// Literals
snap!(snapper, int, process("fixtures/literals/int.php"));
snap!(snapper, float, process("fixtures/literals/float.php"));
snap!(snapper, string, process("fixtures/literals/string.php"));
snap!(snapper, null, process("fixtures/literals/null.php"));
snap!(snapper, bool, process("fixtures/literals/bool.php"));
snap!(snapper, empty_array, process("fixtures/literals/empty-array.php"));
snap!(snapper, single_item_array, process("fixtures/literals/single-item-array.php"));
snap!(snapper, multi_item_array, process("fixtures/literals/multi-item-array.php"));
snap!(snapper, nested_array, process("fixtures/literals/nested-array.php"));
snap!(snapper, more_nested_array, process("fixtures/literals/more-nested-array.php"));
snap!(snapper, array_trailing_comma, process("fixtures/literals/array-trailing-comma.php"));
snap!(snapper, legacy_array, process("fixtures/literals/legacy-array.php"));
snap!(snapper, legacy_array_single_item, process("fixtures/literals/legacy-array-single-item.php"));

// Doc-strings
snap!(snapper, empty_heredoc, process("fixtures/docstrings/empty-heredoc.php"));
snap!(snapper, simple_heredoc, process("fixtures/docstrings/simple-heredoc.php"));
snap!(snapper, heredoc_interpolation, process("fixtures/docstrings/heredoc-interpolation.php"));
snap!(snapper, empty_nowdoc, process("fixtures/docstrings/empty-nowdoc.php"));
snap!(snapper, simple_nowdoc, process("fixtures/docstrings/simple-nowdoc.php"));

// Variables
snap!(snapper, variable, process("fixtures/variables/variable.php"));
snap!(snapper, variable_variable, process("fixtures/variables/variable-variable.php"));
snap!(snapper, variable_variable_complex, process("fixtures/variables/variable-variable-complex.php"));

// Constants
snap!(snapper, constant, process("fixtures/constants/constant.php"));
snap!(snapper, magic_constants, process("fixtures/constants/magic-constants.php"));

// Functions
snap!(snapper, simple_function, process("fixtures/functions/simple-function.php"));
snap!(snapper, simple_function_call, process("fixtures/functions/simple-function-call.php"));
snap!(snapper, simple_function_call_args, process("fixtures/functions/simple-function-call-args.php"));
snap!(snapper, simple_function_call_args_trailing_comma, process("fixtures/functions/simple-function-call-args-trailing-comma.php"));
snap!(snapper, function_with_parameter_types, process("fixtures/functions/function-with-parameter-types.php"));
snap!(snapper, function_with_return_type, process("fixtures/functions/function-with-return-type.php"));

// Control Structures
snap!(snapper, if_statement, process("fixtures/control/if-statement.php"));
snap!(snapper, if_else_statement, process("fixtures/control/if-else-statement.php"));
snap!(snapper, if_elseif_else_statement, process("fixtures/control/if-elseif-else-statement.php"));
snap!(snapper, if_elseif_else_statement_no_else, process("fixtures/control/if-elseif-statement-no-else.php"));
snap!(snapper, switch_statement, process("fixtures/control/switch-statement.php"));
snap!(snapper, switch_statement_no_case, process("fixtures/control/switch-statement-no-case.php"));
// snap!(snapper, while_statement, process("fixtures/control/while-statement.php"));
// snap!(snapper, do_while_statement, process("fixtures/control/do-while-statement.php"));
// snap!(snapper, for_statement, process("fixtures/control/for-statement.php"));
// snap!(snapper, foreach_statement, process("fixtures/control/foreach-statement.php"));
// snap!(snapper, foreach_statement_with_key, process("fixtures/control/foreach-statement-with-key.php"));
// snap!(snapper, match_expression, process("fixtures/control/match-expression.php"));
// snap!(snapper, match_expression_no_default, process("fixtures/control/match-expression-no-default.php"));
// snap!(snapper, match_expression_multiple_cases, process("fixtures/control/match-expression-multiple-cases.php"));

// Class
// snap!(snapper, simple_class, process("fixtures/simple-class.php"));
// snap!(snapper, class_with_properties, process("fixtures/class-with-properties.php"));
// snap!(snapper, class_with_methods, process("fixtures/class-with-methods.php"));
// snap!(snapper, class_with_constants, process("fixtures/class-with-constants.php"));
// snap!(snapper, class_with_static_properties, process("fixtures/class-with-static-properties.php"));
// snap!(snapper, class_with_static_methods, process("fixtures/class-with-static-methods.php"));
// snap!(snapper, class_with_extends, process("fixtures/class-with-extends.php"));
// snap!(snapper, class_with_implements, process("fixtures/class-with-implements.php"));
// snap!(snapper, class_with_extends_and_implements, process("fixtures/class-with-extends-and-implements.php"));
// snap!(snapper, final_class, process("fixtures/final-class.php"));
// snap!(snapper, abstract_class, process("fixtures/abstract-class.php"));
// snap!(snapper, readonly_class, process("fixtures/readonly-class.php"));
// snap!(snapper, class_with_traits, process("fixtures/class-with-traits.php"));
// snap!(snapper, class_with_traits_and_alias, process("fixtures/class-with-traits-and-alias.php"));
// snap!(snapper, class_with_traits_and_insteadof, process("fixtures/class-with-traits-and-insteadof.php"));
// snap!(snapper, class_with_traits_and_visibility, process("fixtures/class-with-traits-and-visibility.php"));
// snap!(snapper, class_with_attributes, process("fixtures/class-with-attributes.php"));

// Interfaces
// snap!(snapper, simple_interface, process("fixtures/simple-interface.php"));
// snap!(snapper, interface_with_constants, process("fixtures/interface-with-constants.php"));
// snap!(snapper, interface_with_methods, process("fixtures/interface-with-methods.php"));
// snap!(snapper, interface_with_extends, process("fixtures/interface-with-extends.php"));

// Traits
// snap!(snapper, simple_trait, process("fixtures/simple-trait.php"));
// snap!(snapper, trait_with_properties, process("fixtures/trait-with-properties.php"));
// snap!(snapper, trait_with_methods, process("fixtures/trait-with-methods.php"));
// snap!(snapper, trait_with_abstract_methods, process("fixtures/trait-with-abstract-methods.php"));
// snap!(snapper, trait_with_static_methods, process("fixtures/trait-with-static-methods.php"));
// snap!(snapper, trait_with_trait_use, process("fixtures/trait-with-trait-use.php"));

// Enums
// snap!(snapper, simple_enum, process("fixtures/simple-enum.php"));
// snap!(snapper, backed_enum_string, process("fixtures/backed-enum-string.php"));
// snap!(snapper, backed_enum_int, process("fixtures/backed-enum-int.php"));
// snap!(snapper, enum_with_constants, process("fixtures/enum-with-constants.php"));
// snap!(snapper, enum_with_methods, process("fixtures/enum-with-methods.php"));
// snap!(snapper, enum_with_implements, process("fixtures/enum-with-implements.php"));
// snap!(snapper, enum_with_attributes, process("fixtures/enum-with-attributes.php"));

// Namespaces
// snap!(snapper, unbraced_namespace, process("fixtures/unbraced-namespace.php"));
// snap!(snapper, braced_namespace, process("fixtures/braced-namespace.php"));
// snap!(snapper, global_namespace, process("fixtures/global-namespace.php"));

// Class Constants
snap!(snapper, class_const, process("fixtures/class-constants/class-const.php"));
snap!(snapper, multi_class_const, process("fixtures/class-constants/multi-class-const.php"));
snap!(snapper, typed_class_const, process("fixtures/class-constants/typed-class-const.php"));
snap!(snapper, dynamic_class_const, process("fixtures/class-constants/dynamic-class-const.php"));
// snap!(snapper, class_const_with_attributes, process("fixtures/class-constants/class-const-with-attributes.php"));

// Methods
// snap!(snapper, method, process("fixtures/method.php"));
// snap!(snapper, method_with_visibility, process("fixtures/method-with-visibility.php"));
// snap!(snapper, method_with_static, process("fixtures/method-with-static.php"));
// snap!(snapper, method_with_final, process("fixtures/method-with-final.php"));
// snap!(snapper, method_with_abstract, process("fixtures/method-with-abstract.php"));
// snap!(snapper, method_with_return_type, process("fixtures/method-with-return-type.php"));
// snap!(snapper, method_with_parameters, process("fixtures/method-with-parameters.php"));
// snap!(snapper, method_with_parameters_with_default, process("fixtures/method-with-parameters-with-default.php"));
// snap!(snapper, method_with_parameters_with_type, process("fixtures/method-with-parameters-with-type.php"));
// snap!(snapper, method_with_parameters_with_type_and_default, process("fixtures/method-with-parameters-with-type-and-default.php"));
// snap!(snapper, method_with_attributes, process("fixtures/method-with-attributes.php"));

// Identifier Qualification
snap!(snapper, unqualified_identifier, process("fixtures/identifiers/unqualified-identifier.php"));
snap!(snapper, qualified_identifier, process("fixtures/identifiers/qualified-identifier.php"));
snap!(snapper, fully_qualified_identifier, process("fixtures/identifiers/fully-qualified-identifier.php"));

fn snapper() -> Snapper {
    Snapper::new(
        format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "tests/__snapshots__").into()
    )
}

fn process(string_or_file: &str) -> String {
    let path = format!("{}/tests/{}", env!("CARGO_MANIFEST_DIR"), string_or_file);
    let path = PathBuf::from(path);
    let input = if path.exists() {
        std::fs::read(path).unwrap()
    } else {
        string_or_file.as_bytes().to_vec()
    };

    let mut symbol_table = SymbolTable::the();
    let result = parse(&input, &mut symbol_table);
    let mut output = format!("{:#?}\n---\n", result.ast);

    if !result.diagnostics.is_empty() {
        output.push_str(
            &result.diagnostics.iter().map(|d| d.to_string()).collect::<Vec<String>>().join("\n")
        );
    }

    output
}