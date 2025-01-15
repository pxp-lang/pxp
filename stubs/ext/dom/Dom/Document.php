<?php 

namespace Dom;

#[\Since('8.4')]
abstract class Document extends \Dom\Node implements \Dom\ParentNode
{
    /** @readonly */
    public Implementation $implementation;
    /** @virtual */
    public string $URL;
    /** @virtual */
    public string $documentURI;
    /** @virtual */
    public string $characterSet;
    /** @virtual */
    public string $charset;
    /** @virtual */
    public string $inputEncoding;
    /**
     * @readonly
     * @virtual
     */
    public ?DocumentType $doctype;
    /**
     * @readonly
     * @virtual
     */
    public ?Element $documentElement;
    /** @implementation-alias Dom\Element::getElementsByTagName */
    public function getElementsByTagName(string $qualifiedName): HTMLCollection
    {
    }
    /** @implementation-alias Dom\Element::getElementsByTagNameNS */
    public function getElementsByTagNameNS(?string $namespace, string $localName): HTMLCollection
    {
    }
    public function createElement(string $localName): Element
    {
    }
    public function createElementNS(?string $namespace, string $qualifiedName): Element
    {
    }
    /** @implementation-alias DOMDocument::createDocumentFragment */
    public function createDocumentFragment(): DocumentFragment
    {
    }
    /** @implementation-alias DOMDocument::createTextNode */
    public function createTextNode(string $data): Text
    {
    }
    /** @implementation-alias DOMDocument::createCDATASection */
    public function createCDATASection(string $data): CDATASection
    {
    }
    /** @implementation-alias DOMDocument::createComment */
    public function createComment(string $data): Comment
    {
    }
    public function createProcessingInstruction(string $target, string $data): ProcessingInstruction
    {
    }
    public function importNode(?Node $node, bool $deep = false): Node
    {
    }
    public function adoptNode(Node $node): Node
    {
    }
    /** @implementation-alias DOMDocument::createAttribute */
    public function createAttribute(string $localName): Attr
    {
    }
    /** @implementation-alias DOMDocument::createAttributeNS */
    public function createAttributeNS(?string $namespace, string $qualifiedName): Attr
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
    /** @implementation-alias DOMDocument::getElementById */
    public function getElementById(string $elementId): ?Element
    {
    }
    public function registerNodeClass(string $baseClass, ?string $extendedClass): void
    {
    }
    #ifdef LIBXML_SCHEMAS_ENABLED
    /** @implementation-alias DOMDocument::schemaValidate */
    public function schemaValidate(string $filename, int $flags = 0): bool
    {
    }
    /** @implementation-alias DOMDocument::schemaValidateSource */
    public function schemaValidateSource(string $source, int $flags = 0): bool
    {
    }
    /** @implementation-alias DOMDocument::relaxNGValidate */
    public function relaxNgValidate(string $filename): bool
    {
    }
    /** @implementation-alias DOMDocument::relaxNGValidateSource */
    public function relaxNgValidateSource(string $source): bool
    {
    }
    #endif
    /** @implementation-alias DOMElement::append */
    public function append(Node|string ...$nodes): void
    {
    }
    /** @implementation-alias DOMElement::prepend */
    public function prepend(Node|string ...$nodes): void
    {
    }
    /** @implementation-alias DOMDocument::replaceChildren */
    public function replaceChildren(Node|string ...$nodes): void
    {
    }
    public function importLegacyNode(\DOMNode $node, bool $deep = false): Node
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
    /** @virtual */
    public ?HTMLElement $body;
    /**
     * @readonly
     * @virtual
     */
    public ?HTMLElement $head;
    /** @virtual */
    public string $title;
}