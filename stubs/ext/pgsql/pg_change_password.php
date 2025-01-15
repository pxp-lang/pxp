<?php 

#endif
#[\Since('8.4')]
function pg_change_password(\PgSql\Connection $connection, string $user, #[\SensitiveParameter] string $password): bool
{
}