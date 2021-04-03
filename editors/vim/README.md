# Vim support for Quench

If you use a version of Vim or Neovim supported by [vim-lsp][], you can use the
below configuration to enable Quench support. Note that since [vim-lsp does not
support semantic tokens][semantic tokens], this won't give you syntax
highlighting, but it should work for other LSP features.

First install Quench itself (on your PATH) and [vim-plug][], if you haven't
already. Put the following in your Vim config file (if you're using Neovim on
Linux or macOS, for instance, that means `~/.config/nvim/init.vim`):

```vim
call plug#begin()
Plug 'prabirshrestha/vim-lsp'
call plug#end()

if executable('quench')
  augroup quench
    autocmd!

    " https://vi.stackexchange.com/a/23168
    autocmd BufNewFile,BufRead *.qn set filetype=quench

    autocmd User lsp_setup call lsp#register_server({
      \ 'name': 'quench lsp',
      \ 'cmd': {server_info->['quench', 'lsp']},
      \ 'allowlist': ['quench'],
      \ })
  augroup END
endif
```

Then start Vim (or Neovim), execute the `:PlugInstall` command, and open a file
whose name ends with `.qn`.

[semantic tokens]: https://github.com/prabirshrestha/vim-lsp/pull/974
[vim-lsp]: https://github.com/prabirshrestha/vim-lsp
[vim-plug]: https://github.com/junegunn/vim-plug
