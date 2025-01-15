<?php 

namespace BcMath;

/** @strict-properties */
#[\Since('8.4')]
final readonly class Number implements \Stringable
{
    /** @virtual */
    public string $value;
    /** @virtual */
    public int $scale;
    public function __construct(string|int $num)
    {
    }
    public function add(Number|string|int $num, ?int $scale = null): Number
    {
    }
    public function sub(Number|string|int $num, ?int $scale = null): Number
    {
    }
    public function mul(Number|string|int $num, ?int $scale = null): Number
    {
    }
    public function div(Number|string|int $num, ?int $scale = null): Number
    {
    }
    public function mod(Number|string|int $num, ?int $scale = null): Number
    {
    }
    /** @return Number[] */
    public function divmod(Number|string|int $num, ?int $scale = null): array
    {
    }
    public function powmod(Number|string|int $exponent, Number|string|int $modulus, ?int $scale = null): Number
    {
    }
    public function pow(Number|string|int $exponent, ?int $scale = null): Number
    {
    }
    public function sqrt(?int $scale = null): Number
    {
    }
    public function floor(): Number
    {
    }
    public function ceil(): Number
    {
    }
    public function round(int $precision = 0, \RoundingMode $mode = \RoundingMode::HalfAwayFromZero): Number
    {
    }
    public function compare(Number|string|int $num, ?int $scale = null): int
    {
    }
    public function __toString(): string
    {
    }
    public function __serialize(): array
    {
    }
    public function __unserialize(array $data): void
    {
    }
}