<?php 

class ZipArchive
{
    /**
     * @tentative-return-type
     * @return bool
     */
    #[\Until('8.3')]
    public function addFile(string $filepath, string $entryname = "", int $start = 0, int $length = 0, int $flags = ZipArchive::FL_OVERWRITE)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    #[\Until('8.3')]
    public function replaceFile(string $filepath, int $index, int $start = 0, int $length = 0, int $flags = 0)
    {
    }
    /**
     * @var int
     * @cvalue ZIP_CREATE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CREATE = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_EXCL
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const EXCL = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CHECKCONS
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CHECKCONS = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OVERWRITE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OVERWRITE = UNKNOWN;
    #ifdef ZIP_RDONLY
    /**
     * @var int
     * @cvalue ZIP_RDONLY
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const RDONLY = UNKNOWN;
    #endif
    /**
     * @var int
     * @cvalue ZIP_FL_NOCASE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const FL_NOCASE = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_FL_NODIR
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const FL_NODIR = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_FL_COMPRESSED
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const FL_COMPRESSED = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_FL_UNCHANGED
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const FL_UNCHANGED = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_FL_RECOMPRESS
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const FL_RECOMPRESS = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_FL_ENCRYPTED
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const FL_ENCRYPTED = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_FL_OVERWRITE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const FL_OVERWRITE = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_FL_LOCAL
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const FL_LOCAL = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_FL_CENTRAL
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const FL_CENTRAL = UNKNOWN;
    /* Default filename encoding policy. */
    /**
     * @var int
     * @cvalue ZIP_FL_ENC_GUESS
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const FL_ENC_GUESS = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_FL_ENC_RAW
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const FL_ENC_RAW = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_FL_ENC_STRICT
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const FL_ENC_STRICT = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_FL_ENC_UTF_8
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const FL_ENC_UTF_8 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_FL_ENC_CP437
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const FL_ENC_CP437 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CM_DEFAULT
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_DEFAULT = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CM_STORE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_STORE = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CM_SHRINK
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_SHRINK = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CM_REDUCE_1
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_REDUCE_1 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CM_REDUCE_2
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_REDUCE_2 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CM_REDUCE_3
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_REDUCE_3 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CM_REDUCE_4
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_REDUCE_4 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CM_IMPLODE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_IMPLODE = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CM_DEFLATE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_DEFLATE = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CM_DEFLATE64
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_DEFLATE64 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CM_PKWARE_IMPLODE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_PKWARE_IMPLODE = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CM_BZIP2
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_BZIP2 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CM_LZMA
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_LZMA = UNKNOWN;
    #ifdef ZIP_CM_LZMA2
    /**
     * @var int
     * @cvalue ZIP_CM_LZMA2
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_LZMA2 = UNKNOWN;
    #endif
    #ifdef ZIP_CM_ZSTD
    /**
     * @var int
     * @cvalue ZIP_CM_ZSTD
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_ZSTD = UNKNOWN;
    #endif
    #ifdef ZIP_CM_XZ
    /**
     * @var int
     * @cvalue ZIP_CM_XZ
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_XZ = UNKNOWN;
    #endif
    /**
     * @var int
     * @cvalue ZIP_CM_TERSE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_TERSE = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CM_LZ77
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_LZ77 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CM_WAVPACK
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_WAVPACK = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_CM_PPMD
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const CM_PPMD = UNKNOWN;
    /* Error code */
    /**
     * N No error
     * @var int
     * @cvalue ZIP_ER_OK
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_OK = UNKNOWN;
    /**
     * N Multi-disk zip archives not supported
     * @var int
     * @cvalue ZIP_ER_MULTIDISK
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_MULTIDISK = UNKNOWN;
    /**
     * S Renaming temporary file failed
     * @var int
     * @cvalue ZIP_ER_RENAME
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_RENAME = UNKNOWN;
    /**
     * S Closing zip archive failed
     * @var int
     * @cvalue ZIP_ER_CLOSE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_CLOSE = UNKNOWN;
    /**
     * S Seek error
     * @var int
     * @cvalue ZIP_ER_SEEK
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_SEEK = UNKNOWN;
    /**
     * S Read error
     * @var int
     * @cvalue ZIP_ER_READ
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_READ = UNKNOWN;
    /**
     * S Write error
     * @var int
     * @cvalue ZIP_ER_WRITE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_WRITE = UNKNOWN;
    /**
     * N CRC error
     * @var int
     * @cvalue ZIP_ER_CRC
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_CRC = UNKNOWN;
    /**
     * N Containing zip archive was closed
     * @var int
     * @cvalue ZIP_ER_ZIPCLOSED
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_ZIPCLOSED = UNKNOWN;
    /**
     * N No such file
     * @var int
     * @cvalue ZIP_ER_NOENT
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_NOENT = UNKNOWN;
    /**
     * N File already exists
     * @var int
     * @cvalue ZIP_ER_EXISTS
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_EXISTS = UNKNOWN;
    /**
     * S Can't open file
     * @var int
     * @cvalue ZIP_ER_OPEN
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_OPEN = UNKNOWN;
    /**
     * S Failure to create temporary file
     * @var int
     * @cvalue ZIP_ER_TMPOPEN
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_TMPOPEN = UNKNOWN;
    /**
     * Z Zlib error
     * @var int
     * @cvalue ZIP_ER_ZLIB
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_ZLIB = UNKNOWN;
    /**
     * N Malloc failure
     * @var int
     * @cvalue ZIP_ER_MEMORY
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_MEMORY = UNKNOWN;
    /**
     * N Entry has been changed
     * @var int
     * @cvalue ZIP_ER_CHANGED
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_CHANGED = UNKNOWN;
    /**
     * N Compression method not supported
     * @var int
     * @cvalue ZIP_ER_COMPNOTSUPP
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_COMPNOTSUPP = UNKNOWN;
    /**
     * N Premature EOF
     * @var int
     * @cvalue ZIP_ER_EOF
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_EOF = UNKNOWN;
    /**
     * N Invalid argument
     * @var int
     * @cvalue ZIP_ER_INVAL
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_INVAL = UNKNOWN;
    /**
     * N Not a zip archive
     * @var int
     * @cvalue ZIP_ER_NOZIP
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_NOZIP = UNKNOWN;
    /**
     * N Internal error
     * @var int
     * @cvalue ZIP_ER_INTERNAL
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_INTERNAL = UNKNOWN;
    /**
     * N Zip archive inconsistent
     * @var int
     * @cvalue ZIP_ER_INCONS
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_INCONS = UNKNOWN;
    /**
     * S Can't remove file
     * @var int
     * @cvalue ZIP_ER_REMOVE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_REMOVE = UNKNOWN;
    /**
     * N Entry has been deleted
     * @var int
     * @cvalue ZIP_ER_DELETED
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_DELETED = UNKNOWN;
    /**
     * N Encryption method not supported
     * @var int
     * @cvalue ZIP_ER_ENCRNOTSUPP
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_ENCRNOTSUPP = UNKNOWN;
    /**
     * N Read-only archive
     * @var int
     * @cvalue ZIP_ER_RDONLY
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_RDONLY = UNKNOWN;
    /**
     * N Entry has been deleted
     * @var int
     * @cvalue ZIP_ER_NOPASSWD
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_NOPASSWD = UNKNOWN;
    /**
     * N Wrong password provided
     * @var int
     * @cvalue ZIP_ER_WRONGPASSWD
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_WRONGPASSWD = UNKNOWN;
    /* since 1.0.0 */
    #ifdef ZIP_ER_OPNOTSUPP
    /**
     * N Operation not supported
     * @var int
     * @cvalue ZIP_ER_OPNOTSUPP
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_OPNOTSUPP = UNKNOWN;
    #endif
    #ifdef ZIP_ER_INUSE
    /**
     * N Resource still in use
     * @var int
     * @cvalue ZIP_ER_INUSE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_INUSE = UNKNOWN;
    #endif
    #ifdef ZIP_ER_TELL
    /**
     * S Tell error
     * @var int
     * @cvalue ZIP_ER_TELL
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_TELL = UNKNOWN;
    #endif
    /* since 1.6.0 */
    #ifdef ZIP_ER_COMPRESSED_DATA
    /**
     * N Compressed data invalid
     * @var int
     * @cvalue ZIP_ER_COMPRESSED_DATA
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_COMPRESSED_DATA = UNKNOWN;
    #endif
    #ifdef ZIP_ER_CANCELLED
    /**
     * N Operation cancelled
     * @var int
     * @cvalue ZIP_ER_CANCELLED
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ER_CANCELLED = UNKNOWN;
    #endif
    #ifdef ZIP_OPSYS_DEFAULT
    /**
     * @var int
     * @cvalue ZIP_OPSYS_DOS
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_DOS = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_AMIGA
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_AMIGA = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_OPENVMS
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_OPENVMS = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_UNIX
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_UNIX = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_VM_CMS
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_VM_CMS = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_ATARI_ST
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_ATARI_ST = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_OS_2
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_OS_2 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_MACINTOSH
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_MACINTOSH = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_Z_SYSTEM
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_Z_SYSTEM = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_CPM
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_CPM = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_WINDOWS_NTFS
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_WINDOWS_NTFS = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_MVS
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_MVS = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_VSE
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_VSE = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_ACORN_RISC
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_ACORN_RISC = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_VFAT
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_VFAT = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_ALTERNATE_MVS
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_ALTERNATE_MVS = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_BEOS
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_BEOS = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_TANDEM
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_TANDEM = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_OS_400
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_OS_400 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_OS_X
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_OS_X = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_OPSYS_DEFAULT
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const OPSYS_DEFAULT = UNKNOWN;
    #endif
    /**
     * @var int
     * @cvalue ZIP_EM_NONE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const EM_NONE = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_EM_TRAD_PKWARE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const EM_TRAD_PKWARE = UNKNOWN;
    #ifdef HAVE_ENCRYPTION
    /**
     * @var int
     * @cvalue ZIP_EM_AES_128
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const EM_AES_128 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_EM_AES_192
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const EM_AES_192 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZIP_EM_AES_256
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const EM_AES_256 = UNKNOWN;
    #endif
    /**
     * @var int
     * @cvalue ZIP_EM_UNKNOWN
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const EM_UNKNOWN = UNKNOWN;
    /**
     * @var string
     * @cvalue LIBZIP_VERSION_STR
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const LIBZIP_VERSION = UNKNOWN;
    /**
     * @tentative-return-type
     * @return (bool | int)
     */
    public function open(string $filename, int $flags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setPassword(string $password)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function close()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function count()
    {
    }
    /**
     * @tentative-return-type
     * @return string
     */
    public function getStatusString()
    {
    }
    #[\Since('8.2')]
    public function clearError(): void
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function addEmptyDir(string $dirname, int $flags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function addFromString(string $name, string $content, int $flags = ZipArchive::FL_OVERWRITE)
    {
    }
    /** @tentative-return-type */
    #[\Since('8.3')]
    public function addFile(string $filepath, string $entryname = "", int $start = 0, int $length = ZipArchive::LENGTH_TO_END, int $flags = ZipArchive::FL_OVERWRITE)
    {
    }
    /** @tentative-return-type */
    #[\Since('8.3')]
    public function replaceFile(string $filepath, int $index, int $start = 0, int $length = ZipArchive::LENGTH_TO_END, int $flags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return (array | false)
     */
    public function addGlob(string $pattern, int $flags = 0, array $options = [])
    {
    }
    /**
     * @tentative-return-type
     * @return (array | false)
     */
    public function addPattern(string $pattern, string $path = ".", array $options = [])
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function renameIndex(int $index, string $new_name)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function renameName(string $name, string $new_name)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setArchiveComment(string $comment)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public function getArchiveComment(int $flags = 0)
    {
    }
    #[\Since('8.3')]
    public function setArchiveFlag(int $flag, int $value): bool
    {
    }
    #[\Since('8.3')]
    public function getArchiveFlag(int $flag, int $flags = 0): int
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setCommentIndex(int $index, string $comment)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setCommentName(string $name, string $comment)
    {
    }
    #ifdef HAVE_SET_MTIME
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setMtimeIndex(int $index, int $timestamp, int $flags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setMtimeName(string $name, int $timestamp, int $flags = 0)
    {
    }
    #endif
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public function getCommentIndex(int $index, int $flags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public function getCommentName(string $name, int $flags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function deleteIndex(int $index)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function deleteName(string $name)
    {
    }
    /**
     * @tentative-return-type
     * @return (array | false)
     */
    public function statName(string $name, int $flags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return (array | false)
     */
    public function statIndex(int $index, int $flags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return (int | false)
     */
    public function locateName(string $name, int $flags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public function getNameIndex(int $index, int $flags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function unchangeArchive()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function unchangeAll()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function unchangeIndex(int $index)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function unchangeName(string $name)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function extractTo(string $pathto, array|string|null $files = null)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public function getFromName(string $name, int $len = 0, int $flags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public function getFromIndex(int $index, int $len = 0, int $flags = 0)
    {
    }
    /** @return resource|false */
    #[\Since('8.2')]
    public function getStreamIndex(int $index, int $flags = 0)
    {
    }
    /** @return resource|false */
    #[\Since('8.2')]
    public function getStreamName(string $name, int $flags = 0)
    {
    }
    /** @return resource|false */
    public function getStream(string $name)
    {
    }
    #ifdef ZIP_OPSYS_DEFAULT
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setExternalAttributesName(string $name, int $opsys, int $attr, int $flags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setExternalAttributesIndex(int $index, int $opsys, int $attr, int $flags = 0)
    {
    }
    /**
     * @param int $opsys
     * @param int $attr
     * @tentative-return-type
     * @return bool
     */
    public function getExternalAttributesName(string $name, &$opsys, &$attr, int $flags = 0)
    {
    }
    /**
     * @param int $opsys
     * @param int $attr
     * @tentative-return-type
     * @return bool
     */
    public function getExternalAttributesIndex(int $index, &$opsys, &$attr, int $flags = 0)
    {
    }
    #endif
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setCompressionName(string $name, int $method, int $compflags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setCompressionIndex(int $index, int $method, int $compflags = 0)
    {
    }
    #ifdef HAVE_ENCRYPTION
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setEncryptionName(string $name, int $method, ?string $password = null)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setEncryptionIndex(int $index, int $method, ?string $password = null)
    {
    }
    #endif
    #ifdef HAVE_PROGRESS_CALLBACK
    /**
     * @tentative-return-type
     * @return bool
     */
    public function registerProgressCallback(float $rate, callable $callback)
    {
    }
    #endif
    #ifdef HAVE_CANCEL_CALLBACK
    /**
     * @tentative-return-type
     * @return bool
     */
    public function registerCancelCallback(callable $callback)
    {
    }
    #endif
    #ifdef HAVE_METHOD_SUPPORTED
    /** @return bool */
    public static function isCompressionMethodSupported(int $method, bool $enc = true): bool
    {
    }
    /** @return bool */
    public static function isEncryptionMethodSupported(int $method, bool $enc = true): bool
    {
    }
    /**
     * @cvalue ZIP_CREATE
     */
    #[\Since('8.3')]
    public const int CREATE = UNKNOWN;
    /**
     * @cvalue ZIP_EXCL
     */
    #[\Since('8.3')]
    public const int EXCL = UNKNOWN;
    /**
     * @cvalue ZIP_CHECKCONS
     */
    #[\Since('8.3')]
    public const int CHECKCONS = UNKNOWN;
    /**
     * @cvalue ZIP_OVERWRITE
     */
    #[\Since('8.3')]
    public const int OVERWRITE = UNKNOWN;
    #ifdef ZIP_RDONLY
    /**
     * @cvalue ZIP_RDONLY
     */
    #[\Since('8.3')]
    public const int RDONLY = UNKNOWN;
    #endif
    /**
     * @cvalue ZIP_FL_NOCASE
     */
    #[\Since('8.3')]
    public const int FL_NOCASE = UNKNOWN;
    /**
     * @cvalue ZIP_FL_NODIR
     */
    #[\Since('8.3')]
    public const int FL_NODIR = UNKNOWN;
    /**
     * @cvalue ZIP_FL_COMPRESSED
     */
    #[\Since('8.3')]
    public const int FL_COMPRESSED = UNKNOWN;
    /**
     * @cvalue ZIP_FL_UNCHANGED
     */
    #[\Since('8.3')]
    public const int FL_UNCHANGED = UNKNOWN;
    /* deprecated in libzip 1.10.0 */
    #ifdef ZIP_FL_RECOMPRESS
    /**
     * @cvalue ZIP_FL_RECOMPRESS
     * @deprecated
     */
    #[\Since('8.3')]
    #[\Until('8.4')]
    public const int FL_RECOMPRESS = UNKNOWN;
    /* deprecated in libzip 1.10.0 */
    #ifdef ZIP_FL_RECOMPRESS
    /**
     * @cvalue ZIP_FL_RECOMPRESS
     */
    #[\Deprecated(since: '8.3')]
    #[\Since('8.4')]
    public const int FL_RECOMPRESS = UNKNOWN;
    #endif
    /**
     * @cvalue ZIP_FL_ENCRYPTED
     */
    #[\Since('8.3')]
    public const int FL_ENCRYPTED = UNKNOWN;
    /**
     * @cvalue ZIP_FL_OVERWRITE
     */
    #[\Since('8.3')]
    public const int FL_OVERWRITE = UNKNOWN;
    /**
     * @cvalue ZIP_FL_LOCAL
     */
    #[\Since('8.3')]
    public const int FL_LOCAL = UNKNOWN;
    /**
     * @cvalue ZIP_FL_CENTRAL
     */
    #[\Since('8.3')]
    public const int FL_CENTRAL = UNKNOWN;
    /* Default filename encoding policy. */
    /**
     * @cvalue ZIP_FL_ENC_GUESS
     */
    #[\Since('8.3')]
    public const int FL_ENC_GUESS = UNKNOWN;
    /**
     * @cvalue ZIP_FL_ENC_RAW
     */
    #[\Since('8.3')]
    public const int FL_ENC_RAW = UNKNOWN;
    /**
     * @cvalue ZIP_FL_ENC_STRICT
     */
    #[\Since('8.3')]
    public const int FL_ENC_STRICT = UNKNOWN;
    /**
     * @cvalue ZIP_FL_ENC_UTF_8
     */
    #[\Since('8.3')]
    public const int FL_ENC_UTF_8 = UNKNOWN;
    /**
     * @cvalue ZIP_FL_ENC_CP437
     */
    #[\Since('8.3')]
    public const int FL_ENC_CP437 = UNKNOWN;
    /**
     * Additionnal flags not from libzip
     * @cvalue ZIP_FL_OPEN_FILE_NOW
     */
    #[\Since('8.3')]
    public const int FL_OPEN_FILE_NOW = UNKNOWN;
    /**
     * @cvalue ZIP_CM_DEFAULT
     */
    #[\Since('8.3')]
    public const int CM_DEFAULT = UNKNOWN;
    /**
     * @cvalue ZIP_CM_STORE
     */
    #[\Since('8.3')]
    public const int CM_STORE = UNKNOWN;
    /**
     * @cvalue ZIP_CM_SHRINK
     */
    #[\Since('8.3')]
    public const int CM_SHRINK = UNKNOWN;
    /**
     * @cvalue ZIP_CM_REDUCE_1
     */
    #[\Since('8.3')]
    public const int CM_REDUCE_1 = UNKNOWN;
    /**
     * @cvalue ZIP_CM_REDUCE_2
     */
    #[\Since('8.3')]
    public const int CM_REDUCE_2 = UNKNOWN;
    /**
     * @cvalue ZIP_CM_REDUCE_3
     */
    #[\Since('8.3')]
    public const int CM_REDUCE_3 = UNKNOWN;
    /**
     * @cvalue ZIP_CM_REDUCE_4
     */
    #[\Since('8.3')]
    public const int CM_REDUCE_4 = UNKNOWN;
    /**
     * @cvalue ZIP_CM_IMPLODE
     */
    #[\Since('8.3')]
    public const int CM_IMPLODE = UNKNOWN;
    /**
     * @cvalue ZIP_CM_DEFLATE
     */
    #[\Since('8.3')]
    public const int CM_DEFLATE = UNKNOWN;
    /**
     * @cvalue ZIP_CM_DEFLATE64
     */
    #[\Since('8.3')]
    public const int CM_DEFLATE64 = UNKNOWN;
    /**
     * @cvalue ZIP_CM_PKWARE_IMPLODE
     */
    #[\Since('8.3')]
    public const int CM_PKWARE_IMPLODE = UNKNOWN;
    /**
     * @cvalue ZIP_CM_BZIP2
     */
    #[\Since('8.3')]
    public const int CM_BZIP2 = UNKNOWN;
    /**
     * @cvalue ZIP_CM_LZMA
     */
    #[\Since('8.3')]
    public const int CM_LZMA = UNKNOWN;
    #ifdef ZIP_CM_LZMA2
    /**
     * @cvalue ZIP_CM_LZMA2
     */
    #[\Since('8.3')]
    public const int CM_LZMA2 = UNKNOWN;
    #endif
    #ifdef ZIP_CM_ZSTD
    /**
     * @cvalue ZIP_CM_ZSTD
     */
    #[\Since('8.3')]
    public const int CM_ZSTD = UNKNOWN;
    #endif
    #ifdef ZIP_CM_XZ
    /**
     * @cvalue ZIP_CM_XZ
     */
    #[\Since('8.3')]
    public const int CM_XZ = UNKNOWN;
    #endif
    /**
     * @cvalue ZIP_CM_TERSE
     */
    #[\Since('8.3')]
    public const int CM_TERSE = UNKNOWN;
    /**
     * @cvalue ZIP_CM_LZ77
     */
    #[\Since('8.3')]
    public const int CM_LZ77 = UNKNOWN;
    /**
     * @cvalue ZIP_CM_WAVPACK
     */
    #[\Since('8.3')]
    public const int CM_WAVPACK = UNKNOWN;
    /**
     * @cvalue ZIP_CM_PPMD
     */
    #[\Since('8.3')]
    public const int CM_PPMD = UNKNOWN;
    /* Error code */
    /**
     * N No error
     * @cvalue ZIP_ER_OK
     */
    #[\Since('8.3')]
    public const int ER_OK = UNKNOWN;
    /**
     * N Multi-disk zip archives not supported
     * @cvalue ZIP_ER_MULTIDISK
     */
    #[\Since('8.3')]
    public const int ER_MULTIDISK = UNKNOWN;
    /**
     * S Renaming temporary file failed
     * @cvalue ZIP_ER_RENAME
     */
    #[\Since('8.3')]
    public const int ER_RENAME = UNKNOWN;
    /**
     * S Closing zip archive failed
     * @cvalue ZIP_ER_CLOSE
     */
    #[\Since('8.3')]
    public const int ER_CLOSE = UNKNOWN;
    /**
     * S Seek error
     * @cvalue ZIP_ER_SEEK
     */
    #[\Since('8.3')]
    public const int ER_SEEK = UNKNOWN;
    /**
     * S Read error
     * @cvalue ZIP_ER_READ
     */
    #[\Since('8.3')]
    public const int ER_READ = UNKNOWN;
    /**
     * S Write error
     * @cvalue ZIP_ER_WRITE
     */
    #[\Since('8.3')]
    public const int ER_WRITE = UNKNOWN;
    /**
     * N CRC error
     * @cvalue ZIP_ER_CRC
     */
    #[\Since('8.3')]
    public const int ER_CRC = UNKNOWN;
    /**
     * N Containing zip archive was closed
     * @cvalue ZIP_ER_ZIPCLOSED
     */
    #[\Since('8.3')]
    public const int ER_ZIPCLOSED = UNKNOWN;
    /**
     * N No such file
     * @cvalue ZIP_ER_NOENT
     */
    #[\Since('8.3')]
    public const int ER_NOENT = UNKNOWN;
    /**
     * N File already exists
     * @cvalue ZIP_ER_EXISTS
     */
    #[\Since('8.3')]
    public const int ER_EXISTS = UNKNOWN;
    /**
     * S Can't open file
     * @cvalue ZIP_ER_OPEN
     */
    #[\Since('8.3')]
    public const int ER_OPEN = UNKNOWN;
    /**
     * S Failure to create temporary file
     * @cvalue ZIP_ER_TMPOPEN
     */
    #[\Since('8.3')]
    public const int ER_TMPOPEN = UNKNOWN;
    /**
     * Z Zlib error
     * @cvalue ZIP_ER_ZLIB
     */
    #[\Since('8.3')]
    public const int ER_ZLIB = UNKNOWN;
    /**
     * N Malloc failure
     * @cvalue ZIP_ER_MEMORY
     */
    #[\Since('8.3')]
    public const int ER_MEMORY = UNKNOWN;
    /**
     * N Entry has been changed
     * @cvalue ZIP_ER_CHANGED
     */
    #[\Since('8.3')]
    public const int ER_CHANGED = UNKNOWN;
    /**
     * N Compression method not supported
     * @cvalue ZIP_ER_COMPNOTSUPP
     */
    #[\Since('8.3')]
    public const int ER_COMPNOTSUPP = UNKNOWN;
    /**
     * N Premature EOF
     * @cvalue ZIP_ER_EOF
     */
    #[\Since('8.3')]
    public const int ER_EOF = UNKNOWN;
    /**
     * N Invalid argument
     * @cvalue ZIP_ER_INVAL
     */
    #[\Since('8.3')]
    public const int ER_INVAL = UNKNOWN;
    /**
     * N Not a zip archive
     * @cvalue ZIP_ER_NOZIP
     */
    #[\Since('8.3')]
    public const int ER_NOZIP = UNKNOWN;
    /**
     * N Internal error
     * @cvalue ZIP_ER_INTERNAL
     */
    #[\Since('8.3')]
    public const int ER_INTERNAL = UNKNOWN;
    /**
     * N Zip archive inconsistent
     * @cvalue ZIP_ER_INCONS
     */
    #[\Since('8.3')]
    public const int ER_INCONS = UNKNOWN;
    /**
     * S Can't remove file
     * @cvalue ZIP_ER_REMOVE
     */
    #[\Since('8.3')]
    public const int ER_REMOVE = UNKNOWN;
    /**
     * N Entry has been deleted
     * @cvalue ZIP_ER_DELETED
     */
    #[\Since('8.3')]
    public const int ER_DELETED = UNKNOWN;
    /**
     * N Encryption method not supported
     * @cvalue ZIP_ER_ENCRNOTSUPP
     */
    #[\Since('8.3')]
    public const int ER_ENCRNOTSUPP = UNKNOWN;
    /**
     * N Read-only archive
     * @cvalue ZIP_ER_RDONLY
     */
    #[\Since('8.3')]
    public const int ER_RDONLY = UNKNOWN;
    /**
     * N Entry has been deleted
     * @cvalue ZIP_ER_NOPASSWD
     */
    #[\Since('8.3')]
    public const int ER_NOPASSWD = UNKNOWN;
    /**
     * N Wrong password provided
     * @cvalue ZIP_ER_WRONGPASSWD
     */
    #[\Since('8.3')]
    public const int ER_WRONGPASSWD = UNKNOWN;
    /* since 1.0.0 */
    #ifdef ZIP_ER_OPNOTSUPP
    /**
     * N Operation not supported
     * @cvalue ZIP_ER_OPNOTSUPP
     */
    #[\Since('8.3')]
    public const int ER_OPNOTSUPP = UNKNOWN;
    #endif
    #ifdef ZIP_ER_INUSE
    /**
     * N Resource still in use
     * @cvalue ZIP_ER_INUSE
     */
    #[\Since('8.3')]
    public const int ER_INUSE = UNKNOWN;
    #endif
    #ifdef ZIP_ER_TELL
    /**
     * S Tell error
     * @cvalue ZIP_ER_TELL
     */
    #[\Since('8.3')]
    public const int ER_TELL = UNKNOWN;
    #endif
    /* since 1.6.0 */
    #ifdef ZIP_ER_COMPRESSED_DATA
    /**
     * N Compressed data invalid
     * @cvalue ZIP_ER_COMPRESSED_DATA
     */
    #[\Since('8.3')]
    public const int ER_COMPRESSED_DATA = UNKNOWN;
    #endif
    #ifdef ZIP_ER_CANCELLED
    /**
     * N Operation cancelled
     * @cvalue ZIP_ER_CANCELLED
     */
    #[\Since('8.3')]
    public const int ER_CANCELLED = UNKNOWN;
    #endif
    /* since 1.10.0 */
    #ifdef ZIP_ER_DATA_LENGTH
    /**
     * N Unexpected length of data
     * @cvalue ZIP_ER_DATA_LENGTH
     */
    #[\Since('8.3')]
    public const int ER_DATA_LENGTH = UNKNOWN;
    #endif
    #ifdef ZIP_ER_NOT_ALLOWED
    /**
     * Not allowed in torrentzip
     * @cvalue ZIP_ER_NOT_ALLOWED
     */
    #[\Since('8.3')]
    public const int ER_NOT_ALLOWED = UNKNOWN;
    #endif
    #ifdef ZIP_ER_TRUNCATED_ZIP
    /**
     * Possibly truncated or corrupted zip archive
     * @cvalue ZIP_ER_TRUNCATED_ZIP
     */
    #[\Since('8.4')]
    public const int ER_TRUNCATED_ZIP = UNKNOWN;
    #endif
    #ifdef ZIP_AFL_RDONLY
    /**
     * read only -- cannot be cleared
     * @cvalue ZIP_AFL_RDONLY
     */
    #[\Since('8.3')]
    public const int AFL_RDONLY = UNKNOWN;
    #endif
    #ifdef ZIP_AFL_IS_TORRENTZIP
    /**
     * current archive is torrentzipped
     * @cvalue ZIP_AFL_IS_TORRENTZIP
     */
    #[\Since('8.3')]
    public const int AFL_IS_TORRENTZIP = UNKNOWN;
    #endif
    #ifdef ZIP_AFL_WANT_TORRENTZIP
    /**
     * write archive in torrentzip format
     * @cvalue ZIP_AFL_WANT_TORRENTZIP
     */
    #[\Since('8.3')]
    public const int AFL_WANT_TORRENTZIP = UNKNOWN;
    #endif
    #ifdef ZIP_AFL_CREATE_OR_KEEP_FILE_FOR_EMPTY_ARCHIVE
    /**
     * don't remove file if archive is empty
     * @cvalue ZIP_AFL_CREATE_OR_KEEP_FILE_FOR_EMPTY_ARCHIVE
     */
    #[\Since('8.3')]
    public const int AFL_CREATE_OR_KEEP_FILE_FOR_EMPTY_ARCHIVE = UNKNOWN;
    #endif
    #ifdef ZIP_OPSYS_DEFAULT
    /**
     * @cvalue ZIP_OPSYS_DOS
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_DOS = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_AMIGA
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_AMIGA = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_OPENVMS
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_OPENVMS = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_UNIX
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_UNIX = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_VM_CMS
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_VM_CMS = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_ATARI_ST
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_ATARI_ST = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_OS_2
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_OS_2 = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_MACINTOSH
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_MACINTOSH = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_Z_SYSTEM
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_Z_SYSTEM = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_CPM
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_CPM = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_WINDOWS_NTFS
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_WINDOWS_NTFS = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_MVS
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_MVS = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_VSE
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_VSE = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_ACORN_RISC
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_ACORN_RISC = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_VFAT
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_VFAT = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_ALTERNATE_MVS
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_ALTERNATE_MVS = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_BEOS
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_BEOS = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_TANDEM
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_TANDEM = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_OS_400
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_OS_400 = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_OS_X
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_OS_X = UNKNOWN;
    /**
     * @cvalue ZIP_OPSYS_DEFAULT
     * @link ziparchive.constants.opsys
     */
    #[\Since('8.3')]
    public const int OPSYS_DEFAULT = UNKNOWN;
    #endif
    /**
     * @cvalue ZIP_EM_NONE
     */
    #[\Since('8.3')]
    public const int EM_NONE = UNKNOWN;
    /**
     * @cvalue ZIP_EM_TRAD_PKWARE
     */
    #[\Since('8.3')]
    public const int EM_TRAD_PKWARE = UNKNOWN;
    #ifdef HAVE_ENCRYPTION
    /**
     * @cvalue ZIP_EM_AES_128
     */
    #[\Since('8.3')]
    public const int EM_AES_128 = UNKNOWN;
    /**
     * @cvalue ZIP_EM_AES_192
     */
    #[\Since('8.3')]
    public const int EM_AES_192 = UNKNOWN;
    /**
     * @cvalue ZIP_EM_AES_256
     */
    #[\Since('8.3')]
    public const int EM_AES_256 = UNKNOWN;
    #endif
    /**
     * @cvalue ZIP_EM_UNKNOWN
     */
    #[\Since('8.3')]
    public const int EM_UNKNOWN = UNKNOWN;
    /**
     * @cvalue LIBZIP_VERSION_STR
     */
    #[\Since('8.3')]
    public const string LIBZIP_VERSION = UNKNOWN;
    /**
     * @cvalue ZIP_LENGTH_TO_END
     */
    #[\Since('8.3')]
    public const int LENGTH_TO_END = UNKNOWN;
    /* since 1.10.1 */
    #ifdef ZIP_LENGTH_UNCHECKED
    /**
     * @cvalue ZIP_LENGTH_UNCHECKED
     */
    #[\Since('8.3')]
    public const int LENGTH_UNCHECKED = UNKNOWN;
}