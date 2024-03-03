<?php

/**
 * @param resource $ldap
 * @param resource $entry
 */
function ldap_get_dn($ldap, $entry): string|false
{
}
function ldap_get_dn(\LDAP\Connection $ldap, \LDAP\ResultEntry $entry): string|false
{
}