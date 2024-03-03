<?php

#ifdef LDAP_API_FEATURE_X_OPENLDAP
function ldap_connect_wallet(?string $uri = null, string $wallet, #[\SensitiveParameter] string $password, int $auth_mode = GSLC_SSL_NO_AUTH): \LDAP\Connection|false
{
}