(function () {
  try {
    var theme = localStorage.getItem("theme") || "system";
    var root = document.documentElement;
    var prefersDark =
      window.matchMedia &&
      window.matchMedia("(prefers-color-scheme: dark)").matches;

    root.classList.remove("dark");
    root.classList.remove("light");

    if (theme === "dark") {
      root.classList.add("dark");
    } else if (theme === "light") {
      root.classList.add("light");
    } else if (prefersDark) {
      root.classList.add("dark");
    }
  } catch (_) {
    // Ignore storage or DOM access issues.
  }
})();
