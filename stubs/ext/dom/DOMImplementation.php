<?php 

class DOMImplementation
{
    /**
     * @tentative-return-type
     * @return bool
     */
    public function hasFeature(string $feature, string $version)
    {
    }
    /** @return DOMDocumentType|false */
    public function createDocumentType(string $qualifiedName, string $publicId = "", string $systemId = "")
    {
    }
    /**
     * @tentative-return-type
     * @return (DOMDocument | false)
     */
    public function createDocument(?string $namespace = null, string $qualifiedName = "", ?DOMDocumentType $doctype = null)
    {
    }
}