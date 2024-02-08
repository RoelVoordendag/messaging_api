## Todo:
- [ ] Create routes for sending messages
    - [ ] Handling the saving of the new items inside it
- [x] Setup database (POSTGRES) -> docker?
- [ ] Research what kind of cargos can I use to connect to a DB (Diesel)
- [ ] Do initial bootstrap for connection with DB.
- [ ] Setup socket connections
- [ ] During socket connection save messages
- [ ] Users?

use migration::{Migrator, MigratorTrait};

let connection = sea_orm::Database::connect(&database_url).await?;
Migrator::up(&connection, None).await?;
