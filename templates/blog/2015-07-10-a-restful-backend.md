---
title: Reasons for choosing Mojolicious
---

This blog post is a response to <http://perlmonks.org/?node_id=1133505>.
I tried to reply directly, but I got "access denied" for some reason, so
I decided to write a blog post about it instead.

The post on perlmonks was:

> Good day fellow monks,
>
> I am starting a new project, with a clean slate, this is a wonderful
> and rare delight.
>
> I need to create a back end web service and being a man of the times,
> I plan to use a REST style API.
>
> The back end needs to provide User Signup, User Authentication, Order
> creation, Payment, Job Acceptance and Geolocation of assets.
>
> There will be more, but that's some of the main features. I do not
> have to write it in Perl, but that is where I know my way the best, so
> I am minded to make it so.
>
> I have started looking at Mojolicious, but then thought I'd back up a
> step and ask for some advice.
>
> Is Perl still a good language to use, or am I a dinosaur? Is
> Mojolicious a good way to build a REST service What other frameworks
> would you folk recommend I look at Does anyone know of a nice open
> source project providing some set of the above features, that could
> get me started? Any other warnings or guidance from those with
> experience in REST Many thanks for your time
>
> R.

## Reason for choosing Mojolicious

I would go for Mojolicious. It's a very modern framework that keeps up
with the specifications of the web. To me, one of the killer features is
that it supports a non-blocking environment. This means that you can
spin up one process and serve thousands of requests simultaneously. When
that is said, non-blocking might be a bit alien in the beginning, so
Mojolicious support standard blocking programming as well.

Another great feature is the ecosystem: There's a bunch of [Mojolicious
extensions](https://metacpan.org/search?q=Mojolicious%3A%3APlugin) on
cpan, and there's also written quite a few [Mojo based
projects](https://metacpan.org/requires/distribution/Mojolicious) as
well. Here are the modules I use the most frequent:

-   <https://metacpan.org/pod/Mojo::Pg>
-   <https://metacpan.org/pod/Mojo::mysql>
-   <https://metacpan.org/pod/Mojolicious::Plugin::Webpack>
-   <https://metacpan.org/pod/Mojolicious::Plugin::OpenAPI>

## User Signup / authentication

There's a lot of authentication modules on CPAN, but here is a short
list, just to give an idea of some of the projects that exists:

-   <https://metacpan.org/pod/Mojolicious::Plugin::Authentication>
-   <https://metacpan.org/pod/Mojolicious::Plugin::Web::Auth>
-   <https://metacpan.org/pod/Mojolicious::Plugin::OAuth2>
-   Or just roll your own and use
    <https://metacpan.org/pod/Mojolicious::Plugin::Bcrypt> for passwords

## Order creation

I guess any any ORM or just Mojo::Pg / ::mysql directly will help you
out. Also, Mojolicious has an [awesome
validation](https://metacpan.org/pod/distribution/Mojolicious/lib/Mojolicious/Guides/Rendering.pod#Form-validation)
built in.

## Payment

You can use any of the payment modules on CPAN, but here are two
projects I've written:

-   <https://metacpan.org/pod/Mojolicious::Plugin::StripePayment>
-   <https://metacpan.org/pod/Mojolicious::Plugin::PayPal>

The good thing about both of those is that they are non-blocking from
the ground up. The bad thing is that they don't do more than what I
needed at the time. Any feedback is greatly appreciated.

I would also recommend going with <https://stripe.com/>. Their service
is very simple to work with.

## Job Acceptance

Not sure if you're talking about an object in your backend model or if
you're asking for a job queue... If the latter, Mojo has your back:
<https://metacpan.org/pod/Minion>

## Geolocation of assets

I think I would simply put my application behind
<https://www.cloudflare.com/> and use their geolocation feature. In
addition to being a CDN, they add headers with geo information, before
sending it to your backend.

## REST

The [Swagger](https://metacpan.org/pod/Swagger2) project and the
Mojolicious plugin makes it a breeze to create RESTful interfaces with
input/output validation. You can read more about it in my [blog
post](http://thorsen.pm/perl/programming/2015-07-05-mojolicious-swagger2).

## Projects

You could have a look at <https://github.com/mojoconf/MCT>. It does
OAuth2, payment and has it's own model to a postgres backend, using
Mojo::Pg.

## Summary

I think Mojolicious is a great web framework. If I would choose any
other, I would probably go for a node or go based framework.

I also don't think "R" is a dinosaur for choosing Perl. Perl is indeed
live and kicking and will be for many years ahead.
