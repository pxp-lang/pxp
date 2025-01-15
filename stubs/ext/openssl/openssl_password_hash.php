<?php 

#if defined(HAVE_OPENSSL_ARGON2)
#[\Since('8.4')]
function openssl_password_hash(string $algo, #[\SensitiveParameter] string $password, array $options = []): string
{
}