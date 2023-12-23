<?php

namespace App;

use Nested\Interfaces;
use DateTimeInterface;

class User implements Interfaces\Model
{
    private string $name;

    private string $email;

    private DateTimeInterface $createdAt;

    public function __construct(string $name, string $email, DateTimeInterface $createdAt)
    {
        $this->name = $name;
        $this->email = $email;
        $this->createdAt = $createdAt;
    }

    public function getName(): string
    {
        return $this->name;
    }

    public function getEmail(): string
    {
        return $this->email;
    }

    public function getCreatedAt(): DateTimeInterface
    {
        return $this->createdAt;
    }
}
