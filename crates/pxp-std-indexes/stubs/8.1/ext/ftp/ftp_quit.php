<?php

/**
 * @param resource $ftp
 * @alias ftp_close
 */
function ftp_quit($ftp): bool
{
}
/** @alias ftp_close */
function ftp_quit(\FTP\Connection $ftp): bool
{
}