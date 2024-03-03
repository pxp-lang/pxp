<?php

namespace Random;

/**
 * @strict-properties
 */
final class Randomizer
{
    public function __construct(?Engine $engine = null)
    {
    }
    public function nextInt(): int
    {
    }
    public function getInt(int $min, int $max): int
    {
    }
    public function getBytes(int $length): string
    {
    }
    public function shuffleArray(array $array): array
    {
    }
    public function shuffleBytes(string $bytes): string
    {
    }
    public function pickArrayKeys(array $array, int $num): array
    {
    }
    public function __serialize(): array
    {
    }
    public function __unserialize(array $data): void
    {
    }
}