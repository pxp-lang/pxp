<?php

namespace App\Traits;

trait Fooable
{
    use Barable;

    public $foo;

    public function foo()
    {
        return 'foo';
    }

    abstract public function bar();

    public function baz()
    {
        return 'baz';
    }
}
