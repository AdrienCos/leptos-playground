// const { default: daisyui } = require('daisyui');
import catppuccin from "@catppuccin/daisyui";
import daisyui from "daisyui";

/** @type {import('tailwindcss').Config} */
module.exports = {
	content: {
		files: ["*.html", "./src/**/*.rs"],
		transform: {
			rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
		},
	},
	theme: {
		extend: {},
	},
	plugins: [require("daisyui")],
	daisyui: {
		themes: [catppuccin("frappe", { primary: "sky", secondary: "lavender" })],
	},
};
