<?php

#endif
#ifdef HAVE_LDAP_EXTENDED_OPERATION_S
/**
 * @param resource $ldap
 * @param string $response_data
 * @param string $response_oid
 * @return resource|bool
 */
function ldap_exop($ldap, string $request_oid, ?string $request_data = null, ?array $controls = NULL, &$response_data = UNKNOWN, &$response_oid = null)
{
}