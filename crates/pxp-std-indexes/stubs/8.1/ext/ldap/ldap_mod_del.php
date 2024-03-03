<?php

/** @param resource $ldap */
function ldap_mod_del($ldap, string $dn, array $entry, ?array $controls = null): bool
{
}
function ldap_mod_del(\LDAP\Connection $ldap, string $dn, array $entry, ?array $controls = null): bool
{
}