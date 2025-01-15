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
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CURRENT_MODE_MASK = UNKNOWN;
    /** @cvalue SPL_FILE_DIR_CURRENT_MODE_MASK */
    #[\Since('8.4')]
    public const int CURRENT_MODE_MASK = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_CURRENT_AS_PATHNAME
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CURRENT_AS_PATHNAME = UNKNOWN;
    /** @cvalue SPL_FILE_DIR_CURRENT_AS_PATHNAME */
    #[\Since('8.4')]
    public const int CURRENT_AS_PATHNAME = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_CURRENT_AS_FILEINFO
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CURRENT_AS_FILEINFO = UNKNOWN;
    /** @cvalue SPL_FILE_DIR_CURRENT_AS_FILEINFO */
    #[\Since('8.4')]
    public const int CURRENT_AS_FILEINFO = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_CURRENT_AS_SELF
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CURRENT_AS_SELF = UNKNOWN;
    /** @cvalue SPL_FILE_DIR_CURRENT_AS_SELF */
    #[\Since('8.4')]
    public const int CURRENT_AS_SELF = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_KEY_MODE_MASK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const KEY_MODE_MASK = UNKNOWN;
    /** @cvalue SPL_FILE_DIR_KEY_MODE_MASK */
    #[\Since('8.4')]
    public const int KEY_MODE_MASK = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_KEY_AS_PATHNAME
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const KEY_AS_PATHNAME = UNKNOWN;
    /** @cvalue SPL_FILE_DIR_KEY_AS_PATHNAME */
    #[\Since('8.4')]
    public const int KEY_AS_PATHNAME = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_FOLLOW_SYMLINKS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FOLLOW_SYMLINKS = UNKNOWN;
    /** @cvalue SPL_FILE_DIR_FOLLOW_SYMLINKS */
    #[\Since('8.4')]
    public const int FOLLOW_SYMLINKS = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_KEY_AS_FILENAME
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const KEY_AS_FILENAME = UNKNOWN;
    /** @cvalue SPL_FILE_DIR_KEY_AS_FILENAME */
    #[\Since('8.4')]
    public const int KEY_AS_FILENAME = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_NEW_CURRENT_AND_KEY
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const NEW_CURRENT_AND_KEY = UNKNOWN;
    /** @cvalue SPL_FILE_NEW_CURRENT_AND_KEY */
    #[\Since('8.4')]
    public const int NEW_CURRENT_AND_KEY = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_OTHERS_MASK
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const OTHER_MODE_MASK = UNKNOWN;
    /** @cvalue SPL_FILE_DIR_OTHERS_MASK */
    #[\Since('8.4')]
    public const int OTHER_MODE_MASK = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_SKIPDOTS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SKIP_DOTS = UNKNOWN;
    /** @cvalue SPL_FILE_DIR_SKIPDOTS */
    #[\Since('8.4')]
    public const int SKIP_DOTS = UNKNOWN;
    /**
     * @var int
     * @cvalue SPL_FILE_DIR_UNIXPATHS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const UNIX_PATHS = UNKNOWN;
    /** @cvalue SPL_FILE_DIR_UNIXPATHS */
    #[\Since('8.4')]
    public const int UNIX_PATHS = UNKNOWN;
}