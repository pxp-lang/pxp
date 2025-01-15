<?php 

/**
 * @param array $values
 * @param array $index
 */
#[\Until('8.1')]
function xml_parse_into_struct(\XMLParser $parser, string $data, &$values, &$index = null): int
{
}
/**
 * @param array $values
 * @param array $index
 */
#[\Since('8.1')]
function xml_parse_into_struct(\XMLParser $parser, string $data, &$values, &$index = null): int|false
{
}