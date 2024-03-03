<?php

/**
 * @param resource $ftp
 * @param string $response
 */
function ftp_alloc($ftp, int $size, &$response = null): bool
{
}
/** @param string $response */
function ftp_alloc(\FTP\Connection $ftp, int $size, &$response = null): bool
{
}