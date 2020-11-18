<?php

class Auth {

    static private array $users = [
        [
            "name" => "test",
            "password" => "hogehoge",
            "permissions" => ['cart'],
            "token" => "abc123456"
        ],
        [
            "name" => "test2",
            "password" => "hogehoge2",
            "permissions" => [],
            "token" => "xyz123456"
        ]
    ];

    static public function getUser(string $name): ?array{
        foreach(self::$users as $v) {
            if($v["name"] == $name) {
                return $v;
            }
        }
        return null;
    }

    static public function getUserByToken(string $token): ?array{
        foreach(self::$users as $v) {
            if($v["token"] == $token) {
                return $v;
            }
        }
        return null;
    }

};


