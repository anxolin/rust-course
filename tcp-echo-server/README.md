 ## TCP Ecp server
 
 To test the server
 ```bash
 # Get payload
 echo -n '"GET"' | nc -c localhost 8080

# Set payload
 echo -n '{ "SET": { "count": 1, "data": "Hi from client ❤👋"} }' | nc -c localhost 8080
 ```

 