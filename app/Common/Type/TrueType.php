<?php

namespace App\Common\Type;

class TrueType implements Type
{
    public function __toString(): string
    {
        return 'true';
    }
}
