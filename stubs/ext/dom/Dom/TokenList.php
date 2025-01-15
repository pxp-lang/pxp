<?php 

namespace Dom;

/**
 * @not-serializable
 * @strict-properties
 */
#[\Since('8.4')]
final class TokenList implements \IteratorAggregate, \Countable
{
    /** @implementation-alias Dom\Node::__construct */
    private function __construct()
    {
    }
    /**
     * @readonly
     * @virtual
     */
    public int $length;
    public function item(int $index): ?string
    {
    }
    public function contains(string $token): bool
    {
    }
    public function add(string ...$tokens): void
    {
    }
    public function remove(string ...$tokens): void
    {
    }
    public function toggle(string $token, ?bool $force = null): bool
    {
    }
    public function replace(string $token, string $newToken): bool
    {
    }
    public function supports(string $token): bool
    {
    }
    /** @virtual */
    public string $value;
    public function count(): int
    {
    }
    public function getIterator(): \Iterator
    {
    }
}