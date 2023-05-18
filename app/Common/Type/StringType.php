<?php

namespace App\Common\Type;

class StringType implements Type
{
    public function __toString(): string
    {
        return 'string';
    }
}
