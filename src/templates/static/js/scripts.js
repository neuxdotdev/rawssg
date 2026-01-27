// ==== Theme Toggle ====
const themeToggle = document.createElement('button');
themeToggle.className = 'theme-toggle';
themeToggle.textContent = '🌓';
themeToggle.title = 'Toggle theme';

themeToggle.onclick = () => {
    const current = document.documentElement.getAttribute('data-theme');
    const next = current === 'light' ? 'dark' : 'light';
    document.documentElement.setAttribute('data-theme', next);
    localStorage.setItem('theme', next);
};

// ==== Sidebar Toggle (Mobile) ====
const sidebarToggle = document.querySelector('.sidebar-toggle');
const sidebar = document.querySelector('.sidebar');

if (sidebarToggle && sidebar) {
    sidebarToggle.onclick = () => sidebar.classList.toggle('active');

    // Close sidebar when clicking outside
    document.addEventListener('click', (e) => {
        if (window.innerWidth <= 768) {
            if (!sidebar.contains(e.target) && !sidebarToggle.contains(e.target)) {
                sidebar.classList.remove('active');
            }
        }
    });
}

// ==== Append Theme Toggle Button ====
const sidebarHeader = document.querySelector('.sidebar-header');
if (sidebarHeader) sidebarHeader.appendChild(themeToggle);

// ==== Init Theme From Storage ====
const savedTheme = localStorage.getItem('theme');
if (savedTheme) document.documentElement.setAttribute('data-theme', savedTheme);
