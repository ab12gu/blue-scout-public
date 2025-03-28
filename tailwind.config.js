const daisyui = require("daisyui");

module.exports = {
  content: ["./src/**/*.{html,js,ts,jsx,tsx,rs}"],
  theme: {
    extend: {},
  },
  plugins: [daisyui],
  daisyui: {
    themes: ["light", "dark"],
  },
};
