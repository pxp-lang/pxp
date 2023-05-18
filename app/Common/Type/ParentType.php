<?php

namespace App\Common\Type;

class ParentType implements Type
{
    public function __toString(): string
    {
        return 'parent';
    }
}
