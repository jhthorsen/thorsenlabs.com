---
title: @swaggerapi and #Mojolicious -- How to validate your input/output with a schema
---

## OUTDATED

**This blog post is outdated. You might want to check out [A RESTful API
with
OpenAPI](https://mojolicious.io/blog/2017/12/22/day-22-how-to-build-a-public-rest-api/)
instead.**

------------------------------------------------------------------------

[Mojolicious](http://mojolicious.org) is an awesome web framework which
allow you concentrate on the business logic, while the rest Just Works
™.

[Swagger](http://swagger.io/) is "The World's Most Popular Framework for
APIs". Swagger is a language for specifying the input and output to your
HTTP API. (REST- or RPC API, if you like). The API rules are based on
top of the [JSON schema](http://json-schema.org/documentation.html)
rules, but extends beyond basic data validation and allows you to define
a complete API spec.

[Mojolicious::Plugin::Swagger2](https://metacpan.org/pod/Mojolicious::Plugin::Swagger2)
is a plugin for Mojolicious which ties the web framework together with
the API specification and automatically builds input/output validation
rules.

This tutorial show how to build a working blog app, based on
[kraih](https://twitter.com/kraih)'s [Mojo::Pg blog
example](https://github.com/kraih/mojo-pg/tree/master/examples/blog).

## Why you want to use swagger

This question comes up quite often after telling people about Swagger:
"but...why??" The people asking this often come from the same background
as myself, where you both write the producer (backend web server) and
consumer (javascript, ...) code. When you're in complete control of both
sides you don't really need to write any formal specification or
document your API, since you already *know* how it works. This can be
very true -- at least if you make sure you have tests of all your
endpoints.

Personally I'm a [huge
fan](http://www.linkognito.com/images/05-08-08/metal.gif) of documenting
as well as testing. I often say that if you can't document (describe)
the API/method/whatever, something is wrong and you should reconsider
what you're coding. Documenting an API on the other hand is awful IMO,
especially during rapid prototyping/developing where it's hard to keep
the code and documentation up to date.

So how does
[swagger](https://metacpan.org/pod/Mojolicious::Plugin::Swagger2) fix
this? Since the input/ouput Perl code is generated *from* the swagger
document, you know that the backend is always running code accordingly
to the specification. Also, since the documentation you generate is not
hand written, but generated from the same swagger document you can know
that the code the server is running is in sync with the documentation.

When "generated code" is mentioned, it's not just the routing, but also
input and output validation. This means that when any data has made it
through to your controller action, you know that the data is valid. On
the other side, the consumer (let's say a javascript that cares about
the difference between an integer and string) will know that it has
received the correct data, since invalid output will result in a
[500](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes#5xx_Server_Error).

So... If you don't care about documenation or collaberation with others,
then I'm not sure if I would care much about swagger either.

Note that the swagger spec is not just for the server, but can also be
used to generate [javascript](int://github.com/swagger-api/swagger-js)
and [perl client side](https://metacpan.org/pod/Swagger2::Client) code.

## Swagger specification

This first thing you need to do is to design you API. I'm not going into
the test-driven-, design-driven- or whatever-driven-development
discussion, but for this to work you need to design you API. The reason
for this is that the design/documentation process will define the rules
which again generates in-memory Perl code used to validate the
input/output of your Mojolicious application.

[This
specification](https://mojolicious.io/blog/2017/12/22/day-22-how-to-build-a-public-rest-api/)
is written in JSON, but you can use YAML instead if you have a [YAML
parser](https://metacpan.org/pod/YAML::XS) installed.

If you're new to the whole Swagger concept, I suggest start looking at
the resources under "[See also](#see-also)".

There's a lot of details in the specification, which is better
exaplained in the [official
documentation](http://swagger.io/specification/) but there are some
important parts worth mentioning:

-   `schemes` are ignored by the code generator. This means that even
    though your specification says "https", the API can still be served
    over plain http if that is what the server is accepting. Have a look
    at
    [DEPLOYMENT](https://metacpan.org/pod/distribution/Mojolicious/lib/Mojolicious/Guides/Cookbook.pod#DEPLOYMENT)
    on how to set up a secure server with TLS.

-   All the `paths` are mounted under `basePath` in the specification.

-   `x-mojo-controller` is special attribute under each HTTP method
    which tells `Mojolicious::Plugin::Swagger2` which controller to
    dispatch the incoming request to.

-   `operationId` is used to find the method in the `x-mojo-controller`
    controller. Any camel-case operationId will be normalized. Example:

        .----------------------------------.
        | Operation Id | Controller method |
        |--------------|-------------------|
        | userLogin    | user_login        |
        | UserLogin    | user_login        |
        | user_login   | user_login        |
        '----------------------------------'

## The application

The example application is a fully working
[blog](https://mojolicious.io/blog/2017/12/22/day-22-how-to-build-a-public-rest-api/),
with a [PostgreSQL](http://www.postgresql.org/) backend.

If you have a PostgreSQL server running and Mojolicious installed, you
can start the application with these simple steps:

```bash
git clone https://github.com/jhthorsen/swagger2.git
cd swagger2/t/blog/
BLOG_PG_URL=postgresql://postgres@/test perl script/blog daemon
```

To see all the routes generated, you can run:

```bash
> BLOG_PG_URL=postgresql://postgres@/test perl script/blog routes
/api
  +/posts        GET     "index"
  +/posts        POST    "store"
  +/posts/(:id)  PUT     "update"
  +/posts/(:id)  GET     "show"
  +/posts/(:id)  DELETE  "remove"
...

In addition to the manually added routes, Mojolicious::Plugin::Swagger2
has added five more routes, which are automatically generated from the
[API
spec](https://mojolicious.io/blog/2017/12/22/day-22-how-to-build-a-public-rest-api/).

The generated routes differ from the standard routes, since they point
to a generated callback where the input and output validation is done.
This means that when a request hit `http://example.com/api/posts/42`, it
will go through these steps:

1.  Check input against the swagger spec. Render a 400 error document if
    input validation fail.
2.  Call the
    [method](https://metacpan.org/pod/Mojolicious::Plugin::Swagger2#Controller)
    (operationId) in the specified `x-mojo-controller`, but with two
    extra parameters: `$args` and `$cb`. `$args` is the validated input,
    and `$cb` is a callback used to pass the response back to the user
    agent.
3.  Verify the response passed to `$cb` against the swagger spec. Render
    a 500 error document if the output validation fail.

These steps make sure that the input/output of your application will
always follow the specification.

Below is an [example
action](https://mojolicious.io/blog/2017/12/22/day-22-how-to-build-a-public-rest-api/)
which will be called when a "GET" request with the path part
"/api/posts/42" hit your application:

```perl
package Blog::Controller::Posts; # "x-mojo-controller"

sub show { # "operationId"

  # This method is called with $args and $cb,
  # in addition to the controller object ($self)
  my ($self, $args, $cb) = @_;

  # Find post with id 42 (from the request URL)
  my $entry = $self->posts->find($args->{id});

  # Serialize the blog post as JSON if found
  return $self->$cb($entry, 200) if $entry;

  # Render a 404 error document if blog post was not found
  return $self->$cb({errors => [{message => 'Blog post not found.', path => '/id'}]}, 404);
}
```

`$args` above will only contain "id", since that is the only parameter
specified for this resource. The response passed on to `$cb` need to
match either the "200" or "default" response in the API spec.

Note that the "default" response defined in the
[specification](https://mojolicious.io/blog/2017/12/22/day-22-how-to-build-a-public-rest-api/)
match the default error generated by
[Mojolicious::Plugin::Swagger2](https://metacpan.org/pod/Mojolicious::Plugin::Swagger2#render_swagger).

## Authentication

It's possible to specify a [custom
route](https://metacpan.org/pod/Mojolicious::Plugin::Swagger2#Protected-API)
which does authentication:

```perl
$app->plugin(swagger2 => {
  url   => "...",
  route => $app->routes->under->to(cb => sub {
    my $c = shift;

    # Authenticated
    return 1 if $c->param('secret');

    # Not authenticated
    $c->render(
      status => 401,
      json   => {
        errors => [{message => 'Not authenticated', path => '/'}]
      }
    );
    return;
  });
});
```

## Mojolicious commands

The [Swagger2 distribution](https://metacpan.org/release/Swagger2) comes
with a Mojolicious [command
extension](https://metacpan.org/pod/Mojolicious::Command::swagger2)
which gives you these command line tools:

```bash
mojo swagger2 client path/to/spec.json <method> [args]
mojo swagger2 edit
mojo swagger2 edit path/to/spec.json --listen http://*:5000
mojo swagger2 pod path/to/spec.json
mojo swagger2 perldoc path/to/spec.json
mojo swagger2 validate path/to/spec.json
```

### mojo swagger2 client

The "client" command generates a [swagger
client](https://metacpan.org/pod/Swagger2::Client) and calls the swagger
ready server. The input specification file need to have `host`,
`basePath` and `schemes` defined for the client to just work.

Example usage:

```bash
mojo swagger2 client \
    https://raw.githubusercontent.com/jhthorsen/swagger2/master/t/blog/api.json \
    show id=42
```

### mojo swagger2 edit

The "edit" command starts a Mojolicious server where you can edit and
read the swagger specification in your browser. The browser component
uses localStorage, which automatically saves your changes locally even
if your browser crash.

Example usage:

```bash
mojo swagger2 edit \
  https://raw.githubusercontent.com/jhthorsen/swagger2/master/t/blog/api.json
```

And then visit <http://localhost:3000/> in your browser.

### mojo swagger2 pod

You can generate pod and read the specification as perldoc. This again
enables such thing as swagger-to-pdf:

```bash
sudo apt-get install pod2pdf
mojo swagger2 pod t/blog/api.json > blog.pod
pod2pdf blog.pod > blog.pdf
```

Just to read the documentation:

```bash
mojo swagger2 perldoc t/blog/api.json
```

### mojo swagger2 validate

The last command can be used to validate that the API spec against the
swagger specification:

```bash
mojo swagger2 validate t/blog/api.json
```

## The end

This tutorial should give you a simple understanding on how to add input
and output validation to your application, based on a Swagger spec.

Got questions or feedback? Contact me on
[twitter](http://twitter.com/jhthorsen), join the IRC channel #swagger
on irc.perl.org. or drop me an [email](mailto:jhthorsen@cpan.org).

## See also

-   [A RESTful API with
    OpenAPI](https://mojolicious.io/blog/2017/12/22/day-22-how-to-build-a-public-rest-api/)
-   [Swagger webpage](http://swagger.io/)
-   [Swagger overview](http://swagger.io/getting-started/)
-   [Swagger2 specification](http://swagger.io/specification/)
