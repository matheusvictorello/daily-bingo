<script lang="ts">
	import BingoGrid from '$lib/BingoGrid.svelte';

	let { data } = $props();
</script>

<header>
	<h1>Your Bingos</h1>
	<a class="button" href="/bingos/new">+ New Bingo</a>
</header>

{#if data.bingos.length === 0}
	<p class="card muted">No bingos yet. Create one to get started.</p>
{/if}

<ul>
	{#each data.bingos as bingo (bingo.id)}
		<li class="card">
			<a class="preview" href="/bingos/{bingo.id}" aria-label="Open {bingo.cols}×{bingo.rows} bingo">
				<BingoGrid {bingo} small />
			</a>
			<span class="muted">{bingo.cols}×{bingo.rows}</span>
			<span class="row">
				<a class="button" href="/bingos/{bingo.id}/play">Play</a>
				<a class="button secondary" href="/bingos/{bingo.id}/edit">Edit</a>
			</span>
		</li>
	{/each}
</ul>

<style>
	header { display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 1rem; margin-bottom: 1rem; }
	header h1 { margin: 0; }
	ul { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: repeat(auto-fill, minmax(15rem, 1fr)); gap: 1rem; }
	li { display: flex; flex-direction: column; gap: 0.75rem; }
	.preview { display: grid; place-items: center; min-height: 10rem; border-radius: var(--radius); background: var(--surface-muted); padding: 0.75rem; }
	.row { display: flex; gap: 0.5rem; }
	.row .button { flex: 1; }
</style>
