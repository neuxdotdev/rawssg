document.addEventListener("DOMContentLoaded", function () {
	console.log("Raw Features JS loaded");
	initThemeSystem();
	initSidebar();
	initCodeBlocks();
	initLinkEnhancements();
	initScrollEffects();
	addMobileThemeToggle();
	initEnhancedFeatures();
});
function initThemeSystem() {
	const themeToggle = document.createElement("button");
	themeToggle.className = "theme-toggle";
	themeToggle.innerHTML = '<i class="fa-solid fa-circle-half-stroke"></i>';
	themeToggle.title = "Toggle theme";
	themeToggle.setAttribute("aria-label", "Toggle dark/light theme");
	themeToggle.addEventListener("click", function () {
		const currentTheme = document.documentElement.getAttribute("data-theme");
		const newTheme = currentTheme === "light" ? "dark" : "light";
		document.documentElement.setAttribute("data-theme", newTheme);
		localStorage.setItem("theme", newTheme);
		updateThemeIcon(themeToggle, newTheme);
		document.dispatchEvent(new CustomEvent("themechange", { detail: { theme: newTheme } }));
		this.style.transform = "rotate(180deg)";
		setTimeout(() => {
			this.style.transform = "";
		}, 300);
	});
	const sidebarHeader = document.querySelector(".sidebar-header");
	if (sidebarHeader && !sidebarHeader.querySelector(".theme-toggle")) {
		const h2 = sidebarHeader.querySelector("h2");
		if (h2) {
			h2.parentNode.insertBefore(themeToggle, h2.nextSibling);
		}
	}
	const savedTheme = localStorage.getItem("theme");
	const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
	let initialTheme = savedTheme || (prefersDark ? "dark" : "light");
	document.documentElement.setAttribute("data-theme", initialTheme);
	updateThemeIcon(themeToggle, initialTheme);
	window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", (e) => {
		if (!localStorage.getItem("theme")) {
			const newTheme = e.matches ? "dark" : "light";
			document.documentElement.setAttribute("data-theme", newTheme);
			updateThemeIcon(themeToggle, newTheme);
		}
	});
	function updateThemeIcon(button, theme) {
		if (theme === "dark") {
			button.innerHTML = '<i class="fa-solid fa-sun"></i>';
			button.title = "Switch to light theme";
		} else {
			button.innerHTML = '<i class="fa-solid fa-moon"></i>';
			button.title = "Switch to dark theme";
		}
	}
}
function initSidebar() {
	const sidebar = document.querySelector(".sidebar");
	const sidebarToggle = document.querySelector(".sidebar-toggle");
	if (!sidebar || !sidebarToggle) return;
	sidebarToggle.addEventListener("click", function (e) {
		e.stopPropagation();
		sidebar.classList.toggle("active");
		if (sidebar.classList.contains("active")) {
			this.innerHTML = "✕";
			this.setAttribute("aria-expanded", "true");
		} else {
			this.innerHTML = "☰";
			this.setAttribute("aria-expanded", "false");
		}
	});
	document.addEventListener("click", function (e) {
		if (window.innerWidth <= 768) {
			if (!sidebar.contains(e.target) && !sidebarToggle.contains(e.target)) {
				sidebar.classList.remove("active");
				sidebarToggle.innerHTML = "☰";
				sidebarToggle.setAttribute("aria-expanded", "false");
			}
		}
	});
	const sidebarLinks = document.querySelectorAll(".sidebar-link");
	sidebarLinks.forEach((link) => {
		link.addEventListener("click", function () {
			if (window.innerWidth <= 768) {
				sidebar.classList.remove("active");
				sidebarToggle.innerHTML = "☰";
				sidebarToggle.setAttribute("aria-expanded", "false");
			}
		});
	});
	highlightActiveLink();
}
function initCodeBlocks() {
	document.querySelectorAll("pre.raw-code-block").forEach((pre) => {
		if (pre.querySelector(".copy-raw-code")) return;
		const copyButton = document.createElement("button");
		copyButton.className = "copy-raw-code";
		copyButton.innerHTML = '<i class="fa-regular fa-copy"></i> Copy';
		copyButton.setAttribute("aria-label", "Copy code to clipboard");
		copyButton.addEventListener("click", function () {
			const code = pre.querySelector("code")?.textContent || pre.textContent;
			navigator.clipboard
				.writeText(code)
				.then(() => {
					this.innerHTML = '<i class="fa-solid fa-check"></i> Copied';
					this.classList.add("copied");
					setTimeout(() => {
						this.innerHTML = '<i class="fa-regular fa-copy"></i> Copy';
						this.classList.remove("copied");
					}, 2000);
				})
				.catch((err) => {
					console.error("Failed to copy code:", err);
					this.innerHTML = '<i class="fa-solid fa-xmark"></i> Error';
					setTimeout(() => {
						this.innerHTML = '<i class="fa-regular fa-copy"></i> Copy';
					}, 2000);
				});
		});
		pre.appendChild(copyButton);
		if (
			pre.textContent.includes("function") ||
			pre.textContent.includes("const") ||
			pre.textContent.includes("let")
		) {
			applySyntaxHighlighting(pre);
		}
	});
	document.querySelectorAll("pre.raw-code-block").forEach((pre) => {
		const lines = pre.textContent.split("\n").length;
		if (lines > 10) {
			pre.style.position = "relative";
			pre.style.paddingLeft = "3.5em";
			const lineNumbers = document.createElement("div");
			lineNumbers.className = "line-numbers";
			lineNumbers.style.position = "absolute";
			lineNumbers.style.left = "0";
			lineNumbers.style.top = "0";
			lineNumbers.style.bottom = "0";
			lineNumbers.style.width = "2.5em";
			lineNumbers.style.padding = "1.25rem 0.5rem";
			lineNumbers.style.backgroundColor = "var(--bg-secondary)";
			lineNumbers.style.borderRight = "1px solid var(--border)";
			lineNumbers.style.textAlign = "right";
			lineNumbers.style.color = "var(--text-muted)";
			lineNumbers.style.fontFamily = "inherit";
			lineNumbers.style.fontSize = "13.5px";
			lineNumbers.style.lineHeight = "1.5";
			lineNumbers.style.overflow = "hidden";
			lineNumbers.style.userSelect = "none";
			let numbersHTML = "";
			for (let i = 1; i <= lines; i++) {
				numbersHTML += `<div>${i}</div>`;
			}
			lineNumbers.innerHTML = numbersHTML;
			pre.appendChild(lineNumbers);
		}
	});
}
function initLinkEnhancements() {
	document
		.querySelectorAll('a[href^="http"]:not([href*="' + window.location.host + '"])')
		.forEach((link) => {
			if (!link.classList.contains("rw-link")) {
				link.setAttribute("target", "_blank");
				link.setAttribute("rel", "noopener noreferrer");
				const icon = document.createElement("span");
				icon.innerHTML = ' <i class="fa-solid fa-arrow-up-right-from-square"></i>';
				icon.style.fontSize = "0.85em";
				icon.style.opacity = "0.7";
				if (!link.innerHTML.includes("↗") && !link.innerHTML.includes("external")) {
					link.appendChild(icon);
				}
			}
		});
	document.querySelectorAll('a[href^="#"]').forEach((anchor) => {
		anchor.addEventListener("click", function (e) {
			const targetId = this.getAttribute("href");
			if (targetId === "#") return;
			const targetElement = document.querySelector(targetId);
			if (targetElement) {
				e.preventDefault();
				window.scrollTo({
					top: targetElement.offsetTop - 80,
					behavior: "smooth",
				});
			}
		});
	});
}
function initScrollEffects() {
	const sidebar = document.querySelector(".sidebar");
	if (sidebar) {
		sidebar.addEventListener("scroll", function () {
			if (this.scrollTop > 0) {
				this.style.boxShadow = "inset 0 1px 0 var(--border)";
			} else {
				this.style.boxShadow = "var(--shadow)";
			}
		});
	}
	const backToTop = document.createElement("button");
	backToTop.innerHTML = "↑";
	backToTop.className = "back-to-top";
	backToTop.setAttribute("aria-label", "Back to top");
	backToTop.style.cssText = `
        position: fixed;
        bottom: 2rem;
        right: 2rem;
        width: 44px;
        height: 44px;
        background: var(--bg);
        border: 1px solid var(--border);
        border-radius: 50%;
        color: var(--text);
        font-size: 1.2rem;
        cursor: pointer;
        opacity: 0;
        visibility: hidden;
        transition: all 0.3s ease;
        z-index: 999;
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: var(--shadow-md);
    `;
	document.body.appendChild(backToTop);
	backToTop.addEventListener("click", () => {
		window.scrollTo({ top: 0, behavior: "smooth" });
	});
	window.addEventListener("scroll", () => {
		if (window.scrollY > 500) {
			backToTop.style.opacity = "1";
			backToTop.style.visibility = "visible";
		} else {
			backToTop.style.opacity = "0";
			backToTop.style.visibility = "hidden";
		}
	});
}
function addMobileThemeToggle() {
	if (window.innerWidth <= 768) {
		if (!document.querySelector(".mobile-theme-toggle")) {
			const mobileThemeToggle = document.createElement("button");
			mobileThemeToggle.className = "theme-toggle mobile-theme-toggle";
			mobileThemeToggle.innerHTML = '<i class="fa-solid fa-circle-half-stroke"></i>';
			mobileThemeToggle.title = "Toggle theme";
			mobileThemeToggle.setAttribute("aria-label", "Toggle theme");
			const currentTheme = document.documentElement.getAttribute("data-theme");
			if (currentTheme === "dark") {
				mobileThemeToggle.innerHTML = '<i class="fa-solid fa-sun"></i>';
			} else {
				mobileThemeToggle.innerHTML = '<i class="fa-solid fa-moon"></i>';
			}
			mobileThemeToggle.addEventListener("click", function () {
				const currentTheme = document.documentElement.getAttribute("data-theme");
				const newTheme = currentTheme === "light" ? "dark" : "light";
				document.documentElement.setAttribute("data-theme", newTheme);
				localStorage.setItem("theme", newTheme);
				document.querySelectorAll(".theme-toggle").forEach((btn) => {
					if (newTheme === "dark") {
						btn.innerHTML = '<i class="fa-solid fa-sun"></i>';
						btn.title = "Switch to light theme";
					} else {
						btn.innerHTML = '<i class="fa-solid fa-moon"></i>';
						btn.title = "Switch to dark theme";
					}
				});
			});
			document.body.appendChild(mobileThemeToggle);
		}
	}
}
function highlightActiveLink() {
	const currentPath = window.location.pathname;
	const sidebarLinks = document.querySelectorAll(".sidebar-link");
	sidebarLinks.forEach((link) => {
		const linkPath = link.getAttribute("href");
		if (
			linkPath === currentPath ||
			(currentPath.endsWith("/") && linkPath === currentPath.slice(0, -1)) ||
			(!currentPath.endsWith("/") && linkPath === currentPath + "/")
		) {
			link.classList.add("active");
		} else {
			link.classList.remove("active");
		}
	});
}
function applySyntaxHighlighting(preElement) {
	let code = preElement.innerHTML;
	const patterns = [
		{ regex: /(\/\/.*)/g, class: "code-comment" },
		{ regex: /(\/\*[\s\S]*?\*\/)/g, class: "code-comment" },
		{
			regex: /\b(const|let|var|function|if|else|return|class|export|import|from|default|try|catch|finally|throw|new|typeof|instanceof|in|of|async|await|yield)\b/g,
			class: "code-keyword",
		},
		{ regex: /(['"`])(?:(?=(\\?))\2.)*?\1/g, class: "code-string" },
		{ regex: /\b(\d+\.?\d*|0x[\da-f]+)\b/gi, class: "code-number" },
		{ regex: /\b([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\(/g, class: "code-function" },
		{
			regex: /(\+|\-|\*|\/|\=\=|\=\=\=|\!\=|\!\=\=|\>|\<|\>\=|\<\=|\&\&|\|\||\!|\=)/g,
			class: "code-operator",
		},
	];
	patterns.forEach((pattern) => {
		code = code.replace(pattern.regex, `<span class="${pattern.class}">$1</span>`);
	});
	preElement.innerHTML = code;
}
function initEnhancedFeatures() {
	console.log("Enhanced Features JS loaded");
	initToastSystem();
	initSearchHighlight();
	initReadingProgress();
	initEnhancedTables();
	initTooltips();
	initKeyboardShortcuts();
	initPrintButton();
	initImageLazyLoading();
	initAccessibility();
}
function initToastSystem() {
	const toastContainer = document.createElement("div");
	toastContainer.className = "toast-container";
	document.body.appendChild(toastContainer);
	window.showToast = function (options) {
		const toast = document.createElement("div");
		toast.className = `toast ${options.type || "info"}`;
		const icons = {
			success: '<i class="fa-solid fa-circle-check" style="color:#2da44e"></i>',
			error: '<i class="fa-solid fa-circle-xmark" style="color:#cf222e"></i>',
			warning: '<i class="fa-solid fa-triangle-exclamation" style="color:#d4a72c"></i>',
			info: '<i class="fa-solid fa-circle-info" style="color:#54aeff"></i>',
		};
		toast.innerHTML = `
            <span class="toast-icon">${icons[options.type] || icons.info}</span>
            <div class="toast-content">
                <div class="toast-title">${options.title || ""}</div>
                <div class="toast-message">${options.message || ""}</div>
            </div>
            <button class="toast-close" aria-label="Close toast">&times;</button>
        `;
		toastContainer.appendChild(toast);
		const closeBtn = toast.querySelector(".toast-close");
		closeBtn.addEventListener("click", () => dismissToast(toast));
		if (options.duration && options.duration > 0) {
			setTimeout(() => dismissToast(toast), options.duration);
		}
		return {
			dismiss: () => dismissToast(toast),
		};
	};
	function dismissToast(toast) {
		toast.classList.add("hiding");
		setTimeout(() => {
			if (toast.parentNode) {
				toast.parentNode.removeChild(toast);
			}
		}, 300);
	}
}
function initSearchHighlight() {
	const urlParams = new URLSearchParams(window.location.search);
	const searchTerm = urlParams.get("q");
	if (searchTerm) {
		highlightText(searchTerm);
	}
	if (!document.querySelector(".search-form")) {
		addSearchForm();
	}
}
function highlightText(searchTerm) {
	const content = document.querySelector(".raw-content");
	if (!content) return;
	const text = content.textContent;
	const regex = new RegExp(`(${searchTerm})`, "gi");
	const highlighted = text.replace(regex, '<mark class="highlight">$1</mark>');
	const temp = document.createElement("div");
	temp.innerHTML = highlighted;
	while (content.firstChild) {
		content.removeChild(content.firstChild);
	}
	Array.from(temp.childNodes).forEach((node) => {
		content.appendChild(node.cloneNode(true));
	});
	const resultsCount = (text.match(regex) || []).length;
	if (resultsCount > 0) {
		const info = document.createElement("div");
		info.className = "search-info meta";
		info.innerHTML = `Found ${resultsCount} results for "${searchTerm}"`;
		content.parentNode.insertBefore(info, content);
	}
}
function addSearchForm() {
	const sidebar = document.querySelector(".sidebar");
	if (!sidebar) return;
	const searchForm = document.createElement("form");
	searchForm.className = "search-form";
	searchForm.innerHTML = `
        <input type="search" 
               placeholder="Search content..." 
               aria-label="Search content"
               class="search-input">
        <button type="submit" class="search-button">
			<i class="fa-solid fa-magnifying-glass"></i>
		</button>`;

	searchForm.style.cssText = `
        margin: 1rem 0;
        display: flex;
        gap: 0.5rem;
    `;
	const searchInput = searchForm.querySelector(".search-input");
	searchInput.style.cssText = `
        flex: 1;
        padding: 0.5rem;
        border: 1px solid var(--border);
        border-radius: 6px;
        background: var(--bg);
        color: var(--text);
        font-size: 0.9rem;
    `;
	const searchButton = searchForm.querySelector(".search-button");
	searchButton.style.cssText = `
        padding: 0.5rem 1rem;
        background: var(--primary);
        color: white;
        border: none;
        border-radius: 6px;
        cursor: pointer;
        font-size: 0.9rem;
    `;
	searchForm.addEventListener("submit", function (e) {
		e.preventDefault();
		const searchTerm = searchInput.value.trim();
		if (searchTerm) {
			const url = new URL(window.location);
			url.searchParams.set("q", searchTerm);
			window.location.href = url.toString();
		}
	});
	const sidebarHeader = document.querySelector(".sidebar-header");
	if (sidebarHeader) {
		sidebar.insertBefore(searchForm, sidebarHeader.nextSibling);
	}
}
function initReadingProgress() {
	const progressBar = document.createElement("div");
	progressBar.className = "progress-bar";
	progressBar.style.cssText = `
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 3px;
        background: linear-gradient(90deg, var(--primary), var(--primary-light));
        z-index: 10001;
        transform-origin: 0 0;
        transform: scaleX(0);
        transition: transform 0.2s ease;
    `;
	document.body.appendChild(progressBar);
	window.addEventListener("scroll", updateProgressBar);
	function updateProgressBar() {
		const windowHeight = window.innerHeight;
		const documentHeight = document.documentElement.scrollHeight - windowHeight;
		const scrolled = window.scrollY;
		const progress = scrolled / documentHeight;
		progressBar.style.transform = `scaleX(${progress})`;
	}
	updateProgressBar();
}
function initEnhancedTables() {
	document.querySelectorAll(".raw-content table").forEach((table) => {
		table.classList.add("enhanced-table");
		const wrapper = document.createElement("div");
		wrapper.style.cssText = "overflow-x: auto; margin: 1.5rem 0;";
		table.parentNode.insertBefore(wrapper, table);
		wrapper.appendChild(table);
		if (!table.querySelector("caption")) {
			const caption = document.createElement("caption");
			caption.textContent = "Table Data";
			caption.style.cssText = `
                caption-side: bottom;
                padding: 1rem;
                color: var(--text-muted);
                font-size: 0.9rem;
                border-top: 1px solid var(--border);
            `;
			table.appendChild(caption);
		}
		const ths = table.querySelectorAll("th");
		ths.forEach((th) => {
			th.style.cssText = `
                background: var(--bg-secondary);
                padding: 1rem;
                text-align: left;
                font-weight: 600;
                border-bottom: 2px solid var(--border);
                color: var(--text);
            `;
		});
		const tds = table.querySelectorAll("td");
		tds.forEach((td) => {
			td.style.cssText = `
                padding: 0.75rem 1rem;
                border-bottom: 1px solid var(--border);
                color: var(--text-secondary);
            `;
		});
	});
}
function initTooltips() {
	document.querySelectorAll("[data-tooltip]").forEach((element) => {
		const tooltipText = element.getAttribute("data-tooltip");
		const tooltip = document.createElement("span");
		tooltip.className = "tooltip";
		tooltip.textContent = tooltipText;
		tooltip.style.cssText = `
            position: absolute;
            bottom: 100%;
            left: 50%;
            transform: translateX(-50%);
            padding: 0.5rem 0.75rem;
            background: var(--bg-dark);
            color: var(--text-dark);
            border-radius: 4px;
            font-size: 0.8rem;
            white-space: nowrap;
            pointer-events: none;
            opacity: 0;
            transition: opacity 0.2s ease, transform 0.2s ease;
            z-index: 1000;
            box-shadow: 0 8px 24px rgba(140, 149, 159, 0.2);
        `;
		const wrapper = document.createElement("span");
		wrapper.className = "tooltip-wrapper";
		wrapper.style.cssText = "position: relative; display: inline-block;";
		element.parentNode.insertBefore(wrapper, element);
		wrapper.appendChild(element);
		wrapper.appendChild(tooltip);
		wrapper.addEventListener("mouseenter", () => {
			tooltip.style.opacity = "1";
			tooltip.style.transform = "translateX(-50%) translateY(-5px)";
		});
		wrapper.addEventListener("mouseleave", () => {
			tooltip.style.opacity = "0";
			tooltip.style.transform = "translateX(-50%)";
		});
	});
}
function initKeyboardShortcuts() {
	const shortcuts = {
		"?": () => showHelpModal(),
		t: () => window.RawFeatures?.toggleTheme(),
		s: () => {
			const searchInput = document.querySelector(".search-input");
			if (searchInput) {
				searchInput.focus();
				return false;
			}
		},
		Escape: () => closeAllModals(),
	};
	document.addEventListener("keydown", function (e) {
		if (
			e.target.tagName === "INPUT" ||
			e.target.tagName === "TEXTAREA" ||
			e.target.isContentEditable
		) {
			return;
		}
		const key = e.key;
		if (shortcuts[key] && !e.ctrlKey && !e.altKey && !e.metaKey) {
			if (shortcuts[key]() === false) {
				e.preventDefault();
			}
		}
	});
	function showHelpModal() {
		window.showToast({
			title: "Keyboard Shortcuts",
			message: "t: Toggle theme • s: Search • ?: Show help",
			type: "info",
			duration: 5000,
		});
	}
	function closeAllModals() {
		document.querySelectorAll(".modal").forEach((modal) => {
			modal.style.display = "none";
		});
	}
}
function initPrintButton() {
	const printBtn = document.createElement("button");
	printBtn.className = "print-button";
	printBtn.innerHTML = '<i class="fa-solid fa-print"></i> Print';
	printBtn.title = "Print this page";
	printBtn.setAttribute("aria-label", "Print page");
	printBtn.style.cssText = `
        position: fixed;
        bottom: 2rem;
        left: 2rem;
        background: var(--bg);
        border: 1px solid var(--border);
        border-radius: 6px;
        padding: 0.5rem 1rem;
        cursor: pointer;
        color: var(--text);
        font-size: 0.9rem;
        z-index: 999;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        box-shadow: 0 3px 6px rgba(140, 149, 159, 0.15);
    `;
	printBtn.addEventListener("click", () => window.print());
	document.body.appendChild(printBtn);
}
function initImageLazyLoading() {
	const images = document.querySelectorAll(".raw-content img");
	const observer = new IntersectionObserver(
		(entries) => {
			entries.forEach((entry) => {
				if (entry.isIntersecting) {
					const img = entry.target;
					img.src = img.dataset.src || img.src;
					img.classList.add("loaded");
					observer.unobserve(img);
				}
			});
		},
		{
			rootMargin: "50px",
		},
	);
	images.forEach((img) => {
		if (!img.loading) {
			img.loading = "lazy";
		}
		if (!img.classList.contains("responsive-image")) {
			img.classList.add("responsive-image");
			img.style.cssText = `
                max-width: 100%;
                height: auto;
                display: block;
                margin: 1.5rem auto;
                border-radius: 6px;
                border: 1px solid var(--border);
            `;
		}
		observer.observe(img);
	});
}
function initAccessibility() {
	const skipLink = document.createElement("a");
	skipLink.href = "#main-content";
	skipLink.className = "skip-to-content";
	skipLink.textContent = "Skip to content";
	skipLink.style.cssText = `
        position: absolute;
        top: -40px;
        left: 10px;
        padding: 0.5rem 1rem;
        background: var(--primary);
        color: white;
        text-decoration: none;
        border-radius: 6px;
        z-index: 10000;
    `;
	const mainContent = document.querySelector(".content");
	if (mainContent) {
		mainContent.id = "main-content";
		document.body.insertBefore(skipLink, document.body.firstChild);
		skipLink.addEventListener("focus", () => {
			skipLink.style.top = "10px";
		});
		skipLink.addEventListener("blur", () => {
			skipLink.style.top = "-40px";
		});
	}
	document.querySelectorAll("button:not([aria-label])").forEach((button) => {
		if (!button.textContent.trim()) {
			const label = button.title || button.getAttribute("data-tooltip");
			if (label) {
				button.setAttribute("aria-label", label);
			}
		}
	});
	document.addEventListener("keydown", function (e) {
		if (e.key === "Tab") {
			const modals = document.querySelectorAll(
				'.modal[style*="display: block"], .modal[style*="display:flex"]',
			);
			if (modals.length > 0) {
				const focusable = modals[modals.length - 1].querySelectorAll(
					'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])',
				);
				if (focusable.length > 0) {
					const first = focusable[0];
					const last = focusable[focusable.length - 1];
					if (e.shiftKey) {
						if (document.activeElement === first) {
							last.focus();
							e.preventDefault();
						}
					} else {
						if (document.activeElement === last) {
							first.focus();
							e.preventDefault();
						}
					}
				}
			}
		}
	});
}
let resizeTimer;
window.addEventListener("resize", () => {
	clearTimeout(resizeTimer);
	resizeTimer = setTimeout(() => {
		const mobileToggle = document.querySelector(".mobile-theme-toggle");
		if (window.innerWidth <= 768 && !mobileToggle) {
			addMobileThemeToggle();
		} else if (window.innerWidth > 768 && mobileToggle) {
			mobileToggle.remove();
		}
	}, 250);
});
window.RawFeatures = {
	toggleTheme: function () {
		const currentTheme = document.documentElement.getAttribute("data-theme");
		const newTheme = currentTheme === "light" ? "dark" : "light";
		document.documentElement.setAttribute("data-theme", newTheme);
		localStorage.setItem("theme", newTheme);
		document.querySelectorAll(".theme-toggle").forEach((btn) => {
			if (newTheme === "dark") {
				btn.innerHTML = '<i class="fa-solid fa-sun"></i>';
				btn.title = "Switch to light theme";
			} else {
				btn.innerHTML = '<i class="fa-solid fa-moon"></i>';
				btn.title = "Switch to dark theme";
			}
		});
	},
	copyCode: function (button) {
		const pre = button.closest("pre.raw-code-block");
		const code = pre.querySelector("code")?.textContent || pre.textContent;
		navigator.clipboard
			.writeText(code)
			.then(() => {
				button.innerHTML = '<i class="fa-solid fa-check"></i> Copied';
				button.classList.add("copied");
				setTimeout(() => {
					button.innerHTML = '<i class="fa-regular fa-copy"></i> Copy';
					button.classList.remove("copied");
				}, 2000);
			})
			.catch(() => {
				button.innerHTML = '<i class="fa-solid fa-xmark"></i> Error';
				setTimeout(() => {
					button.innerHTML = '<i class="fa-regular fa-copy"></i> Copy';
				}, 2000);
			});
	},
};
window.EnhancedFeatures = {
	showToast: window.showToast,
	highlightSearch: highlightText,
	toggleSidebar: function () {
		const sidebar = document.querySelector(".sidebar");
		if (sidebar) sidebar.classList.toggle("active");
	},
	copyAllCode: function () {
		const allCode = Array.from(document.querySelectorAll("pre.raw-code-block code"))
			.map((code) => code.textContent)
			.join("\n\n");
		navigator.clipboard.writeText(allCode).then(() => {
			window.showToast({
				title: "Success",
				message: "All code copied to clipboard!",
				type: "success",
				duration: 3000,
			});
		});
	},
	exportAsMarkdown: function () {
		const title = document.querySelector(".page-title")?.textContent || "Document";
		const content = document.querySelector(".raw-content")?.textContent || "";
		const markdown = `# ${title}\n\n${content}`;
		const blob = new Blob([markdown], { type: "text/markdown" });
		const url = URL.createObjectURL(blob);
		const a = document.createElement("a");
		a.href = url;
		a.download = `${title.toLowerCase().replace(/\s+/g, "-")}.md`;
		document.body.appendChild(a);
		a.click();
		document.body.removeChild(a);
		URL.revokeObjectURL(url);
	},
};
window.addEventListener("error", function (e) {
	console.error("RawSSG Error:", e.error);
	if (window.showToast) {
		window.showToast({
			title: "Error",
			message: "An error occurred. Check console for details.",
			type: "error",
			duration: 5000,
		});
	}
});

// Footer-specific JavaScript (add at the end of existing JS file)
function initFooterFeatures() {
    // Back to top button functionality
    const backToTopBtn = document.querySelector('.footer-back-to-top');
    if (backToTopBtn) {
        backToTopBtn.addEventListener('click', () => {
            window.scrollTo({
                top: 0,
                behavior: 'smooth'
            });
        });
    }
    
    // Footer theme toggle
    const footerThemeToggle = document.querySelector('.footer-theme-toggle');
    if (footerThemeToggle) {
        footerThemeToggle.addEventListener('click', () => {
            if (window.RawFeatures && typeof window.RawFeatures.toggleTheme === 'function') {
                window.RawFeatures.toggleTheme();
            }
        });
    }
    
    // External links in footer
    document.querySelectorAll('.footer a[href^="http"]:not([href*="' + window.location.host + '"])').forEach(link => {
        link.setAttribute('target', '_blank');
        link.setAttribute('rel', 'noopener noreferrer');
        if (!link.classList.contains('external-link') && !link.querySelector('.fa-external-link')) {
            link.classList.add('external-link');
        }
    });
    
    // Current year for copyright
    const copyrightElements = document.querySelectorAll('.footer-copyright');
    copyrightElements.forEach(el => {
        if (el.textContent.includes('{{year}}')) {
            el.textContent = el.textContent.replace('{{year}}', new Date().getFullYear());
        }
    });
}

// Initialize footer features when DOM is loaded
document.addEventListener('DOMContentLoaded', function() {
    initFooterFeatures();
});