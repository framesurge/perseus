const defaultTheme = require("tailwindcss/defaultTheme");
const colors = require("tailwindcss/colors");

module.exports = {
    content: [
        "./src/**/*.{rs,html,css}",
        "./index.html",
        "./static/styles/**/*.css",
    ],
    theme: {
        fontFamily: {
            heading: ["Comfortaa", "sans"],
            ...defaultTheme.fontFamily,
        },
        screens: {
            "2xs": "370px",
            xs: "475px",
            ...defaultTheme.screens,
            "3xl": "1792px",
        },
        colors: {
            // Full color palette
            transparent: "transparent",
            current: "currentColor",
            navy: "#001122",
            "navy-deep": "#000B14",
            black: colors.black,
            white: colors.white,
            gray: colors.coolGray,
            red: colors.red,
            orange: colors.orange,
            amber: colors.amber,
            yellow: colors.amber,
            green: colors.emerald,
            lime: colors.lime,
            indigo: colors.indigo,
            // Colors from meshes
            "mesh-purple": "#7566e4",
            "mesh-lilac-dark": "#a06fd2",
            "mesh-lilac-light": "#b18ed7",
            "mesh-pink": "#db80bd"
        },
    },
    variants: {},
    plugins: [require("tailwind-hamburgers")],
};
