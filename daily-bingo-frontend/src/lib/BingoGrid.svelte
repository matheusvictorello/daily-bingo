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

<div class="grid" class:small style:grid-template-columns="repeat({bingo.cols}, 1fr)">
	{#each bingo.values as cell, i}
		{#if cell === 'Gap'}
			<div></div>
		{:else if cell === 'Empty'}
			<div class="cell free" class:won={won.has(i)}>Free</div>
		{:else if ontoggle}
			<button
				class="cell"
				class:marked={marked[i]}
				class:won={won.has(i)}
				aria-pressed={!!marked[i]}
				onclick={() => ontoggle(i)}>{cell.Filled}</button
			>
		{:else}
			<div class="cell">{cell.Filled}</div>
		{/if}
	{/each}
</div>

<style>
	.grid { display: grid; gap: 4px; max-width: 36rem; }
	.cell {
		aspect-ratio: 1; display: grid; place-items: center; padding: 0.25rem;
		border: 1px solid #999; border-radius: 4px; background: none; font: inherit;
		font-size: 0.85rem; text-align: center; overflow-wrap: anywhere; overflow: hidden;
	}
	button.cell { cursor: pointer; }
	.free { background: #eee; color: #666; }
	.marked { background: #ffd54f; }
	.won { background: #66bb6a; color: white; }
	.small { max-width: 10rem; gap: 2px; }
	.small .cell { font-size: 0.5rem; padding: 1px; border-radius: 2px; }
</style>
