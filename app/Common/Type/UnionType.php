<?php

namespace App\Common\Type;

class UnionType implements Type
{
    public function __construct(
        public array $types,
    ) {}

    public function __toString(): string
    {
        return implode('|', $this->types);
    }
}
