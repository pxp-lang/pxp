<?php

/**
 * @param resource $ldap
 * @param resource $entry
 * @return resource|false
 */
function ldap_next_reference($ldap, $entry)
{
}
function ldap_next_reference(\LDAP\Connection $ldap, \LDAP\ResultEntry $entry): \LDAP\ResultEntry|false
{
}