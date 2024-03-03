<?php

/**
 * @param resource $ldap
 * @param resource $entry
 */
function ldap_next_attribute($ldap, $entry): string|false
{
}
function ldap_next_attribute(\LDAP\Connection $ldap, \LDAP\ResultEntry $entry): string|false
{
}