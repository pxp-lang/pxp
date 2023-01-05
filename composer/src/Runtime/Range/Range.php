<?php

namespace Pxp\Runtime\Range;

use Countable;
use IteratorAggregate;

interface Range extends Countable, IteratorAggregate
{
    public function inclusive(): bool;
    public function exclusive(): bool;
    public function endless(): bool;
}