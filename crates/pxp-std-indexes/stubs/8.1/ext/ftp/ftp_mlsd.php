<?php

/** @param resource $ftp */
function ftp_mlsd($ftp, string $directory): array|false
{
}
/**
 * @return array<int, array>|false
 * @refcount 1
 */
function ftp_mlsd(\FTP\Connection $ftp, string $directory): array|false
{
}