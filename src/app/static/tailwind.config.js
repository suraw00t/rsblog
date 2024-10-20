/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "../templates/**/*.{html,js}",
    // "./src/**/*.{html,js}",
    "./node_modules/preline/dist/*.js",
  ],
  darkMode: "selector",
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms"), require("preline/plugin")],
};
