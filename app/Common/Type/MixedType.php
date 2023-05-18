<?php

namespace App\Common\Type;

class MixedType implements Type
{
    public function __toString(): string
    {
        return 'mixed';
    }
}
