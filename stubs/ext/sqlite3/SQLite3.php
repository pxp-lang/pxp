<?php 

/** @generate-function-entries */
class SQLite3
{
    /** @implementation-alias SQLite3::open */
    public function __construct(string $filename, int $flags = SQLITE3_OPEN_READWRITE | SQLITE3_OPEN_CREATE, string $encryptionKey = "")
    {
    }
    /**
     * @tentative-return-type
     * @todo SQLite3::open should really be static
     * @return void
     */
    public function open(string $filename, int $flags = SQLITE3_OPEN_READWRITE | SQLITE3_OPEN_CREATE, string $encryptionKey = "")
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
     * @return array
     */
    public static function version()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function lastInsertRowID()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function lastErrorCode()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function lastExtendedErrorCode()
    {
    }
    /**
     * @tentative-return-type
     * @return string
     */
    public function lastErrorMsg()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function changes()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function busyTimeout(int $milliseconds)
    {
    }
    #ifndef SQLITE_OMIT_LOAD_EXTENSION
    /**
     * @tentative-return-type
     * @return bool
     */
    public function loadExtension(string $name)
    {
    }
    #endif
    #if SQLITE_VERSION_NUMBER >= 3006011
    /**
     * @tentative-return-type
     * @return bool
     */
    public function backup(SQLite3 $destination, string $sourceDatabase = "main", string $destinationDatabase = "main")
    {
    }
    #endif
    /**
     * @tentative-return-type
     * @return string
     */
    public static function escapeString(string $string)
    {
    }
    /**
     * @tentative-return-type
     * @return (SQLite3Stmt | false)
     */
    public function prepare(string $query)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function exec(string $query)
    {
    }
    /**
     * @tentative-return-type
     * @return (SQLite3Result | false)
     */
    public function query(string $query)
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function querySingle(string $query, bool $entireRow = false)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function createFunction(string $name, callable $callback, int $argCount = -1, int $flags = 0)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function createAggregate(string $name, callable $stepCallback, callable $finalCallback, int $argCount = -1)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function createCollation(string $name, callable $callback)
    {
    }
    /** @return resource|false */
    public function openBlob(string $table, string $column, int $rowid, string $database = "main", int $flags = SQLITE3_OPEN_READONLY)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function enableExceptions(bool $enable = false)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function enableExtendedResultCodes(bool $enable = true)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setAuthorizer(?callable $callback)
    {
    }
    /**
     * @var int
     * @cvalue SQLITE_OK
     * @link sqlite3.class.constants.ok
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const OK = UNKNOWN;
    /** @cvalue SQLITE_OK */
    #[\Since('8.4')]
    public const int OK = UNKNOWN;
    /* Constants for authorizer return */
    /**
     * @var int
     * @cvalue SQLITE_DENY
     * @link sqlite3.class.constants.deny
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DENY = UNKNOWN;
    /* Constants for authorizer return */
    /** @cvalue SQLITE_DENY */
    #[\Since('8.4')]
    public const int DENY = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_IGNORE
     * @link sqlite3.class.constants.ignore
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const IGNORE = UNKNOWN;
    /** @cvalue SQLITE_IGNORE */
    #[\Since('8.4')]
    public const int IGNORE = UNKNOWN;
    /* Constants for authorizer actions */
    /**
     * @var int
     * @cvalue SQLITE_CREATE_INDEX
     * @link sqlite3.class.constants.create-index
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CREATE_INDEX = UNKNOWN;
    /* Constants for authorizer actions */
    /** @cvalue SQLITE_CREATE_INDEX */
    #[\Since('8.4')]
    public const int CREATE_INDEX = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_CREATE_TABLE
     * @link sqlite3.class.constants.create-table
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CREATE_TABLE = UNKNOWN;
    /** @cvalue SQLITE_CREATE_TABLE */
    #[\Since('8.4')]
    public const int CREATE_TABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_CREATE_TEMP_INDEX
     * @link sqlite3.class.constants.create-temp-index
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CREATE_TEMP_INDEX = UNKNOWN;
    /** @cvalue SQLITE_CREATE_TEMP_INDEX */
    #[\Since('8.4')]
    public const int CREATE_TEMP_INDEX = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_CREATE_TEMP_TABLE
     * @link sqlite3.class.constants.create-temp-table
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CREATE_TEMP_TABLE = UNKNOWN;
    /** @cvalue SQLITE_CREATE_TEMP_TABLE */
    #[\Since('8.4')]
    public const int CREATE_TEMP_TABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_CREATE_TEMP_TRIGGER
     * @link sqlite3.class.constants.create-temp-trigger
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CREATE_TEMP_TRIGGER = UNKNOWN;
    /** @cvalue SQLITE_CREATE_TEMP_TRIGGER */
    #[\Since('8.4')]
    public const int CREATE_TEMP_TRIGGER = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_CREATE_TEMP_VIEW
     * @link sqlite3.class.constants.create-temp-view
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CREATE_TEMP_VIEW = UNKNOWN;
    /** @cvalue SQLITE_CREATE_TEMP_VIEW */
    #[\Since('8.4')]
    public const int CREATE_TEMP_VIEW = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_CREATE_TRIGGER
     * @link sqlite3.class.constants.create-trigger
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CREATE_TRIGGER = UNKNOWN;
    /** @cvalue SQLITE_CREATE_TRIGGER */
    #[\Since('8.4')]
    public const int CREATE_TRIGGER = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_CREATE_VIEW
     * @link sqlite3.class.constants.create-view
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CREATE_VIEW = UNKNOWN;
    /** @cvalue SQLITE_CREATE_VIEW */
    #[\Since('8.4')]
    public const int CREATE_VIEW = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DELETE
     * @link sqlite3.class.constants.delete
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DELETE = UNKNOWN;
    /** @cvalue SQLITE_DELETE */
    #[\Since('8.4')]
    public const int DELETE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_INDEX
     * @link sqlite3.class.constants.drop-index
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DROP_INDEX = UNKNOWN;
    /** @cvalue SQLITE_DROP_INDEX */
    #[\Since('8.4')]
    public const int DROP_INDEX = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_TABLE
     * @link sqlite3.class.constants.drop-table
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DROP_TABLE = UNKNOWN;
    /** @cvalue SQLITE_DROP_TABLE */
    #[\Since('8.4')]
    public const int DROP_TABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_TEMP_INDEX
     * @link sqlite3.class.constants.drop-temp-index
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DROP_TEMP_INDEX = UNKNOWN;
    /** @cvalue SQLITE_DROP_TEMP_INDEX */
    #[\Since('8.4')]
    public const int DROP_TEMP_INDEX = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_TEMP_TABLE
     * @link sqlite3.class.constants.drop-temp-table
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DROP_TEMP_TABLE = UNKNOWN;
    /** @cvalue SQLITE_DROP_TEMP_TABLE */
    #[\Since('8.4')]
    public const int DROP_TEMP_TABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_TEMP_TRIGGER
     * @link sqlite3.class.constants.drop-temp-trigger
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DROP_TEMP_TRIGGER = UNKNOWN;
    /** @cvalue SQLITE_DROP_TEMP_TRIGGER */
    #[\Since('8.4')]
    public const int DROP_TEMP_TRIGGER = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_TEMP_VIEW
     * @link sqlite3.class.constants.drop-temp-view
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DROP_TEMP_VIEW = UNKNOWN;
    /** @cvalue SQLITE_DROP_TEMP_VIEW */
    #[\Since('8.4')]
    public const int DROP_TEMP_VIEW = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_TRIGGER
     * @link sqlite3.class.constants.drop-trigger
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DROP_TRIGGER = UNKNOWN;
    /** @cvalue SQLITE_DROP_TRIGGER */
    #[\Since('8.4')]
    public const int DROP_TRIGGER = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_VIEW
     * @link sqlite3.class.constants.drop-view
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DROP_VIEW = UNKNOWN;
    /** @cvalue SQLITE_DROP_VIEW */
    #[\Since('8.4')]
    public const int DROP_VIEW = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_INSERT
     * @link sqlite3.class.constants.insert
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const INSERT = UNKNOWN;
    /** @cvalue SQLITE_INSERT */
    #[\Since('8.4')]
    public const int INSERT = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_PRAGMA
     * @link sqlite3.class.constants.pragma
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PRAGMA = UNKNOWN;
    /** @cvalue SQLITE_PRAGMA */
    #[\Since('8.4')]
    public const int PRAGMA = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_READ
     * @link sqlite3.class.constants.read
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const READ = UNKNOWN;
    /** @cvalue SQLITE_READ */
    #[\Since('8.4')]
    public const int READ = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_SELECT
     * @link sqlite3.class.constants.select
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SELECT = UNKNOWN;
    /** @cvalue SQLITE_SELECT */
    #[\Since('8.4')]
    public const int SELECT = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_TRANSACTION
     * @link sqlite3.class.constants.transaction
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const TRANSACTION = UNKNOWN;
    /** @cvalue SQLITE_TRANSACTION */
    #[\Since('8.4')]
    public const int TRANSACTION = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_UPDATE
     * @link sqlite3.class.constants.update
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const UPDATE = UNKNOWN;
    /** @cvalue SQLITE_UPDATE */
    #[\Since('8.4')]
    public const int UPDATE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_ATTACH
     * @link sqlite3.class.constants.attach
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTACH = UNKNOWN;
    /** @cvalue SQLITE_ATTACH */
    #[\Since('8.4')]
    public const int ATTACH = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DETACH
     * @link sqlite3.class.constants.detach
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DETACH = UNKNOWN;
    /** @cvalue SQLITE_DETACH */
    #[\Since('8.4')]
    public const int DETACH = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_ALTER_TABLE
     * @link sqlite3.class.constants.alter-table
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ALTER_TABLE = UNKNOWN;
    /** @cvalue SQLITE_ALTER_TABLE */
    #[\Since('8.4')]
    public const int ALTER_TABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_REINDEX
     * @link sqlite3.class.constants.reindex
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const REINDEX = UNKNOWN;
    /** @cvalue SQLITE_REINDEX */
    #[\Since('8.4')]
    public const int REINDEX = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_ANALYZE
     * @link sqlite3.class.constants.analyze
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ANALYZE = UNKNOWN;
    /** @cvalue SQLITE_ANALYZE */
    #[\Since('8.4')]
    public const int ANALYZE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_CREATE_VTABLE
     * @link sqlite3.class.constants.create-vtable
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CREATE_VTABLE = UNKNOWN;
    /** @cvalue SQLITE_CREATE_VTABLE */
    #[\Since('8.4')]
    public const int CREATE_VTABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_VTABLE
     * @link sqlite3.class.constants.drop-vtable
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DROP_VTABLE = UNKNOWN;
    /** @cvalue SQLITE_DROP_VTABLE */
    #[\Since('8.4')]
    public const int DROP_VTABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_FUNCTION
     * @link sqlite3.class.constants.function
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FUNCTION = UNKNOWN;
    /** @cvalue SQLITE_FUNCTION */
    #[\Since('8.4')]
    public const int FUNCTION = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_SAVEPOINT
     * @link sqlite3.class.constants.savepoint
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SAVEPOINT = UNKNOWN;
    /** @cvalue SQLITE_SAVEPOINT */
    #[\Since('8.4')]
    public const int SAVEPOINT = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_COPY
     * @link sqlite3.class.constants.copy
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const COPY = UNKNOWN;
    /** @cvalue SQLITE_COPY */
    #[\Since('8.4')]
    public const int COPY = UNKNOWN;
    #ifdef SQLITE_RECURSIVE
    /**
     * @var int
     * @cvalue SQLITE_RECURSIVE
     * @link sqlite3.class.constants.recursive
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const RECURSIVE = UNKNOWN;
    #ifdef SQLITE_RECURSIVE
    /** @cvalue SQLITE_RECURSIVE */
    #[\Since('8.4')]
    public const int RECURSIVE = UNKNOWN;
}