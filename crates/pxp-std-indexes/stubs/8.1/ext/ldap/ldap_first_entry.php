<?php

/**
 * @param resource $ldap
 * @param resource $result
 * @return resource|false
 */
function ldap_first_entry($ldap, $result)
{
}
function ldap_first_entry(\LDAP\Connection $ldap, \LDAP\Result $result): \LDAP\ResultEntry|false
{
}