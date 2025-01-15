<?php 

namespace Dom;

#[\Since('8.4')]
final class XMLDocument extends \Dom\Document
{
    public static function createEmpty(string $version = "1.0", string $encoding = "UTF-8"): XMLDocument
    {
    }
    public static function createFromFile(string $path, int $options = 0, ?string $overrideEncoding = null): XMLDocument
    {
    }
    public static function createFromString(string $source, int $options = 0, ?string $overrideEncoding = null): XMLDocument
    {
    }
    /**
     * @readonly
     * @virtual
     */
    public string $xmlEncoding;
    /** @virtual */
    public bool $xmlStandalone;
    /** @virtual */
    public string $xmlVersion;
    /** @virtual */
    public bool $formatOutput;
    /** @implementation-alias DOMDocument::createEntityReference */
    public function createEntityReference(string $name): EntityReference
    {
    }
    /** @implementation-alias DOMDocument::validate */
    public function validate(): bool
    {
    }
    public function xinclude(int $options = 0): int
    {
    }
    public function saveXml(?Node $node = null, int $options = 0): string|false
    {
    }
    /** @implementation-alias DOMDocument::save */
    public function saveXmlFile(string $filename, int $options = 0): int|false
    {
    }
}