 ## TCP Ecp server

 ```bash
 cargo run --bin server
 cargo run --bin client
 ```
 
 To test the server
 ```bash
 echo -n 'Line of text' | nc -c localhost 8080
 ```