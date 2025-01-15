<?php 

namespace Dom;

#ifdef LIBXML_XPATH_ENABLED
/** @not-serializable */
#[\Since('8.4')]
final class XPath
{
    /**
     * @readonly
     * @virtual
     */
    public Document $document;
    /** @virtual */
    public bool $registerNodeNamespaces;
    public function __construct(Document $document, bool $registerNodeNS = true)
    {
    }
    public function evaluate(string $expression, ?Node $contextNode = null, bool $registerNodeNS = true): null|bool|float|string|NodeList
    {
    }
    public function query(string $expression, ?Node $contextNode = null, bool $registerNodeNS = true): NodeList
    {
    }
    /** @implementation-alias DOMXPath::registerNamespace */
    public function registerNamespace(string $prefix, string $namespace): bool
    {
    }
    /** @implementation-alias DOMXPath::registerPhpFunctions */
    public function registerPhpFunctions(string|array|null $restrict = null): void
    {
    }
    /** @implementation-alias DOMXPath::registerPhpFunctionNS */
    public function registerPhpFunctionNS(string $namespaceURI, string $name, callable $callable): void
    {
    }
    /** @implementation-alias DOMXPath::quote */
    public static function quote(string $str): string
    {
    }
}