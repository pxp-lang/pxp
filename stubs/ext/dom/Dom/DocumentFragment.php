<?php 

namespace Dom;

#[\Since('8.4')]
class DocumentFragment extends \Dom\Node implements \Dom\ParentNode
{
    /**
     * @readonly
     * @virtual
     */
    public ?Element $firstElementChild;
    /**
     * @readonly
     * @virtual
     */
    public ?Element $lastElementChild;
    /**
     * @readonly
     * @virtual
     */
    public int $childElementCount;
    /** @implementation-alias DOMDocumentFragment::appendXML */
    public function appendXml(string $data): bool
    {
    }
    /** @implementation-alias DOMElement::append */
    public function append(Node|string ...$nodes): void
    {
    }
    /** @implementation-alias DOMElement::prepend */
    public function prepend(Node|string ...$nodes): void
    {
    }
    /** @implementation-alias DOMElement::replaceChildren */
    public function replaceChildren(Node|string ...$nodes): void
    {
    }
    /** @implementation-alias Dom\Element::querySelector */
    public function querySelector(string $selectors): ?Element
    {
    }
    /** @implementation-alias Dom\Element::querySelectorAll */
    public function querySelectorAll(string $selectors): NodeList
    {
    }
}