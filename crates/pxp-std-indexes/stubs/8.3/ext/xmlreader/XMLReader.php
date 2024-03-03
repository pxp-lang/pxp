<?php

/** @generate-function-entries */
class XMLReader
{
    /** @return bool */
    public function close()
    {
    }
    /**
     * @tentative-return-type
     * @return (string | null)
     */
    public function getAttribute(string $name)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | null)
     */
    public function getAttributeNo(int $index)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | null)
     */
    public function getAttributeNs(string $name, string $namespace)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function getParserProperty(int $property)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function isValid()
    {
    }
    /**
     * @tentative-return-type
     * @return (string | null)
     */
    public function lookupNamespace(string $prefix)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function moveToAttribute(string $name)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function moveToAttributeNo(int $index)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function moveToAttributeNs(string $name, string $namespace)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function moveToElement()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function moveToFirstAttribute()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function moveToNextAttribute()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function read()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function next(?string $name = null)
    {
    }
    /** @return bool|XMLReader */
    public static function open(string $uri, ?string $encoding = null, int $flags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return string
     */
    public function readInnerXml()
    {
    }
    /**
     * @tentative-return-type
     * @return string
     */
    public function readOuterXml()
    {
    }
    /**
     * @tentative-return-type
     * @return string
     */
    public function readString()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setSchema(?string $filename)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setParserProperty(int $property, bool $value)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setRelaxNGSchema(?string $filename)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setRelaxNGSchemaSource(?string $source)
    {
    }
    /** @return bool|XMLReader */
    public static function XML(string $source, ?string $encoding = null, int $flags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return (DOMNode | false)
     */
    public function expand(?DOMNode $baseNode = null)
    {
    }
    /* Constants for NodeType - cannot define common types to share with dom as there are differences in these types */
    /**
     * @var int
     * @cvalue XML_READER_TYPE_NONE
     */
    public const NONE = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_ELEMENT
     */
    public const ELEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_ATTRIBUTE
     */
    public const ATTRIBUTE = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_TEXT
     */
    public const TEXT = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_CDATA
     */
    public const CDATA = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_ENTITY_REFERENCE
     */
    public const ENTITY_REF = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_ENTITY
     */
    public const ENTITY = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_PROCESSING_INSTRUCTION
     */
    public const PI = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_COMMENT
     */
    public const COMMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_DOCUMENT
     */
    public const DOC = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_DOCUMENT_TYPE
     */
    public const DOC_TYPE = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_DOCUMENT_FRAGMENT
     */
    public const DOC_FRAGMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_NOTATION
     */
    public const NOTATION = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_WHITESPACE
     */
    public const WHITESPACE = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_SIGNIFICANT_WHITESPACE
     */
    public const SIGNIFICANT_WHITESPACE = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_END_ELEMENT
     */
    public const END_ELEMENT = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_END_ENTITY
     */
    public const END_ENTITY = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_READER_TYPE_XML_DECLARATION
     */
    public const XML_DECLARATION = UNKNOWN;
    /* Constants for Parser options */
    /**
     * @var int
     * @cvalue XML_PARSER_LOADDTD
     */
    public const LOADDTD = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_PARSER_DEFAULTATTRS
     */
    public const DEFAULTATTRS = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_PARSER_VALIDATE
     */
    public const VALIDATE = UNKNOWN;
    /**
     * @var int
     * @cvalue XML_PARSER_SUBST_ENTITIES
     */
    public const SUBST_ENTITIES = UNKNOWN;
}