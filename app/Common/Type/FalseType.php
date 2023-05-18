<?php

namespace App\Common\Type;

class FalseType implements Type
{
    public function __toString(): string
    {
        return 'false';
    }
}
