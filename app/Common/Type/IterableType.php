<?php

namespace App\Common\Type;

class IterableType implements Type
{
    public function __toString(): string
    {
        return 'iterable';
    }
}
