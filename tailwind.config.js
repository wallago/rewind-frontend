/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  variants: {
    extend: {
      backgroundColor: ["dark", "hover", "dark:hover"],
    },
  },
  theme: {
    extend: {},
  },
  plugins: [],
};
