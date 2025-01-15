<?php 

/** @return resource|false */
#[\Until('8.4')]
function odbc_pconnect(string $dsn, string $user, string $password, int $cursor_option = SQL_CUR_USE_DRIVER)
{
}
#[\Since('8.4')]
function odbc_pconnect(string $dsn, ?string $user = null, #[\SensitiveParameter] ?string $password = null, int $cursor_option = SQL_CUR_USE_DRIVER): \Odbc\Connection|false
{
}