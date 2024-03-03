<?php

#endif
#ifdef HAVE_LDAP_REFRESH_S
/** @param resource $ldap */
function ldap_exop_refresh($ldap, string $dn, int $ttl): int|false
{
}
#endif
#ifdef HAVE_LDAP_REFRESH_S
function ldap_exop_refresh(\LDAP\Connection $ldap, string $dn, int $ttl): int|false
{
}