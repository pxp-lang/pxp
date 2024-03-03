<?php

/** @param resource $ftp */
function ftp_exec($ftp, string $command): bool
{
}
function ftp_exec(\FTP\Connection $ftp, string $command): bool
{
}