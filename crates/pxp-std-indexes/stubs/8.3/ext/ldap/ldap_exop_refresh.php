<?php

#endif
#ifdef HAVE_LDAP_REFRESH_S
function ldap_exop_refresh(\LDAP\Connection $ldap, string $dn, int $ttl): int|false
{
}