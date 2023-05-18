<?php

namespace App\Common\Type;

class FloatType implements Type
{
    public function __toString(): string
    {
        return 'float';
    }
}
