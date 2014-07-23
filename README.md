# Stompers

Stompers is a STOMP client for rust. It is currently in its early stages and probably shouldn't be used. I should also point out it's my first Rust project. Be warned! Feel free to try it though. 

So far it has only been tested with RabbitMQ, but it should work with other servers. I think. Maybe.

For an example of use, go to [examples/basic/main.rs](https://github.com/mattyhall/stompers/blob/master/examples/basic/main.rs).

## TODO
A list of things to do are available as a [Trello board](https://trello.com/b/bHOrED1w/stompers)

## Make
To build the library and the example. Run

    $ cargo build

You can then run the example by doing

    $ ./target/basic

## Things working
* [ ] Connection
    * [x] Getting a connection
    * [ ] Specifying a version
        * [ ] 1.2
        * [ ] 1.1
        * [ ] 1.0
* [ ] Messages
    * [x] Sending a message
    * [x] Adding headers to a message
    * [ ] Calculating and including a content-length
* [ ] Subscription
    * [x] Subscribing to a queue
    * [x] Getting messages back from the queue
    * [ ] Ack
* [ ] Heartbeat
* [ ] Nice cleanup
    * [ ] Unsubscribe from queues
    * [ ] Send disconnect frame
* [ ] Documentation
