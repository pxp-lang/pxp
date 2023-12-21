<?php

namespace App;

use App\Interfaces\Model;
use DateTimeInterface;

class User implements Model
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
