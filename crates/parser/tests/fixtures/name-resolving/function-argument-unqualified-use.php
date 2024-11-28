<?php

use App\Foo;
use Foo\Bar\Baz as Bar;

function a(Foo $a, Foo | Bar $b, Foo & Bar $c, ?Foo $d) {

}