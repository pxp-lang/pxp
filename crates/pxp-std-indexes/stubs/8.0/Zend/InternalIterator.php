<?php

final class InternalIterator implements \Iterator
{
    /** @return mixed */
    public function current();
    /** @return mixed */
    public function key();
    private function __construct();
    public function next(): void;
    public function valid(): bool;
    public function rewind(): void;
}