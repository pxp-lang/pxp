<?php 

#[\Since('8.4')]
final class StreamBucket
{
    /**
     * @var resource
     * @readonly
     */
    public $bucket;
    /** @readonly */
    public string $data;
    /** @readonly */
    public int $datalen;
    /** @readonly */
    public int $dataLength;
}