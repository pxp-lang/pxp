<?php 

#[\Until('8.4')]
function trigger_error(string $message, int $error_level = E_USER_NOTICE): bool
{
}
#[\Since('8.4')]
function trigger_error(string $message, int $error_level = E_USER_NOTICE): true
{
}