# Roadmap

This chapter contains the Roadmap to imag 1.0. This is, of course, a _really_
long roadmap and should considered to be approximate.

Things done will be moved to the @sec:changelog.

## Modules

These modules are planned right now. Each release should contain 3 new modules
maximum.

- [ ] imag-bibliography - For handling bibliographic references when writing
      scientific papers. Like Citavi, for example.
- [ ] imag-borrow - If you lend something to someone.
- [ ] imag-cuecards - Cuecards for learning things, for example vocabulary.
- [ ] imag-filter - command to read the store from stdin, filter out entries
      based on a predicate specified via the CLI and write the store back todo
      stdout.
- [ ] imag-image - Image referencing, categorizing, etc.
- [ ] imag-item - Creating entries for Items in the store
- [ ] imag-ledger - Ledger functionality like beancountcand others
- [ ] imag-list - Managing lists
- [ ] imag-movie - Managing movies
- [ ] imag-music - Managing music, Possibly a scrobble server.
- [ ] imag-news - A RSS Reader
- [ ] imag-project - A project planner, integrated with imag-timetrack and
      imag-todo
- [ ] imag-read - Command to load the store and pipe it to stdout (usefull for
      piping/ chaining commands)
- [ ] imag-receipt - Creating, categorizing, managing receipts
- [ ] imag-shoppinglists - Managing shopping lists
- [ ] imag-summary - Meta-command to call a set of imag commands (configurable
      which) and displaying their outputs.
- [ ] imag-url - Extracting URLs from enties, saving URLs to the imag store
- [ ] imag-weather - Weather tooling for getting forecast and recording a
      history of weather
- [ ] imag-fitness - A fitness tracker
    - [ ] imag-fitness-workout - Tools for tracking workouts.
    - [ ] imag-fitness-weight - Weight tracker
    - [ ] imag-fitness-step-counter - Step counting tool
- [ ] imag-write - Command to read the store from stdin and write it to the
      filesystem store (usefull for piping/chaining commands)

## Other Todos

* [ ] TUI frontend
  * [ ] TUI for 'imag'
  * [ ] TUI for 'imag-annotate'
  * [ ] TUI for 'imag-diagnostics'
  * [ ] TUI for 'imag-gps'
  * [ ] TUI for 'imag-grep'
  * [ ] TUI for 'imag-link'
  * [ ] TUI for 'imag-mv'
  * [ ] TUI for 'imag-ref'
  * [ ] TUI for 'imag-store'
  * [ ] TUI for 'imag-tag'
  * [ ] TUI for 'imag-view'
  * [ ] TUI for 'imag-bookmark'
  * [ ] TUI for 'imag-contact'
  * [ ] TUI for 'imag-counter'
  * [ ] TUI for 'imag-diary'
  * [ ] TUI for 'imag-habit'
  * [ ] TUI for 'imag-mail'
  * [ ] TUI for 'imag-notes'
  * [ ] TUI for 'imag-timetrack'
  * [ ] TUI for 'imag-todo'
  * [ ] TUI for ...

## 0.6.0

- [ ] imag-wiki - A wiki for personal use
- [ ] imag-init - A command to initialize a imag directory
- [ ] imag-git - wrapper to call git commands on the imag store no matter
  whether the current working directory is the store or not

## 0.7.0

- [ ] imag-rate - Attaching a rating to an entry
- [ ] Move away from github
    - [ ] Have own issue tracking (possibly git-dit)
    - [ ] Find a solution to having no travis-ci via github anymore
    - [ ] Setup a viewer for the mailinglist archive
- [ ] Add maintainer scripts to repository
    - [ ] To check patches for Signed-off-by lines
        - [ ] To automatically craft a reply to a contributor about a missing
              signed-off-by line
        - [ ] To automatically craft a reply to a contributor about a patchset that
              cannot be applied
    - [ ] To check pull requests for GPG signatures
    - [ ] To apply a patchset in a new branch

## 0.8.0

* ...

## 0.9.0

* ...

## 0.10.0

* ...

