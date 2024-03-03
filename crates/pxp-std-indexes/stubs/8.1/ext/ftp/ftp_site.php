<?php

/** @param resource $ftp */
function ftp_site($ftp, string $command): bool
{
}
function ftp_site(\FTP\Connection $ftp, string $command): bool
{
}