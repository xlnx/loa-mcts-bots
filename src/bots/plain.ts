import { Bot } from "../glob/bot";
import { my_plain_solution } from "../../pkg/ai_frontend";

export class PlainBot extends Bot {

	makeMove(board: number[]): Promise<{ x0: number, y0: number, x1: number, y1: number }> {

		return new Promise((resolve, reject) => {

			const res = my_plain_solution(this.turn, Int32Array.from(board))

			resolve({ x0: res.x0, y0: res.y0, x1: res.x1, y1: res.y1 })

		})

	}

}

Bot.register("plain bot", PlainBot)
