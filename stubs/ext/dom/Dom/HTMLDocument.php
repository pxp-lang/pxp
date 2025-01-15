<?php 

namespace Dom;

#[\Since('8.4')]
final class HTMLDocument extends \Dom\Document
{
    public static function createEmpty(string $encoding = "UTF-8"): HTMLDocument
    {
    }
    public static function createFromFile(string $path, int $options = 0, ?string $overrideEncoding = null): HTMLDocument
    {
    }
    public static function createFromString(string $source, int $options = 0, ?string $overrideEncoding = null): HTMLDocument
    {
    }
    /** @implementation-alias Dom\XMLDocument::saveXml */
    public function saveXml(?Node $node = null, int $options = 0): string|false
    {
    }
    /** @implementation-alias DOMDocument::save */
    public function saveXmlFile(string $filename, int $options = 0): int|false
    {
    }
    public function saveHtml(?Node $node = null): string
    {
    }
    public function saveHtmlFile(string $filename): int|false
    {
    }
    #if ZEND_DEBUG
    public function debugGetTemplateCount(): int
    {
    }
    #endif
}