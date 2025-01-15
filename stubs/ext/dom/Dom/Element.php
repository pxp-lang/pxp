<?php 

namespace Dom;

#[\Since('8.4')]
class Element extends \Dom\Node implements \Dom\ParentNode, \Dom\ChildNode
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
    public string $tagName;
    /** @virtual */
    public string $id;
    /** @virtual */
    public string $className;
    /** @readonly */
    public TokenList $classList;
    /** @implementation-alias DOMNode::hasAttributes */
    public function hasAttributes(): bool
    {
    }
    /**
     * @readonly
     * @virtual
     */
    public NamedNodeMap $attributes;
    /** @implementation-alias DOMElement::getAttributeNames */
    public function getAttributeNames(): array
    {
    }
    /** @implementation-alias DOMElement::getAttribute */
    public function getAttribute(string $qualifiedName): ?string
    {
    }
    /** @implementation-alias DOMElement::getAttributeNS */
    public function getAttributeNS(?string $namespace, string $localName): ?string
    {
    }
    /** @implementation-alias DOMElement::setAttribute */
    public function setAttribute(string $qualifiedName, string $value): void
    {
    }
    /** @implementation-alias DOMElement::setAttributeNS */
    public function setAttributeNS(?string $namespace, string $qualifiedName, string $value): void
    {
    }
    public function removeAttribute(string $qualifiedName): void
    {
    }
    /** @implementation-alias DOMElement::removeAttributeNS */
    public function removeAttributeNS(?string $namespace, string $localName): void
    {
    }
    /** @implementation-alias DOMElement::toggleAttribute */
    public function toggleAttribute(string $qualifiedName, ?bool $force = null): bool
    {
    }
    /** @implementation-alias DOMElement::hasAttribute */
    public function hasAttribute(string $qualifiedName): bool
    {
    }
    /** @implementation-alias DOMElement::hasAttributeNS */
    public function hasAttributeNS(?string $namespace, string $localName): bool
    {
    }
    /** @implementation-alias DOMElement::getAttributeNode */
    public function getAttributeNode(string $qualifiedName): ?Attr
    {
    }
    /** @implementation-alias DOMElement::getAttributeNodeNS */
    public function getAttributeNodeNS(?string $namespace, string $localName): ?Attr
    {
    }
    /** @implementation-alias Dom\Element::setAttributeNodeNS */
    public function setAttributeNode(Attr $attr): ?Attr
    {
    }
    public function setAttributeNodeNS(Attr $attr): ?Attr
    {
    }
    public function removeAttributeNode(Attr $attr): Attr
    {
    }
    public function getElementsByTagName(string $qualifiedName): HTMLCollection
    {
    }
    public function getElementsByTagNameNS(?string $namespace, string $localName): HTMLCollection
    {
    }
    public function insertAdjacentElement(AdjacentPosition $where, Element $element): ?Element
    {
    }
    public function insertAdjacentText(AdjacentPosition $where, string $data): void
    {
    }
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
    /**
     * @readonly
     * @virtual
     */
    public ?Element $previousElementSibling;
    /**
     * @readonly
     * @virtual
     */
    public ?Element $nextElementSibling;
    /** @implementation-alias DOMElement::setIdAttribute */
    public function setIdAttribute(string $qualifiedName, bool $isId): void
    {
    }
    /** @implementation-alias DOMElement::setIdAttributeNS */
    public function setIdAttributeNS(?string $namespace, string $qualifiedName, bool $isId): void
    {
    }
    public function setIdAttributeNode(Attr $attr, bool $isId): void
    {
    }
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
    public function querySelector(string $selectors): ?Element
    {
    }
    public function querySelectorAll(string $selectors): NodeList
    {
    }
    public function closest(string $selectors): ?Element
    {
    }
    public function matches(string $selectors): bool
    {
    }
    /** @virtual */
    public string $innerHTML;
    /** @virtual */
    public string $substitutedNodeValue;
    /** @return list<NamespaceInfo> */
    public function getInScopeNamespaces(): array
    {
    }
    /** @return list<NamespaceInfo> */
    public function getDescendantNamespaces(): array
    {
    }
    public function rename(?string $namespaceURI, string $qualifiedName): void
    {
    }
}