<?php

/** @param resource $ldap */
function ldap_modify_batch($ldap, string $dn, array $modifications_info, ?array $controls = null): bool
{
}
function ldap_modify_batch(\LDAP\Connection $ldap, string $dn, array $modifications_info, ?array $controls = null): bool
{
}