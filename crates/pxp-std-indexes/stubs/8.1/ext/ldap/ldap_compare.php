<?php

/** @param resource $ldap */
function ldap_compare($ldap, string $dn, string $attribute, string $value, ?array $controls = null): bool|int
{
}
function ldap_compare(\LDAP\Connection $ldap, string $dn, string $attribute, string $value, ?array $controls = null): bool|int
{
}