<?php

/** @param resource $ftp */
function ftp_delete($ftp, string $filename): bool
{
}
function ftp_delete(\FTP\Connection $ftp, string $filename): bool
{
}