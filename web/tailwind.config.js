const colors = require('tailwindcss/colors');

/** @type {import('tailwindcss/tailwind-config').TailwindConfig} */
module.exports = {
  purge: {
    content:
      process.env.NODE_ENV === 'production'
        ? ['./src/**/*.html', './src/**/*.js', './src/**/*.jsx', './src/**/*.ts', './src/**/*.tsx']
        : [],
    // from CButton.tsx since it doesn't use jsx' className
    // safelist: [
    //   'inline-flex',
    //   'justify-center',
    //   'items-center',
    //   'gap-2',
    //   'px-6',
    //   'm-1',
    //   'h-8',
    //   'select-none',
    //   'uppercase',
    //   'bg-gradient-to-r',
    //   'rounded-md',
    //   'text-black',
    //   'text-sm',
    //   'font-bold',
    //   'shadow-sm',
    //   'disabled:from-gray-400',
    //   'disabled:to-gray-500',
    //   'disabled:cursor-not-allowed',
    //   'disabled:ring-gray-600',
    //   'disabled:text-gray-700',
    //   'hover:bg-red-dark',
    //   'hover:shadow-md',
    //   'transition-colors',
    //   'transition-shadow',
    //   'focus:ring-2',
    //   'focus:outline-none',
    //   'focus:shadow-md',
    //   'from-pink-400',
    //   'to-pink-500',
    //   'focus:ring-pink-700',
    //   'from-red-400',
    //   'to-red-500',
    //   'focus:ring-red-700',
    // ],
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
