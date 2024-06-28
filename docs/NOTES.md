deisel is our ORM and requires our postgres server to have `uuid_generate_v4` enabled which can be done with `CREATE EXTENSION IF NOT EXISTS "uuid-ossp";`
