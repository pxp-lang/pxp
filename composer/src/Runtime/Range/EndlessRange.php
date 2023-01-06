<?php

namespace Pxp\Runtime\Range;

use Pxp\Runtime\Range\Exceptions\InvalidRangeOperationException;
use Traversable;

final class EndlessRange implements Range
{
    public function __construct(
        private int|float $lowerBound,
        private int|float $step = 1,
    ) {}

    public function step(int|float $step): self
    {
        return new self($this->lowerBound, $step);
    }

    public function first(): int|string|float
    {
        return $this->lowerBound;
    }

    public function last(): int|string|float
    {
        return INF;
    }

    public function count(): int
    {
        throw new InvalidRangeOperationException("Cannot call `count()` on an endless range.");
    }

    public function entries(): never
    {
        throw new InvalidRangeOperationException("Cannot call `entries()` on an endless range.");
    }

    public function includes(int|float $needle): bool
    {
        return $needle >= $this->lowerBound;
    }

    public function min(): int|float
    {
        return $this->lowerBound;
    }

    public function max(): float
    {
        return INF;
    }

    public function minmax(): array
    {
        return [$this->min(), $this->max()];
    }

    public function toArray(): never
    {
        $this->entries();
    }

    public function inclusive(): bool
    {
        return false;
    }

    public function exclusive(): bool
    {
        return false;
    }

    public function endless(): bool
    {
        return true;
    }

    public function getIterator(): Traversable
    {
        return (function () {
            $i = 0;
            $value = $this->lowerBound;

            while (true) {
                yield $i => $value;

                $i += 1;
                $value += $this->step;
            }
        })();
    }

    public static function make(int|float|string $lowerBound): self
    {
        return new self($lowerBound);
    }
}