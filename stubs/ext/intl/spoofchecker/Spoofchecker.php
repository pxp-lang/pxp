<?php 

/** @generate-function-entries */
class Spoofchecker
{
    public function __construct()
    {
    }
    /**
     * @param int $errorCode
     * @tentative-return-type
     * @return bool
     */
    public function isSuspicious(string $string, &$errorCode = null)
    {
    }
    /**
     * @param int $errorCode
     * @tentative-return-type
     * @return bool
     */
    public function areConfusable(string $string1, string $string2, &$errorCode = null)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function setAllowedLocales(string $locales)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function setChecks(int $checks)
    {
    }
    #if U_ICU_VERSION_MAJOR_NUM >= 58
    /**
     * @tentative-return-type
     * @return void
     */
    public function setRestrictionLevel(int $level)
    {
    }
    #endif
    #[\Since('8.4')]
    public function setAllowedChars(string $pattern, int $patternOptions = 0): void
    {
    }
    /**
     * @var int
     * @cvalue USPOOF_SINGLE_SCRIPT_CONFUSABLE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SINGLE_SCRIPT_CONFUSABLE = UNKNOWN;
    /** @cvalue USPOOF_SINGLE_SCRIPT_CONFUSABLE */
    #[\Since('8.4')]
    public const int SINGLE_SCRIPT_CONFUSABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue USPOOF_MIXED_SCRIPT_CONFUSABLE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const MIXED_SCRIPT_CONFUSABLE = UNKNOWN;
    /** @cvalue USPOOF_MIXED_SCRIPT_CONFUSABLE */
    #[\Since('8.4')]
    public const int MIXED_SCRIPT_CONFUSABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue USPOOF_WHOLE_SCRIPT_CONFUSABLE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const WHOLE_SCRIPT_CONFUSABLE = UNKNOWN;
    /** @cvalue USPOOF_WHOLE_SCRIPT_CONFUSABLE */
    #[\Since('8.4')]
    public const int WHOLE_SCRIPT_CONFUSABLE = UNKNOWN;
    /**
     * @var int
     * @cvalue USPOOF_ANY_CASE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ANY_CASE = UNKNOWN;
    /** @cvalue USPOOF_ANY_CASE */
    #[\Since('8.4')]
    public const int ANY_CASE = UNKNOWN;
    /**
     * @var int
     * @cvalue USPOOF_SINGLE_SCRIPT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SINGLE_SCRIPT = UNKNOWN;
    /** @cvalue USPOOF_SINGLE_SCRIPT */
    #[\Since('8.4')]
    public const int SINGLE_SCRIPT = UNKNOWN;
    /**
     * @var int
     * @cvalue USPOOF_INVISIBLE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const INVISIBLE = UNKNOWN;
    /** @cvalue USPOOF_INVISIBLE */
    #[\Since('8.4')]
    public const int INVISIBLE = UNKNOWN;
    /**
     * @var int
     * @cvalue USPOOF_CHAR_LIMIT
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CHAR_LIMIT = UNKNOWN;
    /** @cvalue USPOOF_CHAR_LIMIT */
    #[\Since('8.4')]
    public const int CHAR_LIMIT = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 58
    /**
     * @var int
     * @cvalue USPOOF_ASCII
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ASCII = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 58
    /** @cvalue USPOOF_ASCII */
    #[\Since('8.4')]
    public const int ASCII = UNKNOWN;
    /**
     * @var int
     * @cvalue USPOOF_HIGHLY_RESTRICTIVE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const HIGHLY_RESTRICTIVE = UNKNOWN;
    /** @cvalue USPOOF_HIGHLY_RESTRICTIVE */
    #[\Since('8.4')]
    public const int HIGHLY_RESTRICTIVE = UNKNOWN;
    /**
     * @var int
     * @cvalue USPOOF_MODERATELY_RESTRICTIVE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const MODERATELY_RESTRICTIVE = UNKNOWN;
    /** @cvalue USPOOF_MODERATELY_RESTRICTIVE */
    #[\Since('8.4')]
    public const int MODERATELY_RESTRICTIVE = UNKNOWN;
    /**
     * @var int
     * @cvalue USPOOF_MINIMALLY_RESTRICTIVE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const MINIMALLY_RESTRICTIVE = UNKNOWN;
    /** @cvalue USPOOF_MINIMALLY_RESTRICTIVE */
    #[\Since('8.4')]
    public const int MINIMALLY_RESTRICTIVE = UNKNOWN;
    /**
     * @var int
     * @cvalue USPOOF_UNRESTRICTIVE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const UNRESTRICTIVE = UNKNOWN;
    /** @cvalue USPOOF_UNRESTRICTIVE */
    #[\Since('8.4')]
    public const int UNRESTRICTIVE = UNKNOWN;
    /**
     * @var int
     * @cvalue USPOOF_SINGLE_SCRIPT_RESTRICTIVE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SINGLE_SCRIPT_RESTRICTIVE = UNKNOWN;
    /** @cvalue USPOOF_SINGLE_SCRIPT_RESTRICTIVE */
    #[\Since('8.4')]
    public const int SINGLE_SCRIPT_RESTRICTIVE = UNKNOWN;
    /**
     * @var int
     * @cvalue USPOOF_MIXED_NUMBERS
     */
    #[\Since('8.3')]
    #[\Until('8.4')]
    public const MIXED_NUMBERS = UNKNOWN;
    /** @cvalue USPOOF_MIXED_NUMBERS */
    #[\Since('8.4')]
    public const int MIXED_NUMBERS = UNKNOWN;
    #endif
    #if U_ICU_VERSION_MAJOR_NUM >= 62
    /**
     * @var int
     * @cvalue USPOOF_HIDDEN_OVERLAY
     */
    #[\Since('8.3')]
    #[\Until('8.4')]
    public const HIDDEN_OVERLAY = UNKNOWN;
    #endif
    #if U_ICU_VERSION_MAJOR_NUM >= 62
    /** @cvalue USPOOF_HIDDEN_OVERLAY */
    #[\Since('8.4')]
    public const int HIDDEN_OVERLAY = UNKNOWN;
    #endif
    /** @cvalue USET_IGNORE_SPACE */
    #[\Since('8.4')]
    public const int IGNORE_SPACE = UNKNOWN;
    /** @cvalue USET_CASE_INSENSITIVE */
    #[\Since('8.4')]
    public const int CASE_INSENSITIVE = UNKNOWN;
    /** @cvalue USET_ADD_CASE_MAPPINGS */
    #[\Since('8.4')]
    public const int ADD_CASE_MAPPINGS = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 73
    /** @cvalue USET_SIMPLE_CASE_INSENSITIVE */
    #[\Since('8.4')]
    public const int SIMPLE_CASE_INSENSITIVE = UNKNOWN;
}