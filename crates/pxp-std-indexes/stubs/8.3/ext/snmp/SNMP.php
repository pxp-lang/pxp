<?php

class SNMP
{
    public function __construct(int $version, string $hostname, string $community, int $timeout = -1, int $retries = -1)
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
     * @return bool
     */
    public function setSecurity(string $securityLevel, string $authProtocol = "", string $authPassphrase = "", string $privacyProtocol = "", string $privacyPassphrase = "", string $contextName = "", string $contextEngineId = "")
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function get(array|string $objectId, bool $preserveKeys = false)
    {
    }
    /**
     * @tentative-return-type
     * @return mixed
     */
    public function getnext(array|string $objectId)
    {
    }
    /**
     * @tentative-return-type
     * @return (array | false)
     */
    public function walk(array|string $objectId, bool $suffixAsKey = false, int $maxRepetitions = -1, int $nonRepeaters = -1)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function set(array|string $objectId, array|string $type, array|string $value)
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function getErrno()
    {
    }
    /**
     * @tentative-return-type
     * @return string
     */
    public function getError()
    {
    }
    /**
     * @var int
     * @cvalue SNMP_VERSION_1
     * @link snmp.class.constants.version-1
     */
    public const VERSION_1 = UNKNOWN;
    /**
     * @cvalue SNMP_VERSION_1
     * @link snmp.class.constants.version-1
     */
    public const int VERSION_1 = UNKNOWN;
    /**
     * @var int
     * @cvalue SNMP_VERSION_2c
     * @link snmp.class.constants.version-2c
     */
    public const VERSION_2c = UNKNOWN;
    /**
     * @cvalue SNMP_VERSION_2c
     * @link snmp.class.constants.version-2c
     */
    public const int VERSION_2c = UNKNOWN;
    /**
     * @var int
     * @cvalue SNMP_VERSION_2c
     * @link snmp.class.constants.version-2c
     */
    public const VERSION_2C = UNKNOWN;
    /**
     * @cvalue SNMP_VERSION_2c
     * @link snmp.class.constants.version-2c
     */
    public const int VERSION_2C = UNKNOWN;
    /**
     * @var int
     * @cvalue SNMP_VERSION_3
     * @link snmp.class.constants.version-3
     */
    public const VERSION_3 = UNKNOWN;
    /**
     * @cvalue SNMP_VERSION_3
     * @link snmp.class.constants.version-3
     */
    public const int VERSION_3 = UNKNOWN;
    /**
     * @var int
     * @cvalue PHP_SNMP_ERRNO_NOERROR
     * @link snmp.class.constants.errno-noerror
     */
    public const ERRNO_NOERROR = UNKNOWN;
    /**
     * @cvalue PHP_SNMP_ERRNO_NOERROR
     * @link snmp.class.constants.errno-noerror
     */
    public const int ERRNO_NOERROR = UNKNOWN;
    /**
     * @var int
     * @cvalue PHP_SNMP_ERRNO_ANY
     * @link snmp.class.constants.errno-any
     */
    public const ERRNO_ANY = UNKNOWN;
    /**
     * @cvalue PHP_SNMP_ERRNO_ANY
     * @link snmp.class.constants.errno-any
     */
    public const int ERRNO_ANY = UNKNOWN;
    /**
     * @var int
     * @cvalue PHP_SNMP_ERRNO_GENERIC
     * @link snmp.class.constants.errno-generic
     */
    public const ERRNO_GENERIC = UNKNOWN;
    /**
     * @cvalue PHP_SNMP_ERRNO_GENERIC
     * @link snmp.class.constants.errno-generic
     */
    public const int ERRNO_GENERIC = UNKNOWN;
    /**
     * @var int
     * @cvalue PHP_SNMP_ERRNO_TIMEOUT
     * @link snmp.class.constants.errno-timeout
     */
    public const ERRNO_TIMEOUT = UNKNOWN;
    /**
     * @cvalue PHP_SNMP_ERRNO_TIMEOUT
     * @link snmp.class.constants.errno-timeout
     */
    public const int ERRNO_TIMEOUT = UNKNOWN;
    /**
     * @var int
     * @cvalue PHP_SNMP_ERRNO_ERROR_IN_REPLY
     * @link snmp.class.constants.errno-error-in-reply
     */
    public const ERRNO_ERROR_IN_REPLY = UNKNOWN;
    /**
     * @cvalue PHP_SNMP_ERRNO_ERROR_IN_REPLY
     * @link snmp.class.constants.errno-error-in-reply
     */
    public const int ERRNO_ERROR_IN_REPLY = UNKNOWN;
    /**
     * @var int
     * @cvalue PHP_SNMP_ERRNO_OID_NOT_INCREASING
     * @link snmp.class.constants.errno-oid-not-increasing
     */
    public const ERRNO_OID_NOT_INCREASING = UNKNOWN;
    /**
     * @cvalue PHP_SNMP_ERRNO_OID_NOT_INCREASING
     * @link snmp.class.constants.errno-oid-not-increasing
     */
    public const int ERRNO_OID_NOT_INCREASING = UNKNOWN;
    /**
     * @var int
     * @cvalue PHP_SNMP_ERRNO_OID_PARSING_ERROR
     * @link snmp.class.constants.errno-oid-parsing-error
     */
    public const ERRNO_OID_PARSING_ERROR = UNKNOWN;
    /**
     * @cvalue PHP_SNMP_ERRNO_OID_PARSING_ERROR
     * @link snmp.class.constants.errno-oid-parsing-error
     */
    public const int ERRNO_OID_PARSING_ERROR = UNKNOWN;
    /**
     * @var int
     * @cvalue PHP_SNMP_ERRNO_MULTIPLE_SET_QUERIES
     * @link snmp.class.constants.errno-multiple-set-queries
     */
    public const ERRNO_MULTIPLE_SET_QUERIES = UNKNOWN;
    /**
     * @cvalue PHP_SNMP_ERRNO_MULTIPLE_SET_QUERIES
     * @link snmp.class.constants.errno-multiple-set-queries
     */
    public const int ERRNO_MULTIPLE_SET_QUERIES = UNKNOWN;
}