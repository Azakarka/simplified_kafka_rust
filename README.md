Simple asynchronous tcp-server written on Rust. 

There are two types of clients: subscribers and publishers. 

Both types connect to a certain topic and subscribers get the messages that publishers send.

The first client message should be
```
{"method": "subscribe", "topic": "topic_name"}
```
or
```
{"method": "publish", "topic": "topic_name"}
```

Then all other messages should be sent as
```
{"message": "message_content"}
```

# flags
`-a` `--address <address>` specifies address to listen to

`-p` `--port <port>` specifies listening port
