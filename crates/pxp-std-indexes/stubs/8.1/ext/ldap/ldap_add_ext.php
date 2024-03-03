<?php

/**
 * @param resource $ldap
 * @return resource|false
 */
function ldap_add_ext($ldap, string $dn, array $entry, ?array $controls = null)
{
}
function ldap_add_ext(\LDAP\Connection $ldap, string $dn, array $entry, ?array $controls = null): \LDAP\Result|false
{
}