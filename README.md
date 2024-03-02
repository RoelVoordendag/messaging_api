## Todo:
- [ ] Create routes for sending messages
    - [ ] Handling the saving of the new items inside it
    - [ ] We need to learn how to handle json import
- [x] Setup database (POSTGRES) -> docker?
- [X] Research what kind of cargos can I use to connect to a DB (Diesel)
- [X] Do initial bootstrap for connection with DB.
- [ ] Setup socket connections
- [ ] During socket connection save messages
  - [ ] Users we need to save user names
    - [ ] Combine usernames to messages

use migration::{Migrator, MigratorTrait};

let connection = sea_orm::Database::connect(&database_url).await?;
Migrator::up(&connection, None).await?;


Idea

todo:
[ ] Write mermaid

```mermaid
---
Title: Structure of saving message
  
classDiagram
    Users : +string uuid
    Users: +string username
    
    ---
title: Animal example
---
classDiagram
    note "From Duck till Zebra"
    Animal <|-- Duck
    note for Duck "can fly\ncan swim\ncan dive\ncan help in debugging"
    Animal <|-- Fish
    Animal <|-- Zebra
    Animal : +int age
    Animal : +String gender
    Animal: +isMammal()
    Animal: +mate()
    class Duck{
        +String beakColor
        +swim()
        +quack()
    }
    class Fish{
        -int sizeInFeet
        -canEat()
    }
    class Zebra{
        +bool is_wild
        +run()
    }
    
        



```


- Users
  - username

- Connection table
  - Users and room

- Rooms
  - id
  - meta data

- Message
  - id
  - room id -> which room