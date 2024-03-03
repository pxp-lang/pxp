<?php

/** @param resource $ftp */
function ftp_rmdir($ftp, string $directory): bool
{
}
function ftp_rmdir(\FTP\Connection $ftp, string $directory): bool
{
}