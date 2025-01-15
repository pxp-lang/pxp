<?php 

/** @param OpenSSLAsymmetricKey $private_key */
#[\Until('8.2')]
function openssl_csr_new(array $distinguished_names, &$private_key, ?array $options = null, ?array $extra_attributes = null): \OpenSSLCertificateSigningRequest|false
{
}
/**
 * @param OpenSSLAsymmetricKey|null $private_key
 */
#[\Since('8.2')]
function openssl_csr_new(array $distinguished_names, #[\SensitiveParameter] &$private_key, ?array $options = null, ?array $extra_attributes = null): \OpenSSLCertificateSigningRequest|bool
{
}