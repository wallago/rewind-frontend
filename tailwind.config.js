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
    extend: {
      fontFamily: {
        frida: ['"Frida Code"', "monospace"],
      },
      colors: {
        surface: {
          light: "#d8f3dc",
          dark: "#240046",
        },
        text: {
          light: "#081c15",
          dark: "#e0aaff",
        },
        accent: {
          light: "#52b788",
          dark: "#7b2cbf",
        },
        border: {
          light: "#95d5b2",
          dark: "#5a189a",
        },
        muted: {
          light: "#b7e4c7",
          dark: "#3c096c",
        },
        error: {
          light: "#6e1423",
          dark: "#6e1423",
        },
      },
    },
  },
  plugins: [],
};
