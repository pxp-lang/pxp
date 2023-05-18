<?php

namespace App\Common\Type;

class ObjectType implements Type
{
    public function __toString(): string
    {
        return 'object';
    }
}
