<?php 

/**
 * @param callable $start_handler
 * @param callable $end_handler
 */
#[\Until('8.2')]
function xml_set_element_handler(\XMLParser $parser, $start_handler, $end_handler): bool
{
}
/**
 * @param callable $start_handler
 * @param callable $end_handler
 */
#[\Since('8.2')]
#[\Until('8.4')]
function xml_set_element_handler(\XMLParser $parser, $start_handler, $end_handler): true
{
}
#[\Since('8.4')]
function xml_set_element_handler(\XMLParser $parser, callable|string|null $start_handler, callable|string|null $end_handler): true
{
}