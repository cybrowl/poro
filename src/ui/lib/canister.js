import { Actor } from "@dfinity/agent";

import { idlFactory as userIdlFactory } from "../../../.dfx/local/canisters/user/service.did.js";

import { authStore } from "./auth.js";

// Cache actors by canister name for efficiency
const cachedActors = new Map();

// Canister configurations: Map names to IDL and IDs (local/prod)
const canisterConfigs = {
  user: {
    idlFactory: userIdlFactory,
    localId: "uzt4z-lp777-77774-qaabq-cai",
    prodId: "o4m5w-fiaaa-aaaaj-a2fjq-cai",
  },
};

export function getActor(canisterName) {
  if (!canisterConfigs[canisterName]) {
    throw new Error(`Unknown canister: ${canisterName}`);
  }

  if (cachedActors.has(canisterName)) {
    return cachedActors.get(canisterName);
  }

  let agent;
  authStore.subscribe(({ agent: storeAgent }) => {
    agent = storeAgent;
  });

  if (!agent) {
    throw new Error("Not authenticated");
  }

  console.log("import.meta.env:", import.meta.env);
  const config = canisterConfigs[canisterName];
  const canisterId =
    import.meta.env.VITE_DEPLOY_ENV === "local"
      ? config.localId
      : config.prodId;
  console.log(`Using ${canisterName} canisterId:`, canisterId);

  const actor = Actor.createActor(config.idlFactory, { agent, canisterId });
  cachedActors.set(canisterName, actor);
  return actor;
}
