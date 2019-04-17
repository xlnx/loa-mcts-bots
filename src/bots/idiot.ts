import { Bot } from "../bot";
import { UI } from "../uiglobal";

export class IdiotBot extends Bot {

	makeMove(board: number[]) {

		const moves = UI.Grid.callAllMoves(this.turn)

		const idx = Math.floor(Math.random() * moves.length) % moves.length

		return moves[idx]

	}

}