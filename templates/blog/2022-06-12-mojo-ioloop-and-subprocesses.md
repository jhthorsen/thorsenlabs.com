---
title: Mojo::IOLoop likes running programs asynchronously
---

Want to be able to run programs and arbitrary Perl code asynchronously,
while being able to read from STDOUT and STDERR, and also write to
STDIN? Try out [Mojo::Run3](https://metacpan.org/pod/Mojo::Run3)!

When working with Mojo::IOLoop, you might find yourself in a situation
where you want to run another program, but you also don't want to block
the IOLoop from doing other asynchronously tasks. So what to do? You
could use
[Mojo::IOLoop::Subprocess](https://metacpan.org/pod/Mojo::IOLoop::Subprocess).
This module is part of Mojolicious core and works perfectly if you don't
care about I/O from the child process while it is running, but what to
do if you want to write to STDIN, or read STDOUT and STDERR in real-time
while the program is running?
[Mojo::Run3](https://metacpan.org/pod/Mojo::Run3) got your back.

## Example usage

```perl
use Mojo::Base -strict, -signatures;
use Mojo::Run3;

my $run3 = Mojo::Run3->new;
$run3->on(stdout => sub ($run3, $bytes) { print $bytes });
$run3->run_p(sub { exec qw(ls -l /) })->wait;
```

The above is an example on how to run the command "`ls -l /`" in a
subprosess, which does not block the main IOLoop, but instead reads
STDOUT asynchronously through an event. Another more complex example is
"[sshpass](https://www.redhat.com/sysadmin/ssh-automation-sshpass)"
implemented with about [80 lines of pure Perl
code](https://github.com/jhthorsen/mojo-run3/blob/main/examples/sshpass).
That example also uses the "pty"
[driver](https://metacpan.org/pod/Mojo::Run3#driver) instead of a plain
pipe to create a
[pseudoterminal](https://man7.org/linux/man-pages/man4/pts.4.html). This
is useful for interactive programs like ssh, bash or even editors like
vim.

## Using Mojo::Run3 in your Mojolicious web application

Even though you can now run any sort of programs asynchronously in your
web server now, you should consider using a job queue (such as
[Minion](https://docs.mojolicious.org/Minion)) for long running tasks.
The reason is that if the client aborts the request or the webserver is
restarted, then there's no guaranty that the process will finish
running, nor do you have any control that there's not multiple instances
of the same program running at the same time. But for interactive
terminals over a WebSocket, or other short lived commands you can
certainly use [Mojo::Run3](https://metacpan.org/pod/Mojo::Run3).

## What about Mojo::IOLoop::ReadWriteFork?

I have a competing module on CPAN called
[Mojo::IOLoop::ReadWriteFork](https://metacpan.org/pod/Mojo::IOLoop::ReadWriteFork).
Unfortunately, this module was not originally designed with multiple
input and output filehandles in mind, so the internals have gotten a bit
messy over the years. Since I was unable to fix those issues without
breaking compatibility, I decided to make a new module with fresh
internals.

Enjoy a fresh, async day!
