<?php

#endif
#ifdef HAVE_LDAP_START_TLS_S
/** @param resource $ldap */
function ldap_start_tls($ldap): bool
{
}
#endif
#ifdef HAVE_LDAP_START_TLS_S
function ldap_start_tls(\LDAP\Connection $ldap): bool
{
}