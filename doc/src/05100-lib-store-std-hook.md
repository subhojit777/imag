## libimagstorestdhook

The `libimagstorestdhook` is a library for default store hooks which are shipped
with the imag store.
Hooks are actions which can be performed before and after certain store actions,
for example before a file is created, or after a file is removed.

### Long-term TODO

- [ ] Merge with `libimagrt`
- [ ] Merge with `libimagstorestdhook`
- [ ] Create Runtime-wide "Store meta data" storage in the Runtime, which can be
  set by users during the runtime of imag and then used by the hooks to get meta
  information about their own runtime.
- [ ] Implement parallel store hook execution
- [ ] Implement Non-Mutable store hook execution

