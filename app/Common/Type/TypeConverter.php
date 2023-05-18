<?php

namespace App\Common\Type;

use Pxp\Parser\Node\ComplexType;
use Pxp\Parser\Node\Identifier;
use Pxp\Parser\Node\IntersectionType;
use Pxp\Parser\Node\Name;
use Pxp\Parser\Node\NullableType;
use Pxp\Parser\Node\UnionType;

final class TypeConverter
{
    public static function fromParserType(Identifier|Name|NullableType|UnionType|IntersectionType|null $type): Type
    {
        return match (true) {
            $type instanceof Identifier => match ($type->toString()) {
                'self' => new SelfType,
                'static' => new StaticType,
                'parent' => new ParentType,
                'string' => new StringType,
                'bool' => new BoolType,
                'int' => new IntType,
                'object' => new ObjectType,
                'mixed' => new MixedType,
                'callable' => new CallableType,
                'iterable' => new IterableType,
                'array' => new ArrayType,
                'float' => new FloatType,
                'null' => new NullType,
                'false' => new FalseType,
                'void' => new VoidType,
                'true' => new TrueType,
                default => dd($type->toString()),
            },
            $type instanceof Name => new NamedType($type->toCodeString()),
            $type instanceof NullableType => new namespace\NullableType(self::fromParserType($type->type)),
            $type instanceof UnionType => new namespace\UnionType(array_map(fn ($type) => self::fromParserType($type), $type->types)),
            $type instanceof IntersectionType => new namespace\IntersectionType(array_map(fn ($type) => self::fromParserType($type), $type->types)),
            default => new MixedType,
        };
    }
}
