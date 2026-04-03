(() => {
    const init = () => {
        const root = document.documentElement;

        root.classList.add("js-scroll-reveal");

        if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
            document
                .querySelectorAll(".reveal-scroll")
                .forEach((element) => element.classList.add("is-visible"));
            return;
        }

        if (!("IntersectionObserver" in window)) {
            document
                .querySelectorAll(".reveal-scroll")
                .forEach((element) => element.classList.add("is-visible"));
            return;
        }

        const observed = new WeakSet();

        const observer = new IntersectionObserver(
            (entries) => {
                entries.forEach((entry) => {
                    if (!entry.isIntersecting) {
                        return;
                    }

                    entry.target.classList.add("is-visible");
                    observer.unobserve(entry.target);
                });
            },
            {
                threshold: 0.18,
                rootMargin: "0px 0px -10% 0px",
            }
        );

        const observeReveals = () => {
            document.querySelectorAll(".reveal-scroll").forEach((element) => {
                if (observed.has(element)) {
                    return;
                }

                observed.add(element);
                observer.observe(element);
            });
        };

        observeReveals();

        const mutationObserver = new MutationObserver(() => {
            observeReveals();
        });

        mutationObserver.observe(document.body, {
            childList: true,
            subtree: true,
        });
    };

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", init, { once: true });
        return;
    }

    init();
})();
