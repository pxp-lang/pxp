<?php 

class Error implements \Throwable
{
    /** @implementation-alias Exception::__clone */
    #[\Until('8.1')]
    final private function __clone(): void
    {
    }
    /** @implementation-alias Exception::__clone */
    #[\Since('8.1')]
    private function __clone(): void
    {
    }
    /** @implementation-alias Exception::__construct */
    public function __construct(string $message = "", int $code = 0, ?Throwable $previous = null)
    {
    }
    /**
     * @tentative-return-type
     * @implementation-alias Exception::__wakeup
     * @return void
     */
    public function __wakeup()
    {
    }
    /** @implementation-alias Exception::getMessage */
    final public function getMessage(): string
    {
    }
    /**
     * @return int
     * @implementation-alias Exception::getCode
     */
    final public function getCode()
    {
    }
    /** @implementation-alias Exception::getFile */
    final public function getFile(): string
    {
    }
    /** @implementation-alias Exception::getLine */
    final public function getLine(): int
    {
    }
    /** @implementation-alias Exception::getTrace */
    final public function getTrace(): array
    {
    }
    /** @implementation-alias Exception::getPrevious */
    final public function getPrevious(): ?Throwable
    {
    }
    /** @implementation-alias Exception::getTraceAsString */
    final public function getTraceAsString(): string
    {
    }
    /** @implementation-alias Exception::__toString */
    public function __toString(): string
    {
    }
}