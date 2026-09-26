<script lang="ts">
	import type { BingoInfo } from '$lib/bingo';

	let {
		bingo,
		marked = [],
		won = new Set<number>(),
		ontoggle,
		small = false
	}: {
		bingo: BingoInfo;
		marked?: boolean[];
		won?: Set<number>;
		ontoggle?: (i: number) => void;
		small?: boolean;
	} = $props();
</script>

<div class="grid-scroll">
	<div class="grid" class:small style:grid-template-columns="repeat({bingo.cols}, var(--size))">
		{#each bingo.values as cell, i}
			{#if cell === 'Gap'}
				<div></div>
			{:else if cell === 'Empty'}
				<div class="cell free" class:won={won.has(i)}>{small ? '' : '★ Free'}</div>
			{:else if ontoggle}
				<button
					class="cell"
					class:marked={marked[i]}
					class:won={won.has(i)}
					aria-pressed={!!marked[i]}
					onclick={() => ontoggle(i)}>{cell.Filled}</button
				>
			{:else}
				<div class="cell">{small ? '' : cell.Filled}</div>
			{/if}
		{/each}
	</div>
</div>

<style>
	.grid { --size: var(--cell); display: grid; gap: var(--cell-gap); width: max-content; }
	.cell {
		width: var(--size);
		height: var(--size);
		display: grid;
		place-items: center;
		padding: 0.35rem;
		border: 1px solid var(--border);
		border-radius: var(--radius);
		background: var(--surface);
		color: var(--text);
		font: inherit;
		font-size: 0.85rem;
		font-weight: 500;
		line-height: 1.2;
		text-align: center;
		overflow-wrap: anywhere;
		overflow: hidden;
		transition: background 0.15s, transform 0.1s;
	}
	button.cell:hover { border-color: var(--primary); background: var(--surface-muted); color: var(--text); }
	button.cell:active { transform: scale(0.96); }
	.free { background: var(--surface-muted); color: var(--text-muted); font-weight: 600; }
	.marked, button.marked:hover { background: var(--marked); border-color: var(--marked); color: var(--on-marked); }
	.won, button.won:hover { background: var(--won); border-color: var(--won); color: var(--on-won); }

	.small { --size: var(--cell-sm); gap: 3px; }
	.small .cell { padding: 0; border-radius: 4px; }
	.small .cell:not(.free) { background: var(--primary); border-color: var(--primary); opacity: 0.8; }
</style>
