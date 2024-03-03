<?php

/**
 * @param resource $ldap
 * @alias ldap_unbind
 */
function ldap_close($ldap): bool
{
}
/** @alias ldap_unbind */
function ldap_close(\LDAP\Connection $ldap): bool
{
}