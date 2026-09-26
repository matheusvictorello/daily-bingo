/** Serde's default enum encoding of the backend's BingoCellType. */
export type Cell = 'Gap' | 'Empty' | { Filled: string };
export type BingoInfo = { cols: number; rows: number; values: Cell[] };
export type Bingo = BingoInfo & { id: string };

/**
 * Rows, columns and (on square grids) diagonals whose cells are all marked.
 * Gaps are skipped; Empty cells count as free spaces.
 */
export function winningLines({ cols, rows, values }: BingoInfo, marked: boolean[]): number[][] {
	const line = (n: number, at: (i: number) => number) => Array.from({ length: n }, (_, i) => at(i));
	const lines = [
		...line(rows, (r) => r).map((r) => line(cols, (c) => r * cols + c)),
		...line(cols, (c) => c).map((c) => line(rows, (r) => r * cols + c))
	];
	if (rows === cols) lines.push(line(rows, (i) => i * cols + i), line(rows, (i) => i * cols + cols - 1 - i));

	return lines.filter((l) => {
		const cells = l.filter((i) => values[i] !== 'Gap');
		return l.length > 1 && cells.length > 0 && cells.every((i) => values[i] === 'Empty' || marked[i]);
	});
}
