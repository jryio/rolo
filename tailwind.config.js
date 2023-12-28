/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    // HTML template is done via Maud in Rust, so classes will be there
    "./api/src/**/*.rs",
    // Optionally some static assets may have tailwind classes, look for those as well
    // "./static/**/*.{html,js}",
  ],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
};
