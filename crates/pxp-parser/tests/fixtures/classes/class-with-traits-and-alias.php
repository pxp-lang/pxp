<?php

class Foo
{
    use Bar {
        Bar::a as b;
    }
}