<?php 

class Phar extends \RecursiveDirectoryIterator implements \Countable, \ArrayAccess
{
    /**
     * @var int
     * @cvalue PHAR_ENT_COMPRESSED_BZ2
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    const BZ2 = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_ENT_COMPRESSED_GZ
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    const GZ = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_ENT_COMPRESSED_NONE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    const NONE = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_FORMAT_PHAR
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    const PHAR = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_FORMAT_TAR
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    const TAR = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_FORMAT_ZIP
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    const ZIP = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_ENT_COMPRESSION_MASK
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    const COMPRESSED = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_MIME_PHP
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    const PHP = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_MIME_PHPS
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    const PHPS = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_SIG_MD5
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    const MD5 = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_SIG_OPENSSL
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    const OPENSSL = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_SIG_OPENSSL_SHA256
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    const OPENSSL_SHA256 = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_SIG_OPENSSL_SHA512
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    const OPENSSL_SHA512 = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_SIG_SHA1
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    const SHA1 = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_SIG_SHA256
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    const SHA256 = UNKNOWN;
    /**
     * @var int
     * @cvalue PHAR_SIG_SHA512
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    const SHA512 = UNKNOWN;
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
    /**
     * @tentative-return-type
     * @return bool
     */
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
    /**
     * @tentative-return-type
     * @return bool
     */
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
    /**
     * @tentative-return-type
     * @return bool
     */
    public function delete(string $localName)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
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
     * @param (resource | string) $stub
     * @tentative-return-type
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
    #[\Until('8.4')]
    final public static function unlinkArchive(string $filename): bool
    {
    }
    #[\Since('8.4')]
    final public static function unlinkArchive(string $filename): true
    {
    }
    final public static function webPhar(?string $alias = null, ?string $index = null, ?string $fileNotFoundScript = null, array $mimeTypes = [], ?callable $rewrite = null): void
    {
    }
    /**
     * @cvalue PHAR_ENT_COMPRESSED_BZ2
     */
    #[\Since('8.3')]
    const int BZ2 = UNKNOWN;
    /**
     * @cvalue PHAR_ENT_COMPRESSED_GZ
     */
    #[\Since('8.3')]
    const int GZ = UNKNOWN;
    /**
     * @cvalue PHAR_ENT_COMPRESSED_NONE
     */
    #[\Since('8.3')]
    const int NONE = UNKNOWN;
    /**
     * @cvalue PHAR_FORMAT_PHAR
     */
    #[\Since('8.3')]
    const int PHAR = UNKNOWN;
    /**
     * @cvalue PHAR_FORMAT_TAR
     */
    #[\Since('8.3')]
    const int TAR = UNKNOWN;
    /**
     * @cvalue PHAR_FORMAT_ZIP
     */
    #[\Since('8.3')]
    const int ZIP = UNKNOWN;
    /**
     * @cvalue PHAR_ENT_COMPRESSION_MASK
     */
    #[\Since('8.3')]
    const int COMPRESSED = UNKNOWN;
    /**
     * @cvalue PHAR_MIME_PHP
     */
    #[\Since('8.3')]
    const int PHP = UNKNOWN;
    /**
     * @cvalue PHAR_MIME_PHPS
     */
    #[\Since('8.3')]
    const int PHPS = UNKNOWN;
    /**
     * @cvalue PHAR_SIG_MD5
     */
    #[\Since('8.3')]
    const int MD5 = UNKNOWN;
    /**
     * @cvalue PHAR_SIG_OPENSSL
     */
    #[\Since('8.3')]
    const int OPENSSL = UNKNOWN;
    /**
     * @cvalue PHAR_SIG_OPENSSL_SHA256
     */
    #[\Since('8.3')]
    const int OPENSSL_SHA256 = UNKNOWN;
    /**
     * @cvalue PHAR_SIG_OPENSSL_SHA512
     */
    #[\Since('8.3')]
    const int OPENSSL_SHA512 = UNKNOWN;
    /**
     * @cvalue PHAR_SIG_SHA1
     */
    #[\Since('8.3')]
    const int SHA1 = UNKNOWN;
    /**
     * @cvalue PHAR_SIG_SHA256
     */
    #[\Since('8.3')]
    const int SHA256 = UNKNOWN;
    /**
     * @cvalue PHAR_SIG_SHA512
     */
    #[\Since('8.3')]
    const int SHA512 = UNKNOWN;
}