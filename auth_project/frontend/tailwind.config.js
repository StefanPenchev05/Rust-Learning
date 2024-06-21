/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{js,jsx,tx,tsx}"],
  theme: {
    extend: {
      keyframes: {
        reveal: {
          '0%': { 'width': '0' },
          '100%': { 'width': '100%' },
        },
      },
      animation: {
        reveal: 'reveal 2s linear forwards',
      },
    },
  },
  plugins: [],
}

