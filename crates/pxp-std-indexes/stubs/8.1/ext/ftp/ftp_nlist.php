<?php

/** @param resource $ftp */
function ftp_nlist($ftp, string $directory): array|false
{
}
/**
 * @return array<int, string>|false
 * @refcount 1
 */
function ftp_nlist(\FTP\Connection $ftp, string $directory): array|false
{
}