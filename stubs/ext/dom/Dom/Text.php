<?php 

namespace Dom;

#[\Since('8.4')]
class Text extends \Dom\CharacterData
{
    /* No constructor because Node has a final private constructor, so PHP does not allow overriding that. */
    /** @implementation-alias DOMText::splitText */
    public function splitText(int $offset): Text
    {
    }
    /**
     * @readonly
     * @virtual
     */
    public string $wholeText;
}