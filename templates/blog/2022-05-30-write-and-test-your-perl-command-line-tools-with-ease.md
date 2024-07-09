---
title: Write and test Perl command line tools with ease
---

Writing command line tools often involves a lot of boilerplate and they
can be hard to test. [Getopt::App](https://metacpan.org/pod/Getopt::App)
is a module that helps you structure your command line applications
without getting in the way.

I already have a competing module called
[Applify](https://metacpan.org/pod/Applify) on CPAN. This module aims to
solve the same problems as Getopt::App, but does so in a very different
way. [Getopt::App](https://metacpan.org/pod/Getopt::App) is much simpler
(about half the code), but has more or less the same features as
Applify:

-   [Getopt::Long](https://metacpan.org/pod/Getopt::Long) integration:
    Applify provides a function, while Getopt::App uses `Getopt::Long`
    syntax. This means that if you already know `Getopt::Long` then you
    don't have to learn a different syntax.
-   Applify allows for one level of subcommands, while Getopt::App can
    handle an infinite number of subcommands. Getopt::App can also
    isolate the different subcommands in separate files, which makes the
    main script easier to read.
-   [Pod::Usage](https://metacpan.org/pod/Pod::Usage) functionality:
    Applify provides a function that can be used to describe where to
    read documentation from, while Getopt::App provides a function that
    reads documentation and returns it as a string that you can pass on
    to print. This makes Getopt::App much more flexible since you can
    manipulate the help text before printing it. Example: Replace the
    string "APPLICATION" with `basename($0)`.
-   Applify has automatic parsing of `--version`, which Getopt::App does
    not have. It is very easy to add to your Getopt::App powered script
    though -- See below for an example.
-   Both modules provide
    [hooks](https://metacpan.org/pod/Getopt::App#APPLICATION-METHODS)
    and allow for
    [inheritance](https://metacpan.org/pod/Getopt::App#import), but
    Getopt::App implement both in a much less invasive way.

If you still like Applify more than Getopt::App and want to maintain it,
then please let me know and I'll gladly give you commit bits.

## Example script {#example-script .wp-block-heading}

``` wp-block-code
#!/usr/bin/env perl
package My::Script;
use Getopt::App -signatures;

sub name ($app) { $app->{name} // 'no name' }

run(
  'name=s  # Specify a name',
  'v+      # Verbose output',
  'h|help  # Output help',
  'version # Print application version',
  sub ($app, @extra) {
    return print extract_usage()    if $app->{h};
    return say "example.pl v1.00\n" if $app->{version};
    say $app->name;
    return 0;
  }
);
```

The "package" statement is optional for simple scripts, but *required*
for subcommands to prevent method collision. Even though it's optional,
it's highly suggested. After defining your package, you have to use the
`Getopt::App` module which can take [optional
flags](https://metacpan.org/pod/Getopt::App#import). Even with no flags,
the module will import `strict`, `warnings`, and a bunch of other useful
features. The last statement in the script must be the `run()` function:
This function will understand if you source the script in a unit test or
running it form the command line. When sourcing the script, no code will
actually be run before you explicitly want it to.

## Example test {#example-test .wp-block-heading}

``` wp-block-code
#!/usr/bin/env perl
use Getopt::App -capture;
use File::Spec::Functions qw(catfile rel2abs);
use Test::More;

my $app = do(rel2abs(catfile qw(script example.pl)));
my $res = capture($app, [qw(--help)]);
like $res->[0], qr{Usage:}, 'stdout';
is   $res->[1], '',         'stderr';
is   $res->[2], 0,          'exit code';

done_testing;
```

Importing Getopt::App with the `-capture` flag will export the
[`capture()`](https://metacpan.org/pod/Getopt::App#capture) utility
function which can be used to run the application and capture STDOUT,
STDERR and the exit code. `capture()` takes two arguments: The first is
the sourced application and the second is the command line arguments as
an array-ref. The test above will call the code block provided to
`run()` above, but since the whole package is sourced, there is nothing
wrong with calling methods directly:

``` wp-block-code
#!/usr/bin/env perl
use Getopt::App -capture;
use File::Spec::Functions qw(catfile rel2abs);
use Test::More;

my $app = do(rel2abs(catfile qw(script example.pl)));
my $obj = My::Script->new;
is $obj->name, 'no name', 'default name';

done_testing
```

## Hooks, customization and subcommands {#hooks-customization-and-subcommands .wp-block-heading}

If you don't like the defaults set up by Getopt::App, then there are
many [hooks](https://metacpan.org/pod/Getopt::App#APPLICATION-METHODS)
to customize it to your liking. Each hook must be defined as a method
inside your script. To prevent naming collisions, the hook methods are
prefixed with "**getopt\_**". Here is an example:

``` wp-block-code
#!/usr/bin/env perl
package My::Script;
use Getopt::App -signatures;

sub getopt_configure ($app) {
  return qw(default no_auto_abbrev no_ignore_case);
}

sub getopt_pre_process_argv ($app, $argv) {
  push @$argv, 'man' unless @$argv;
}

sub getopt_subcommands ($app) {
  return [
    ['bar', '/path/to/bar.pl', 'Bar help text'],
    ['foo', '/path/to/foo.pl', 'Foo help text'],
    ['man', '/path/to/man.pl', 'Show manual'],
  ];
}

sub getopt_unknown_subcommand ($app, $argv) {
  die "Not cool.\n";
}

run(sub { print extract_usage() });
```

## Bundling {#bundling .wp-block-heading}

I often want to write scripts that can be easily downloaded and run by
others. Depending on `Applify` will add an extra hurdle that your users
have to jump through to be able to run the script. **Getopt::App** on
the other hand can easily be bundled with your script. To do so, simply
call the [`bundle()`](https://metacpan.org/pod/Getopt::App#bundle)
method from a oneliner:

``` wp-block-code
$ perl -MGetopt::App -e'Getopt::App->bundle(shift)' src/myscript.pl > script/myscript
```

The output `script/myscript` application now has Getopt::App inline!

## Conclusion {#conclusion .wp-block-heading}

I hope this introduction to Getopt::App gave you some ideas about how
easy a script can be better structured and tested with functions like
`capture()`. If you want to see more examples then please have a look at
the
[examples](https://github.com/jhthorsen/getopt-app/tree/main/example)
and [test suite](https://github.com/jhthorsen/getopt-app/tree/main/t).

Have a nice day!
