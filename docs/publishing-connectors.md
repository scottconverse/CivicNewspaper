# Publishing Connectors

The Civic Desk has two publishing modes:

- **API publish**: the app uploads the generated issue through an official provider path.
- **Assisted publish**: the app generates the package and share copy, then records the public URL after the editor publishes elsewhere.

## API Publish Connectors

### Netlify

- Requires: Netlify personal access token, Netlify site ID.
- Publishes: `site-package.zip`.
- Method: Netlify deploy API, ZIP upload to the configured site.
- Output: provider deployment ID and live URL returned by Netlify when available.

### GitHub Pages

- Requires: GitHub token with repository contents write access, `owner/repo`, branch, optional folder path.
- Publishes: generated static files, excluding `site-package.zip`.
- Method: GitHub REST Contents API writes each generated file to the configured branch/path.
- Output: configured public URL, or a derived `https://owner.github.io/repo/` URL.

### Cloudflare Pages

- Requires: Cloudflare API token, account ID, Pages project name, branch.
- Publishes: generated site folder.
- Method: official `wrangler pages deploy` direct-upload path run through `npx`.
- Output: deployment URL parsed from Wrangler output when available.

### WordPress

- Requires: WordPress site URL, username, application password.
- Publishes: generated `substack.md` package as a WordPress post.
- Method: WordPress REST API `wp/v2/posts` with application-password authentication.
- Output: WordPress post ID and public post URL.

## Assisted Connectors

### Substack

Substack does not provide a supported public publishing API for third-party apps to create posts. The app generates `substack.md` for copy/paste publishing, then the editor records the public post URL.

### Other Host

Use this for static hosts, local web servers, or platforms without a supported API connector. The app records the public URL and refreshes the generated manifest, RSS, newsletter, and social copy.
