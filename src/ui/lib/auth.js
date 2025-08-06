import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent } from '@dfinity/agent';
import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import * as dotenv from 'dotenv';

if (!browser) dotenv.config(); // Load .env server-side

export const authStore = writable({
	isAuthenticated: false,
	identity: null,
	agent: null,
	principal: null
});

let authClient;

export async function initAuth() {
	authClient = await AuthClient.create({
		idleOptions: { disableIdle: false, idleTimeout: 30 * 60 * 1000 } // Auto-logout after 30 min inactivity
	});
	if (await authClient.isAuthenticated()) {
		handleSuccess(authClient);
	}
}

export async function login() {
	const options = {
		identityProvider:
			process.env.DEPLOY_ENV === 'local'
				? 'http://uxrrr-q7777-77774-qaaaq-cai.localhost:4943'
				: 'https://id.ai',
		maxTimeToLive: BigInt(7 * 24 * 60 * 60 * 1000 * 1000 * 1000), // 7 days for prod UX
		windowOpenerFeatures: 'toolbar=0,location=0,menubar=0,width=500,height=500,left=100,top=100', // Popup for seamless flow
		onSuccess: () => {
			console.log('Login success! Updating store...');
			handleSuccess(authClient);
		},
		onError: (err) => {
			console.error('Login failed:', err);
			alert('Auth error: ' + err.message);
		}
	};

	// Add derivationOrigin only in production for privacy
	if (process.env.DEPLOY_ENV !== 'local') {
		options.derivationOrigin = window.location.origin;
	}

	await authClient.login(options);
}

function handleSuccess(client) {
	console.log('Handling success...');
	const identity = client.getIdentity();
	const agent = new HttpAgent({ identity });

	if (process.env.DEPLOY_ENV === 'local') {
		agent.fetchRootKey().catch(console.error);
	}

	authStore.set({
		isAuthenticated: true,
		identity,
		agent,
		principal: identity.getPrincipal().toText()
	});
	console.log('Store updated with principal:', identity.getPrincipal().toText());
}

export async function logout() {
	await authClient?.logout();
	authStore.set({ isAuthenticated: false, identity: null, agent: null, principal: null });
}
