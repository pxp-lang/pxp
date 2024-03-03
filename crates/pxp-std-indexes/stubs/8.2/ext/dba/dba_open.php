<?php

/**
 * @param string $path
 * @param string $mode
 * @param string $handler
 * @param string $handler_params
 * @return resource|false
 */
function dba_open($path, $mode, $handler = UNKNOWN, ...$handler_params)
{
}
/** @return resource|false */
function dba_open(string $path, string $mode, ?string $handler = null, int $permission = 0644, int $map_size = 0, ?int $flags = null)
{
}