/** @type {import('tailwindcss').Config} */
export default {
    mode: "all",
    content: [
        "./src/**/*.{js,rs,html,css}",
        "./dist/**/*.html"
    ],
    theme: {
        extend: {},
    },
    plugins: [
        require('daisyui'),
    ],
    daisyui: {
        themes: [false],
    },
}