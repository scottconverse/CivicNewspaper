# Publishing Connectors

The Civic Desk has two publishing modes:

- **API publish**: the app uploads the generated issue through an official provider path.
- **Assisted publish**: the app generates the package and share copy, then records the public URL after the editor publishes elsewhere.

Recommended stack: publish instantly with here.now, use GitHub Pages when the editor wants a durable public archive in their own repository, keep Netlify and WordPress for credentialed technical users, and treat Cloudflare Pages, Substack, and newsletter distribution as assisted workflows unless a later release records connector proof.

## Beta Verification Status

For this release line, anonymous here.now preview publishing is the tested default fast path. GitHub Pages, Netlify, WordPress, and permanent here.now account publishing require user-owned credentials and real target accounts. Cloudflare Pages API publishing is disabled in this public beta because the old Wrangler/npx path was not release-grade. Release evidence should state exactly which connector was live-verified for that release. Do not treat GitHub Pages, Netlify, WordPress, or permanent here.now account publishing as stable-grade verified unless the release notes include a live credentialed connector proof.

## API Publish Connectors

### here.now

- Requires: no credential for 24-hour preview publishing; here.now API key for permanent account-owned sites.
- Publishes: generated static files, excluding `site-package.zip`.
- Method: here.now Sites API creates a pending Site version, uploads each file to presigned URLs, then finalizes the version.
- Output: live here.now URL and version metadata. Anonymous preview sites expire after 24 hours.
- Optional target: here.now slug. Leave blank to create a new site, or enter an existing slug to update it.

### Netlify

- Requires: Netlify personal access token, Netlify site ID.
- Publishes: `site-package.zip`.
- Method: Netlify deploy API, ZIP upload to the configured site.
- Output: provider deployment ID and live URL returned by Netlify when available.

### GitHub Pages

- Requires: GitHub token with repository contents write access, `owner/repo`, branch, optional folder path.
- Publishes: generated static files, excluding `site-package.zip`.
- Method: GitHub REST Contents API writes each generated file to the configured branch/path. The connector creates the publish branch from the default branch when needed, configures GitHub Pages from root or `/docs`, and removes stale generated files listed in the previous Civic Desk manifest while preserving files such as `CNAME`.
- Output: configured public URL, or a derived `https://owner.github.io/repo/` URL. Best fit: durable/open repository-backed archive.

### WordPress

- Requires: WordPress site URL, username, application password.
- Publishes: a WordPress issue page and child article pages from the generated site package.
- Method: WordPress REST API `wp/v2/pages` with application-password authentication.
- Output: WordPress issue page ID and public page URL.

## Assisted Connectors

### Substack

Substack does not provide a supported public publishing API for third-party apps to create posts. The app generates `substack.md` for copy/paste publishing, then the editor records the public post URL.

### Other Host

Use this for static hosts, local web servers, or platforms without a supported API connector. The app records the public URL and refreshes the generated manifest, RSS, newsletter, and social copy.

### Cloudflare Pages

Cloudflare Pages is an assisted workflow in this public beta. Export the generated folder or `site-package.zip`, deploy it through Cloudflare's dashboard or your own CLI, then paste the public URL into The Civic Desk so the manifest, RSS, newsletter, and share copy are updated.
