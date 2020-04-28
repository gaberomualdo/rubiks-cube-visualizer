<?php

/*
A typical request (body) looks like:

{
    "left_face": [
        ["green","red","green"],
        ["blue","green","green"],
        ["green","green","green"]
    ],
    "right_face": [
        ["red","red","red"],
        ["red","red","red"],
        ["red","red","red"]
    ],
    "top_face": [
        ["white","white","white"],
        ["white","white","white"],
        ["white","white","white"]
    ],
    "large_image":true
}
*/

header('Content-Type: application/json');

try {
    $request_data = json_decode(file_get_contents('php://input'), true);


    // check all required keys are present in POST request body
    $required_keys = ["left_face", "right_face", "top_face", "large_image"];

    foreach($required_keys as $key) {
        if (!array_key_exists($key, $request_data)) {
            http_response_code(400);
            echo json_encode([ "error" => "Missing required parameter '" . $key . "'" ]);
            exit();
        }
    }

    // evaluate large image flag
    $large_image_flag = "";

    if($request_data["large_image"] == true) {
        $large_image_flag = "-L";
    }

    // create generation command and output image path
    $output_path = "../img/out/";

    if ($request_data["large_image"] == true) {
        $output_path .= "large/";
    } else {
        $output_path .= "small/";
    }

    $generation_command = "./target/release/rubiks-cube-visualizer " . $large_image_flag;

    // add each color to generation command and output path
    foreach([$request_data["left_face"], $request_data["right_face"], $request_data["top_face"]] as $face) {
        foreach($face as $row) {
            foreach($row as $color) {
                // invalid color checking
                if(!($color == "orange" || $color == "green" || $color == "red" || $color == "blue" || $color == "white" || $color == "yellow")) {
                    http_response_code(400);
                    echo json_encode([ "error" => "Invalid color '" . $color . "'." ]);
                    exit();
                }
    
                $generation_command .= " " . $color;
                $output_path .= $color[0];
            }
        }
    }

    // execute command and return output path
    $output_path .= ".jpg";

    exec("cd .. && " . $generation_command);

    echo json_encode([ "output_path" => $output_path ]);
} catch (Exception $e) {
    echo "Server Error";
}
?>