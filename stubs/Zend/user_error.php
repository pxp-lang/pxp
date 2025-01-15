<?php 

/** @alias trigger_error */
#[\Until('8.4')]
function user_error(string $message, int $error_level = E_USER_NOTICE): bool
{
}
/** @alias trigger_error */
#[\Since('8.4')]
function user_error(string $message, int $error_level = E_USER_NOTICE): true
{
}