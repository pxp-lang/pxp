<?php

class Foo
{
    use Bar {
        a as protected;
    }
}