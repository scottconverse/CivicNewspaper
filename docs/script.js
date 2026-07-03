document.addEventListener('DOMContentLoaded', () => {
    // Reveal the scroll-animated sections FIRST, before touching any
    // third-party dependency. The `.fade-up` sections start at opacity:0 (see
    // style.css) and are only shown once revealed; if a later line in this
    // handler threw, the page would render blank. Doing this first guarantees
    // content always appears.
    revealOnScroll();

    // Initialize Lucide icons. Guarded: the library loads from a CDN
    // (unpkg, see index.html), so a blocked/slow/offline CDN would make
    // `lucide` undefined. Without this guard the resulting ReferenceError would
    // abort the whole handler and leave the page unstyled/blank.
    try {
        if (window.lucide && typeof window.lucide.createIcons === 'function') {
            window.lucide.createIcons();
        }
    } catch (err) {
        console.error('Lucide icons failed to initialize:', err);
    }

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
    
    if (detectedPlatform === 'win') {
        const targetCard = document.getElementById(`download-${detectedPlatform}`);
        if (targetCard) {
            targetCard.classList.add('highlighted');
        }
    }

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

    // Keep the download buttons fixed to the audited release page. Do not
    // auto-rewrite them to GitHub's `/releases/latest` API; older releases can
    // remain latest while a new beta is being staged.
});

// Reveal `.fade-up` sections as they scroll into view. Defined as a standalone
// function so it can run as the very first thing in the load handler. Falls
// back to revealing everything immediately when IntersectionObserver is
// unavailable, so content is never left hidden.
function revealOnScroll() {
    const fadeElements = document.querySelectorAll('.fade-up');

    if (!('IntersectionObserver' in window)) {
        fadeElements.forEach((el) => el.classList.add('visible'));
        return;
    }

    const observer = new IntersectionObserver(
        (entries, obs) => {
            entries.forEach((entry) => {
                if (entry.isIntersecting) {
                    entry.target.classList.add('visible');
                    obs.unobserve(entry.target);
                }
            });
        },
        { root: null, rootMargin: '0px', threshold: 0.1 }
    );

    fadeElements.forEach((el) => observer.observe(el));
}
