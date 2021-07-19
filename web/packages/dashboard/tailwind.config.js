const colors = require('tailwindcss/colors');

/** @type {import('tailwindcss/tailwind-config').TailwindConfig} */
module.exports = {
  purge: {
    content:
      process.env.NODE_ENV === 'production'
        ? ['./src/**/*.html', './src/**/*.js', './src/**/*.jsx', './src/**/*.ts', './src/**/*.tsx']
        : [],
  },
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {},
    fontFamily: {
      serif: ['"Zilla Slab"', 'serif'],
      sans: ['Lato', 'Helvetica', 'Arial', 'sans-serif'],
      mono: ['"JetBrains Mono"', 'ui-monospace', 'monospace'],
    },
    colors: {
      gray: colors.trueGray,
      red: colors.rose,
      green: colors.green,
      blue: colors.sky,
      pink: colors.fuchsia,
      purple: colors.purple,
      violet: colors.violet,
      white: colors.white,
      black: colors.black,
    },
  },
  variants: {
    extend: {
      backgroundColor: ['selection', 'disabled'],
      textColor: ['selection', 'disabled'],
      cursor: ['disabled'],
      ringColor: ['disabled'],
      gradientColorStops: ['disabled'],
    },
  },
  plugins: [require('tailwindcss-selection-variant')],
};
