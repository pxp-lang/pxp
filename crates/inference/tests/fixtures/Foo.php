<?php

namespace App;

class Foo
{
    public $foo;
    public string $foop;
    public Bar $bar;

    public function getBar(): Bar
    {
        return $this->bar;
    }
}