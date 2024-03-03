<?php

/**
 * @param resource $ldap
 * @param resource $entry
 * @alias ldap_get_values_len
 */
function ldap_get_values($ldap, $entry, string $attribute): array|false
{
}
/**
 * @return array<int|string, int|string>|false
 * @refcount 1
 * @alias ldap_get_values_len
 */
function ldap_get_values(\LDAP\Connection $ldap, \LDAP\ResultEntry $entry, string $attribute): array|false
{
}