<?php

#ifdef HAVE_FTP_SSL
function ftp_ssl_connect(string $hostname, int $port = 21, int $timeout = 90): \FTP\Connection|false
{
}