/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  variants: {
    extend: {
      backgroundColor: ["dark"],
    },
  },
  theme: {
    extend: {
      captionSide: {
        bottom: "bottom",
      },
      colors: {
        primary: "var(--primary-color)",
        "primary-1": "var(--primary-color-1)",
        "primary-2": "var(--primary-color-2)",

        secondary: "var(--secondary-color)",
        "secondary-1": "var(--secondary-color-1)",
        "secondary-2": "var(--secondary-color-2)",

        "priority-low": "var(--priority-low-color)",
        "priority-medium": "var(--priority-medium-color)",
        "priority-high": "var(--priority-high-color)",

        // focus: "var(--focused-border-color)",
        // "success-primary": "var(--primary-success-color)",
        // "success-secondary": "var(--secondary-success-color)",
        // "warning-primary": "var(--primary-warning-color)",
        // "warning-secondary": "var(--secondary-warning-color)",
        // "error-primary": "var(--primary-error-color)",
        // "error-secondary": "var(--secondary-error-color)",
        // "info-primary": "var(--primary-info-color)",
        // "info-secondary": "var(--secondary-info-color)",
        // "error-contrast": "var(--contrast-error-color)",
      },
    },
  },
  plugins: [],
};
