<?php

/** @param resource $ldap */
function ldap_delete($ldap, string $dn, ?array $controls = null): bool
{
}
function ldap_delete(\LDAP\Connection $ldap, string $dn, ?array $controls = null): bool
{
}