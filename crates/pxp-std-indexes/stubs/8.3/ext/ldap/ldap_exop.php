<?php

#endif
#ifdef HAVE_LDAP_EXTENDED_OPERATION_S
/**
 * @param string $response_data
 * @param string $response_oid
 */
function ldap_exop(\LDAP\Connection $ldap, string $request_oid, ?string $request_data = null, ?array $controls = NULL, &$response_data = UNKNOWN, &$response_oid = null): \LDAP\Result|bool
{
}