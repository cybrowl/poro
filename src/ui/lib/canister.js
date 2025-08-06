import { Actor } from '@dfinity/agent';
import { idlFactory } from '../../../.dfx/local/canisters/user/service.did.js'; // Use local-generated IDL; consistent for prod if interface matches
import { authStore } from './auth.js';

let cachedActor = null;

export function getActor() {
	let agent;
	authStore.subscribe(({ agent: storeAgent }) => {
		agent = storeAgent;
	});

	if (!agent) throw new Error('Not authenticated');

	if (!cachedActor) {
		console.log('import.meta.env:', import.meta.env);
		const canisterId =
			import.meta.env.VITE_DEPLOY_ENV === 'local'
				? 'uzt4z-lp777-77774-qaabq-cai'
				: 'o4m5w-fiaaa-aaaaj-a2fjq-cai';
		console.log('Using canisterId:', canisterId);
		cachedActor = Actor.createActor(idlFactory, { agent, canisterId });
	}
	return cachedActor;
}
