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
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const REASON_UNASSIGNED = UNKNOWN;
    /* enum UConverterCallbackReason */
    /** @cvalue UCNV_UNASSIGNED */
    #[\Since('8.4')]
    public const int REASON_UNASSIGNED = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_ILLEGAL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const REASON_ILLEGAL = UNKNOWN;
    /** @cvalue UCNV_ILLEGAL */
    #[\Since('8.4')]
    public const int REASON_ILLEGAL = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_IRREGULAR
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const REASON_IRREGULAR = UNKNOWN;
    /** @cvalue UCNV_IRREGULAR */
    #[\Since('8.4')]
    public const int REASON_IRREGULAR = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_RESET
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const REASON_RESET = UNKNOWN;
    /** @cvalue UCNV_RESET */
    #[\Since('8.4')]
    public const int REASON_RESET = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_CLOSE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const REASON_CLOSE = UNKNOWN;
    /** @cvalue UCNV_CLOSE */
    #[\Since('8.4')]
    public const int REASON_CLOSE = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_CLONE
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const REASON_CLONE = UNKNOWN;
    /** @cvalue UCNV_CLONE */
    #[\Since('8.4')]
    public const int REASON_CLONE = UNKNOWN;
    /* enum UConverterType */
    /**
     * @var int
     * @cvalue UCNV_UNSUPPORTED_CONVERTER
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const UNSUPPORTED_CONVERTER = UNKNOWN;
    /* enum UConverterType */
    /** @cvalue UCNV_UNSUPPORTED_CONVERTER */
    #[\Since('8.4')]
    public const int UNSUPPORTED_CONVERTER = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_SBCS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SBCS = UNKNOWN;
    /** @cvalue UCNV_SBCS */
    #[\Since('8.4')]
    public const int SBCS = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_DBCS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const DBCS = UNKNOWN;
    /** @cvalue UCNV_DBCS */
    #[\Since('8.4')]
    public const int DBCS = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_MBCS
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const MBCS = UNKNOWN;
    /** @cvalue UCNV_MBCS */
    #[\Since('8.4')]
    public const int MBCS = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LATIN_1
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LATIN_1 = UNKNOWN;
    /** @cvalue UCNV_LATIN_1 */
    #[\Since('8.4')]
    public const int LATIN_1 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_UTF8
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const UTF8 = UNKNOWN;
    /** @cvalue UCNV_UTF8 */
    #[\Since('8.4')]
    public const int UTF8 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_UTF16_BigEndian
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const UTF16_BigEndian = UNKNOWN;
    /** @cvalue UCNV_UTF16_BigEndian */
    #[\Since('8.4')]
    public const int UTF16_BigEndian = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_UTF16_LittleEndian
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const UTF16_LittleEndian = UNKNOWN;
    /** @cvalue UCNV_UTF16_LittleEndian */
    #[\Since('8.4')]
    public const int UTF16_LittleEndian = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_UTF32_BigEndian
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const UTF32_BigEndian = UNKNOWN;
    /** @cvalue UCNV_UTF32_BigEndian */
    #[\Since('8.4')]
    public const int UTF32_BigEndian = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_UTF32_LittleEndian
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const UTF32_LittleEndian = UNKNOWN;
    /** @cvalue UCNV_UTF32_LittleEndian */
    #[\Since('8.4')]
    public const int UTF32_LittleEndian = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_EBCDIC_STATEFUL
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const EBCDIC_STATEFUL = UNKNOWN;
    /** @cvalue UCNV_EBCDIC_STATEFUL */
    #[\Since('8.4')]
    public const int EBCDIC_STATEFUL = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_ISO_2022
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ISO_2022 = UNKNOWN;
    /** @cvalue UCNV_ISO_2022 */
    #[\Since('8.4')]
    public const int ISO_2022 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_1
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LMBCS_1 = UNKNOWN;
    /** @cvalue UCNV_LMBCS_1 */
    #[\Since('8.4')]
    public const int LMBCS_1 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_2
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LMBCS_2 = UNKNOWN;
    /** @cvalue UCNV_LMBCS_2 */
    #[\Since('8.4')]
    public const int LMBCS_2 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_3
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LMBCS_3 = UNKNOWN;
    /** @cvalue UCNV_LMBCS_3 */
    #[\Since('8.4')]
    public const int LMBCS_3 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_4
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LMBCS_4 = UNKNOWN;
    /** @cvalue UCNV_LMBCS_4 */
    #[\Since('8.4')]
    public const int LMBCS_4 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_5
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LMBCS_5 = UNKNOWN;
    /** @cvalue UCNV_LMBCS_5 */
    #[\Since('8.4')]
    public const int LMBCS_5 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_6
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LMBCS_6 = UNKNOWN;
    /** @cvalue UCNV_LMBCS_6 */
    #[\Since('8.4')]
    public const int LMBCS_6 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_8
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LMBCS_8 = UNKNOWN;
    /** @cvalue UCNV_LMBCS_8 */
    #[\Since('8.4')]
    public const int LMBCS_8 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_11
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LMBCS_11 = UNKNOWN;
    /** @cvalue UCNV_LMBCS_11 */
    #[\Since('8.4')]
    public const int LMBCS_11 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_16
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LMBCS_16 = UNKNOWN;
    /** @cvalue UCNV_LMBCS_16 */
    #[\Since('8.4')]
    public const int LMBCS_16 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_17
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LMBCS_17 = UNKNOWN;
    /** @cvalue UCNV_LMBCS_17 */
    #[\Since('8.4')]
    public const int LMBCS_17 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_18
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LMBCS_18 = UNKNOWN;
    /** @cvalue UCNV_LMBCS_18 */
    #[\Since('8.4')]
    public const int LMBCS_18 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_19
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LMBCS_19 = UNKNOWN;
    /** @cvalue UCNV_LMBCS_19 */
    #[\Since('8.4')]
    public const int LMBCS_19 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_LMBCS_LAST
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const LMBCS_LAST = UNKNOWN;
    /** @cvalue UCNV_LMBCS_LAST */
    #[\Since('8.4')]
    public const int LMBCS_LAST = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_HZ
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const HZ = UNKNOWN;
    /** @cvalue UCNV_HZ */
    #[\Since('8.4')]
    public const int HZ = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_SCSU
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const SCSU = UNKNOWN;
    /** @cvalue UCNV_SCSU */
    #[\Since('8.4')]
    public const int SCSU = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_ISCII
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const ISCII = UNKNOWN;
    /** @cvalue UCNV_ISCII */
    #[\Since('8.4')]
    public const int ISCII = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_US_ASCII
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const US_ASCII = UNKNOWN;
    /** @cvalue UCNV_US_ASCII */
    #[\Since('8.4')]
    public const int US_ASCII = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_UTF7
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const UTF7 = UNKNOWN;
    /** @cvalue UCNV_UTF7 */
    #[\Since('8.4')]
    public const int UTF7 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_BOCU1
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const BOCU1 = UNKNOWN;
    /** @cvalue UCNV_BOCU1 */
    #[\Since('8.4')]
    public const int BOCU1 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_UTF16
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const UTF16 = UNKNOWN;
    /** @cvalue UCNV_UTF16 */
    #[\Since('8.4')]
    public const int UTF16 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_UTF32
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const UTF32 = UNKNOWN;
    /** @cvalue UCNV_UTF32 */
    #[\Since('8.4')]
    public const int UTF32 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_CESU8
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const CESU8 = UNKNOWN;
    /** @cvalue UCNV_CESU8 */
    #[\Since('8.4')]
    public const int CESU8 = UNKNOWN;
    /**
     * @var int
     * @cvalue UCNV_IMAP_MAILBOX
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const IMAP_MAILBOX = UNKNOWN;
    /** @cvalue UCNV_IMAP_MAILBOX */
    #[\Since('8.4')]
    public const int IMAP_MAILBOX = UNKNOWN;
}