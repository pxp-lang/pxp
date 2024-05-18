<?php

use Foo\Baz;
use Baz\Bar as Qux;

interface Foo extends Bar, Baz, Qux\Test
{

}