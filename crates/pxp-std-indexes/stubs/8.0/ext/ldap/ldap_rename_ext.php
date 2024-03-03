<?php

/**
 * @param resource $ldap
 * @return resource|false
 */
function ldap_rename_ext($ldap, string $dn, string $new_rdn, string $new_parent, bool $delete_old_rdn, ?array $controls = null)
{
}