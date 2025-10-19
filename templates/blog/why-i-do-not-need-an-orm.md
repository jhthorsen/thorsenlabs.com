---
title: Why I don't need an ORM
date: 2024-10-30
---

Once upon a time, I thought ORM's was the meaning of life. I bought into the
idea that it was much better to leave generating the SQL to a library that
was smarter than me. And also, of course when you change between SQLite,
Postgres and MySQL *every* week, then you would be crazy to maintain your own
SQL right?

Wrong.

Here are two reasons why you might choose one database over another:

1. You don't have a choice: Your workplace has already made a decision, and you're with it.
2. One database has some must have features. (And then you're stuck with it)

There's simply no reason to write SQL that doesn't leverage the database you
are using. It's just silly to loose out on functionality, just because
you want to be compatible with every database that you *won't* use in the
future.

Even though I haven't been on the "ORM train" for a long time, I used to be a
huge fan of SQL builders. The reason was that I thought it was a hassle to map
optional query parameters from ex. an URL to `where` statement. But not any
more. Look at this example:

```sql
select content from blog where (length($1) < 1 or author = $2);
```

The trick is to set `$1` and `$2` to the *same* value, and by combining two
statements (A and B) with an `or`, then the database can ignore certain parts of
the `where`:

| Input value      | A                           | B               |
|------------------|-----------------------------|-----------------|
| $1 = $2 = ''     | length('') < 1 == true      | Ignored         |
| $1 = $2 = 'jane' | length('jane') < 1 == false | author = 'jane' |

I think it's wonderful that the SQL above is always the same, and very
predictable.

Here is another example, which uses different logic in the part before the `or`
to address more complex cases:

```sql
select first_name, last_name from person
where ($1 is null or first_name = $2)
  and (coalesce($3, '') not regexp '@' or email = $4)
  and (length(coalesce($5, '') < 1 or address like $6);
```

| Input value          | A                                   | B                   |
|----------------------|-------------------------------------|---------------------|
| $1 = $2 = null       | $1 is null == true                  | Ignored             |
| $1 = $2 = ''         | $1 is null == false                 | first_name = ''     |
| $1 = $2 = 'Jane'     | $1 is null == false                 | first_name = 'Jane' |
| $3 = $4 = null       | '' not regexp '@' == true           | Ignored             |
| $3 = $4 = 'jane'     | 'jane' not regexp '@' == true       | Ignored             |
| $3 = $4 = 'a@b.com'  | 'a@b.com' not regexp '@' == false   | email = 'a@b.com'   |
| $5 = null, $6 = '%%' | length('') < 1 == true              | Ignored             |
| $5 = '', $6 = '%%'   | length('') < 1 == true              | Ignored             |
| $5 = 'a', $6 = '%a%' | length('a') < 1 == false            | address like '%a%'  |

I think this is way easier to reason with, since I can *see* the actual SQL
right on the screen, instead of guessing what an ORM might do. It also doesn't
get any easier guessing what the ORM might do, if it includes an inner select
or a join.

Next is the actual "Object Relational Mapping": Is this part really that
difficult? I think that freeing your objects from the database schema also
makes it easier to model your objects, since they are not restricted to how the
database is layed out. Here is a simple example for how you could map a row
from the database to an object:

```javascript
// This code is not meant to be production ready, but rather illustrate how it could work
function getUserFromDatabase(username) {
  row = db.query('select name, timestampdiff(year, birthdate, curdate()) as age from person where username = ?', username);
  return new User({age: row.age, name: row.name, username: username});
}

user = getUserFromDatabase('jane');
```

By hiding the SQL inside a function, it is protected from the user of your
library, and not spread out all over their codebase. At the same time, it
uses plain SQL and does not hide any logic away.

Here is how you could also delete the user:

```javascript
function deleteFromDatabase(user) {
  db.query('delete from person where username = ?', user.username);
}
```

In other cases you might just want to retrieve some primitives, which doesn't
really map to an object at all. Example:

```javascript
function howManyUsers() {
  row = db.query('select count(*) as n from person');
  return row.n;
}
```

I can understand that using an ORM might be compelling, since you don't have to
learn SQL, but SQL isn't really that difficult. At least not for most of the
usecases that an ORM can help you with.

Two bonuses by using the pattern above, is that you can easily copy/paste a SQL
statement that you tested in your database into your code. Second: You can very
quickly copy an SQL statement from one programming language into another,
instead of learning how to use the ORM in language A, and then translate that
into the ORM used by language B.

Last a small disclaimer: I haven't benchmarked how this query pattern performs
with and without the extra `or` logic, so if anyone can generate a big enough
dataset and see if there's really a difference, then I would love to hear about
that.
