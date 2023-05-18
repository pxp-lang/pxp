<?php

namespace App\Common\Type;

class SelfType implements Type
{
    public function __toString(): string
    {
        return 'self';
    }
}
