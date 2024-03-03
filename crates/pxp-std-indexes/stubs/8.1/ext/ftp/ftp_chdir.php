<?php

/** @param resource $ftp */
function ftp_chdir($ftp, string $directory): bool
{
}
function ftp_chdir(\FTP\Connection $ftp, string $directory): bool
{
}