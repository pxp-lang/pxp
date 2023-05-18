<?php

namespace App\Common\Type;

class ArrayType implements Type
{
    public function __toString(): string
    {
        return 'array';
    }
}
