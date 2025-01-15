<?php 

namespace Dom;

#[\Since('8.4')]
class CharacterData extends \Dom\Node implements \Dom\ChildNode
{
    /**
     * @readonly
     * @virtual
     */
    public ?Element $previousElementSibling;
    /**
     * @readonly
     * @virtual
     */
    public ?Element $nextElementSibling;
    /** @virtual */
    public string $data;
    /**
     * @readonly
     * @virtual
     */
    public int $length;
    /** @implementation-alias DOMCharacterData::substringData */
    public function substringData(int $offset, int $count): string
    {
    }
    public function appendData(string $data): void
    {
    }
    public function insertData(int $offset, string $data): void
    {
    }
    public function deleteData(int $offset, int $count): void
    {
    }
    public function replaceData(int $offset, int $count, string $data): void
    {
    }
    /** @implementation-alias DOMElement::remove */
    public function remove(): void
    {
    }
    /** @implementation-alias DOMElement::before */
    public function before(Node|string ...$nodes): void
    {
    }
    /** @implementation-alias DOMElement::after */
    public function after(Node|string ...$nodes): void
    {
    }
    /** @implementation-alias DOMElement::replaceWith */
    public function replaceWith(Node|string ...$nodes): void
    {
    }
}