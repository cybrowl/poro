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
	await authClient.login({
		identityProvider:
			process.env.DEPLOY_ENV === 'local'
				? 'http://localhost:4943?canisterId=uzt4z-lp777-77774-qaabq-cai'
				: 'https://identity.ic0.app',
		maxTimeToLive: BigInt(8 * 60 * 60 * 1000 * 1000 * 1000), // 8 hours
		onSuccess: () => handleSuccess(authClient),
		onError: (err) => console.error('Login failed:', err)
	});
}

function handleSuccess(client) {
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
}

export async function logout() {
	await authClient?.logout();
	authStore.set({ isAuthenticated: false, identity: null, agent: null, principal: null });
}
