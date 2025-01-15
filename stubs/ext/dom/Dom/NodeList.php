<?php 

namespace Dom;

#[\Since('8.4')]
class NodeList implements \IteratorAggregate, \Countable
{
    /**
     * @readonly
     * @virtual
     */
    public int $length;
    /** @implementation-alias DOMNodeList::count */
    public function count(): int
    {
    }
    /** @implementation-alias DOMNodeList::getIterator */
    public function getIterator(): \Iterator
    {
    }
    /** @implementation-alias DOMNodeList::item */
    public function item(int $index): ?Node
    {
    }
}