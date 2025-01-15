<?php 

/** @generate-function-entries */
class PDO
{
    #[\Until('8.1')]
    public function setAttribute(int $attribute, mixed $value)
    {
    }
    public function __construct(string $dsn, ?string $username = null, ?string $password = null, ?array $options = null)
    {
    }
    #[\Since('8.4')]
    public static function connect(string $dsn, ?string $username = null, #[\SensitiveParameter] ?string $password = null, ?array $options = null): static
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function beginTransaction()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function commit()
    {
    }
    /**
     * @tentative-return-type
     * @return (string | null)
     */
    public function errorCode()
    {
    }
    /**
     * @tentative-return-type
     * @return array
     */
    public function errorInfo()
    {
    }
    /**
     * @tentative-return-type
     * @return (int | false)
     */
    public function exec(string $statement)
    {
    }
    /**
     * @tentative-return-type
     * @return (bool | int | string | array | null)
     */
    public function getAttribute(int $attribute)
    {
    }
    /**
     * @tentative-return-type
     * @return array
     */
    public static function getAvailableDrivers()
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function inTransaction()
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public function lastInsertId(?string $name = null)
    {
    }
    /**
     * @tentative-return-type
     * @return (PDOStatement | false)
     */
    public function prepare(string $query, array $options = [])
    {
    }
    /**
     * @tentative-return-type
     * @return (PDOStatement | false)
     */
    public function query(string $query, ?int $fetchMode = null, mixed ...$fetchModeArgs)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public function quote(string $string, int $type = PDO::PARAM_STR)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function rollBack()
    {
    }
    #[\Since('8.1')]
    public function setAttribute(int $attribute, mixed $value): bool
    {
    }
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_PARAM_NULL)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARAM_NULL = 0;
    /** @cvalue LONG_CONST(PDO_PARAM_NULL) */
    #[\Since('8.4')]
    public const int PARAM_NULL = 0;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_PARAM_BOOL)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARAM_BOOL = 5;
    /** @cvalue LONG_CONST(PDO_PARAM_BOOL) */
    #[\Since('8.4')]
    public const int PARAM_BOOL = 5;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_PARAM_INT)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARAM_INT = 1;
    /** @cvalue LONG_CONST(PDO_PARAM_INT) */
    #[\Since('8.4')]
    public const int PARAM_INT = 1;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_PARAM_STR)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARAM_STR = 2;
    /** @cvalue LONG_CONST(PDO_PARAM_STR) */
    #[\Since('8.4')]
    public const int PARAM_STR = 2;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_PARAM_LOB)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARAM_LOB = 3;
    /** @cvalue LONG_CONST(PDO_PARAM_LOB) */
    #[\Since('8.4')]
    public const int PARAM_LOB = 3;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_PARAM_STMT)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARAM_STMT = 4;
    /** @cvalue LONG_CONST(PDO_PARAM_STMT) */
    #[\Since('8.4')]
    public const int PARAM_STMT = 4;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_PARAM_INPUT_OUTPUT)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARAM_INPUT_OUTPUT = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_PARAM_INPUT_OUTPUT) */
    #[\Since('8.4')]
    public const int PARAM_INPUT_OUTPUT = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_PARAM_STR_NATL)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARAM_STR_NATL = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_PARAM_STR_NATL) */
    #[\Since('8.4')]
    public const int PARAM_STR_NATL = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_PARAM_STR_CHAR)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARAM_STR_CHAR = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_PARAM_STR_CHAR) */
    #[\Since('8.4')]
    public const int PARAM_STR_CHAR = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_PARAM_EVT_ALLOC)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARAM_EVT_ALLOC = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_PARAM_EVT_ALLOC) */
    #[\Since('8.4')]
    public const int PARAM_EVT_ALLOC = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_PARAM_EVT_FREE)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARAM_EVT_FREE = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_PARAM_EVT_FREE) */
    #[\Since('8.4')]
    public const int PARAM_EVT_FREE = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_PARAM_EVT_EXEC_PRE)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARAM_EVT_EXEC_PRE = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_PARAM_EVT_EXEC_PRE) */
    #[\Since('8.4')]
    public const int PARAM_EVT_EXEC_PRE = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_PARAM_EVT_EXEC_POST)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARAM_EVT_EXEC_POST = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_PARAM_EVT_EXEC_POST) */
    #[\Since('8.4')]
    public const int PARAM_EVT_EXEC_POST = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_PARAM_EVT_FETCH_PRE)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARAM_EVT_FETCH_PRE = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_PARAM_EVT_FETCH_PRE) */
    #[\Since('8.4')]
    public const int PARAM_EVT_FETCH_PRE = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_PARAM_EVT_FETCH_POST)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARAM_EVT_FETCH_POST = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_PARAM_EVT_FETCH_POST) */
    #[\Since('8.4')]
    public const int PARAM_EVT_FETCH_POST = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_PARAM_EVT_NORMALIZE)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const PARAM_EVT_NORMALIZE = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_PARAM_EVT_NORMALIZE) */
    #[\Since('8.4')]
    public const int PARAM_EVT_NORMALIZE = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_USE_DEFAULT)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_DEFAULT = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_USE_DEFAULT) */
    #[\Since('8.4')]
    public const int FETCH_DEFAULT = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_LAZY)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_LAZY = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_LAZY) */
    #[\Since('8.4')]
    public const int FETCH_LAZY = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_ASSOC)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_ASSOC = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_ASSOC) */
    #[\Since('8.4')]
    public const int FETCH_ASSOC = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_NUM)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_NUM = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_NUM) */
    #[\Since('8.4')]
    public const int FETCH_NUM = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_BOTH)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_BOTH = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_BOTH) */
    #[\Since('8.4')]
    public const int FETCH_BOTH = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_OBJ)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_OBJ = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_OBJ) */
    #[\Since('8.4')]
    public const int FETCH_OBJ = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_BOUND)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_BOUND = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_BOUND) */
    #[\Since('8.4')]
    public const int FETCH_BOUND = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_COLUMN)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_COLUMN = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_COLUMN) */
    #[\Since('8.4')]
    public const int FETCH_COLUMN = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_CLASS)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_CLASS = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_CLASS) */
    #[\Since('8.4')]
    public const int FETCH_CLASS = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_INTO)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_INTO = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_INTO) */
    #[\Since('8.4')]
    public const int FETCH_INTO = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_FUNC)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_FUNC = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_FUNC) */
    #[\Since('8.4')]
    public const int FETCH_FUNC = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_GROUP)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_GROUP = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_GROUP) */
    #[\Since('8.4')]
    public const int FETCH_GROUP = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_UNIQUE)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_UNIQUE = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_UNIQUE) */
    #[\Since('8.4')]
    public const int FETCH_UNIQUE = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_KEY_PAIR)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_KEY_PAIR = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_KEY_PAIR) */
    #[\Since('8.4')]
    public const int FETCH_KEY_PAIR = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_CLASSTYPE)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_CLASSTYPE = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_CLASSTYPE) */
    #[\Since('8.4')]
    public const int FETCH_CLASSTYPE = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_SERIALIZE)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_SERIALIZE = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_SERIALIZE) */
    #[\Since('8.4')]
    public const int FETCH_SERIALIZE = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_PROPS_LATE)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_PROPS_LATE = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_PROPS_LATE) */
    #[\Since('8.4')]
    public const int FETCH_PROPS_LATE = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_NAMED)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_NAMED = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_NAMED) */
    #[\Since('8.4')]
    public const int FETCH_NAMED = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_AUTOCOMMIT)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_AUTOCOMMIT = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_AUTOCOMMIT) */
    #[\Since('8.4')]
    public const int ATTR_AUTOCOMMIT = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_PREFETCH)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_PREFETCH = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_PREFETCH) */
    #[\Since('8.4')]
    public const int ATTR_PREFETCH = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_TIMEOUT)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_TIMEOUT = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_TIMEOUT) */
    #[\Since('8.4')]
    public const int ATTR_TIMEOUT = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_ERRMODE)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_ERRMODE = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_ERRMODE) */
    #[\Since('8.4')]
    public const int ATTR_ERRMODE = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_SERVER_VERSION)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_SERVER_VERSION = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_SERVER_VERSION) */
    #[\Since('8.4')]
    public const int ATTR_SERVER_VERSION = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_CLIENT_VERSION)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_CLIENT_VERSION = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_CLIENT_VERSION) */
    #[\Since('8.4')]
    public const int ATTR_CLIENT_VERSION = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_SERVER_INFO)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_SERVER_INFO = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_SERVER_INFO) */
    #[\Since('8.4')]
    public const int ATTR_SERVER_INFO = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_CONNECTION_STATUS)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_CONNECTION_STATUS = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_CONNECTION_STATUS) */
    #[\Since('8.4')]
    public const int ATTR_CONNECTION_STATUS = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_CASE)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_CASE = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_CASE) */
    #[\Since('8.4')]
    public const int ATTR_CASE = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_CURSOR_NAME)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_CURSOR_NAME = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_CURSOR_NAME) */
    #[\Since('8.4')]
    public const int ATTR_CURSOR_NAME = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_CURSOR)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_CURSOR = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_CURSOR) */
    #[\Since('8.4')]
    public const int ATTR_CURSOR = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_ORACLE_NULLS)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_ORACLE_NULLS = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_ORACLE_NULLS) */
    #[\Since('8.4')]
    public const int ATTR_ORACLE_NULLS = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_PERSISTENT)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_PERSISTENT = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_PERSISTENT) */
    #[\Since('8.4')]
    public const int ATTR_PERSISTENT = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_STATEMENT_CLASS)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_STATEMENT_CLASS = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_STATEMENT_CLASS) */
    #[\Since('8.4')]
    public const int ATTR_STATEMENT_CLASS = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_FETCH_TABLE_NAMES)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_FETCH_TABLE_NAMES = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_FETCH_TABLE_NAMES) */
    #[\Since('8.4')]
    public const int ATTR_FETCH_TABLE_NAMES = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_FETCH_CATALOG_NAMES)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_FETCH_CATALOG_NAMES = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_FETCH_CATALOG_NAMES) */
    #[\Since('8.4')]
    public const int ATTR_FETCH_CATALOG_NAMES = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_DRIVER_NAME)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_DRIVER_NAME = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_DRIVER_NAME) */
    #[\Since('8.4')]
    public const int ATTR_DRIVER_NAME = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_STRINGIFY_FETCHES)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_STRINGIFY_FETCHES = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_STRINGIFY_FETCHES) */
    #[\Since('8.4')]
    public const int ATTR_STRINGIFY_FETCHES = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_MAX_COLUMN_LEN)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_MAX_COLUMN_LEN = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_MAX_COLUMN_LEN) */
    #[\Since('8.4')]
    public const int ATTR_MAX_COLUMN_LEN = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_EMULATE_PREPARES)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_EMULATE_PREPARES = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_EMULATE_PREPARES) */
    #[\Since('8.4')]
    public const int ATTR_EMULATE_PREPARES = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_DEFAULT_FETCH_MODE)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_DEFAULT_FETCH_MODE = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_DEFAULT_FETCH_MODE) */
    #[\Since('8.4')]
    public const int ATTR_DEFAULT_FETCH_MODE = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ATTR_DEFAULT_STR_PARAM)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ATTR_DEFAULT_STR_PARAM = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ATTR_DEFAULT_STR_PARAM) */
    #[\Since('8.4')]
    public const int ATTR_DEFAULT_STR_PARAM = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ERRMODE_SILENT)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ERRMODE_SILENT = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ERRMODE_SILENT) */
    #[\Since('8.4')]
    public const int ERRMODE_SILENT = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ERRMODE_WARNING)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ERRMODE_WARNING = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ERRMODE_WARNING) */
    #[\Since('8.4')]
    public const int ERRMODE_WARNING = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_ERRMODE_EXCEPTION)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ERRMODE_EXCEPTION = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_ERRMODE_EXCEPTION) */
    #[\Since('8.4')]
    public const int ERRMODE_EXCEPTION = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_CASE_NATURAL)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CASE_NATURAL = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_CASE_NATURAL) */
    #[\Since('8.4')]
    public const int CASE_NATURAL = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_CASE_LOWER)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CASE_LOWER = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_CASE_LOWER) */
    #[\Since('8.4')]
    public const int CASE_LOWER = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_CASE_UPPER)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CASE_UPPER = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_CASE_UPPER) */
    #[\Since('8.4')]
    public const int CASE_UPPER = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_NULL_NATURAL)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const NULL_NATURAL = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_NULL_NATURAL) */
    #[\Since('8.4')]
    public const int NULL_NATURAL = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_NULL_EMPTY_STRING)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const NULL_EMPTY_STRING = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_NULL_EMPTY_STRING) */
    #[\Since('8.4')]
    public const int NULL_EMPTY_STRING = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_NULL_TO_STRING)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const NULL_TO_STRING = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_NULL_TO_STRING) */
    #[\Since('8.4')]
    public const int NULL_TO_STRING = UNKNOWN;
    /**
     * @var string
     * @cvalue PDO_ERR_NONE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ERR_NONE = UNKNOWN;
    /** @cvalue PDO_ERR_NONE */
    #[\Since('8.4')]
    public const string ERR_NONE = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_ORI_NEXT)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_ORI_NEXT = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_ORI_NEXT) */
    #[\Since('8.4')]
    public const int FETCH_ORI_NEXT = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_ORI_PRIOR)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_ORI_PRIOR = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_ORI_PRIOR) */
    #[\Since('8.4')]
    public const int FETCH_ORI_PRIOR = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_ORI_FIRST)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_ORI_FIRST = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_ORI_FIRST) */
    #[\Since('8.4')]
    public const int FETCH_ORI_FIRST = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_ORI_LAST)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_ORI_LAST = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_ORI_LAST) */
    #[\Since('8.4')]
    public const int FETCH_ORI_LAST = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_ORI_ABS)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_ORI_ABS = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_ORI_ABS) */
    #[\Since('8.4')]
    public const int FETCH_ORI_ABS = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_FETCH_ORI_REL)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const FETCH_ORI_REL = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_FETCH_ORI_REL) */
    #[\Since('8.4')]
    public const int FETCH_ORI_REL = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_CURSOR_FWDONLY)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CURSOR_FWDONLY = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_CURSOR_FWDONLY) */
    #[\Since('8.4')]
    public const int CURSOR_FWDONLY = UNKNOWN;
    /**
     * @var int
     * @cvalue LONG_CONST(PDO_CURSOR_SCROLL)
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CURSOR_SCROLL = UNKNOWN;
    /** @cvalue LONG_CONST(PDO_CURSOR_SCROLL) */
    #[\Since('8.4')]
    public const int CURSOR_SCROLL = UNKNOWN;
}