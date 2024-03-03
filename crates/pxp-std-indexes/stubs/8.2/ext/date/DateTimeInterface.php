<?php

// NB: Adding return types to methods is a BC break!
// For now only using @return annotations here.
interface DateTimeInterface
{
    /**
     * @tentative-return-type
     * @return string
     */
    public function format(string $format);
    /**
     * @tentative-return-type
     * @return (DateTimeZone | false)
     */
    public function getTimezone();
    /**
     * @tentative-return-type
     * @return int
     */
    public function getOffset();
    /**
     * @tentative-return-type
     * @return (int | false)
     */
    public function getTimestamp();
    /**
     * @tentative-return-type
     * @return (DateInterval | false)
     */
    public function diff(DateTimeInterface $targetObject, bool $absolute = false);
    /**
     * @tentative-return-type
     * @return void
     */
    public function __wakeup();
    public function __serialize(): array;
    public function __unserialize(array $data): void;
    /** @var string */
    public const ATOM = DATE_ATOM;
    /** @var string */
    public const COOKIE = DATE_COOKIE;
    /** @var string */
    public const ISO8601 = DATE_ISO8601;
    /** @var string */
    public const ISO8601_EXPANDED = DATE_ISO8601_EXPANDED;
    /** @var string */
    public const RFC822 = DATE_RFC822;
    /** @var string */
    public const RFC850 = DATE_RFC850;
    /** @var string */
    public const RFC1036 = DATE_RFC1036;
    /** @var string */
    public const RFC1123 = DATE_RFC1123;
    /** @var string */
    public const RFC7231 = DATE_RFC7231;
    /** @var string */
    public const RFC2822 = DATE_RFC2822;
    /** @var string */
    public const RFC3339 = DATE_RFC3339;
    /** @var string */
    public const RFC3339_EXTENDED = DATE_RFC3339_EXTENDED;
    /** @var string */
    public const RSS = DATE_RSS;
    /** @var string */
    public const W3C = DATE_W3C;
}