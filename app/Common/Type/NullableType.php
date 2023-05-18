<?php

namespace App\Common\Type;

class NullableType implements Type
{
    public function __construct(
        public Type $type,
    ) {}

    public function __toString(): string
    {
        return "?{$this->type}";
    }
}
