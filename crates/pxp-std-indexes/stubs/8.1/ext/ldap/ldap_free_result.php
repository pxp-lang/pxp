<?php

/** @param resource $ldap */
function ldap_free_result($ldap): bool
{
}
function ldap_free_result(\LDAP\Result $result): bool
{
}