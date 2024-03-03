<?php

/**
 * @param resource $ldap
 * @param resource $entry
 * @return resource|false
 */
function ldap_next_entry($ldap, $entry)
{
}
function ldap_next_entry(\LDAP\Connection $ldap, \LDAP\ResultEntry $entry): \LDAP\ResultEntry|false
{
}