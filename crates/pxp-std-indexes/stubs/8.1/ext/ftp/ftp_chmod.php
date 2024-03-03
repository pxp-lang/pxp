<?php

/** @param resource $ftp */
function ftp_chmod($ftp, int $permissions, string $filename): int|false
{
}
function ftp_chmod(\FTP\Connection $ftp, int $permissions, string $filename): int|false
{
}