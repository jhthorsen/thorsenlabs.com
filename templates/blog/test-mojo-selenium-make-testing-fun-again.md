---
title: Browser testing is hard. Let's go shopping
date: 2017-05-03
---

Tonight at the monthly Oslo.pm
[meetup](https://www.meetup.com/Oslo-pm/events/239533530/), I gave a
talk about how to test your web application in a real browser. This blog
entry is a collection of the steps I went through while running live
demos showcasing my module
[Test::Mojo::Role::Selenium](https://metacpan.org/pod/Test::Mojo::Role::Selenium).

My module uses
[Selenium::Remote::Driver](https://metacpan.org/pod/Selenium::Remote::Driver)
which is a library that communicates with desktop browsers, such as
Google Chrome, Firefox or even Internet Explorer. It comes bundled with
test modules, but I wanted an interface that looks and feels like
Mojolicious's test module
[Test::Mojo](https://metacpan.org/pod/Test::Mojo), since that module
really makes testing fun.

Even if the module is a
[Test::Mojo](https://metacpan.org/pod/Test::Mojo) role, it is not
restricted to the [Mojolicious](http://mojolicious.org/) web framework.
The module can test any web site, as long as you have a web server
running.

## Why do you want to use this module?

Testing the HTTP messages for headers and body is great, but as soon as
you make something other than an API or very simple web page, you should
also test the user experience of the web app. Testing the JavaScript for
dynamic web pages is the first that comes to mind, but
[responsive](https://developers.google.com/web/fundamentals/design-and-ui/responsive/)
web pages also need testing, to see how elements are laid out on
different screen sizes.

[Test::Mojo::Role::Selenium](https://metacpan.org/pod/Test::Mojo::Role::Selenium#SYNOPSIS)
allows you to write and run user experience tests in the browser of your
[selection](https://metacpan.org/pod/Selenium::Remote::Driver#USAGE).
The syntax is very simple and it has sane defaults to avoid boilerplate
in each test.

## Prerequisites

To get started, you need to install the module and some executables that
act as a glue between your test script and the browser of your
selection.

```
# Install the test module
$ cpanm Test::Mojo::Role::Selenium

# Install Google Chrome, Firefox and PhantomJS drivers
$ brew install chromedriver
$ brew install geckodriver
$ brew install phantomjs

# Install the Selenium driver (requires jdk8)
# brew install selenium-server-standalone
```

There are probably similar packages for your favorite operating system.
The `brew` commands above are simply a cheat sheet for the presentation.

While preparing this article I wanted to get the demos running with
Firefox, but I was not able to get the
[Selenium::Firefox](https://metacpan.org/pod/Selenium::Firefox) module
to work together with the `geckodriver` executable. Seems like the
integration between `firefox` and `geckodriver` are under heavy
development. Please let me know in the comments area below if I'm wrong.
I did however manage to get Firefox running using the `selenium-server`.
(I had to install Java though...)

You can change between the different backends using the
[MOJO_SELENIUM_DRIVER](https://metacpan.org/pod/Test::Mojo::Role::Selenium#MOJO_SELENIUM_DRIVER).
environment variable. Note that
[Selenium::Chrome](https://metacpan.org/pod/Selenium::Chrome) and
[Selenium::Firefox](https://metacpan.org/pod/Selenium::Firefox) will
start and stop the browser together with the test script, while
[Selenium::Remote::Driver](https://metacpan.org/pod/Selenium::Remote::Driver)
(which uses `selenium-server`) require an external Selenium service to
run.

## Testing against a live web server

The first demo was to show that you can use the module to test any web
site. The test [mojolicious.t](#tmojolicioust) connects to
[mojolicious.org](http://mojolicious.org/), checks for certain elements
and fills in the search form, runs some JavaScript commands and then
checks if the search result page was loaded.

Here are the commands I went through to run the demo:

```
$ mkdir -p test-selenium/t
$ cd test-selenium
$ vim t/mojolicious.t
# copy/paste from t/mojolicious.t below

# Test with Google chrome
$ TEST_SELENIUM=http://mojolicious.org prove -vl t/mojolicious.t

# Test with Firefox
$ MOJO_SELENIUM_DRIVER=Selenium::Firefox \
  TEST_SELENIUM=http://mojolicious.org \
  prove -vl t/mojolicious.t

# or...
$ MOJO_SELENIUM_DRIVER=Selenium::Remote::Driver \
  TEST_SELENIUM=http://mojolicious.org \
  prove -vl t/mojolicious.t
```

The commands above should run the test script in various browser and
result in a successful test run. Note that the environment variable
`TEST_SELENIUM` need to be set, or the tests will be skipped. The reason
for this is that I think in most cases the Selenium tests should not be
run when installing a cpan module, nor being run on services such as
[Travis CI](https://travis-ci.org/).

## Testing against a local Mojolicious application

The next demo, [internal.t](#tinternalt), run tests agains a Mojolicious
application. Using a Mojo app gives you some more features: In addition
to test what is shown inside the browser, you can test headers and other
"hidden" information that is exchanged over the HTTP protocol.

```
$ vim t/internal.t
# copy/paste from t/internal.t below

# Test with PhantomJS
$ TEST_SELENIUM=1 prove -vl t/internal.t
```

Since the test script does not set
[MOJO_SELENIUM_DRIVER](https://metacpan.org/pod/Test::Mojo::Role::Selenium#MOJO_SELENIUM_DRIVER),
it will use the default browser which is the headless browser
[PhantomJS](http://phantomjs.org/). This browser is quite fast to start
up, but might miss some features that is only available in Chrome,
Firefox or Internet Explorer.

The headless version also takes screenshots which are saved to the
operating system's temp directory. This can be changed by specifying
[screenshot_directory](https://metacpan.org/pod/Test::Mojo::Role::Selenium#screenshot_directory).

## A more complex real life example

The last demo was to look at some tests for the
[Convos](https://convos.by/) chat web application. The web app is a
Mojolicious server that allows you to be persistently conected to IRC
servers and communicate with other IRC users through your web browser.
The frontend is powered by Vuejs, which is a reactive JavaScript library
that can only be tested through the web browser.

The [test suite](https://github.com/Nordaaker/convos/tree/master/t)
feature many browser tests, (Look for the tests starting with
`selenium-`) but the two tests that was demoed was `selenium-url.t` and
`selenium-register.t`.

### selenium-url.t

[selenium-url.t](https://github.com/Nordaaker/convos/blob/master/t/selenium-url.t)
simply tests the URL library
[url.js](https://github.com/Nordaaker/convos/blob/master/assets/js/url.js)
which is a URL parser and generator.

It does that by calling `$t->driver->execute_script(...)` which is a
[Selenium::Remote::Driver](https://metacpan.org/pod/Selenium::Remote::Driver#execute_script)
method for running JavaScript code inside the browser. The result from
the method is then tested with normal
[Test::More](https://metacpan.org/pod/Test::More) functions.

### selenium-register.t

[selenium-register.t](https://github.com/Nordaaker/convos/blob/master/t/selenium-register.t)
is a bit more complicated test that uses more features from
[Test::Mojo::Role::Selenium](https://metacpan.org/pod/Test::Mojo::Role::Selenium).

It uses
[wait_for](https://metacpan.org/pod/Test::Mojo::Role::Selenium#wait_for)
to wait for elements that are injected dynamically to the document.
`wait_for()` is a simple version of the more complex
[wait_until](https://metacpan.org/pod/Test::Mojo::Role::Selenium#wait_until)
method that runs a function until the function returns a true value or a
timeout runs out.

## The end

I hope this introduction gave you an idea of what
[Test::Mojo::Role::Selenium](https://metacpan.org/pod/Test::Mojo::Role::Selenium)
can do, and makes testing fun again.

## Resources

### Links

-   [Test::Mojo::Role::Selenium](https://metacpan.org/pod/Test::Mojo::Role::Selenium):
    API documentation.
-   [Selenium::Remote::Driver](https://metacpan.org/pod/Selenium::Remote::Driver):
    Holds information about what you can do with [the
    driver](https://metacpan.org/pod/Test::Mojo::Role::Selenium#driver).
-   [Selenium::Remote::WebElement](https://metacpan.org/pod/Selenium::Remote::WebElement):
    Holds information about what you can do with an element.
-   [Selenium::Remote::WDKeys](https://metacpan.org/pod/Selenium::Remote::WDKeys):
    Check the source code for actual keys you can send to html elements.

### t/mojolicious.t

```perl
use Mojo::Base -strict;
use Test::Mojo::WithRoles "Selenium";
use Test::More;

$ENV{MOJO_SELENIUM_DRIVER} ||= 'Selenium::Chrome';

my $t = Test::Mojo::WithRoles->new->setup_or_skip_all;

$t->set_window_size([1024, 768]);

$t->navigate_ok('/perldoc');
$t->current_url_is("http://mojolicious.org/perldoc");
$t->live_text_is('a[href="#GUIDES"]' => 'GUIDES');

$t->driver->execute_script(qq[document.querySelector("form").removeAttribute("target")]);
$t->element_is_displayed("input[name=q]")
  ->send_keys_ok("input[name=q]", ["render", \"return"]);

$t->wait_until(sub { $_->get_current_url =~ qr{q=render} });
$t->live_value_is("input[name=search]", "render");

done_testing;
```

### t/internal.t

```perl
use Mojo::Base -strict;
use Test::Mojo::WithRoles "Selenium";
use Test::More;

use Mojolicious::Lite;
get "/home" => "index";

my $t = Test::Mojo::WithRoles->new->setup_or_skip_all;

$t->navigate_ok("/home")
  ->status_is(200)
  ->capture_screenshot
  ->header_is("Server" => "Mojolicious (Perl)")
  ->text_is("p" => "Hello!")
  ->live_text_is("p" => "Hello!")
  ->element_is_displayed("input")
  ->active_element_is("input[name=test]")
  ->send_keys_ok("input[name=test]", ["Yikes", \"enter"]);

$t->current_url_like(qr{/home\?test=Yikes})
  ->status_is(200)
  ->capture_screenshot
  ->live_element_exists("input[name=test][value=Yikes]");

done_testing;

__DATA__
@@ index.html.ep
<!DOCTYPE html>
<html>
<head>
  <title>Test</title>
</head>
<body>
  <p>Hello!</p>
  %= form_for "index", begin
    %= text_field 'test', autofocus => 1
    %= submit_button
  % end
</body>
</html>
```
