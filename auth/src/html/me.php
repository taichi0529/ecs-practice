<?php
require "Response.php";
require "Auth.php";

if (isset($_GET['token']) && $user = Auth::getUserByToken($_GET['token'])) {
    Response::json([
        'name' => $user["name"],
        'permissions' => $user["permissions"],
    ]);
}
Response::json([], '401');