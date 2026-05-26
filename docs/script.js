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
});
