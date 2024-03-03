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
     * @return void
     */
    public function open(string $filename, int $flags = SQLITE3_OPEN_READWRITE | SQLITE3_OPEN_CREATE, string $encryptionKey = "")
    {
    }
    /** @return bool */
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
    public const OK = UNKNOWN;
    /* Constants for authorizer return */
    /**
     * @var int
     * @cvalue SQLITE_DENY
     * @link sqlite3.class.constants.deny
     */
    public const DENY = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_IGNORE
     * @link sqlite3.class.constants.ignore
     */
    public const IGNORE = UNKNOWN;
    /* Constants for authorizer actions */
    /**
     * @var int
     * @cvalue SQLITE_CREATE_INDEX
     * @link sqlite3.class.constants.create-index
     */
    public const CREATE_INDEX = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_CREATE_TABLE
     * @link sqlite3.class.constants.create-table
     */
    public const CREATE_TABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_CREATE_TEMP_INDEX
     * @link sqlite3.class.constants.create-temp-index
     */
    public const CREATE_TEMP_INDEX = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_CREATE_TEMP_TABLE
     * @link sqlite3.class.constants.create-temp-table
     */
    public const CREATE_TEMP_TABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_CREATE_TEMP_TRIGGER
     * @link sqlite3.class.constants.create-temp-trigger
     */
    public const CREATE_TEMP_TRIGGER = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_CREATE_TEMP_VIEW
     * @link sqlite3.class.constants.create-temp-view
     */
    public const CREATE_TEMP_VIEW = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_CREATE_TRIGGER
     * @link sqlite3.class.constants.create-trigger
     */
    public const CREATE_TRIGGER = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_CREATE_VIEW
     * @link sqlite3.class.constants.create-view
     */
    public const CREATE_VIEW = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DELETE
     * @link sqlite3.class.constants.delete
     */
    public const DELETE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_INDEX
     * @link sqlite3.class.constants.drop-index
     */
    public const DROP_INDEX = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_TABLE
     * @link sqlite3.class.constants.drop-table
     */
    public const DROP_TABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_TEMP_INDEX
     * @link sqlite3.class.constants.drop-temp-index
     */
    public const DROP_TEMP_INDEX = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_TEMP_TABLE
     * @link sqlite3.class.constants.drop-temp-table
     */
    public const DROP_TEMP_TABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_TEMP_TRIGGER
     * @link sqlite3.class.constants.drop-temp-trigger
     */
    public const DROP_TEMP_TRIGGER = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_TEMP_VIEW
     * @link sqlite3.class.constants.drop-temp-view
     */
    public const DROP_TEMP_VIEW = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_TRIGGER
     * @link sqlite3.class.constants.drop-trigger
     */
    public const DROP_TRIGGER = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_VIEW
     * @link sqlite3.class.constants.drop-view
     */
    public const DROP_VIEW = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_INSERT
     * @link sqlite3.class.constants.insert
     */
    public const INSERT = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_PRAGMA
     * @link sqlite3.class.constants.pragma
     */
    public const PRAGMA = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_READ
     * @link sqlite3.class.constants.read
     */
    public const READ = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_SELECT
     * @link sqlite3.class.constants.select
     */
    public const SELECT = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_TRANSACTION
     * @link sqlite3.class.constants.transaction
     */
    public const TRANSACTION = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_UPDATE
     * @link sqlite3.class.constants.update
     */
    public const UPDATE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_ATTACH
     * @link sqlite3.class.constants.attach
     */
    public const ATTACH = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DETACH
     * @link sqlite3.class.constants.detach
     */
    public const DETACH = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_ALTER_TABLE
     * @link sqlite3.class.constants.alter-table
     */
    public const ALTER_TABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_REINDEX
     * @link sqlite3.class.constants.reindex
     */
    public const REINDEX = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_ANALYZE
     * @link sqlite3.class.constants.analyze
     */
    public const ANALYZE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_CREATE_VTABLE
     * @link sqlite3.class.constants.create-vtable
     */
    public const CREATE_VTABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_DROP_VTABLE
     * @link sqlite3.class.constants.drop-vtable
     */
    public const DROP_VTABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_FUNCTION
     * @link sqlite3.class.constants.function
     */
    public const FUNCTION = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_SAVEPOINT
     * @link sqlite3.class.constants.savepoint
     */
    public const SAVEPOINT = UNKNOWN;
    /**
     * @var int
     * @cvalue SQLITE_COPY
     * @link sqlite3.class.constants.copy
     */
    public const COPY = UNKNOWN;
    #ifdef SQLITE_RECURSIVE
    /**
     * @var int
     * @cvalue SQLITE_RECURSIVE
     * @link sqlite3.class.constants.recursive
     */
    public const RECURSIVE = UNKNOWN;
}