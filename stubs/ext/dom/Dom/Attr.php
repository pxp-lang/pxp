<?php 

namespace Dom;

#[\Since('8.4')]
class Attr extends \Dom\Node
{
    /**
     * @readonly
     * @virtual
     */
    public ?string $namespaceURI;
    /**
     * @readonly
     * @virtual
     */
    public ?string $prefix;
    /**
     * @readonly
     * @virtual
     */
    public string $localName;
    /**
     * @readonly
     * @virtual
     */
    public string $name;
    /** @virtual */
    public string $value;
    /**
     * @readonly
     * @virtual
     */
    public ?Element $ownerElement;
    /**
     * @readonly
     * @virtual
     */
    public bool $specified;
    /** @implementation-alias DOMAttr::isId */
    public function isId(): bool
    {
    }
    /** @implementation-alias Dom\Element::rename */
    public function rename(?string $namespaceURI, string $qualifiedName): void
    {
    }
}