<?php

namespace Pxp\Runtime\Range;

use ArrayIterator;
use Traversable;

final class InclusiveRange implements Range
{
    private array $range;

    private function __construct(
        private int|float|string $lowerBound,
        private int|float|string $upperBound,
        private int|float $step = 1,
    ) {
        $this->range = range($this->lowerBound, $this->upperBound, $this->step);
    }

    public function step(int|float $step): self
    {
        return new self($this->lowerBound, $this->upperBound, $step);
    }

    public function first(): int|string|float
    {
        return $this->range[0];
    }

    public function last(): int|string|float
    {
        return $this->range[count($this->range) - 1];
    }

    public function count(): int
    {
        return count($this->range);
    }

    public function entries(): array
    {
        return $this->range;
    }

    public function includes(int|float|string $needle): bool
    {
        return in_array($needle, $this->range, strict: true);
    }

    public function min(): int|float|string
    {
        return min($this->range);
    }

    public function max(): int|float|string
    {
        return max($this->range);
    }

    public function minmax(): array
    {
        return [$this->min(), $this->max()];
    }

    public function toArray(): array
    {
        return $this->entries();
    }

    public function inclusive(): bool
    {
        return true;
    }

    public function exclusive(): bool
    {
        return false;
    }

    public function endless(): bool
    {
        return false;
    }

    public function getIterator(): Traversable
    {
        return new ArrayIterator($this->range);
    }

    public static function make(int|float|string $lowerBound, int|float|string $upperBound): self
    {
        return new self($lowerBound, $upperBound);
    }
}