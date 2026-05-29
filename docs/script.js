document.addEventListener('DOMContentLoaded', () => {
    // Initialize Mermaid
    if (typeof mermaid !== 'undefined') {
        mermaid.initialize({ startOnLoad: true, theme: 'dark' });
    }

    // Initialize Lucide Icons
    lucide.createIcons();

    // Platform detection for download highlights
    const userAgent = navigator.userAgent.toLowerCase();
    const platform = navigator.platform ? navigator.platform.toLowerCase() : '';
    
    let detectedPlatform = '';
    if (userAgent.includes('win') || platform.includes('win')) {
        detectedPlatform = 'win';
    } else if (userAgent.includes('mac') || platform.includes('mac') || platform.includes('ipad') || platform.includes('iphone')) {
        detectedPlatform = 'mac';
    } else if (userAgent.includes('linux') || platform.includes('linux')) {
        detectedPlatform = 'linux';
    }
    
    if (detectedPlatform) {
        const targetCard = document.getElementById(`download-${detectedPlatform}`);
        if (targetCard) {
            targetCard.classList.add('highlighted');
        }
    }

    // Intersection Observer for scroll animations
    const observerOptions = {
        root: null,
        rootMargin: '0px',
        threshold: 0.1
    };

    const observer = new IntersectionObserver((entries, observer) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('visible');
                // Stop observing once it has faded in
                observer.unobserve(entry.target);
            }
        });
    }, observerOptions);

    // Select all elements with fade-up class
    const fadeElements = document.querySelectorAll('.fade-up');
    fadeElements.forEach(el => {
        observer.observe(el);
    });

    // Smooth scrolling for anchor links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const targetId = this.getAttribute('href');
            if (targetId === '#') return;
            
            const targetElement = document.querySelector(targetId);
            if (targetElement) {
                const navHeight = document.querySelector('nav').offsetHeight;
                const targetPosition = targetElement.getBoundingClientRect().top + window.pageYOffset - navHeight - 20;
                
                window.scrollTo({
                    top: targetPosition,
                    behavior: 'smooth'
                });
            }
        });
    });

    // Resolve each download button to the matching installer from the latest
    // GitHub release. The HTML ships with safe `releases/latest` fallback hrefs,
    // so if the API is unavailable or an asset is missing we leave those in
    // place rather than rewriting to a broken link.
    const REPO_RELEASES_API =
        'https://api.github.com/repos/scottconverse/CivicNewspaper/releases/latest';

    const lower = (s) => (s || '').toLowerCase();
    const isArm = (name) => /aarch64|arm64/.test(name);
    const isX64 = (name) => /x86_64|amd64|x64/.test(name);

    // The browser UA reports "Intel" even on Apple Silicon, so fall back to
    // User-Agent Client Hints when available. Returns 'arm', 'x86', or '' (unknown).
    async function detectMacArch() {
        try {
            const uaData = navigator.userAgentData;
            if (uaData && uaData.getHighEntropyValues) {
                const hints = await uaData.getHighEntropyValues(['architecture']);
                if (hints && hints.architecture) {
                    return hints.architecture.includes('arm') ? 'arm' : 'x86';
                }
            }
        } catch (_) {
            /* fall through to unknown */
        }
        return '';
    }

    function pickWindowsAsset(assets) {
        return (
            assets.find((a) => lower(a.name).endsWith('.exe')) ||
            assets.find((a) => lower(a.name).endsWith('.msi')) ||
            null
        );
    }

    // The Linux release target is deb-only (see src-tauri/tauri.linux.conf.json),
    // so we only ever resolve a .deb asset here.
    function pickLinuxAsset(assets) {
        return assets.find((a) => lower(a.name).endsWith('.deb')) || null;
    }

    function pickMacAsset(assets, arch) {
        const dmgs = assets.filter((a) => lower(a.name).endsWith('.dmg'));
        if (dmgs.length === 0) return null;
        if (dmgs.length === 1) return dmgs[0];
        if (arch === 'arm') {
            const m = dmgs.find((a) => isArm(lower(a.name)));
            if (m) return m;
        } else if (arch === 'x86') {
            const m = dmgs.find((a) => isX64(lower(a.name)));
            if (m) return m;
        }
        // Multiple builds and an unknown architecture: send the user to the
        // release page to choose, rather than risk the wrong dmg.
        return null;
    }

    function setButtonHref(cardId, asset) {
        if (!asset || !asset.browser_download_url) return;
        // Defense-in-depth: the asset URL comes from the GitHub API response, so
        // only follow it if it parses as an https URL. This keeps a tampered or
        // unexpected response from rewriting a download button to an http,
        // javascript:, or data: link. On a bad URL we keep the HTML fallback.
        let parsed;
        try {
            parsed = new URL(asset.browser_download_url);
        } catch (_) {
            return;
        }
        if (parsed.protocol !== 'https:') return;
        const btn = document.querySelector(`#${cardId} .download-btn`);
        if (btn) btn.href = asset.browser_download_url;
    }

    (async () => {
        try {
            const response = await fetch(REPO_RELEASES_API);
            if (!response.ok) throw new Error(`GitHub API ${response.status}`);
            const data = await response.json();
            const assets = Array.isArray(data.assets) ? data.assets : [];
            if (assets.length === 0) return;

            setButtonHref('download-win', pickWindowsAsset(assets));
            setButtonHref('download-linux', pickLinuxAsset(assets));
            // Only probe for CPU architecture when the visitor is actually on a
            // Mac; the high-entropy UA hint call is pointless for other platforms.
            const macArch = detectedPlatform === 'mac' ? await detectMacArch() : '';
            setButtonHref('download-mac', pickMacAsset(assets, macArch));
        } catch (err) {
            console.error('Could not resolve per-platform download links:', err);
            // Leave the releases/latest fallback hrefs from the HTML in place.
        }
    })();
});
