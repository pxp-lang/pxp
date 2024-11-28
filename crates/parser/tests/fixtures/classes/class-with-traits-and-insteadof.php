<?php

class Foo
{
    use Bar, Baz {
        Bar::a insteadof Baz;
    }
}