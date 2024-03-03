<?php

/** @param resource $ftp */
function ftp_pasv($ftp, bool $enable): bool
{
}
function ftp_pasv(\FTP\Connection $ftp, bool $enable): bool
{
}