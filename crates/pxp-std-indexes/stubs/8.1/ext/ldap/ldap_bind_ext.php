<?php

/**
 * @param resource $ldap
 * @return resource|false
 */
function ldap_bind_ext($ldap, ?string $dn = null, ?string $password = null, ?array $controls = null)
{
}
function ldap_bind_ext(\LDAP\Connection $ldap, ?string $dn = null, ?string $password = null, ?array $controls = null): \LDAP\Result|false
{
}