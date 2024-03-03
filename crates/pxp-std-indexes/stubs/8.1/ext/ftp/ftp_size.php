<?php

/** @param resource $ftp */
function ftp_size($ftp, string $filename): int
{
}
function ftp_size(\FTP\Connection $ftp, string $filename): int
{
}