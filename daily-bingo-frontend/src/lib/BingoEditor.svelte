<script lang="ts">
	import { untrack } from 'svelte';
	import { isHttpError } from '@sveltejs/kit';
	import type { BingoInfo, Cell } from '$lib/bingo';

	const MAX = 10;

	let {
		initial = { cols: 5, rows: 5, values: Array(25).fill('Empty') },
		submitLabel,
		onsave
	}: {
		initial?: BingoInfo;
		submitLabel: string;
		onsave: (info: BingoInfo) => Promise<void>;
	} = $props();

	// Editor state starts from `initial` and is owned here afterwards.
	const start = untrack(() => initial);
	let cols = $state(start.cols);
	let rows = $state(start.rows);
	let values = $state<Cell[]>([...start.values]);
	let error = $state('');
	let saving = $state(false);

	/** Keeps each cell at its (row, col) position; new cells start Empty. */
	function resize(c: number, r: number) {
		if (!(c >= 1 && r >= 1)) return;
		c = Math.min(c, MAX);
		r = Math.min(r, MAX);
		values = Array.from({ length: r * c }, (_, i) => {
			const y = Math.floor(i / c);
			const x = i % c;
			return y < rows && x < cols ? values[y * cols + x] : 'Empty';
		});
		cols = c;
		rows = r;
	}

	function setText(i: number, text: string) {
		values[i] = text.trim() ? { Filled: text } : 'Empty';
	}

	async function submit(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		saving = true;
		try {
			await onsave({ cols, rows, values });
		} catch (err) {
			error = isHttpError(err) ? err.body.message : 'Could not reach the server';
		} finally {
			saving = false;
		}
	}
</script>

<form onsubmit={submit}>
	<div class="size">
		<label>
			Columns
			<input type="number" min="1" max={MAX} value={cols} oninput={(e) => resize(e.currentTarget.valueAsNumber, rows)} />
		</label>
		<label>
			Rows
			<input type="number" min="1" max={MAX} value={rows} oninput={(e) => resize(cols, e.currentTarget.valueAsNumber)} />
		</label>
	</div>

	<p class="hint">Blank cells are free spaces. ✕ turns a cell into a gap.</p>

	<div class="grid" style:grid-template-columns="repeat({cols}, 1fr)">
		{#each values as cell, i (i)}
			{#if cell === 'Gap'}
				<button type="button" class="cell gap" aria-label="Restore cell {i + 1}" onclick={() => (values[i] = 'Empty')}>+</button>
			{:else}
				<div class="cell">
					<textarea
						aria-label="Cell {i + 1}"
						placeholder="Free"
						value={cell === 'Empty' ? '' : cell.Filled}
						oninput={(e) => setText(i, e.currentTarget.value)}
					></textarea>
					<button type="button" class="remove" aria-label="Make cell {i + 1} a gap" onclick={() => (values[i] = 'Gap')}>✕</button>
				</div>
			{/if}
		{/each}
	</div>

	{#if error}<p role="alert">{error}</p>{/if}
	<p><button disabled={saving}>{submitLabel}</button></p>
</form>

<style>
	.size { display: flex; gap: 1rem; }
	.size input { width: 4rem; }
	.hint { color: #666; font-size: 0.85rem; }
	.grid { display: grid; gap: 4px; max-width: 36rem; }
	.cell { position: relative; aspect-ratio: 1; }
	.cell textarea {
		width: 100%; height: 100%; box-sizing: border-box; resize: none; padding: 0.25rem;
		border: 1px solid #999; border-radius: 4px; font: inherit; font-size: 0.85rem; text-align: center;
	}
	.remove {
		position: absolute; top: 2px; right: 2px; padding: 0 0.25rem; border: none;
		background: none; color: #999; cursor: pointer; font-size: 0.7rem;
	}
	.gap { border: 1px dashed #ccc; border-radius: 4px; background: none; color: #bbb; cursor: pointer; }
	[role='alert'] { color: crimson; }
</style>
