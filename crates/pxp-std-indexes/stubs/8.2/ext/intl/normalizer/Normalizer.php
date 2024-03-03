<?php

/** @generate-function-entries */
class Normalizer
{
    /**
     * @tentative-return-type
     * @alias normalizer_normalize
     * @return (string | false)
     */
    public static function normalize(string $string, int $form = Normalizer::FORM_C)
    {
    }
    /**
     * @tentative-return-type
     * @alias normalizer_is_normalized
     * @return bool
     */
    public static function isNormalized(string $string, int $form = Normalizer::FORM_C)
    {
    }
    #if U_ICU_VERSION_MAJOR_NUM >= 56
    /**
     * @tentative-return-type
     * @alias normalizer_get_raw_decomposition
     * @return (string | null)
     */
    public static function getRawDecomposition(string $string, int $form = Normalizer::FORM_C)
    {
    }
    /**
     * @var int
     * @cvalue NORMALIZER_FORM_D
     */
    public const FORM_D = UNKNOWN;
    /**
     * @var int
     * @cvalue NORMALIZER_NFD
     */
    public const NFD = UNKNOWN;
    /**
     * @var int
     * @cvalue NORMALIZER_FORM_KD
     */
    public const FORM_KD = UNKNOWN;
    /**
     * @var int
     * @cvalue NORMALIZER_NFKD
     */
    public const NFKD = UNKNOWN;
    /**
     * @var int
     * @cvalue NORMALIZER_FORM_C
     */
    public const FORM_C = UNKNOWN;
    /**
     * @var int
     * @cvalue NORMALIZER_NFC
     */
    public const NFC = UNKNOWN;
    /**
     * @var int
     * @cvalue NORMALIZER_FORM_KC
     */
    public const FORM_KC = UNKNOWN;
    /**
     * @var int
     * @cvalue NORMALIZER_NFKC
     */
    public const NFKC = UNKNOWN;
    #if U_ICU_VERSION_MAJOR_NUM >= 56
    /**
     * @var int
     * @cvalue NORMALIZER_FORM_KC_CF
     */
    public const FORM_KC_CF = UNKNOWN;
    /**
     * @var int
     * @cvalue NORMALIZER_NFKC_CF
     */
    public const NFKC_CF = UNKNOWN;
}