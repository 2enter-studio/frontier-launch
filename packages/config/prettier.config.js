export default {
	useTabs: true,
	singleQuote: true,
	trailingComma: 'none',
	printWidth: 150,
	plugins: ['prettier-plugin-svelte', 'prettier-plugin-tailwindcss'],
	overrides: [{ files: '*.svelte', options: { parser: 'svelte' } }]
};
