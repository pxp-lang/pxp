<?php

/** @param resource $ldap */
function ldap_bind($ldap, ?string $dn = null, ?string $password = null): bool
{
}
function ldap_bind(\LDAP\Connection $ldap, ?string $dn = null, ?string $password = null): bool
{
}