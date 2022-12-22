<?php

/** Gets the pdo instance */
function getPDO(): PDO
{
    static $pdo = new PDO("mysql:host=localhost;dbname=de-blauwe-loper", "root", "");

    return $pdo;
}
