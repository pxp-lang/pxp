<?php 

namespace Dom;

#[\Since('8.4')]
class ProcessingInstruction extends \Dom\CharacterData
{
    /**
     * @readonly
     * @virtual
     */
    public string $target;
}