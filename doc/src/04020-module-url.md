## Url {#sec:modules:url}

The Url module is a plumbing module to put URLs into the imag store.

### Implementation

The implementation of the URL module saves URLs on a per-entry basis. This means that each URL is hashed (with something like SHA512) and the hash is used as filename. The scheme is as follows:

    /url/<hash of the domain>/<hash of the full URL>

This scheme results in grouping URLs of the same domain (for example https://imag-pim.org) but distinction of the actual full URL, while still deduplicating URLs. Entering the same URL twice results in the same entry.

This module does nothing more on its own. Its functionality may be used elsewhere (for example a bookmark module).
