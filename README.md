# Microurse

This branch contains a simple Microservice for creating and commenting on posts
built during the Microservices Course by Stephen Grider on [Udemy](https://www.udemy.com/course/microservices-with-node-js-and-react/).
While the original course uses NodeJS and React for its projects, this repository
will have their equivalent architecture and functionality in Rust and Vue. All services use an
in-memory database (using a HashMap).

## Services

This project includes the following services:

* [posts](./posts)

  * An axum webserver that runs on port 4000
  * Responsible for creating posts

* [comments](./comments)
  
  * An axum webserver that runs on port 4001
  * Responsible for storing comments for each post

* [client](./client)

  * A Vue app that runs on port 4002
  * Responsible for the frontend of the overall app.

* [event-bus](./event-bus)

  * An axum webserver that runs on port 4003
  * Responsible for receiving and dispatching events from and to all the other
    services.

* [query](./query)

  * An axum webserver that runs on port 4004
  * Responsible for fast querying of all posts and related comments

* [moderation](./moderation)

  * An axum webserver that runs on port 4005
  * Responsible for ensuring that no comment contains any blacklisted words.
