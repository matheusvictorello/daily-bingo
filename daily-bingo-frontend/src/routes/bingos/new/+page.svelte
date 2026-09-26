<script lang="ts">
	import { goto } from '$app/navigation';
	import { api } from '$lib/auth.svelte';
	import BingoEditor from '$lib/BingoEditor.svelte';
	import type { BingoInfo } from '$lib/bingo';

	async function create(info: BingoInfo) {
		const { id } = await api<{ id: string }>('/bingos', { method: 'POST', body: JSON.stringify(info) });
		await goto(`/bingos/${id}`);
	}
</script>

<a class="back" href="/bingos">← All bingos</a>
<h1>New Bingo</h1>
<BingoEditor submitLabel="Create" onsave={create} />
