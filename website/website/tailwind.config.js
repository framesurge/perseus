const defaultTheme = require("tailwindcss/defaultTheme");
const colors = require("tailwindcss/colors");

module.exports = {
    purge: {
        mode: "all",
        content: [
            "./src/**/*.rs",
            "./index.html",
            "./src/**/*.html",
            "./src/**/*.css",
            "./static/**/*.css",
        ],
    },
    darkMode: process.env.NODE_ENV == "production" ? "media" : "class",
    theme: {
        fontFamily: {
            heading: ["Comfortaa", "sans"],
            ...defaultTheme.fontFamily,
        },
        screens: {
            "2xs": "370px",
            xs: "475px",
            ...defaultTheme.screens,
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
            indigo: colors.indigo,
        },
    },
    variants: {},
    plugins: [require("tailwind-hamburgers")],
};
