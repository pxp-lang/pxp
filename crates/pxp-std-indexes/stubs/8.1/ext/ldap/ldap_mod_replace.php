<?php

/** @param resource $ldap */
function ldap_mod_replace($ldap, string $dn, array $entry, ?array $controls = null): bool
{
}
function ldap_mod_replace(\LDAP\Connection $ldap, string $dn, array $entry, ?array $controls = null): bool
{
}