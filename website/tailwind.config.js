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
        extend: {
            colors: {
                navy: "#001122",
                "navy-deep": "#000B14",
                // Colors from meshes
                "mesh-purple": "#7566e4",
                "mesh-lilac-dark": "#a06fd2",
                "mesh-lilac-light": "#b18ed7",
                "mesh-pink": "#db80bd",
                "dark-mesh-purple": "#52479F",
                "dark-mesh-lilac-dark": "#704E92",
                "dark-mesh-lilac-light": "#7C6396",
                "dark-mesh-pink": "#9A5984",
            },
        },
    },
    variants: {},
    plugins: [require("tailwind-hamburgers")],
};
