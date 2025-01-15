<?php 

namespace Dom;

#[\Since('8.4')]
class DocumentType extends \Dom\Node implements \Dom\ChildNode
{
    /**
     * @readonly
     * @virtual
     */
    public string $name;
    /**
     * @readonly
     * @virtual
     */
    public DtdNamedNodeMap $entities;
    /**
     * @readonly
     * @virtual
     */
    public DtdNamedNodeMap $notations;
    /**
     * @readonly
     * @virtual
     */
    public string $publicId;
    /**
     * @readonly
     * @virtual
     */
    public string $systemId;
    /**
     * @readonly
     * @virtual
     */
    public ?string $internalSubset;
    /** @implementation-alias DOMElement::remove */
    public function remove(): void
    {
    }
    /** @implementation-alias DOMElement::before */
    public function before(Node|string ...$nodes): void
    {
    }
    /** @implementation-alias DOMElement::after */
    public function after(Node|string ...$nodes): void
    {
    }
    /** @implementation-alias DOMElement::replaceWith */
    public function replaceWith(Node|string ...$nodes): void
    {
    }
}