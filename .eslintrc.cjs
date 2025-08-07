import js from '@eslint/js';
import svelte from 'eslint-plugin-svelte';
import globals from 'globals';
import svelteConfig from './svelte.config.js';

/** @type {import('eslint').Linter.Config[]} */
export default [
	js.configs.recommended,
	...svelte.configs.recommended,
	...svelte.configs.prettier,
	{
		languageOptions: {
			ecmaVersion: 'latest',
			sourceType: 'module',
			globals: {
				...globals.browser,
				...globals.es2021,
				...globals.node
			}
		}
	},
	{
		files: ['**/*.svelte'],
		languageOptions: {
			parserOptions: {
				svelteConfig
			}
		}
	},
	{
		rules: {}
	}
];
