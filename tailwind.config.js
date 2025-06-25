/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  variants: {
    extend: {
      backgroundColor: ["dark", "hover", "dark:hover", "active", "dark:active"],
    },
  },
  theme: {
    extend: {
      captionSide: {
        bottom: "bottom",
      },
      fontFamily: {
        frida: ['"Frida Code"', "monospace"],
      },
      colors: {
        surface: {
          light: "#dad7cd",
          dark: "#0c1821",
        },
        "surface-variant": {
          light: "#BFBAAB",
          dark: "#142532",
        },
        text: {
          light: "#344e41",
          dark: "#ccc9dc",
        },
        border: {
          light: "#588157",
          dark: "#324a5f",
        },
        muted: {
          light: "#a3b18a",
          dark: "#1b2a41",
        },
        error: {
          light: "#9d0208",
          dark: "#f28482",
        },
        low: {
          light: "#91CBBB",
          dark: "#5AAA95",
        },
        medium: {
          light: "#DEC746",
          dark: "#F4D35E",
        },
        high: {
          light: "#fc7753",
          dark: "#D64933",
        },
      },
    },
  },
  plugins: [],
};
