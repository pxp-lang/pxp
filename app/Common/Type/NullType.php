<?php

namespace App\Common\Type;

class NullType implements Type
{
    public function __toString(): string
    {
        return 'null';
    }
}
