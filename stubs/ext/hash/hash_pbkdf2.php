<?php 

#[\Until('8.1')]
function hash_pbkdf2(string $algo, string $password, string $salt, int $iterations, int $length = 0, bool $binary = false): string
{
}
/** @refcount 1 */
#[\Since('8.1')]
function hash_pbkdf2(string $algo, string $password, string $salt, int $iterations, int $length = 0, bool $binary = false, array $options = []): string
{
}