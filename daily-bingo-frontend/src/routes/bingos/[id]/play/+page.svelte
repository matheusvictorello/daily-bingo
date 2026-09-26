<script lang="ts">
	import BingoGrid from '$lib/BingoGrid.svelte';
	import { winningLines } from '$lib/bingo';

	let { data } = $props();

	// Marks are per bingo per day, kept in this browser only.
	let key = $derived(`bingo:${data.bingo.id}:${new Date().toDateString()}`);
	let marked: boolean[] = $derived(read(key));
	let lines = $derived(winningLines(data.bingo, marked));
	let won = $derived(new Set(lines.flat()));
	let complete = $derived(
		data.bingo.values.every((cell, i) => cell === 'Gap' || cell === 'Empty' || marked[i])
	);

	function read(key: string): boolean[] {
		try {
			return JSON.parse(localStorage.getItem(key) ?? '[]');
		} catch {
			return [];
		}
	}

	function save(next: boolean[]) {
		marked = next;
		try {
			localStorage.setItem(key, JSON.stringify(next));
		} catch {}
	}

	function toggle(i: number) {
		const next = [...marked];
		next[i] = !next[i];
		save(next);
	}
</script>

<p><a href="/bingos/{data.bingo.id}">← Back</a></p>
<h1>Play</h1>

<p aria-live="polite">
	{#if complete}
		<strong>BINGO!</strong>
	{:else}
		Mark cells as they happen.
	{/if}
</p>

<BingoGrid bingo={data.bingo} {marked} {won} ontoggle={toggle} />

<p><button onclick={() => save([])}>Reset</button></p>
