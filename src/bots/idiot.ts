import { Bot } from "../bot";
import { UI } from "../uiglobal";

export class IdiotBot extends Bot {

	makeMove(board: number[]): Promise<{ x0: number, y0: number, x1: number, y1: number }> {

		return new Promise((resolve, reject) => {

			const moves = UI.Grid.callAllMoves(this.turn)

			const idx = Math.floor(Math.random() * moves.length) % moves.length

			resolve(moves[idx])

		})

	}

}