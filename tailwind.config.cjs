module.exports = {
	mode: 'jit',
	content: ['./src/ui/**/*.svelte', './src/**/*.{html,js}'],
	darkMode: 'class', // or 'media' or 'class'
	theme: {
		extend: {
			fontFamily: {
				sans: ['Roboto', 'sans-serif']
			},
			colors: {
				backdrop: '#1B1A22'
			}
		}
	},
	variants: {
		extend: {}
	},
	plugins: []
};
