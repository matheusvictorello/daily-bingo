import { api } from '$lib/auth.svelte';
import type { Bingo } from '$lib/bingo';

export const load = async () => ({ bingos: await api<Bingo[]>('/bingos') });
