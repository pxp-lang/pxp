<?php

namespace App;

class Bar
{
    public string $baz;

    public static function the(): Bar
    {
        return new Bar();
    }
}