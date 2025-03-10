---
title: My Neovim setup
---

Earlier in February I got inspired by [LazyVim](https://www.lazyvim.org) and started converting [my neovim configuration](https://github.com/jhthorsen/batphone.nvim) into a plugin. I must say though, that I am a bit envious to newcomers that can start out fresh with LazyVim, since it just looks amazing. I do think my setup is worth a look, since it's a bit closer to the core (neo)vim experience, and even though opinionated, it's easy to fork and make it into your own.

## Why I created a plugin

I dont't think it's worth the time to move  my [dotfiles](https://github.com/jhthorsen/dotfiles) around to different servers, but I do like moving my neovim config around. So instead of having to move my dotfiles around, I simply made a plugin for my neovim that will with very little effort get me up to speed on every server I log in to.

## Prerequisites

You need [Neovim](https://github.com/neovim/neovim/releases) 0.10.x or later.

If you already have neovim set up, then you want to (back up) and clear out the
following directories first:

    $ rm -rf ~/.config/nvim ~/.cache/nvim ~/.local/share/nvim;

## Installation

    $ mkdir -p "$HOME/.config/nvim";
    $ curl -L https://github.com/jhthorsen/batphone.nvim/raw/refs/heads/main/init.lazy.lua \
      > "$HOME/.config/nvim/init.lua";
    $ neovim;

## Plugins

The "core experience" mentioned in the introduction might be a big lie, when you
look at the list of plugins below, but the big difference between LazyVim and my
setup are all the UI changes.

- [echasnovski/mini.nvim](https://github.com/echasnovski/mini.nvim)
- [fang2hou/blink-copilot](https://github.com/fang2hou/blink-copilot)
- [folke/lazy.nvim](https://github.com/folke/lazy.nvim)
- [folke/lazydev.nvim](https://github.com/folke/lazydev.nvim)
- [folke/snacks.nvim](https://github.com/folke/snacks.nvim)
- [folke/which-key.nvim](https://github.com/folke/which-key.nvim)
- [mg979/vim-visual-multi](https://github.com/mg979/vim-visual-multi)
- [neovim/nvim-lspconfig](https://github.com/neovim/nvim-lspconfig)
- [nvim-lua/plenary.nvim](https://github.com/nvim-lua/plenary.nvim)
- [nvim-lualine/lualine.nvim](https://github.com/nvim-lualine/lualine.nvim)
- [nvim-tree/nvim-web-devicons](https://github.com/nvim-tree/nvim-web-devicons)
- [nvim-treesitter/nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter)
- [rafamadriz/friendly-snippets](https://github.com/rafamadriz/friendly-snippets)
- [rebelot/kanagawa.nvim](https://github.com/rebelot/kanagawa.nvim)
- [saghen/blink.cmp](https://github.com/saghen/blink.cmp)
- [stevearc/conform.nvim](https://github.com/stevearc/conform.nvim)
- [williamboman/mason-lspconfig.nvim](https://github.com/williamboman/mason-lspconfig.nvim)
- [williamboman/mason.nvim](https://github.com/williamboman/mason.nvim)
- [zbirenbaum/copilot.lua](https://github.com/zbirenbaum/copilot.lua)

## Useful bash function

This function will invoke "nvim" with a file picker, when called without any
arguments.

    vi() {
      if [ -n "$*" ]; then nvim "$@";
      elif [ -d ".git" ]; then nvim -c ":Telescope git_files";
      else nvim -c ":Telescope oldfiles";
      fi
    }

## Final comparison

If you come from [VSCode](https://code.visualstudio.com/) and you're curious
about [Neovim](https://neovim.io), then [Lazyvim](https://www.lazyvim.org) is
what you want. It's truly a complete experience, but with the speed,
convenience, and flexibility provided by Neovim. On the other side, if you are
used to working with vanilla vim or neovim, then you might want to try out my
setup, for a more minimalistic setup, that you can make into your own.
