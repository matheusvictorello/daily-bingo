<script lang="ts">
	import { goto } from '$app/navigation';
	import { api } from '$lib/auth.svelte';
	import BingoEditor from '$lib/BingoEditor.svelte';
	import type { BingoInfo } from '$lib/bingo';

	let { data } = $props();

	async function save(info: BingoInfo) {
		await api(`/bingos/${data.bingo.id}`, { method: 'PUT', body: JSON.stringify(info) });
		// Re-run the [id] layout load so the detail page shows the saved bingo.
		await goto(`/bingos/${data.bingo.id}`, { invalidateAll: true });
	}
</script>

<p><a href="/bingos/{data.bingo.id}">← Back</a></p>
<h1>Edit Bingo</h1>
<BingoEditor initial={data.bingo} submitLabel="Save" onsave={save} />
