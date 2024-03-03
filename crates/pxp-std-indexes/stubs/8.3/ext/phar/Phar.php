<?php

class Phar extends \RecursiveDirectoryIterator implements \Countable, \ArrayAccess
{
    public function __construct(string $filename, int $flags = FilesystemIterator::SKIP_DOTS | FilesystemIterator::UNIX_PATHS, ?string $alias = null)
    {
    }
    public function __destruct()
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function addEmptyDir(string $directory)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function addFile(string $filename, ?string $localName = null)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function addFromString(string $localName, string $contents)
    {
    }
    /**
     * @tentative-return-type
     * @return (array | false)
     */
    public function buildFromDirectory(string $directory, string $pattern = "")
    {
    }
    /**
     * @tentative-return-type
     * @return (array | false)
     */
    public function buildFromIterator(Traversable $iterator, ?string $baseDirectory = null)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function compressFiles(int $compression)
    {
    }
    /** @return bool */
    public function decompressFiles()
    {
    }
    /**
     * @tentative-return-type
     * @return (Phar | null)
     */
    public function compress(int $compression, ?string $extension = null)
    {
    }
    /**
     * @tentative-return-type
     * @return (Phar | null)
     */
    public function decompress(?string $extension = null)
    {
    }
    /**
     * @tentative-return-type
     * @return (Phar | null)
     */
    public function convertToExecutable(?int $format = null, ?int $compression = null, ?string $extension = null)
    {
    }
    /**
     * @tentative-return-type
     * @return (PharData | null)
     */
    public function convertToData(?int $format = null, ?int $compression = null, ?string $extension = null)
    {
    }
    /** @return bool */
    public function copy(string $from, string $to)
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function count(int $mode = COUNT_NORMAL)
    {
    }
    /** @return bool */
    public function delete(string $localName)
    {
    }
    /** @return bool */
    public function delMetadata()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function extractTo(string $directory, array|string|null $files = null, bool $overwrite = false)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | null)
     */
    public function getAlias()
    {
    }
    /**
     * @tentative-return-type
     * @return string
     */
    public function getPath()
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function getMetadata(array $unserializeOptions = [])
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function getModified()
    {
    }
    /**
     * @tentative-return-type
     * @return (array | false)
     */
    public function getSignature()
    {
    }
    /**
     * @tentative-return-type
     * @return string
     */
    public function getStub()
    {
    }
    /**
     * @tentative-return-type
     * @return string
     */
    public function getVersion()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function hasMetadata()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function isBuffering()
    {
    }
    /**
     * @tentative-return-type
     * @return (int | false)
     */
    public function isCompressed()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function isFileFormat(int $format)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function isWritable()
    {
    }
    /**
     * @param string $localName
     * @tentative-return-type
     * @return bool
     */
    public function offsetExists($localName)
    {
    }
    /**
     * @param string $localName
     * @tentative-return-type
     * @return PharFileInfo
     */
    public function offsetGet($localName)
    {
    }
    /**
     * @param string $localName
     * @param (resource | string) $value
     * @tentative-return-type
     * @return void
     */
    public function offsetSet($localName, $value)
    {
    }
    /**
     * @param string $localName
     * @tentative-return-type
     * @return bool
     */
    public function offsetUnset($localName)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setAlias(string $alias)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setDefaultStub(?string $index = null, ?string $webIndex = null)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function setMetadata(mixed $metadata)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function setSignatureAlgorithm(int $algo, ?string $privateKey = null)
    {
    }
    /**
     * @param resource|string $stub
     * @return bool
     */
    public function setStub($stub, int $length = UNKNOWN)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function startBuffering()
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function stopBuffering()
    {
    }
    final public static function apiVersion(): string
    {
    }
    final public static function canCompress(int $compression = 0): bool
    {
    }
    final public static function canWrite(): bool
    {
    }
    final public static function createDefaultStub(?string $index = null, ?string $webIndex = null): string
    {
    }
    final public static function getSupportedCompression(): array
    {
    }
    final public static function getSupportedSignatures(): array
    {
    }
    final public static function interceptFileFuncs(): void
    {
    }
    final public static function isValidPharFilename(string $filename, bool $executable = true): bool
    {
    }
    final public static function loadPhar(string $filename, ?string $alias = null): bool
    {
    }
    final public static function mapPhar(?string $alias = null, int $offset = 0): bool
    {
    }
    final public static function running(bool $returnPhar = true): string
    {
    }
    final public static function mount(string $pharPath, string $externalPath): void
    {
    }
    final public static function mungServer(array $variables): void
    {
    }
    final public static function unlinkArchive(string $filename): bool
    {
    }
    final public static function webPhar(?string $alias = null, ?string $index = null, ?string $fileNotFoundScript = null, array $mimeTypes = [], ?callable $rewrite = null): void
    {
    }
    /**
     * @var int
     * @cvalue PHAR_ENT_COMPRESSED_BZ2
     */
    const BZ2 = UNKNOWN;
    /**
     * @cvalue PHAR_ENT_COMPRESSED_BZ2
     */
    const int BZ2 = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_ENT_COMPRESSED_GZ
     */
    const GZ = UNKNOWN;
    /**
     * @cvalue PHAR_ENT_COMPRESSED_GZ
     */
    const int GZ = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_ENT_COMPRESSED_NONE
     */
    const NONE = UNKNOWN;
    /**
     * @cvalue PHAR_ENT_COMPRESSED_NONE
     */
    const int NONE = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_FORMAT_PHAR
     */
    const PHAR = UNKNOWN;
    /**
     * @cvalue PHAR_FORMAT_PHAR
     */
    const int PHAR = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_FORMAT_TAR
     */
    const TAR = UNKNOWN;
    /**
     * @cvalue PHAR_FORMAT_TAR
     */
    const int TAR = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_FORMAT_ZIP
     */
    const ZIP = UNKNOWN;
    /**
     * @cvalue PHAR_FORMAT_ZIP
     */
    const int ZIP = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_ENT_COMPRESSION_MASK
     */
    const COMPRESSED = UNKNOWN;
    /**
     * @cvalue PHAR_ENT_COMPRESSION_MASK
     */
    const int COMPRESSED = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_MIME_PHP
     */
    const PHP = UNKNOWN;
    /**
     * @cvalue PHAR_MIME_PHP
     */
    const int PHP = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_MIME_PHPS
     */
    const PHPS = UNKNOWN;
    /**
     * @cvalue PHAR_MIME_PHPS
     */
    const int PHPS = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_SIG_MD5
     */
    const MD5 = UNKNOWN;
    /**
     * @cvalue PHAR_SIG_MD5
     */
    const int MD5 = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_SIG_OPENSSL
     */
    const OPENSSL = UNKNOWN;
    /**
     * @cvalue PHAR_SIG_OPENSSL
     */
    const int OPENSSL = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_SIG_OPENSSL_SHA256
     */
    const OPENSSL_SHA256 = UNKNOWN;
    /**
     * @cvalue PHAR_SIG_OPENSSL_SHA256
     */
    const int OPENSSL_SHA256 = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_SIG_OPENSSL_SHA512
     */
    const OPENSSL_SHA512 = UNKNOWN;
    /**
     * @cvalue PHAR_SIG_OPENSSL_SHA512
     */
    const int OPENSSL_SHA512 = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_SIG_SHA1
     */
    const SHA1 = UNKNOWN;
    /**
     * @cvalue PHAR_SIG_SHA1
     */
    const int SHA1 = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_SIG_SHA256
     */
    const SHA256 = UNKNOWN;
    /**
     * @cvalue PHAR_SIG_SHA256
     */
    const int SHA256 = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_SIG_SHA512
     */
    const SHA512 = UNKNOWN;
    /**
     * @cvalue PHAR_SIG_SHA512
     */
    const int SHA512 = UNKNOWN;
}