<?php

namespace App\Common\Type;

class BoolType implements Type
{
    public function __toString(): string
    {
        return 'bool';
    }
}
