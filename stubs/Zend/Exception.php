<?php 

class Exception implements \Throwable
{
    #[\Until('8.1')]
    final private function __clone(): void
    {
    }
    #[\Since('8.1')]
    private function __clone(): void
    {
    }
    public function __construct(string $message = "", int $code = 0, ?Throwable $previous = null)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function __wakeup()
    {
    }
    final public function getMessage(): string
    {
    }
    final public function getCode()
    {
    }
    final public function getFile(): string
    {
    }
    final public function getLine(): int
    {
    }
    final public function getTrace(): array
    {
    }
    final public function getPrevious(): ?Throwable
    {
    }
    final public function getTraceAsString(): string
    {
    }
    public function __toString(): string
    {
    }
}