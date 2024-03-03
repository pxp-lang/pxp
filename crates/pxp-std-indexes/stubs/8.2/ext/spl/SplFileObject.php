<?php

#endif
class SplFileObject extends \SplFileInfo implements \RecursiveIterator, \SeekableIterator
{
    /** @param resource|null $context */
    public function __construct(string $filename, string $mode = "r", bool $useIncludePath = false, $context = null)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function rewind()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function eof()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function valid()
    {
    }
    /**
     * @tentative-return-type
     * @return string
     */
    public function fgets()
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public function fread(int $length)
    {
    }
    /**
     * @tentative-return-type
     * @return (array | false)
     */
    public function fgetcsv(string $separator = ",", string $enclosure = "\"", string $escape = "\\")
    {
    }
    /** @tentative-return-type */
    public function fputcsv(array $fields, string $separator = ",", string $enclosure = "\"", string $escape = "\\", string $eol = "\n")
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function setCsvControl(string $separator = ",", string $enclosure = "\"", string $escape = "\\")
    {
    }
    /**
     * @tentative-return-type
     * @return array
     */
    public function getCsvControl()
    {
    }
    /**
     * @param int $wouldBlock
     * @tentative-return-type
     * @return bool
     */
    public function flock(int $operation, &$wouldBlock = null)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function fflush()
    {
    }
    /**
     * @tentative-return-type
     * @return (int | false)
     */
    public function ftell()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function fseek(int $offset, int $whence = SEEK_SET)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public function fgetc()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function fpassthru()
    {
    }
    /**
     * @tentative-return-type
     * @return (array | int | null)
     */
    public function fscanf(string $format, mixed &...$vars)
    {
    }
    /**
     * @tentative-return-type
     * @return (int | false)
     */
    public function fwrite(string $data, int $length = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return array
     */
    public function fstat()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function ftruncate(int $size)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | array | false)
     */
    public function current()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function key()
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function next()
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function setFlags(int $flags)
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function getFlags()
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function setMaxLineLen(int $maxLength)
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function getMaxLineLen()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function hasChildren()
    {
    }
    /**
     * @tentative-return-type
     * @return (RecursiveIterator | null)
     */
    public function getChildren()
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function seek(int $line)
    {
    }
    /**
     * @tentative-return-type
     * @alias SplFileObject::fgets
     * @return string
     */
    public function getCurrentLine()
    {
    }
    /** @implementation-alias SplFileObject::fgets */
    public function __toString(): string
    {
    }
    /**
     * @var int
     * @cvalue SPL_FILE_OBJECT_DROP_NEW_LINE
     */
    public const DROP_NEW_LINE = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_OBJECT_READ_AHEAD
     */
    public const READ_AHEAD = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_OBJECT_SKIP_EMPTY
     */
    public const SKIP_EMPTY = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_OBJECT_READ_CSV
     */
    public const READ_CSV = UNKNOWN;
}