<?php 

/**
 * @param resource $socket
 */
#[\Since('8.4')]
function pg_socket_poll($socket, int $read, int $write, int $timeout = -1): int
{
}