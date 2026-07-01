document.addEventListener('DOMContentLoaded', () => {
    document.querySelectorAll('.gallery').forEach(gallery => {
        const images = Array.from(gallery.querySelectorAll('img'));
        if (images.length === 0) return;

        const delay = parseInt(gallery.dataset.delay) || 5000;
        let currentIndex = 0;
        let interval;

        // Touch handling variables
        let startX = 0;
        let currentX = 0;
        let isDragging = false;

        // Create structure
        const wrapper = document.createElement('div');
        wrapper.className = 'gallery-slides';
        
        images.forEach(img => {
            const slide = document.createElement('div');
            slide.className = 'gallery-slide';
            slide.appendChild(img.cloneNode(true));
            wrapper.appendChild(slide);
            img.remove();
        });
        gallery.appendChild(wrapper);

        // Add caption
        const caption = document.createElement('div');
        caption.className = 'gallery-caption';
        gallery.appendChild(caption);

        const updateGallery = (withTransition = true) => {
            wrapper.style.transition = withTransition ? 'transform 0.5s ease-in-out' : 'none';
            wrapper.style.transform = `translateX(-${currentIndex * 100}%)`;
            const currentImg = wrapper.querySelectorAll('img')[currentIndex];
            caption.textContent = currentImg.alt || currentImg.title || '';
            caption.style.display = caption.textContent ? 'block' : 'none';

            // Update button visibility
            const nav = gallery.querySelector('.gallery-nav');
            if (nav) {
                nav.querySelector('.prev').style.opacity = currentIndex === 0 ? '0.3' : '1';
                nav.querySelector('.prev').style.cursor = currentIndex === 0 ? 'default' : 'pointer';
                nav.querySelector('.next').style.opacity = currentIndex === images.length - 1 ? '0.3' : '1';
                nav.querySelector('.next').style.cursor = currentIndex === images.length - 1 ? 'default' : 'pointer';
            }
        };

        const next = (loop = false) => {
            if (currentIndex < images.length - 1) {
                currentIndex++;
            } else if (loop) {
                currentIndex = 0;
            }
            updateGallery();
        };

        const prev = (loop = false) => {
            if (currentIndex > 0) {
                currentIndex--;
            } else if (loop) {
                currentIndex = images.length - 1;
            }
            updateGallery();
        };

        // Navigation buttons
        if (images.length > 1) {
            const nav = document.createElement('div');
            nav.className = 'gallery-nav';
            nav.innerHTML = `
                <button class="prev"><i class="fa-solid fa-chevron-left"></i></button>
                <button class="next"><i class="fa-solid fa-chevron-right"></i></button>
            `;
            nav.querySelector('.prev').onclick = (e) => { e.preventDefault(); prev(false); resetTimer(); };
            nav.querySelector('.next').onclick = (e) => { e.preventDefault(); next(false); resetTimer(); };
            gallery.appendChild(nav);
        }

        const resetTimer = () => {
            clearInterval(interval);
            interval = setInterval(() => next(true), delay);
        };

        // Touch Events
        gallery.addEventListener('touchstart', (e) => {
            startX = e.touches[0].clientX;
            currentX = startX;
            isDragging = true;
            clearInterval(interval);
            wrapper.style.transition = 'none';
        }, { passive: true });

        gallery.addEventListener('touchmove', (e) => {
            if (!isDragging) return;
            currentX = e.touches[0].clientX;
            let diff = currentX - startX;
            
            // Rubber banding at edges
            if (currentIndex === 0 && diff > 0) diff /= 3;
            if (currentIndex === images.length - 1 && diff < 0) diff /= 3;

            wrapper.style.transform = `translateX(calc(-${currentIndex * 100}% + ${diff}px))`;
        }, { passive: true });

        gallery.addEventListener('touchend', (e) => {
            if (!isDragging) return;
            isDragging = false;
            const diff = currentX - startX;
            const threshold = gallery.offsetWidth * 0.2;

            if (Math.abs(diff) > threshold) {
                if (diff > 0 && currentIndex > 0) {
                    prev(false);
                } else if (diff < 0 && currentIndex < images.length - 1) {
                    next(false);
                } else {
                    updateGallery();
                }
            } else {
                updateGallery();
            }
            resetTimer();
        });

        updateGallery();
        resetTimer();
    });
});
