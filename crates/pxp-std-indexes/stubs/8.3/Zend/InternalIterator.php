<?php

final class InternalIterator implements \Iterator
{
    private function __construct();
    public function current(): mixed
    {
    }
    public function key(): mixed
    {
    }
    public function next(): void;
    public function valid(): bool;
    public function rewind(): void;
}