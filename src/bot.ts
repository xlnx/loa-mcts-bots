export abstract class Bot {

	constructor(public readonly turn: number) { }

	abstract makeMove(board: number[]): { x0: number, y0: number, x1: number, y1: number }

}