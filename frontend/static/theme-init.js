(() => {
  try {
    const saved = localStorage.getItem('steamer-theme') || 'system';
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    const isDark = saved === 'dark' || (saved === 'system' && prefersDark);
    document.documentElement.classList.toggle('dark', isDark);
  } catch {}
})();
