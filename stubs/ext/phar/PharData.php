<?php 

class PharData extends \RecursiveDirectoryIterator implements \Countable, \ArrayAccess
{
    /** @implementation-alias Phar::__construct */
    public function __construct(string $filename, int $flags = FilesystemIterator::SKIP_DOTS | FilesystemIterator::UNIX_PATHS, ?string $alias = null, int $format = 0)
    {
    }
    /** @implementation-alias Phar::__destruct */
    public function __destruct()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::addEmptyDir
     * @return void
     */
    public function addEmptyDir(string $directory)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::addFile
     * @return void
     */
    public function addFile(string $filename, ?string $localName = null)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::addFromString
     * @return void
     */
    public function addFromString(string $localName, string $contents)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::buildFromDirectory
     * @return (array | false)
     */
    public function buildFromDirectory(string $directory, string $pattern = "")
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::buildFromIterator
     * @return (array | false)
     */
    public function buildFromIterator(Traversable $iterator, ?string $baseDirectory = null)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::compressFiles
     * @return void
     */
    public function compressFiles(int $compression)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::decompressFiles
     * @return bool
     */
    public function decompressFiles()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::compress
     * @return (PharData | null)
     */
    public function compress(int $compression, ?string $extension = null)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::decompress
     * @return (PharData | null)
     */
    public function decompress(?string $extension = null)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::convertToExecutable
     * @return (Phar | null)
     */
    public function convertToExecutable(?int $format = null, ?int $compression = null, ?string $extension = null)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::convertToData
     * @return (PharData | null)
     */
    public function convertToData(?int $format = null, ?int $compression = null, ?string $extension = null)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::copy
     * @return bool
     */
    public function copy(string $from, string $to)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::count
     * @return int
     */
    public function count(int $mode = COUNT_NORMAL)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::delete
     * @return bool
     */
    public function delete(string $localName)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::delMetadata
     * @return bool
     */
    public function delMetadata()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::extractTo
     * @return bool
     */
    public function extractTo(string $directory, array|string|null $files = null, bool $overwrite = false)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::getAlias
     * @return (string | null)
     */
    public function getAlias()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::getPath
     * @return string
     */
    public function getPath()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::getMetadata
     * @return mixed
     */
    public function getMetadata(array $unserializeOptions = [])
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::getModified
     * @return bool
     */
    public function getModified()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::getSignature
     * @return (array | false)
     */
    public function getSignature()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::getStub
     * @return string
     */
    public function getStub()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::getVersion
     * @return string
     */
    public function getVersion()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::hasMetadata
     * @return bool
     */
    public function hasMetadata()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::isBuffering
     * @return bool
     */
    public function isBuffering()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::isCompressed
     * @return (int | false)
     */
    public function isCompressed()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::isFileFormat
     * @return bool
     */
    public function isFileFormat(int $format)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::isWritable
     * @return bool
     */
    public function isWritable()
    {
    }
    /**
     * @param string $localName
     * @tentative-return-type
     * @implementation-alias Phar::offsetExists
     * @return bool
     */
    public function offsetExists($localName)
    {
    }
    /**
     * @param string $localName
     * @tentative-return-type
     * @implementation-alias Phar::offsetGet
     * @return PharFileInfo
     */
    public function offsetGet($localName)
    {
    }
    /**
     * @param string $localName
     * @param (resource | string) $value
     * @tentative-return-type
     * @implementation-alias Phar::offsetSet
     * @return void
     */
    public function offsetSet($localName, $value)
    {
    }
    /**
     * @param string $localName
     * @tentative-return-type
     * @implementation-alias Phar::offsetUnset
     * @return bool
     */
    public function offsetUnset($localName)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::setAlias
     * @return bool
     */
    public function setAlias(string $alias)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::setDefaultStub
     * @return bool
     */
    public function setDefaultStub(?string $index = null, ?string $webIndex = null)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::setMetadata
     * @return void
     */
    public function setMetadata(mixed $metadata)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::setSignatureAlgorithm
     * @return void
     */
    public function setSignatureAlgorithm(int $algo, ?string $privateKey = null)
    {
    }
    /**
     * @param resource|string $stub
     * @return bool
     * @implementation-alias Phar::setStub
     */
    #[\Until('8.4')]
    public function setStub($stub, int $length = UNKNOWN)
    {
    }
    /**
     * @param resource|string $stub
     * @implementation-alias Phar::setStub
     */
    #[\Since('8.4')]
    public function setStub($stub, int $length = UNKNOWN): true
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::startBuffering
     * @return void
     */
    public function startBuffering()
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Phar::stopBuffering
     * @return void
     */
    public function stopBuffering()
    {
    }
    /** @implementation-alias Phar::apiVersion */
    final public static function apiVersion(): string
    {
    }
    /** @implementation-alias Phar::canCompress */
    final public static function canCompress(int $compression = 0): bool
    {
    }
    /** @implementation-alias Phar::canWrite */
    final public static function canWrite(): bool
    {
    }
    /** @implementation-alias Phar::createDefaultStub */
    final public static function createDefaultStub(?string $index = null, ?string $webIndex = null): string
    {
    }
    /** @implementation-alias Phar::getSupportedCompression */
    final public static function getSupportedCompression(): array
    {
    }
    /** @implementation-alias Phar::getSupportedSignatures */
    final public static function getSupportedSignatures(): array
    {
    }
    /** @implementation-alias Phar::interceptFileFuncs */
    final public static function interceptFileFuncs(): void
    {
    }
    /** @implementation-alias Phar::isValidPharFilename */
    final public static function isValidPharFilename(string $filename, bool $executable = true): bool
    {
    }
    /** @implementation-alias Phar::loadPhar */
    final public static function loadPhar(string $filename, ?string $alias = null): bool
    {
    }
    /** @implementation-alias Phar::mapPhar */
    final public static function mapPhar(?string $alias = null, int $offset = 0): bool
    {
    }
    /** @implementation-alias Phar::running */
    final public static function running(bool $returnPhar = true): string
    {
    }
    /** @implementation-alias Phar::mount */
    final public static function mount(string $pharPath, string $externalPath): void
    {
    }
    /** @implementation-alias Phar::mungServer */
    final public static function mungServer(array $variables): void
    {
    }
    /** @implementation-alias Phar::unlinkArchive */
    #[\Until('8.4')]
    final public static function unlinkArchive(string $filename): bool
    {
    }
    /** @implementation-alias Phar::unlinkArchive */
    #[\Since('8.4')]
    final public static function unlinkArchive(string $filename): true
    {
    }
    /** @implementation-alias Phar::webPhar */
    final public static function webPhar(?string $alias = null, ?string $index = null, ?string $fileNotFoundScript = null, array $mimeTypes = [], ?callable $rewrite = null): void
    {
    }
}