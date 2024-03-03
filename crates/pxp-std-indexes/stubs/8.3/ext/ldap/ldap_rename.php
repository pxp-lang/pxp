<?php

#if (LDAP_API_VERSION > 2000) || defined(HAVE_ORALDAP)
function ldap_rename(\LDAP\Connection $ldap, string $dn, string $new_rdn, string $new_parent, bool $delete_old_rdn, ?array $controls = null): bool
{
}