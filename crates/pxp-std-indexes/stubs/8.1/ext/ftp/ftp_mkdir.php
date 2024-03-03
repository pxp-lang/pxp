<?php

/** @param resource $ftp */
function ftp_mkdir($ftp, string $directory): string|false
{
}
function ftp_mkdir(\FTP\Connection $ftp, string $directory): string|false
{
}