<?php

namespace App\Common\Type;

class NamedType implements Type
{
    public function __construct(
        public readonly string $name,
    ) {}

    public function __toString(): string
    {
        return $this->name;
    }
}
