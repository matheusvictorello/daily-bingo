import { api } from '$lib/auth.svelte';
import type { Bingo, BingoInfo } from '$lib/bingo';

export const load = async ({ params }) => ({
	bingo: { id: params.id, ...(await api<BingoInfo>(`/bingos/${params.id}`)) } satisfies Bingo
});
