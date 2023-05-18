<?php

namespace App\Indexer\Entities;

use App\Common\Location;
use App\Common\Type\Type;

class FunctionEntity
{
    public function __construct(
        public readonly string $namespacedName,
        public readonly string $name,
        public readonly array $parameters,
        public readonly Type $returnType,
        public readonly bool $returnsByRef,
        public readonly Location $location,
    ) {}
}
