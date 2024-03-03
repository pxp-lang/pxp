<?php

/** @param resource $ldap */
function ldap_mod_add($ldap, string $dn, array $entry, ?array $controls = null): bool
{
}
function ldap_mod_add(\LDAP\Connection $ldap, string $dn, array $entry, ?array $controls = null): bool
{
}