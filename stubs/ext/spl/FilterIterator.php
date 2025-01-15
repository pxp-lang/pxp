<?php 

abstract class FilterIterator extends \IteratorIterator
{
    /**
     * @tentative-return-type
     * @return bool
     */
    abstract public function accept();
    public function __construct(Iterator $iterator)
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function rewind()
    {
    }
    /**
     * @tentative-return-type
     * @return void
     */
    public function next()
    {
    }
}