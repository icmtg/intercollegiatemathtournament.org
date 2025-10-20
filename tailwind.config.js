/** @type {import('tailwindcss').Config} */
module.exports = {
  theme: {
    extend: {
      typography: {
        DEFAULT: {
          css: {
            maxWidth: "none",
            a: {
              textDecoration: "none",
              fontWeight: "medium",
              color: "var(--color-blue-500)",
              transitionProperty: "box-shadow",
              transitionTimingFunction: "var(--default-transition-timing-function)",
              transitionDuration: "var(--default-transition-duration)",
              boxShadow: `inset 0 -0.4em var(--color-blue-100)`,
              "&:hover": {
                boxShadow: `inset 0 -0.6em var(--color-blue-200)`,
              },
            },
          },
        },
      },
    },
  },
};
