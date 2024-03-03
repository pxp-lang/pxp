<?php

#endif
/**
 * @param resource|array $ldap
 * @return resource|array|false
 */
function ldap_read($ldap, array|string $base, array|string $filter, array $attributes = [], int $attributes_only = 0, int $sizelimit = -1, int $timelimit = -1, int $deref = LDAP_DEREF_NEVER, ?array $controls = null)
{
}