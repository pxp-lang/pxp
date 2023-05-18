<?php

namespace App\Indexer;

use App\Indexer\Entities\FunctionEntity;

final class Index
{
    private array $functions = [];

    public function addFunction(string $name, FunctionEntity $function): void
    {
        $this->functions[$name] = $function;
    }

    public function function(string $name): ?FunctionEntity
    {
        return $this->functions[$name] ?? null;
    }
}
