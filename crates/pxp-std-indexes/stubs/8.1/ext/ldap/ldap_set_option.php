<?php

/**
 * @param resource|null $ldap
 * @param array|string|int|bool $value
 */
function ldap_set_option($ldap, int $option, $value): bool
{
}
/** @param array|string|int|bool $value */
function ldap_set_option(?\LDAP\Connection $ldap, int $option, $value): bool
{
}