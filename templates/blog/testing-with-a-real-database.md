---
title: Testing with a real database
date: 2015-09-20
---

Writing tests against a database is a struggle. Maybe the best way is to
not test against a database at all, but providing a mocked interface
instead. Even if this could be true in many cases, you still need to
write integration tests for your models against a real database.

This post digs into various setups you can choose from, and explains why
one of the solutions was chosen as the specification for a new module
called [DBIx::TempDB](https://metacpan.org/pod/DBIx::TempDB).

The term "unit test" will be used on some occasions, even many people
would argue that anything else than a mocked API to your database are
indeed an "integration test". This post will not argue against this, but
the important thing is not using the right words, but that you actually
test your code.

## The goal

The goal is to be able to test the library you're writing actually can
run the SQL in the database and retrieve/insert/update a given set of
data. This is trivial if...

-   you have your own database.
-   you only run one test at a time.
-   you are in control of the schema.
-   you are in control of the initial state.

The challenge comes when...

-   you want to be able to run the tests against a shared database.
-   you want to run the test suite in parallel (`prove -l -j8`)
-   you want to test against a custom schema.
-   you want to be able to reset the database on each run.

## Solutions

### In memory database

For many years I have been writing code that should work against MySQL
and/or PostgresSQL, but running my unit tests against a SQLite database.
The reason for this is that SQLite has this cool feature where you can
create an in-memory database which then will be cleaned up automatically
when the process exit.

The problem is just that writing code that execute Pg/MySQL SQL against
SQLite isn't trivial and often require hacks such as:

```perl
return $NOT_REALLY_MY_DB ? "select 'some sqlite query'" : "select 'some pg query'";
```

Running the test suite with
[Devel::Cover](https://metacpan.org/pod/Devel::Cover) will then reveal
many branches that are not tested, which means that your code isn't
really tested at all.

When that is said: Using this solution when your application is supposed
to use a SQLite backend is a very nice solution. Or... Not really: This
won't work if you run code that forks or makes multiple database
handles.

Dan Book has solved this in
[Mojo::SQLite](https://metacpan.org/pod/Mojo::SQLite) by adding support
for the ":temp:" database which creates a temp file on disk. This is
probably a lot more robust.

### Using a transaction

Running tests inside a transaction isolates the queries, leaving the
initial state untouched. This can work nicely for very small and
specific tests, but it has a number of issues:

-   Many databases (such as MySQL) does not support nested transactions.
    This means that the code you're testing cannot start or commit a
    transaction.
-   It won't work if your code use
    [Mojo::Pg](https://metacpan.org/pod/Mojo::Pg) or another module
    which use a connection pool. The issue is that any other connection
    won't see the changes done inside a transaction.
-   A transaction might lock up the database, making other concurrent
    tests sit and wait, which slows down the whole run time of the test
    suite.

This solution is probably the worst. The reason is that the isolation
level is very sensitive to changes in the code, and tests might start
failing without any apparent reason.

### Personal database server

There are several modules that allow you to spin up a test server for
each test. Modules such as
[Test::mysqld](https://metacpan.org/pod/Test::mysqld) works nicely, but
the problem is that it takes seconds for *each* test to start because of
the overhead of starting the server. This is no good, since a test suite
that takes too long to run is a test suite that won't be run!

For Redis, you can use
[Mojo::Redis2::Server](https://metacpan.org/pod/Mojo::Redis2::Server)
which works since the Redis server starts almost instantly.

A personal server could be the only way to go though, since some shared
servers will not give you enough rights to set up your own test
databases. Also, this is probably required if you want to test cases
such as when a database unexpectedly shuts down.

### Shared database server

A shared database avoids the start up time of a personal database
server, but provides pretty much the same functionality.

The caveat of this solution is, however, that you need to configure the
server with enough permission for each user to create their own
database. This means that this should probably be a custom server, not
shared with any other environment. This should make perfect sense, that
you don't run your tests against the same server as the one used in for
example a production environment.

To be able to run tests in parallel and concurrently with other users,
the module had to be able to generate databases on the fly. It also had
to be able to drop/delete the databases after a test run automatically,
to avoid filling up the server. The result of this specification is a
module called [DBIx::TempDB](https://metacpan.org/pod/DBIx::TempDB).

This module will drop/delete the temporary database when the process
exits. This is done in various ways, which can be configured by the
user: The default is to drop/delete the database when the object goes
out of scope. This is good enough for many cases, but since a test
script might end in various ways there are two other options:

-   Create a child process with a pipe to the parent process. When the
    pipe is closed, the child process will drop/delete the temporary
    database.
-   Create a child process, detached from the parent. This process will
    check (using kill) if the parent is still running, and drop/delete
    the temporary database when the \$parent is no longer responding to
    `kill 0, $ppid`.

It is also possible to disable all of this by setting the environment
variable `DBIX_TEMP_DB_KEEP_DATABASE` to a true value. This require you
to drop/delete the database manually, but it allows you to inspect the
data with standard tools for further investigation if a test fail.

## The end

I hope [DBIx::TempDB](https://metacpan.org/pod/DBIx::TempDB) will
encourage you to write more and better tests.

The module is currently marked as "EXPERIMENTAL". This will go away when
I have battle tested the code at work, but it can also go away if you
tell me that something is useful or awful. Please create an
[issue](https://github.com/jhthorsen/dbix-tempdb/issues), add a comment
below or send me an [email](mailto:jhthorsen@cpan.org) if you have any
feedback.

## References

-   [DBIx::TempDB](https://metacpan.org/pod/DBIx::TempDB)
-   [Project page](https://github.com/jhthorsen/dbix-tempdb)
