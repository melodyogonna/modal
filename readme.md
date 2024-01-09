# Modal

Modern modal-based text editor that tries to understand the code and the developer.

# Idea

Leverage vim-like modal commands and modern improvements like Treesitter and LSPs to provide a text
editing experience that tries to accurately predict what a developer would do next based on source location and context.

The goal is to make editing source files even more natural than it currently is with Vim/Neovim.

Modal will have a native understanding of fundamental programming constructs like functions, loops, variables, identity, types, etc. The idea is to leverage this knowledge to provide
very fast and accurate auto-completion, refactors, source movements, highlighting, and keyboard shortcuts. And to defer to LSP when the editor can't make an accurate assesement.

# Stage

Still very early, unusable.
