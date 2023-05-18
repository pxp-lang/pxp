<?php

namespace App\Common\Type;

class IntersectionType implements Type
{
    public function __construct(
        public array $types,
    ) {}

    public function __toString(): string
    {
        return implode('&', $this->types);
    }
}
