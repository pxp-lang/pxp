<?php

class A
{
    public function __construct(
        public private(set) string $a,
        public protected(set) string $b,
        public public(set) string $c
    ) {}
}
