<?php 

#[\Until('8.4')]
function simplexml_import_dom(\SimpleXMLElement|\DOMNode $node, ?string $class_name = SimpleXMLElement::class): ?\SimpleXMLElement
{
}
#[\Since('8.4')]
function simplexml_import_dom(object $node, ?string $class_name = SimpleXMLElement::class): ?\SimpleXMLElement
{
}