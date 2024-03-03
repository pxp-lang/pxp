<?php

#ifdef HAVE_ORALDAP
function ldap_connect(?string $uri = null, int $port = 389, string $wallet = UNKNOWN, string $password = UNKNOWN, int $auth_mode = GSLC_SSL_NO_AUTH): \LDAP\Connection|false
{
}