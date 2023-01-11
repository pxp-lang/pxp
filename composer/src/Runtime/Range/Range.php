<?php

namespace Pxp\Runtime\Range;

use Countable;
use IteratorAggregate;

interface Range extends Countable, IteratorAggregate
{
    /**
     * Check if the range is inclusive.
     * 
     * @return bool 
     */
    public function inclusive(): bool;

    /**
     * Check if the range is exclusive.
     * 
     * @return bool 
     */
    public function exclusive(): bool;

    /**
     * Check if the range is endless.
     * 
     * @return bool 
     */
    public function endless(): bool;

    /**
     * Check if `$value` is included in the range.
     * 
     * @param int|float|string $needle 
     * @return bool 
     */
    public function includes(int|float|string $needle): bool;

    /**
     * Retrieve the minimum value from the range.
     * 
     * @return int|float|string 
     */
    public function min(): int|float|string;

    /**
     * Retreive the maximum value from the range.
     * 
     * @return int|float|string 
     */
    public function max(): int|float|string;

    /**
     * Retrieve a tuple containing the minimum and maximum values from the range.
     * 
     * @return array{int|float|string, int|float|string}
     */
    public function minmax(): array;

    /**
     * Retreive an array containing all entries in the range.
     * 
     * @return array 
     */
    public function entries(): array;
}