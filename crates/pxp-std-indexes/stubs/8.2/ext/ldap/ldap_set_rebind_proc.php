<?php

#endif
#endif
#if defined(LDAP_API_FEATURE_X_OPENLDAP) && defined(HAVE_3ARG_SETREBINDPROC)
function ldap_set_rebind_proc(\LDAP\Connection $ldap, ?callable $callback): bool
{
}