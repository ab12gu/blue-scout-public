// Navigation and page transition functionality
document.addEventListener("DOMContentLoaded", function () {
  // preloadPage("/view-data");
  // preloadPage("/settings");
  // preloadPage("/");
  // Navigation elements
  const addDataButton = document.getElementById("addDataButton");
  const viewDataButton = document.getElementById("viewDataButton");
  const settingsButton = document.getElementById("settingsButton");

  // Page URLs
  const PAGE_URLS = {
    ADD_DATA: "/",
    VIEW_DATA: "/view-data",
    SETTINGS: "/settings",
  };

  // Apply initial state based on current URL
  updateActiveButton();

  // Set up navigation event listeners
  if (addDataButton) {
    addDataButton.addEventListener("click", function () {
      fadeAndNavigate(PAGE_URLS.ADD_DATA);
    });
  }

  if (viewDataButton) {
    viewDataButton.addEventListener("click", function () {
      fadeAndNavigate(PAGE_URLS.VIEW_DATA);
    });
  }

  if (settingsButton) {
    settingsButton.addEventListener("click", function () {
      fadeAndNavigate(PAGE_URLS.SETTINGS);
    });
  }

  // Helper function to navigate with fade transition
  function fadeAndNavigate(url) {
    // Don't navigate if we're already on this page
    if (isCurrentPage(url)) return;

    // Get the page element
    const page = document.querySelector(".page");

    if (page) {
      // Add the fade-out class to start the transition
      page.style.opacity = "0";

      // Wait for the animation to complete before navigating
      setTimeout(() => {
        window.location.href = url;
      }, 300); // Match this to the CSS transition duration
    } else {
      // If no page element found, navigate immediately
      window.location.href = url;
    }
  }

  // Helper function to check if we're on the current page
  function isCurrentPage(url) {
    const path = window.location.pathname;

    if (url === PAGE_URLS.ADD_DATA) {
      return path === "/" || path === "/index.html";
    } else if (url === PAGE_URLS.VIEW_DATA) {
      return path === "/view-data" || path === "/view_data.html";
    } else if (url === PAGE_URLS.SETTINGS) {
      return path === "/settings" || path === "/settings.html";
    }

    return false;
  }

  function preloadPage(url) {
    const link = document.createElement("link");
    link.rel = "prefetch";
    link.href = url;
    document.head.appendChild(link);
  }

  // Update active button based on current URL
  function updateActiveButton() {
    if (!addDataButton || !viewDataButton || !settingsButton) {
      return;
    }

    // Reset all buttons to default state
    addDataButton.className = "outline-none";
    viewDataButton.className = "outline-none";
    settingsButton.className = "outline-none";

    // Set active button based on current path
    const path = window.location.pathname;

    if (path === "/" || path === "/index.html") {
      addDataButton.className = "dock-active outline-none";
    } else if (path === "/view-data" || path === "/view_data.html") {
      viewDataButton.className = "dock-active outline-none";
    } else if (path === "/settings" || path === "/settings.html") {
      settingsButton.className = "dock-active outline-none";
    }
  }

  // Handle the fade-in animation for the current page
  const page = document.querySelector(".page");
  if (page) {
    // Ensure the page starts at opacity 0
    page.style.transition = "opacity 0.3s ease-in-out";
    page.style.opacity = "0";

    // After a small delay, fade in the page
    setTimeout(() => {
      page.style.opacity = "1";
    }, 50);
  }
});
