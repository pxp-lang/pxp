<?php

/** @param resource $ftp */
function ftp_pwd($ftp): string|false
{
}
function ftp_pwd(\FTP\Connection $ftp): string|false
{
}