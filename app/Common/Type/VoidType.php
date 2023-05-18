<?php

namespace App\Common\Type;

class VoidType implements Type
{
    public function __toString(): string
    {
        return 'void';
    }
}
