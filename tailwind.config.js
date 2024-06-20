const {fontFamily} = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./templates/*.{html,js}'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter var', ...fontFamily.sans],
      },
    },
  },
}
