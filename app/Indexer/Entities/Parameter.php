<?php

namespace App\Indexer\Entities;

use App\Common\Type\Type;

class Parameter
{
    public function __construct(
        public readonly string $name,
        public readonly Type $type,
        public readonly bool $variadic,
    ) {}
}
