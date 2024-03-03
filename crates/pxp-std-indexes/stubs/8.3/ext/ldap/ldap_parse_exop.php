<?php

#endif
#ifdef HAVE_LDAP_PARSE_EXTENDED_RESULT
/**
 * @param string $response_data
 * @param string $response_oid
 */
function ldap_parse_exop(\LDAP\Connection $ldap, \LDAP\Result $result, &$response_data = null, &$response_oid = null): bool
{
}