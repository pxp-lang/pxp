<?php

namespace App\Common\Type;

class IntType implements Type
{
    public function __toString(): string
    {
        return 'int';
    }
}
