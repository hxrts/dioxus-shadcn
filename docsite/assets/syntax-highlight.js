// Syntax highlighting using highlight.js
(function () {
  // Load highlight.js from CDN
  const script = document.createElement("script");
  script.src =
    "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js";
  script.async = true;

  script.onload = function () {
    // Load additional languages
    const rustScript = document.createElement("script");
    rustScript.src =
      "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/rust.min.js";
    rustScript.async = true;
    rustScript.onload = initHighlighting;
    document.head.appendChild(rustScript);
  };

  document.head.appendChild(script);

  // Load the CSS theme (GitHub Dark style to match shadcn)
  const link = document.createElement("link");
  link.rel = "stylesheet";
  link.href =
    "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github-dark.min.css";
  document.head.appendChild(link);

  function initHighlighting() {
    // Configure highlight.js
    if (window.hljs) {
      hljs.configure({
        ignoreUnescapedHTML: true,
        languages: ["rust", "bash", "shell", "toml", "json", "html", "css", "javascript", "typescript"],
      });

      // Initial highlighting
      highlightAll();

      // Set up MutationObserver to highlight dynamically added code blocks
      const observer = new MutationObserver(function (mutations) {
        let shouldHighlight = false;
        mutations.forEach(function (mutation) {
          if (mutation.addedNodes.length) {
            mutation.addedNodes.forEach(function (node) {
              if (node.nodeType === 1) {
                if (
                  node.tagName === "CODE" ||
                  node.querySelector("code[class*='language-']")
                ) {
                  shouldHighlight = true;
                }
              }
            });
          }
        });
        if (shouldHighlight) {
          highlightAll();
        }
      });

      observer.observe(document.body, {
        childList: true,
        subtree: true,
      });
    }
  }

  function highlightAll() {
    document.querySelectorAll("pre code[class*='language-']").forEach(function (block) {
      if (!block.dataset.highlighted) {
        hljs.highlightElement(block);
        block.dataset.highlighted = "yes";
      }
    });
  }

  // Expose for manual triggering if needed
  window.highlightCodeBlocks = function () {
    if (window.hljs) {
      highlightAll();
    }
  };
})();
