<?php
require "Response.php";
require "Auth.php";

if (isset($_POST['name']) && isset($_POST['password']) && $user = Auth::getUser($_POST['name'])) {
    if ($user["password"] == $_POST["password"]) {
        Response::json(['token' => $user["token"]]);
    }
}
Response::json([], '401');

