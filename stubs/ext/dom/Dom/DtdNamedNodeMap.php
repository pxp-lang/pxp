<?php 

namespace Dom;

#[\Since('8.4')]
class DtdNamedNodeMap implements \IteratorAggregate, \Countable
{
    /**
     * @readonly
     * @virtual
     */
    public int $length;
    /** @implementation-alias DOMNamedNodeMap::item */
    public function item(int $index): Entity|Notation|null
    {
    }
    /** @implementation-alias DOMNamedNodeMap::getNamedItem */
    public function getNamedItem(string $qualifiedName): Entity|Notation|null
    {
    }
    /** @implementation-alias DOMNamedNodeMap::getNamedItemNS */
    public function getNamedItemNS(?string $namespace, string $localName): Entity|Notation|null
    {
    }
    /** @implementation-alias DOMNamedNodeMap::count */
    public function count(): int
    {
    }
    /** @implementation-alias DOMNamedNodeMap::getIterator */
    public function getIterator(): \Iterator
    {
    }
}