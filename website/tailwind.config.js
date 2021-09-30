const defaultTheme = require("tailwindcss/defaultTheme");
const colors = require("tailwindcss/colors");
const plugin = require("tailwindcss/plugin");

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
            // TODO trim this down to what's needed
            transparent: "transparent",
            current: "currentColor",
            navy: "#001122",
            "navy-deep": "#000B14",
            black: colors.black,
            white: colors.white,
            gray: colors.coolGray,
            red: colors.red,
            yellow: colors.amber,
            green: colors.emerald,
            blue: colors.blue,
            indigo: colors.indigo,
            purple: colors.violet,
            pink: colors.pink,
            cyan: colors.cyan,
            fuchsia: colors.fuchsia,
            sky: colors.sky,
            teal: colors.teal,
        },
    },
    variants: {},
    plugins: [
        require("tailwind-hamburgers"),
        // Hack to show a different SVg in dark mode
        plugin(function ({ addUtilities }) {
            const newUtilities = {
                ".bg-waves": {
                    background:
                        'url("/.perseus/static/light_landing_page.svg")',
                    "background-size": "cover",
                },
                ".bg-waves-dark": {
                    background: 'url("/.perseus/static/dark_landing_page.svg")',
                    "background-size": "cover",
                },
            };

            addUtilities(newUtilities, ["dark"]);
        }),
    ],
};
