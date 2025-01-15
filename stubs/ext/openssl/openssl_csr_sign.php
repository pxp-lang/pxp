<?php 

/** @param OpenSSLAsymmetricKey|OpenSSLCertificate|array|string $private_key */
#[\Until('8.4')]
function openssl_csr_sign(\OpenSSLCertificateSigningRequest|string $csr, \OpenSSLCertificate|string|null $ca_certificate, $private_key, int $days, ?array $options = null, int $serial = 0): \OpenSSLCertificate|false
{
}
/**
 * @param OpenSSLAsymmetricKey|OpenSSLCertificate|array|string $private_key
 */
#[\Since('8.4')]
function openssl_csr_sign(\OpenSSLCertificateSigningRequest|string $csr, \OpenSSLCertificate|string|null $ca_certificate, #[\SensitiveParameter] $private_key, int $days, ?array $options = null, int $serial = 0, ?string $serial_hex = null): \OpenSSLCertificate|false
{
}