<?php 

namespace Dom;

/**
 * @not-serializable
 * @strict-properties
 */
#[\Since('8.4')]
final readonly class NamespaceInfo
{
    public ?string $prefix;
    public ?string $namespaceURI;
    public Element $element;
    /** @implementation-alias Dom\Node::__construct */
    private function __construct()
    {
    }
}