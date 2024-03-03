<?php

/** @generate-function-entries */
class UConverter
{
    public function __construct(?string $destination_encoding = null, ?string $source_encoding = null)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public function convert(string $str, bool $reverse = false)
    {
    }
    /**
     * @param int $error
     * @tentative-return-type
     * @return (string | int | array | null)
     */
    public function fromUCallback(int $reason, array $source, int $codePoint, &$error)
    {
    }
    /** @return array|false|null */
    public static function getAliases(string $name)
    {
    }
    /** @return array */
    public static function getAvailable()
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false | null)
     */
    public function getDestinationEncoding()
    {
    }
    /**
     * @tentative-return-type
     * @return (int | false | null)
     */
    public function getDestinationType()
    {
    }
    /**
     * @tentative-return-type
     * @return int
     */
    public function getErrorCode()
    {
    }
    /**
     * @tentative-return-type
     * @return (string | null)
     */
    public function getErrorMessage()
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false | null)
     */
    public function getSourceEncoding()
    {
    }
    /**
     * @tentative-return-type
     * @return (int | false | null)
     */
    public function getSourceType()
    {
    }
    /**
     * @tentative-return-type
     * @return (array | null)
     */
    public static function getStandards()
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false | null)
     */
    public function getSubstChars()
    {
    }
    /**
     * @tentative-return-type
     * @return string
     */
    public static function reasonText(int $reason)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setDestinationEncoding(string $encoding)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setSourceEncoding(string $encoding)
    {
    }
    /**
     * @tentative-return-type
     * @return bool
     */
    public function setSubstChars(string $chars)
    {
    }
    /**
     * @param int $error
     * @tentative-return-type
     * @return (string | int | array | null)
     */
    public function toUCallback(int $reason, string $source, string $codeUnits, &$error)
    {
    }
    /**
     * @tentative-return-type
     * @return (string | false)
     */
    public static function transcode(string $str, string $toEncoding, string $fromEncoding, ?array $options = null)
    {
    }
    /* enum UConverterCallbackReason */
    /**
     * @var int
     * @cvalue UCNV_UNASSIGNED
     */
    public const REASON_UNASSIGNED = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_ILLEGAL
     */
    public const REASON_ILLEGAL = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_IRREGULAR
     */
    public const REASON_IRREGULAR = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_RESET
     */
    public const REASON_RESET = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_CLOSE
     */
    public const REASON_CLOSE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_CLONE
     */
    public const REASON_CLONE = UNKNOWN;
    /* enum UConverterType */
    /**
     * @var int
     * @cvalue UCNV_UNSUPPORTED_CONVERTER
     */
    public const UNSUPPORTED_CONVERTER = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_SBCS
     */
    public const SBCS = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_DBCS
     */
    public const DBCS = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_MBCS
     */
    public const MBCS = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LATIN_1
     */
    public const LATIN_1 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_UTF8
     */
    public const UTF8 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_UTF16_BigEndian
     */
    public const UTF16_BigEndian = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_UTF16_LittleEndian
     */
    public const UTF16_LittleEndian = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_UTF32_BigEndian
     */
    public const UTF32_BigEndian = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_UTF32_LittleEndian
     */
    public const UTF32_LittleEndian = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_EBCDIC_STATEFUL
     */
    public const EBCDIC_STATEFUL = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_ISO_2022
     */
    public const ISO_2022 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_1
     */
    public const LMBCS_1 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_2
     */
    public const LMBCS_2 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_3
     */
    public const LMBCS_3 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_4
     */
    public const LMBCS_4 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_5
     */
    public const LMBCS_5 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_6
     */
    public const LMBCS_6 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_8
     */
    public const LMBCS_8 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_11
     */
    public const LMBCS_11 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_16
     */
    public const LMBCS_16 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_17
     */
    public const LMBCS_17 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_18
     */
    public const LMBCS_18 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_19
     */
    public const LMBCS_19 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_LAST
     */
    public const LMBCS_LAST = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_HZ
     */
    public const HZ = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_SCSU
     */
    public const SCSU = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_ISCII
     */
    public const ISCII = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_US_ASCII
     */
    public const US_ASCII = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_UTF7
     */
    public const UTF7 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_BOCU1
     */
    public const BOCU1 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_UTF16
     */
    public const UTF16 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_UTF32
     */
    public const UTF32 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_CESU8
     */
    public const CESU8 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_IMAP_MAILBOX
     */
    public const IMAP_MAILBOX = UNKNOWN;
}