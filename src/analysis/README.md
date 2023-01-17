# Analysis

This directory contains everything required to statically analyse PHP & PXP programs.

## Overview

PXP's static analysis engine has two separate use cases.
1. It can typecheck your code without needing to execute it.
2. It can provide an insight into your code that allows the transpiler to do magical things.

The analyser itself goes through multiple steps, each of the steps is detailed below for contributors and people who are interested.

## Definition Collection and Population

The first step in analysis is collecting information about various things that your code defines, e.g. functions, classes and interfaces. 

To do this, the `DefinitionCollector` will be provided an abstract syntax tree from each and every file found (recursively) within your project. The `DefinitionCollector` isn't responsible for type checking those definitions, it is purely a collection process so that we can use that information later on in the typechecker.

Here's an example of what the collector might produce.

```php
namespace App;

class User
{
    public string $name;

    public function validate(): bool
    {
        return validate($this->toArray(), [
            'name' => ['required', 'string'],
        ]);
    }

    public static function create(array $data = []): static
    {
        // ...
    }
}
```

```php
function validate(array $data, array $rules = []): bool {
    // ...
}
```

```rust
DefinitionCollection {
    classes: [
        r"\App\User" => ClassDefinition {
            properties: [
                "$name" => PropertyDefinition {
                    visibility: Visibility::Public,
                    modifier: None,
                    r#type: Type::String,
                    initialized: false,
                }
            ],
            methods: [
                "validate" => MethodDefinition {
                    visibilty: Visibility::Public,
                    modifier: None,
                    parameters: [],
                    return_type: Type::Bool,
                },
                "create" => StaticMethodDefinition {
                    visibility: Visibility::Public,
                    modifier: None,
                    parameters: [
                        "$data" => Parameter {
                            r#type: Type::Array,
                            variadic: false,
                            required: false,
                        }
                    ],
                    return_type: Type::Static,
                }
            ]
            // ...
        }
    ],
    functions: [
        r"\validate" => FunctionDefinition {
            parameters: [
                "$data" => Parameter {
                    r#type: Type::Array,
                    variadic: false,
                    required: true,
                },
                "$rules" => Parameter {
                    r#type: Type::Array,
                    variadic: false,
                    required: false,
                }
            ]
        }
    ]
}
```

This collection pass is a shallow one. The collector will only store information about the definition itself and will not drill down into inherited methods, etc. The `DefinitionCollection` stores enough information about inherited classes, interfaces implemented and traits being used that the typechecker can resolve that information later on with additional checks on those associated definitions.

## Scope

The `Scope` structure is one of the core components in the analyser. It's responsibility is keeping track of variable types and the current context. 

Taking the PHP code below:

```php
<?php

$guess = readline("Guess a number between 1 and 50: ");
$number = rand(1, 50);

if ($guess === $number) {
    echo "You guessed the number correctly!\n";
}
```

The `DefinitionCollector` will execute first and scan the entire project, including our internal stub files that define types for all of PHP's native functions and various extensions.

The `FileAnalyser` will then come into play and start analysing the script from top to bottom.

It starts by checking the call to `readline()`.
* Is `readline()` a valid function name?
* Does it accept an argument?
* If it does accept an argument, it is a `string` or does it need to be a different type?

It will then find the return type of `readline()` and store that in the `Scope` as the type of the `$guess` variable. The same thing also happens for `$number` and the call to `rand()`.

When the analyser reaches the conditional statement, it needs to do a little bit of extra work.
* Does the condition produce a `bool`?
* Can the left and right hand side of the condition between identically compared?

At this point, the analyser will log an error because `readline()` returns `string|false` but `rand()` will return `int`. It's not possible to identically compare these two values since they're not the same type. It would be silly for the analyser to just stop here though, since it still knows that the if statement has a body and set of statements to analyse.

It will then check the `echo` statement to ensure that the values being `echo`d can be converted into strings, i.e. `(string) $value` is a valid operation. This includes checks on objects to see if they implement the `Stringable` interface or have a `__toString()` method defined / inherited.

At the end of the file, the `Scope` will know that `$guess` is a `string|false` and that `$number` is an `int`. To fix the code, we could cast `$guess` to an `int` using one of the following operations:
* `(int) ...` - explicit cast to integer.
* `+$a` - implicit cast using unary `+` operator.
* `intval()` - call to native function that has a return type of `int`.

## Rules

All of the checks used by the analyser is implemented as a single struct that implements a trait. Implementing the `Rule` trait on a struct only requires defining 2 methods:

```rust
trait Rule {
    /// Decides whether or not this rule should be processed for the current node.
    fn accepts_node(&self, node: &dyn Node) -> bool;

    /// Process the current node and provide messages based on the checks performed by the rule.
    fn process(&mut self, node: &dyn Node, scope: &Scope, messages: &mut MessageCollection);
}
```

If we wanted to write an example rule to check that a function call is calling a function that actually exists, we might do something like this:

```rust
struct ValidFunctionCallRule;

impl Rule for ValidFunctionCallRule {
    fn accepts_node(&self, node: &dyn Node) -> bool {
        downcast::<FunctionCallExpression>(node).is_some()
    }

    fn process(&mut self, node: &dyn Node, scope: &Scope, messages: &mut MessageCollection) {
        let Some(FunctionCallExpression { target, arguments, .. }) = downcast::<FunctionCallExpression>(node);

        match target {
            Expression::Identifier(Identifier::SimpleIdentifier(SimpleIdentifier { value, span })) => {
                // `Scope::definition(name: ByteString)` will search the global `DefinitionCollection`
                // for the specified definition and return a reference to it.
                //
                // Since the scope keeps track of the current namespace during analysis, it is smart
                // enough to fully-qualify the provided name for you, e.g.
                // namespace = App
                // definition("foo")
                // - Checks imported symbols (`use ...`) for something that ends with `foo`.
                // - Checks for `\App\foo` as a definition.
                // - Checks for `\foo` as a definition.
                //
                // If it doesn't find one, it will return `None`.
                if scope.definition(&value).is_none() {
                    messages.error(
                        format!("Function {} not found.", value),
                        span.line,
                        span.column,
                    );
                }
            },
            // Checking dynamic function calls requires looking up variable types in the current scope,
            // so we can skip that for now and do the work in a different rule.
            _ => return,
        }
    }
}
```