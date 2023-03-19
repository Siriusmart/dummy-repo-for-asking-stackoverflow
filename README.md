# Problem

Webassembly is sending a post req to an actix web server, but actix web says it cannot deserialize the json body.

In terminal (for actix web) you can see the error message of

```
Failed to deserialize Json from payload. Request path: /api
Error in response: Deserialize(Error("expected value", line: 1, column: 2))
```

And in javascript console

```
Response { url: "http://localhost:8080/api", status: 400, statusText: "Bad Request", /* ... */ }
```

I have tried sending the same request using curl, and it worked. so there is clearly something wrong with the request the wasm is sending, but im not sure what

```sh
curl --header "Content-Type: application/json" \                                                                                                                         :(
  --request POST \
  --data '{"field1": "one", "field2": "two", "field3": "three"}' \
  http://localhost:8080/api
```

## How to run the code

`wasm-pack build --target web wasm` to recompile the web assembly first

then`cargo run` to start the server

visit http://localhost:8080 to see the site with wasm.

when the button is pressed, there should be output to the js console, saying that its a bad request, at the same time there will also be an error message in the actix web console.

the goal is to make it so that the wasm actually sends a good request to the server.

> Thank you in advance
