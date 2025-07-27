# Yet Another HTTP simple server

A basic implementation in RUST of a HTTP/1.1 server to serve web files

## Environment

You can set custom variables:

* `SERVER_ADDR`: The server address. By default `127.0.0.1:9090`
* `PUBLIC_PATH`: Where the files of the server are read from. By default `public`

## To Do

This is a very basic implementation that can be extended with:

* Add support for headers in request and response
* Handle POST, PUT, etc. methods
* Multithreading for request handling (with std::thread and std::sync)
