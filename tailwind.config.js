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
        "primary-3": "var(--primary-color-3)",
        "primary-4": "var(--primary-color-4)",
        "primary-5": "var(--primary-color-5)",
        "primary-6": "var(--primary-color-6)",
        "primary-7": "var(--primary-color-7)",

        secondary: "var(--secondary-color)",
        "secondary-1": "var(--secondary-color-1)",
        "secondary-2": "var(--secondary-color-2)",
        "secondary-3": "var(--secondary-color-3)",
        "secondary-4": "var(--secondary-color-4)",
        "secondary-5": "var(--secondary-color-5)",
        "secondary-6": "var(--secondary-color-6)",

        focus: "var(--focused-border-color)",
        "success-primary": "var(--primary-success-color)",
        "success-secondary": "var(--secondary-success-color)",
        "warning-primary": "var(--primary-warning-color)",
        "warning-secondary": "var(--secondary-warning-color)",
        "error-primary": "var(--primary-error-color)",
        "error-secondary": "var(--secondary-error-color)",
        "info-primary": "var(--primary-info-color)",
        "info-secondary": "var(--secondary-info-color)",
        "error-contrast": "var(--contrast-error-color)",
      },
    },
  },
  plugins: [],
};
