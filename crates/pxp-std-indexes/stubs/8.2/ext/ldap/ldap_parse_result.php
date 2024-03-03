<?php

#endif
#ifdef HAVE_LDAP_PARSE_RESULT
/**
 * @param int $error_code
 * @param string $matched_dn
 * @param string $error_message
 * @param array $referrals
 * @param array $controls
 */
function ldap_parse_result(\LDAP\Connection $ldap, \LDAP\Result $result, &$error_code, &$matched_dn = null, &$error_message = null, &$referrals = null, &$controls = null): bool
{
}