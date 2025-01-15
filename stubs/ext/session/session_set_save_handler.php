<?php 

/**
 * @param callable|object $open
 * @param callable|bool $close
 */
#[\Until('8.3')]
function session_set_save_handler($open, $close = UNKNOWN, callable $read = UNKNOWN, callable $write = UNKNOWN, callable $destroy = UNKNOWN, callable $gc = UNKNOWN, callable $create_sid = UNKNOWN, callable $validate_sid = UNKNOWN, callable $update_timestamp = UNKNOWN): bool
{
}
/**
 * @param callable|object $open
 * @param callable|bool $close
 */
#[\Since('8.3')]
function session_set_save_handler($open, $close = UNKNOWN, callable $read = UNKNOWN, callable $write = UNKNOWN, callable $destroy = UNKNOWN, callable $gc = UNKNOWN, ?callable $create_sid = null, ?callable $validate_sid = null, ?callable $update_timestamp = null): bool
{
}