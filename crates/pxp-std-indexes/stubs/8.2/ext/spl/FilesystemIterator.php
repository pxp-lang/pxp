<?php

class FilesystemIterator extends \DirectoryIterator
{
    public function __construct(string $directory, int $flags = FilesystemIterator::KEY_AS_PATHNAME | FilesystemIterator::CURRENT_AS_FILEINFO | FilesystemIterator::SKIP_DOTS)
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
     * @return string
     */
    public function key()
    {
    }
    /**
     * @tentative-return-type
     * @return (string | SplFileInfo | FilesystemIterator)
     */
    public function current()
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
    public function setFlags(int $flags)
    {
    }
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_CURRENT_MODE_MASK
     */
    public const CURRENT_MODE_MASK = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_CURRENT_AS_PATHNAME
     */
    public const CURRENT_AS_PATHNAME = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_CURRENT_AS_FILEINFO
     */
    public const CURRENT_AS_FILEINFO = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_CURRENT_AS_SELF
     */
    public const CURRENT_AS_SELF = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_KEY_MODE_MASK
     */
    public const KEY_MODE_MASK = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_KEY_AS_PATHNAME
     */
    public const KEY_AS_PATHNAME = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_FOLLOW_SYMLINKS
     */
    public const FOLLOW_SYMLINKS = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_KEY_AS_FILENAME
     */
    public const KEY_AS_FILENAME = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_NEW_CURRENT_AND_KEY
     */
    public const NEW_CURRENT_AND_KEY = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_OTHERS_MASK
     */
    public const OTHER_MODE_MASK = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_SKIPDOTS
     */
    public const SKIP_DOTS = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_UNIXPATHS
     */
    public const UNIX_PATHS = UNKNOWN;
}