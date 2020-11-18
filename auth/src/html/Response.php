<?php

class Response
{
    static function json(array $body = [], int $status = 200)
    {
        http_response_code($status);
        print (json_encode($body));
        exit();
    }
}