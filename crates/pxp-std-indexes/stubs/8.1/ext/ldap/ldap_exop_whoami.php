<?php

#endif
#ifdef HAVE_LDAP_WHOAMI_S
/** @param resource $ldap */
function ldap_exop_whoami($ldap): string|false
{
}
#endif
#ifdef HAVE_LDAP_WHOAMI_S
function ldap_exop_whoami(\LDAP\Connection $ldap): string|false
{
}