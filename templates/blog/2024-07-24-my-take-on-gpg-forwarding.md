---
title: My take on gpg forwarding
---

I've been searching around the Internet for how to forward gpg sockets,
but after solving and forgetting how to solve the the problem multiple times,
I decided to write down my notes.

Hopefully I will remember to look them up next time...

## Local machine

Make sure the following lines ar present in `$HOME/.gnupg/gpg-agent.conf`:

```
extra-socket $HOME/.gnupg/S.gpg-agent.extra
pinentry-program /opt/homebrew/bin/pinentry-mac
ttyname $GPG_TTY
```

## Remote machine

Make sure the following line is present in `$HOME/.gnupg/gpg.conf`:

```
use-agent
```

Also make sure that gpg-agent is not running:

```bash
sudo systemctl --global mask --now \
  gpg-agent.service \
  gpg-agent.socket \
  gpg-agent-ssh.socket \
  gpg-agent-extra.socket \
  gpg-agent-browser.socket;

ps -A | grep gpg-agent;
killall gpg-agent;
```

## Forwarding

The following script `remote-gpg.sh` can be used to forward the gpg socket from the
local machine to the remote machine. It uses `gpgconf` to figure out the locations
of the sockets, to avoid any mismatch with the filenames.

```bash
#!/bin/bash
REMOTE_HOST="$1";

[ -z "$REMOTE_HOST" ] && exec echo "Usage: remote-gpg.sh <hostname>" >/dev/stderr;

set -e;

LOCAL_SOCKET="$(gpgconf --list-dirs agent-extra-socket)";
REMOTE_SOCKET="$(ssh "$REMOTE_HOST" gpgconf --list-dirs agent-socket)";

[ -z "$LOCAL_SOCKET" ] && exec echo "Can't find agent-extra-socket on $(hostname)" >/dev/stderr;
[ -z "$REMOTE_SOCKET" ] && exec echo "Can't find agent-socket on $REMOTE_HOST" >/dev/stderr;

set -x;
gpg -K;
ssh "$REMOTE_HOST" rm -f "$REMOTE_SOCKET" \
  && ssh -t -R "$REMOTE_SOCKET:$LOCAL_SOCKET" "$REMOTE_HOST" \
    /bin/sh -c "echo 'Hit enter to end session';echo;read;sleep 2;rm $REMOTE_SOCKET";
```

## Test the setup

You can run the following command below on the remote machine to see if gpg
is able to decrypt the encrypted secret:

```bash
# shell 1:
remote-gpg.sh test.thorsenlabs.com;

# shell 2:
echo Yes | gpg --encrypt -r gpg@email | ssh test.thorsenlabs.com echo "Decrypted? $(gpg --decrypt)";
```
