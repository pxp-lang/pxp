<?php

/** @param resource $ftp */
function ftp_systype($ftp): string|false
{
}
function ftp_systype(\FTP\Connection $ftp): string|false
{
}