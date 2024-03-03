<?php

/**
 * @param resource $ldap
 * @param resource $entry
 */
function ldap_first_attribute($ldap, $entry): string|false
{
}
function ldap_first_attribute(\LDAP\Connection $ldap, \LDAP\ResultEntry $entry): string|false
{
}