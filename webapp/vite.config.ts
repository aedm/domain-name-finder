import { defineConfig } from 'vite';
import path from 'path';
import vue from '@vitejs/plugin-vue';

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [vue()],
	resolve: {
		alias: {
			'@': path.resolve(__dirname, '/src'),
		},
	},
	server: {
		proxy: {
			// string shorthand
			'/api': 'http://localhost:8000',
			// with options
			// '/api': {
			// 	target: 'http://jsonplaceholder.typicode.com',
			// 	changeOrigin: true,
			// 	rewrite: (path) => path.replace(/^\/api/, '')
			// },
			// // with RegEx
			// '^/fallback/.*': {
			// 	target: 'http://jsonplaceholder.typicode.com',
			// 	changeOrigin: true,
			// 	rewrite: (path) => path.replace(/^\/fallback/, '')
			// },
			// // Using the proxy instance
			// '/api': {
			// 	target: 'http://jsonplaceholder.typicode.com',
			// 	changeOrigin: true,
			// 	configure: (proxy, options) => {
			// 		// proxy will be an instance of 'http-proxy'
			// 	}
			// }
		}
	}
});
