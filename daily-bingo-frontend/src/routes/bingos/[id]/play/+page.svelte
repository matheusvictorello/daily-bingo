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

<a class="back" href="/bingos/{data.bingo.id}">← Back</a>
<h1>Play</h1>

<p class="status" class:complete aria-live="polite">
	{#if complete}
		<strong>BINGO!</strong>
	{:else}
		Mark cells as they happen.
	{/if}
</p>

<BingoGrid bingo={data.bingo} {marked} {won} ontoggle={toggle} />

<div class="actions"><button class="secondary" onclick={() => save([])}>Reset</button></div>

<style>
	.status { margin: 0 0 1rem; color: var(--text-muted); }
	.complete { color: var(--won); font-size: 2rem; font-weight: 900; letter-spacing: 0.05em; }
</style>
