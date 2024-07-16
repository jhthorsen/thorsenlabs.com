---
title: How I write web applications
---

This blog post is about the standard components I use when I develop and
deploy my web applications.

## Server side components

### Mojolicious

When I make a web page or web application I use
[Mojolicious](https://mojolicious.org). Mojolicious is this awesome
realtime web framework, which makes web development a [walk in the
park](http://mojolicious.org/perldoc/Mojolicious/Guides/Tutorial). It
has a lot of functionality built in, allowing you to make any basic web
application without a single dependency.

Some of the built in
[features](http://mojolicious.org/perldoc#HIGHLIGHTS):

-   User agent for fetching remote documents.
-   JSON support for easy API development.
-   Built in daemons for easy development and high traffic production
    sites.
-   Templating system.
-   Extremely easy to write tests.

### AssetPack

[AssetPack](https://metacpan.org/pod/Mojolicious::Plugin::AssetPack#MANUALS)
is a plugin for Mojolicious which can be used to translate and minify
different languages into proper CSS or JavaScript on the client side.

The best part about AssetPack to me, is that it allows me to structure
the CSS/JavaScript in multiple files, just like I would with my Perl
code. Usually this can be very cumbersome to work with, but since
AssetPack understands the difference between development and production,
it will Do The Right Thing.

One example is if you like to use Sass to write CSS,
[morbo](https://metacpan.org/pod/Mojo::Server::Morbo) will pick up your
changes while developing and AssetPack will translate it into CSS, ready
to use on the next refresh. No additional tools need to be started from
the command line. In production, AssetPack doesn't even need to know how
to translate Sass: It will just use the last compiled asset.

For the end user, the benefits are less requests to the server, and the
minified assets will make the web page faster to download.

Update: I suggest checking out
[Mojolicious::Plugin::Webpack](https://metacpan.org/pod/Mojolicious::Plugin::Webpack)
instead.

### Swagger

[Swagger](http://swagger.io) is "The World's Most Popular Framework for
APIs". I've covered my [Swagger](https://metacpan.org/release/Swagger2)
module and why you want to use Swagger in [another blog
post](/blog/2015-07-05-mojolicious-swagger2)

The reason why I want to have a proper API to my web server is that it
makes it so much easier to develop nice JavaScript powered web
applications.

## Client side components

### jQuery

[jQuery](http://jquery.com) was the first library that made it really
fun for me write JavaScript. It used to be a struggle to make the JS
code work in different browsers, such as IE 4,5,6,7,8,9,..., Firefox,
Chrome and browser X, but with jQuery it Just Worked. These days I don't
use jQuery directly that much, but there are so many good plugins built
on top of jQuery that I often have it as a part of my toolbox.

Since browser support has gotten so mature, I try to use the native DOM
API as much as possible, but it's till a whole lot smoother to use
jQuery.

### Riot

[Riot](http://riotjs.com) is inspired by
[React](https://facebook.github.io/react). Both are a way of mixing
JavaScript and HTML to create re-usable web components. They have a
virtual DOM, which makes it lightning fast to figure out which element
on the web page to update.

From their web page:

> React worked well for us, and we still use it in our Disqus Importer
> but we were bothered by the size and syntax of React (especially the
> syntax). We started thinking it could be simpler; both internally and
> for the user.

I choose Riot, because it's lightweight, the syntax is easy to work with
and the one-way data binding, together the
[observable](http://riotjs.com/api/observable/) pattern makes the code
clean and easy to read.

### Sass

[Sass](http://sass-lang.com/) is one of many technologies I didn't know
why I wanted, before I started using it. But after I have started using
Sass, it is very hard to go back to vanilla CSS.

In addition to having reusable variables, mixins and operators, the the
biggest advantage to me is the ability to nest selectors.

### Materialize

I used to go with [Bootstrap](http://getbootstrap.com) and
[FontAwesome](http://fortawesome.github.io/Font-Awesome/), but now when
I want to make a web app I reach for
[Materialize](http://materializecss.com) instead. I simply like the
Google guidelines better and it makes the product feel more "appy".

Doing custom designs can be a lot of fun, but for a "backend hacker"
like me, (with limited design skills) it's a big help to have a
framework to build on top of.

There are other more slim solution, but if you go for the Sass/LESS
built versions you can strip it down to fit your needs.

## Deployment

### DigitalOcean

[DigitalOcean](https://www.digitalocean.com) is my favorite cloud
service. I started out with [Heroku](https://www.heroku.com), but I
think their pricing model is ridiculous. It's unfair to compare them
one-to-one, since heroku takes care of all the tedious work with
maintaining the base OS and security. Doing this properly can be hard,
but luckily I get help from [Ansible](https://galaxy.ansible.com). The
galaxy web page contains many
[playbooks](http://docs.ansible.com/ansible/playbooks.html) that solve
common problems.

### Cloudflare

[Cloudflare](https://cloudflare.com) is just the best CDN. This is a
subjective opinion based on reading their
[blog](https://blog.cloudflare.com). I just think the way they want to
improve the web is awesome! I also like their openness about the
technologies they choose and the problems they face.

Their service is also a breeze to work with. The web interface got a
proper face lift a couple of months back and they also have an
[API](https://api.cloudflare.com) if you want to automate actions: I
have set up a cronjob locally that will [maintain the
mapping](https://github.com/jhthorsen/mojo-cloudflare/blob/master/examples/maintain-a-records)
between my domain and the IP on my home server, since I don't have a
static IP.

### Toadfarm

[Toadfarm](https://metacpan.org/release/Toadfarm) is an application that
I use to run my Mojolicious applications. It started out as a small tool
for mounting multiple applications into one process to save memory, but
has grown into a DSL with a lot more functionality. My favorite is
probably that you can use it as an [init
script](https://metacpan.org/pod/distribution/Toadfarm/lib/Toadfarm/Manual/RunningToadfarm.pod#Init-script).

### Image resources

It makes a web page more alive if you have nice images. One design is
for example having a [big photo](http://thorsen.pm) at the top of the
page or a blended image in the background. If you need [free (do
whatever you want) high-resolution photos](https://unsplash.com), you
can search for them on
[Google](https://www.google.no/search?q=unsplash+summer+people&tbm=isch&tbs=isz:l).

## The end

I skipped databases on purpose, since I don't have a single database I
use: I use PostgreSQL, SQLite, Redis, or any other backend that makes
sense for a given problem. Even flat files.

What are your basic building blocks? Would be nice to hear about them in
the comments below. You're also more than welcome to drop in a question
if you wonder if there's a common way to solve more specific problems.

## References

-   [jQuery](http://jquery.com)
-   [Mojolicious](http://mojolicious.org)
-   [Materialize](http://materializecss.com)
-   [Riot](http://riotjs.com)
-   [Sass](http://sass-lang.com)
-   [Swagger](http://swagger.io)
-   [Toadfarm](https://metacpan.org/pod/Toadfarm)
-   [Unsplash](https://unsplash.com)
