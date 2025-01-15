<?php 

namespace Dom;

/**
 * @strict-properties
 * @not-serializable
 */
#[\Since('8.4')]
class Implementation
{
    public function createDocumentType(string $qualifiedName, string $publicId, string $systemId): DocumentType
    {
    }
    public function createDocument(?string $namespace, string $qualifiedName, ?DocumentType $doctype = null): XMLDocument
    {
    }
    public function createHTMLDocument(?string $title = null): HTMLDocument
    {
    }
}