<?php

#ifdef HAVE_LDAP_SASL
/** @param resource $ldap */
function ldap_sasl_bind($ldap, ?string $dn = null, ?string $password = null, ?string $mech = null, ?string $realm = null, ?string $authc_id = null, ?string $authz_id = null, ?string $props = null): bool
{
}