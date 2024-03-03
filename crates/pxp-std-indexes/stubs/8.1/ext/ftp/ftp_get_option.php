<?php

/** @param resource $ftp */
function ftp_get_option($ftp, int $option): int|bool
{
}
function ftp_get_option(\FTP\Connection $ftp, int $option): int|bool
{
}