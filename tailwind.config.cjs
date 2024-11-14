module.exports = {
	mode: 'jit',
	content: ['./src/ui/**/*.svelte', './src/**/*.{html,js}'],
	darkMode: 'class', // or 'media' or 'class'
	theme: {
		extend: {
			fontFamily: {
				sans: ['Nano Sans', 'sans-serif']
			},
			colors: {
				'black-a': '#0A0A0B',
				'black-b': '#1E1E1E',
				merigold: '#DAA520'
			},
			boxShadow: {
				gray: '0 10px 15px -3px rgba(58, 58, 80, 0.5), 0 4px 6px -2px rgba(58, 58, 80, 0.3)'
			},
			screens: {
				'3xl': '2222px'
			}
		}
	},
	variants: {
		extend: {}
	},
	plugins: []
};
