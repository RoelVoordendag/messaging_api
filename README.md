## Introduction

Small api for messaging_app to create a messaging app with a rust combined with a tauri FE.

## Todo:
- [X] Create routes for sending messages
    - [X] Handling the saving of the new items inside it
    - [X] We need to learn how to handle json import
- [x] Setup database (POSTGRES) -> docker?
- [X] Research what kind of cargos can I use to connect to a DB (Diesel)
- [X] Do initial bootstrap for connection with DB.
- [ ] Setup socket connections
- [ ] During socket connection save messages
- [X] Users we need to save usernames
  - [ ] Combine usernames to messages
- [ ] Need to better routing inside actix web
  - [X] Pass db connection to endpoints, so we do not have to initialize it everytime
  - [ ] We need to check what the proper way is of handling multiple routes because currently I do not have the feeling this is correct
- Error Handling
  - [X] Learn on how to handle errors from find queries
  - [X] How do I return a error from the API when something is not correct
  - [ ] How do we catch errors when unwrapping uuid parses
- [ ] Update mermaid chart

The idea of how we will save stuff is that there is a room where we connect
the people in it. This means that 2 users in a means it is a personal chat and multiple people.
Inside the message we save to which room id it is stored. (this might change over time)

```mermaid
---
Title: Flow of saving of messages
---

classDiagram

    Message: uuid
    Message: body
    Message: roomid

    Users: uuid
    Users: username


    Rooms: uuid
    Rooms: name

    RoomsUsers: userid
    RoomsUsers: roomsid

    Message <|-- Rooms

    RoomsUsers <|-- Users
    RoomsUsers <|-- Rooms
```
