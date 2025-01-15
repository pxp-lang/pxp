<?php 

#ifdef LIBXML_XPATH_ENABLED
class DOMXPath
{
    public function __construct(DOMDocument $document, bool $registerNodeNS = true)
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function evaluate(string $expression, ?DOMNode $contextNode = null, bool $registerNodeNS = true)
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function query(string $expression, ?DOMNode $contextNode = null, bool $registerNodeNS = true)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function registerNamespace(string $prefix, string $namespace)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function registerPhpFunctions(string|array|null $restrict = null)
    {
    }
    #[\Since('8.4')]
    public function registerPhpFunctionNS(string $namespaceURI, string $name, callable $callable): void
    {
    }
    #[\Since('8.4')]
    public static function quote(string $str): string
    {
    }
}