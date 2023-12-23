<?php

namespace App\Traits;

trait Barable
{
    public $bar;

    public function bar()
    {
        return 'bar';
    }

    abstract public function foo();

    public function baz()
    {
        return 'baz';
    }
}
