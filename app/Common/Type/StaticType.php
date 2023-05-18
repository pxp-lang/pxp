<?php

namespace App\Common\Type;

class StaticType implements Type
{
    public function __toString(): string
    {
        return 'static';
    }
}
