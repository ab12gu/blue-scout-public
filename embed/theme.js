// Function to set theme based on user preference or system preference
function setTheme() {
  // Check if user has previously saved a theme preference
  const savedTheme = localStorage.getItem("theme");

  if (savedTheme) {
    // Apply saved theme preference
    document.documentElement.setAttribute("data-theme", savedTheme);
  } else {
    // If no saved preference, check system preference
    const prefersDarkMode = window.matchMedia(
      "(prefers-color-scheme: dark)",
    ).matches;

    // Set theme based on system preference
    const defaultTheme = prefersDarkMode ? "dark" : "light";
    document.documentElement.setAttribute("data-theme", defaultTheme);

    // Save this preference for future visits
    localStorage.setItem("theme", defaultTheme);
  }
}

// Toggle theme function (can be used with a button)
function toggleTheme() {
  const currentTheme = document.documentElement.getAttribute("data-theme");
  const newTheme = currentTheme === "light" ? "dark" : "light";

  // Apply new theme
  document.documentElement.setAttribute("data-theme", newTheme);

  // Save the preference
  localStorage.setItem("theme", newTheme);
}

setTheme();

// Export toggle function for other scripts to use
window.toggleTheme = toggleTheme;
