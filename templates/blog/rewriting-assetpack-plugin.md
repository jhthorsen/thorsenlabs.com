---
title: Rewriting Mojolicious::Plugin::AssetPack
date: 2016-02-21
---

Update: I suggest checking out
[Mojolicious::Plugin::Webpack](https://metacpan.org/pod/Mojolicious::Plugin::Webpack)
instead.

------------------------------------------------------------------------

[AssetPack](https://metacpan.org/release/Mojolicious-Plugin-AssetPack)
is a [Mojolicious](http://mojolicious.org) plugin which helps you
process CSS and JavaScript assets. It can convert other formats, such as
SASS, Less, CoffeScript (and many more) into a format the browser
understands. This makes the development process a lot smoother.
AssetPack also makes sure the assets are minified and bundled into a
single file in production. This saves bandwidth and round-trip time to
the server, which again helps the browser to render the page faster.

This article is about proposed AssetPack changes and why. I would
appreciate [feedback](mailto:jhthorsen@cpan.org) from existing users, so
if you are an existing AssetPack user and think the changes are bad,
then please don't hesitate to contact me.

## Current version

AssetPack started out as something very simple: Just a single module
with some code blocks that would process the assets: The code blocks
took a reference to a scalar holding the content of the asset and
allowed any modification to that text. After all the registered code
blocks were done, AssetPack would write the final asset to the public
folder, which Mojolicious could serve.

The code blocks worked for a while, but did not scale very well. The
number of
[pre-processors](https://github.com/jhthorsen/mojolicious-plugin-assetpack/tree/0.69/lib/Mojolicious/Plugin/AssetPack/Preprocessor)
grew and made the module very clunky to work with.

I then decided to move the callbacks to a new
"[Preprocessors](https://github.com/jhthorsen/mojolicious-plugin-assetpack/blob/0.23/lib/Mojolicious/Plugin/AssetPack/Preprocessors.pm)"
module, which made the code a bit easier to read. Unfortunately, this
module lived on, even after I decided to split each pre-processor into
separate classes. This design decision made each of the pre-processor
objects impossible to access from the main AssetPack plugin. This means
that you cannot customize a pre-processor after it has been added.

Another thing that makes AssetPack difficult to work with is how it
figures out what to name the output asset files: The simple way to
describe the logic is that it takes all the content of the input files
and calculates an MD5 of the content, and uses that string as part of
the name. (Example: "myapp-d3b07384d113edec49eaa6238ad5ff00.css"). If
this file exists, it will skip the processing step, and simply use the
existing file. For some unexplained reason, AssetPack sometimes
calculates the checksum based on the wrong input files. I've still not
completely understood why this happens, but it happens in some rare
cases when you depend on assets automatically downloaded from the web.

Apart from the drawbacks mentioned above, it works very well: It
supports a variety of input file formats: It can process vanilla CSS,
JavaScript, CoffeScript, Less, Sass and it can even generate [CSS
sprites](https://css-tricks.com/css-sprites/). I intend to support most
of this functionality in the next version, but support for processing
Facebook's React support will be [removed](#facebooks-react).

## Next version

Why the rewrite? I wanted to make AssetPack simple and understandable
again. The module has grown into something that is very difficult to
maintain and extend. The new design enables most of the core
functionality to be overridden in a very simple way. This makes
AssetPack more flexible for end users.

### Overall design

The plugin code is very thin in the new version: Most of the code that
used to be core is now moved to the
"[store](https://github.com/jhthorsen/mojolicious-plugin-assetpack/blob/v2/lib/Mojolicious/Plugin/AssetPack/Store.pm)".
The "store" is an object that can persist assets and serve them to the
browser.

All the `Preprocessor::` classes have been converted to `Pipe::` classes
with a new and more simple API: There's just one method (`process()`)
that receives a list of input assets for a
[topic](https://github.com/jhthorsen/mojolicious-plugin-assetpack/blob/v2/lib/Mojolicious/Plugin/AssetPack/Guides/Tutorial.pod#process-assets).
This means that a "pipe" can change the list in any way it likes: It can
collapse it into one element, add more elements or change just parts of
the assets. One special example of this is the
[Combine](https://github.com/jhthorsen/mojolicious-plugin-assetpack/blob/v2/lib/Mojolicious/Plugin/AssetPack/Pipe/Combine.pm)

### Asset base URL

The `base_url` attribute in the existing version has been replaced with
a
[route](https://github.com/jhthorsen/mojolicious-plugin-assetpack/tree/v2#route)
attribute. This route defaults to dispatching to a callback which will
render the requested assets. This introduces much more flexibility: You
can change the path part completely and/or the
[scheme/host/port](https://github.com/jhthorsen/mojolicious-plugin-assetpack/blob/v2/lib/Mojolicious/Plugin/AssetPack/Guides/Cookbook.pod#assets-from-custom-domain).
A custom route also makes it possible to protect your assets with
authentication if you like.

### Headers

You can still customize response headers:
[headers](https://github.com/jhthorsen/mojolicious-plugin-assetpack/tree/0.69/lib/Mojolicious/Plugin/AssetPack#headers)
is moved to the
[store](https://github.com/jhthorsen/mojolicious-plugin-assetpack/tree/v2#store)
as "default_headers", and it has smart defaults: "Cache-Control" is set
to "max-age=31536000". This is in addition to the standard "ETag" and
"Last-Modified", which is already set by
[Mojolicious::Static](https://metacpan.org/pod/Mojolicious::Static).

### Asset output directory

Figuring out where to store assets was quite painful in the previous
version, and I still don't think it's done right. That's why there's no
`out_dir` in the new version. Instead there's automatic decision making
to save assets to memory, `TMP_DIR`, or as an actual file while
developing. The output files are not stored in the "public" directory
though, so they can't be accessed from the static renderer directly.

### Purging processed assets

`purge()` was added to clean up unused processed files. This was useful
since a development process could create hundreds of files in the
"public/packed" directory. In the new version, there's simply no need
for `purge()` since the checksum in the generated filenames does not
change as much as it used to: The checksum is calculated from the input
filenames/locations, instead of the content of the input assets. (See
[Guessing output filenames](#guessing-output-filenames) if you worry
about caching.)

### Guessing output filenames

This is the main reason for the rewrite: The current version is not very
good at naming the output files on disk. The new version on the other
hand calculates the filename based on the locations of the input files,
and not the content of those files. This means that a new filename will
only be generated when the list of input files change. This makes
finding processed assets a lot more robust. But what about browser
caching? That is still accounted for, since the path generated by the
route contains the checksum of the content of the input files, so it
changes each time the input files are changed.

### Where to find source assets

`source_paths` has been replaced with two more powerful attributes in
the
[store](https://github.com/jhthorsen/mojolicious-plugin-assetpack/tree/v2#store):
The "store" is an object that inherits from
[Mojolicious::Static](https://metacpan.org/pod/Mojolicious::Static)
which, instead of `source_paths`, has `classes` and `paths` attributes
allowing you to also declare your assets in the DATA section of your
classes.

## Major changes for existing users

There is some existing functionality that will (probably) be dropped.

### Defining assets

Assets cannot be defined with the default helper `asset()` anymore. You
need to call
[process()](https://github.com/jhthorsen/mojolicious-plugin-assetpack/tree/v2#process).

### MOJO_ASSETPACK_NO_CACHE

There's an option today to rebuild the assets on each request, instead
of just on startup. I'm not quite sure if I want to keep this or not.
What I might want to do instead is to add support for
`MOJO_ASSETPACK_WATCH` which will rebuild the changed assets and [reload
the
page](https://github.com/jhthorsen/mojolicious-plugin-assetpack/commit/77ef8c53ba8f1ad45af8d1f4a9372727ebe9c795)
in the browser.

### Fetching online assets

The `fetch()` method will be removed. I would much rather want to
improve the process step to do all the fetching.

### Will die instead of showing an error asset in the browser

The current version creates an error asset, which gives visual feedback
in the browser. This might be re-implemented, but right now I consider
the complexity to simply not be worth it. This behavior is especially a
bad idea in production, where you probably want a hot deployment of
[hypnotoad](https://metacpan.org/pod/Mojo::Server::Hypnotoad) to fail
instead of showing an ugly page.

### Wildcards in source path/filenames

A source path cannot have "\*" in it. The star used to be a way of
including all files from a given directory, matching an expression. This
logic simply does not work if you use the standard CPAN toolchain and
remove files in the which again results in the AssetPack finding files
that should not be there. So instead there's now support for creating a
very simple
[assetpack.def](https://github.com/jhthorsen/mojolicious-plugin-assetpack/blob/v2/lib/Mojolicious/Plugin/AssetPack/Guides/Tutorial.pod#defining-assets-in-a-definition-file),
which should be easy to generate from the command line.

### Facebook's React

"jsx" support is incredibly difficult to get right, and the current
version is, at best, buggy. I will not continue to support this.

### Global variables, such as SASS_PATH

This just won't happen. Unless I mess up. Variables such as `SASS_PATH`
are used internally, but it can't be set from the command line.
Environment and global variables are just not predictable enough to be
trusted in this setting.

## New features

The next version has some new cool features:

### Automatic install

AssetPack can automatically install third party node and ruby tools for
processing assets, such as CoffeeScript. "node" and "ruby" are still
required to be pre-installed though. I'm also looking into support for
automatically installing Perl modules, but, ironically, I find that a
bit harder to do right.

### Much better at downloading online assets

The
[Sass](https://github.com/jhthorsen/mojolicious-plugin-assetpack/blob/v2/lib/Mojolicious/Plugin/AssetPack/Pipe/Sass.pm)
pipe is able to download a sass file and all the \@import-ed files
recursively. This automatically deprecates plugins such as
[Mojolicious::Plugin::Bootstrap3](https://metacpan.org/pod/Mojolicious::Plugin::Bootstrap3).

### Access to the "pipe" objects.

The pipe objects can be
[accessed](https://github.com/jhthorsen/mojolicious-plugin-assetpack/tree/v2#pipe)
from the main plugin. This makes it much easier to control the settings
of a pipe.

### Optional dependencies

All
[dependencies](https://github.com/jhthorsen/mojolicious-plugin-assetpack/blob/master/cpanfile)
are optional. This means that you need to install more modules manually
while developing, but running an application in production does not
require any. The idea is that all of the "pipes" will skip processing
the asset when they are already available.

### Riotjs

Support for processing [Riotjs](http://riotjs.com/) is now part of the
[core](https://github.com/jhthorsen/mojolicious-plugin-assetpack/blob/v2/lib/Mojolicious/Plugin/AssetPack/Pipe/Riotjs.pm)
distribution.

## TODO

There is (at least) one missing feature that needs to be implemented --
at least before I completely deprecate the "pre-processors":

### CSS sprites

The idea here is to make a subclass of
[Asset](https://github.com/jhthorsen/mojolicious-plugin-assetpack/blob/v2/lib/Mojolicious/Plugin/AssetPack/Asset.pm)
which can hold all of the images. This object will again be able to
create CSS from the images, just like today.

## Summary

I hope people will try out the new version in the wild before the next
release happens. The merge will happen at some point, but that point
could be delayed if you report back breaking changes. The next version
should also be fully backwards compatible (the test suite says so), but
feedback on both new and old functionality is greatly appreciated.

How to get the next version:

```bash
cpanm https://github.com/jhthorsen/mojolicious-plugin-assetpack/archive/v2.tar.gz
```

## Resources

-   [Main
    documentation](https://github.com/jhthorsen/mojolicious-plugin-assetpack/tree/v2#synopsis)
-   [Tutorial](https://github.com/jhthorsen/mojolicious-plugin-assetpack/blob/v2/lib/Mojolicious/Plugin/AssetPack/Guides/Tutorial.pod)
-   [Cookbook](https://github.com/jhthorsen/mojolicious-plugin-assetpack/blob/v2/lib/Mojolicious/Plugin/AssetPack/Guides/Cookbook.pod)
-   [Pull
    request](https://github.com/jhthorsen/mojolicious-plugin-assetpack/pull/71)
