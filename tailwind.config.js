/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [
		'./src/**/*.html,',
		'./src/**/*.rs',
		'./src/*.rs',
		'index.html'],

	theme: {
		extend: {},
	},
	plugins: [
		require('@tailwindcss/typography'),
    	require('@tailwindcss/forms'),
	],
};
