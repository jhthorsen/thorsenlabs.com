---
title: Finally got Nextcloud running with Caddy and php-fpm
---

I've been running [Nextcloud](https://nextcloud.com) for many years, but it has
annoyed me greatly that I had to use Apache to access it, when there's a
[php-fpm](https://hub.docker.com/_/nextcloud/tags?name=fpm) alternative,
and I'm using [Caddy](https://caddyserver.com) as my reverse proxy.

My setup involves running both Caddy and Nextcloud as containers, with Podman,
and *not* of Docker.

This is just a quick dump of my Caddyfile, which works with pretty URLs. I'm
quite confident there are some bugs I haven't discovered yet, but it seems to
work very nice for now.

```
nextcloud.example.com {
    # Nextcloud is mounted as /var/www/html in the nextcloud container
    root * /srv/nextcloud
    encode gzip zstd

    # This list is a work in progress. Trying to see if there's more that should be forbidden.
    @forbidden {
        path /.htaccess
        path /.xml
        path /console.php
        path /cron.php
        path /3rdparty/*
        path /README
        path /autotest/*
        path /build/*
        path /config/*
        path /console/*
        path /data/*
        path /db_*/*
        path /db_structure
        path /indie/*
        path /issue/*
        path /lib/*
        path /occ
        path /occ/*
        path /templates/*
        path /tests/*
    }

    # I'm not sure if this list is needed, but it seems to be faster
    @static {
        method GET HEAD
        not path /index.php*
        not path /apps/theming/composer/*
        not path /apps/theming/lib/*
        not path /apps/theming/templates/*
        not path /apps/theming/theme/*
        not path /js/core/merged-template-prepend.js
        path *.css
        path *.css.map
        path *.gif
        path *.ico
        path *.jpg
        path *.js
        path *.js.map
        path *.json
        path *.mjs
        path *.otf
        path *.png
        path *.svg
        path *.tflite
        path *.wasm
        path *.webp
        path *.woff2
    }

    # Some resources needs special handling
    rewrite /ocm-provider/ /index.php
    rewrite /ocs-provider/ /ocs-provider/index.php
    rewrite /remote /remote.php
    rewrite /remote/* /remote.php?{query}

    redir /.well-known/caldav /remote.php/dav 301
    redir /.well-known/carddav /remote.php/dav 301
    redir /.well-known/webfinger /index.php/.well-known/webfinger 301
    redir /.well-known/nodeinfo /index.php/.well-known/nodeinfo 301

    # These headers are suggested by Nextcloud
    header Referrer-Policy "no-referrer"
    header Strict-Transport-Security "max-age=15552000; includeSubDomains"
    header X-Content-Type-Options "nosniff"
    header X-Frame-Options "SAMEORIGIN"
    header X-Permitted-Cross-Domain-Policies "none"
    header X-Robots-Tag "noindex, nofollow"
    header X-XSS-Protection "1; mode=block"

    handle @forbidden {
        respond 404 {
            body `Please go away.`
            close
        }
    }

    handle @static {
        map {query} {cache_control} {
            v=* "max-age=15778463: immutable"
            default "max-age=15778463"
        }

        header Cache-Control {cache_control}
        encode gzip
        file_server
    }

    handle * {
        # Allow pretty URLs
        @index_files file {
            try_files {path} {path}/index.php /index.php{path}
            split_path .php
        }

        rewrite @index_files {file_match.relative}

        # IP:PORT of the container running docker.io/nextcloud:30-fpm
        reverse_proxy 10.89.0.39:9000 {
            transport fastcgi {
                # This root is different from the caddy container's root
                root /var/www/html
                split .php
                env modHeadersAvailable true
                env front_controller_active true
                capture_stderr
            }
        }

        file_server
    }
}
```
