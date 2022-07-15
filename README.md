# blog

For some time I've been wanting to write a blog. Sometime ago I had a Medium account and wrote a
few posts, however I don't like so much the idea of using somebody else's platform for that. And
don't get me wrong, online solutions such as Medium and Dev are probably a better idea to write a
blog, they have everything ready to go and their reliability is great. Using an online solution I
don't need to worry about anything else but write an article, but I don't like the idea of having
a paywall to write or read posts, and having more ads in the Internet, just to read something.

Back in the day everybody had their own blog, maybe is a good idea to go back to those early times
of the Internet. But most importantly, I want to write this software, even if its more work for me
in the long run, I want to write a Rust server and to figure out the architecture for the blog, as
it needs to have a client facing UI as well as an admin UI for managing the posts.

## Development

### Requirements

- Rust >= 1.61.0

### How to Run

Start by copying the `.env.example` and make a new `.env` file based on that. Then the DB generation
is possible. But maybe it's necessary to make a new empty file such as `blog.db` to be able to run
the migrations, which happen automatically when the server starts running.

```bash
cargo watch -x run -i schema.gql
```

Alternatively, the migrations are able to run without the server by running the following command

```bash
# To apply migrations
cargo run --package migration -- up

# To rollback a migration
cargo run --package migration -- down

# To rollball all migrations and run them again
cargo run --package migration -- refresh
```

### New migrations

Whenever a new migration is added according to the
[documentation](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration), the entities
must be updated. This project uses code generation to build the entities used in the code by
running the following code.

```bash
sea-orm-cli generate entity -o ./entity/src --with-serde
```

However, the [sea-orm-cli](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/) is not
perfect, there are some discrepancies that need to be fixed by hand, hence the need for the
[defaults.rs](./entity/src/defaults.rs) file.
