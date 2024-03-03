<?php

/**
 * @param resource $ldap
 * @return resource|false
 */
function ldap_delete_ext($ldap, string $dn, ?array $controls = null)
{
}
function ldap_delete_ext(\LDAP\Connection $ldap, string $dn, ?array $controls = null): \LDAP\Result|false
{
}