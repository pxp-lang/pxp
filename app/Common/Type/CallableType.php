<?php

namespace App\Common\Type;

class CallableType implements Type
{
    public function __toString(): string
    {
        return 'callable';
    }
}
