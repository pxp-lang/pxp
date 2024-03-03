<?php

/**
 * @param resource $ldap
 * @param array|string|int $value
 */
function ldap_get_option($ldap, int $option, &$value = null): bool
{
}
/** @param array|string|int $value */
function ldap_get_option(\LDAP\Connection $ldap, int $option, &$value = null): bool
{
}